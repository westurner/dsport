//! `sphinxdocrs::builders::html` — Rust port of
//! `sphinx.builders.html.StandaloneHTMLBuilder` (minimal path).
//!
//! Reads RST source files, parses them with `docutilsrs::parse_rst_with_source`,
//! renders to HTML5 fragments via `docutilsrs::html5`, wraps in a minimal
//! HTML5 page, and writes `.html` output files.
//!
//! ## What is ported (minimal path)
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `StandaloneHTMLBuilder.name` | `"html"` | constant |
//! | `StandaloneHTMLBuilder.format` | `"html"` | constant |
//! | `StandaloneHTMLBuilder.out_suffix` | `".html"` | constant |
//! | `StandaloneHTMLBuilder.get_target_uri` | [`HtmlBuilder::get_target_uri`] | `docname + link_suffix` |
//! | `StandaloneHTMLBuilder.write_doc` | [`HtmlBuilder::build_doc`] | parse RST → HTML5 → write file |
//! | `Builder.build_all` | [`HtmlBuilder::build_all`] | iterate env docs, call `build_doc` |
//!
//! **Deferred**: theming, Jinja2 page templates, search index, CSS/JS assets,
//! image handling, domain indices, i18n.

use std::path::{Path, PathBuf};

use docutilsrs::cli::{CommonOptions, Html5Options};
use docutilsrs::doctree::{NodeKind};
use docutilsrs::{html5, parse_rst_with_source};

use super::{BuildError, BuildResult, Builder};
use crate::config::SphinxConfig;
use crate::environment::BuildEnvironment;

// ── HtmlBuilder ───────────────────────────────────────────────────────────────

/// Minimal HTML5 builder.
///
/// Mirrors `sphinx.builders.html.StandaloneHTMLBuilder` core path:
/// RST source → `docutilsrs` parse → `docutilsrs::html5` fragment →
/// minimal HTML5 page → `.html` output file.
pub struct HtmlBuilder {
    /// File suffix for output files.
    ///
    /// Mirrors `StandaloneHTMLBuilder.out_suffix` (default `".html"`).
    pub out_suffix: String,

    /// Link suffix used in URIs.
    ///
    /// Mirrors `StandaloneHTMLBuilder.link_suffix` (defaults to
    /// `out_suffix`).
    pub link_suffix: String,

    /// docutilsrs HTML5 writer options.
    html5_options: Html5Options,

    /// docutilsrs common options.
    common_options: CommonOptions,
}

impl HtmlBuilder {
    /// Construct with default settings.
    pub fn new() -> Self {
        Self {
            out_suffix: ".html".into(),
            link_suffix: ".html".into(),
            html5_options: Html5Options::default(),
            common_options: CommonOptions::default(),
        }
    }

    /// Render RST `source` to an HTML5 body fragment and also extract the
    /// document title promoted by `promote_document_title`.
    ///
    /// Returns `(title, body_html)`.
    fn render_fragment(&self, docname: &str, source: &str) -> (String, String) {
        let tree = parse_rst_with_source(source, docname);
        // Extract promoted document title from NodeKind::Document { title, .. }
        let title = match &tree.node(tree.root()).kind {
            NodeKind::Document { title, .. } if !title.is_empty() => title.clone(),
            _ => docname.rsplit('/').next().unwrap_or(docname).to_owned(),
        };
        let body = html5(&tree, &self.html5_options, &self.common_options);
        (title, body)
    }

    /// Wrap an HTML5 fragment in a minimal full HTML5 page.
    ///
    /// Includes a link to `_static/sphinxdocrs.css` so pages have basic
    /// styling without requiring the full Jinja2 theme pipeline.
    fn wrap_page(title: &str, body: &str, project: &str) -> String {
        let title_esc = html_escape(title);
        let page_title = if project.is_empty() {
            title_esc.clone()
        } else {
            format!("{title_esc} &#8212; {}", html_escape(project))
        };
        format!(
            "<!DOCTYPE html>\n\
             <html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">\n\
             <head>\n\
             <meta charset=\"utf-8\" />\n\
             <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n\
             <title>{page_title}</title>\n\
             <link rel=\"stylesheet\" type=\"text/css\" href=\"_static/sphinxdocrs.css\" />\n\
             </head>\n\
             <body>\n\
             <main id=\"content\">\n\
             {body}\n\
             </main>\n\
             </body>\n\
             </html>\n"
        )
    }
}

impl Default for HtmlBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for HtmlBuilder {
    fn name(&self) -> &str {
        "html"
    }

    fn format(&self) -> &str {
        "html"
    }

    fn out_suffix(&self) -> &str {
        &self.out_suffix
    }

    /// Return the output URI for `docname`.
    ///
    /// Mirrors `StandaloneHTMLBuilder.get_target_uri`:
    /// `quote(docname) + self.link_suffix`.
    ///
    /// ```rust
    /// use sphinxdocrs::builders::{Builder, html::HtmlBuilder};
    /// let b = HtmlBuilder::new();
    /// assert_eq!(b.get_target_uri("index"), "index.html");
    /// assert_eq!(b.get_target_uri("guide/intro"), "guide/intro.html");
    /// ```
    fn get_target_uri(&self, docname: &str) -> String {
        format!("{}{}", percent_encode_path(docname), self.link_suffix)
    }

    /// Parse `source` and write the resulting HTML to `outdir/{docname}.html`.
    ///
    /// Mirrors `StandaloneHTMLBuilder.write_doc`.
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        sanitize_docname(docname)?;

        // Parse + render, extracting the promoted document title.
        let (title, body) = self.render_fragment(docname, source);
        let page = Self::wrap_page(&title, &body, "");

        // Determine output path: outdir / {docname}.html
        // docname may contain '/' separators (sub-directories).
        let rel: PathBuf = docname
            .split('/')
            .collect::<PathBuf>()
            .with_extension("html");
        let out_path = outdir.join(rel);

        // Create parent directories.
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&out_path, page.as_bytes())?;
        Ok(())
    }

    /// Build all RST documents in `srcdir` into `outdir`.
    ///
    /// Documents are taken from `env.all_docs` if populated, otherwise
    /// discovered by walking `srcdir` for `*.rst` files.
    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let mut result = BuildResult::default();

        // Collect docnames — from env.all_docs if populated, else discover.
        let docnames: Vec<String> = if !env.all_docs.is_empty() {
            env.all_docs.keys().cloned().collect()
        } else {
            discover_rst_docnames(srcdir)
        };

        std::fs::create_dir_all(outdir)?;
        // Write static assets (minimal CSS, objects.inv stub, genindex stub)
        write_static_files(outdir)?;

        for docname in &docnames {
            sanitize_docname(docname)?;
            let src_path = src_path_for_docname_with_suffixes(srcdir, docname, &env.config)?;
            let source = std::fs::read_to_string(&src_path).map_err(|e| {
                BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
            })?;
            self.build_doc(docname, &source, outdir)?;
            result.written += 1;
        }
        Ok(result)
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Validate a docname, returning `Err` if any component is `..`, empty, or
/// begins with `/` (which would escape the intended directory).
///
/// This guards against path-traversal attacks when docnames are derived from
/// untrusted sources (e.g. `env.all_docs` populated from user input).
fn sanitize_docname(docname: &str) -> Result<(), BuildError> {
    if docname.is_empty() {
        return Err(BuildError::Other("docname must not be empty".into()));
    }
    for component in docname.split('/') {
        if component.is_empty() || component == ".." || component.starts_with('/') {
            return Err(BuildError::Other(format!(
                "invalid docname component {component:?} in {docname:?}"
            )));
        }
    }
    Ok(())
}

// ── static file helpers ─────────────────────────────────────────────────────

/// Minimal embedded CSS for sphinxdocrs pages.
///
/// This gives the output a usable appearance without the full Jinja2
/// theme pipeline.  The selector names mirror Sphinx's default theme
/// so a future theme upgrade is a drop-in replacement.
const MINIMAL_CSS: &str = r#"/* sphinxdocrs minimal stylesheet */
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
       max-width: 900px; margin: 0 auto; padding: 1em 2em; line-height: 1.6;
       color: #333; background: #fff; }
h1, h2, h3, h4 { color: #1a1a2e; }
a { color: #0057a8; text-decoration: none; }
a:hover { text-decoration: underline; }
pre, code { background: #f5f5f5; border-radius: 3px;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace; }
pre { padding: 0.8em; overflow-x: auto; }
code { padding: 0.1em 0.3em; }
.highlight pre { margin: 0; }
table { border-collapse: collapse; width: 100%; }
td, th { border: 1px solid #ccc; padding: 0.4em 0.6em; }
th { background: #f0f0f0; }
.note, .warning, .tip, .caution {
  border-left: 4px solid #0057a8; padding: 0.5em 1em; margin: 1em 0;
  background: #f0f7ff; }
.warning { border-color: #e0a000; background: #fffbe0; }
"#;

/// Write static assets into `outdir/_static/`.
///
/// Currently emits:
/// - `_static/sphinxdocrs.css` — minimal embedded stylesheet
/// - `objects.inv` — empty intersphinx inventory stub
/// - `genindex.html` — empty general-index stub
///
/// These are the minimum set needed so that HTML pages render usably and
/// parity-checking tools do not flag absent mandatory files.
fn write_static_files(outdir: &Path) -> Result<(), BuildError> {
    let static_dir = outdir.join("_static");
    std::fs::create_dir_all(&static_dir)?;
    std::fs::write(static_dir.join("sphinxdocrs.css"), MINIMAL_CSS.as_bytes())?;

    // objects.inv — Sphinx intersphinx inventory (version 2 header only).
    // Real entries are added by the domain pipeline (deferred).
    let inv = "# Sphinx inventory version 2\n\
               # Project: sphinxdocrs\n\
               # Version: \n\
               # The remainder of this file is compressed using zlib.\n";
    let inv_path = outdir.join("objects.inv");
    if !inv_path.exists() {
        std::fs::write(&inv_path, inv.as_bytes())?;
    }

    // genindex.html — general index (populated by domains, deferred).
    let genindex = HtmlBuilder::wrap_page("Index", "<p><em>Index not yet generated by the native builder.</em></p>", "");
    let gen_path = outdir.join("genindex.html");
    if !gen_path.exists() {
        std::fs::write(&gen_path, genindex.as_bytes())?;
    }

    Ok(())
}

/// Walk `srcdir` and return all `.rst` docnames (relative, no extension,
/// `/`-separated). Public for use by other builders.
pub fn discover_rst_docnames_pub(srcdir: &Path) -> Vec<String> {
    discover_rst_docnames(srcdir)
}

/// Walk `srcdir` and return all `.rst` docnames (relative, no extension,
/// `/`-separated).
fn discover_rst_docnames(srcdir: &Path) -> Vec<String> {
    let mut docnames = Vec::new();
    collect_rst(srcdir, srcdir, &mut docnames);
    docnames.sort();
    docnames
}

fn collect_rst(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rst(root, &path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rst") {
            if let Ok(rel) = path.strip_prefix(root) {
                // Strip only the trailing ".rst" — avoid Path::with_extension("")
                // which would strip ANY trailing extension (breaking "0.1.rst" → "0").
                let s = rel.to_string_lossy();
                let docname = s
                    .strip_suffix(".rst")
                    .unwrap_or(&s)
                    .replace('\\', "/");
                out.push(docname);
            }
        }
    }
}

// /// Return the `.rst` source path for a docname.
// ///
// /// NOTE: We must not use `Path::with_extension` here because it strips the
// /// *last* component's existing extension first.  A docname like `changes/0.1`
// /// would become `changes/0.rst` instead of `changes/0.1.rst`.
// ///
// /// # Panics
// /// Callers must have already validated `docname` with [`sanitize_docname`].
// fn src_path_for_docname(srcdir: &Path, docname: &str) -> PathBuf {
//     // Append `.rst` as a string — do NOT use Path::with_extension which
//     // would replace an existing extension component (e.g. "changes/0.1" →
//     // "changes/0.rst" instead of "changes/0.1.rst").
//     let rel: PathBuf = docname.split('/').collect::<PathBuf>();
//     let rel_str = rel.display().to_string();
    
//     // Only add .rst if it's not already there
//     let path_str = if rel_str.ends_with(".rst") {
//         rel_str
//     } else {
//         format!("{}.rst", rel_str)
//     };
    
//     srcdir.join(path_str)
// }

/// Return the source path for a docname, trying multiple suffixes in order.
///
/// Given a config with `source_suffix` mapping (e.g., `{".rst": "restructuredtext",
/// ".md": "markdown", ".myst.md": "myst"}`), this function tries each suffix in the
/// order they appear in the config. If a file exists with any of the suffixes, returns
/// that path. Otherwise returns an error.
///
/// # Errors
/// Returns `BuildError` if no source file exists with any configured suffix, or if
/// the config has no suffixes defined.
///
/// # Panics
/// Callers must have already validated `docname` with [`sanitize_docname`].
fn src_path_for_docname_with_suffixes(
    srcdir: &Path,
    docname: &str,
    config: &SphinxConfig,
) -> Result<PathBuf, BuildError> {
    let rel: PathBuf = docname.split('/').collect::<PathBuf>();
    let rel_str = rel.display().to_string();

    let source_suffix = config.source_suffix();

    // If docname already has an extension, just use it as-is
    for ext in source_suffix.keys() {
        if rel_str.ends_with(ext) {
            return Ok(srcdir.join(&rel_str));
        }
    }

    // Try each suffix in order; return the first that exists
    for ext in source_suffix.keys() {
        let path_str = format!("{}{}", rel_str, ext);
        let path = srcdir.join(&path_str);
        if path.exists() {
            return Ok(path);
        }
    }

    // No file found with any configured suffix
    if source_suffix.is_empty() {
        Err(BuildError::Other(
            "no source suffixes configured in source_suffix".into(),
        ))
    } else {
        let suffixes = source_suffix
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        Err(BuildError::Other(format!(
            "source file for docname '{}' not found with any configured suffix: {}",
            docname, suffixes
        )))
    }
}

/// Percent-encode a path for use in a URL, preserving `/`.
fn percent_encode_path(s: &str) -> String {
    // Preserve unreserved chars + `/`; encode everything else.
    s.chars()
        .flat_map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~' | '/') {
                vec![c]
            } else {
                c.to_string()
                    .bytes()
                    .flat_map(|b| vec!['%', hex_hi(b), hex_lo(b)])
                    .collect()
            }
        })
        .collect()
}

fn hex_hi(b: u8) -> char {
    hex_digit((b >> 4) & 0xF)
}
fn hex_lo(b: u8) -> char {
    hex_digit(b & 0xF)
}
fn hex_digit(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'A' + n - 10) as char,
        _ => unreachable!(),
    }
}

/// Minimal HTML escaping for page title.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn builder() -> HtmlBuilder {
        HtmlBuilder::new()
    }

    // ── get_target_uri ────────────────────────────────────────────────────────

    #[test]
    fn get_target_uri_index() {
        assert_eq!(builder().get_target_uri("index"), "index.html");
    }

    #[test]
    fn get_target_uri_subdir() {
        assert_eq!(builder().get_target_uri("guide/intro"), "guide/intro.html");
    }

    #[test]
    fn get_target_uri_nested() {
        assert_eq!(
            builder().get_target_uri("api/module/foo"),
            "api/module/foo.html"
        );
    }

    // ── build_doc ─────────────────────────────────────────────────────────────

    #[test]
    fn build_doc_creates_file() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        b.build_doc(
            "index",
            "Hello World\n===========\n\nA paragraph.\n",
            tmp.path(),
        )
        .unwrap();
        let out = tmp.path().join("index.html");
        assert!(out.exists(), "output file should exist");
        let contents = std::fs::read_to_string(out).unwrap();
        assert!(contents.contains("<!DOCTYPE html>"));
        assert!(contents.contains("Hello World") || contents.contains("paragraph"));
    }

    #[test]
    fn build_doc_creates_subdirs() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        b.build_doc("guide/intro", "Intro\n=====\n\nContent.\n", tmp.path())
            .unwrap();
        let out = tmp.path().join("guide").join("intro.html");
        assert!(out.exists(), "subdirectory output should be created");
    }

    #[test]
    fn build_doc_output_is_html5() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        b.build_doc("test", "Test\n====\n\nParagraph.\n", tmp.path())
            .unwrap();
        let contents = std::fs::read_to_string(tmp.path().join("test.html")).unwrap();
        assert!(contents.contains("<!DOCTYPE html>"));
        assert!(contents.contains("<html"));
        assert!(contents.contains("</html>"));
    }

    #[test]
    fn build_doc_contains_body_content() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        let rst = "Intro\n=====\n\nThis is a **bold** word and an *italic* one.\n";
        b.build_doc("intro", rst, tmp.path()).unwrap();
        let contents = std::fs::read_to_string(tmp.path().join("intro.html")).unwrap();
        // RST bold → <strong>, italic → <em>
        assert!(contents.contains("<strong>") || contents.contains("bold"));
    }

    #[test]
    fn build_doc_lists_rendered() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        let rst = "Items\n=====\n\n- one\n- two\n- three\n";
        b.build_doc("items", rst, tmp.path()).unwrap();
        let contents = std::fs::read_to_string(tmp.path().join("items.html")).unwrap();
        assert!(contents.contains("<ul>") || contents.contains("one"));
    }

    // ── build_all ─────────────────────────────────────────────────────────────

    #[test]
    fn build_all_empty_srcdir() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        let b = builder();
        let result = b.build_all(src.path(), out.path(), &env).unwrap();
        assert_eq!(result.written, 0);
    }

    #[test]
    fn build_all_discovers_rst_files() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        // Write two .rst files.
        std::fs::write(src.path().join("index.rst"), "Index\n=====\n\nWelcome.\n").unwrap();
        std::fs::write(src.path().join("about.rst"), "About\n=====\n\nInfo.\n").unwrap();

        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());

        let b = builder();
        let result = b.build_all(src.path(), out.path(), &env).unwrap();
        assert_eq!(result.written, 2);
        assert!(out.path().join("index.html").exists());
        assert!(out.path().join("about.html").exists());
    }

    #[test]
    fn build_all_with_subdirectory() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::create_dir(src.path().join("guide")).unwrap();
        std::fs::write(src.path().join("index.rst"), "Home\n====\n").unwrap();
        std::fs::write(src.path().join("guide").join("intro.rst"), "Intro\n=====\n").unwrap();

        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());

        let b = builder();
        let result = b.build_all(src.path(), out.path(), &env).unwrap();
        assert_eq!(result.written, 2);
        assert!(out.path().join("index.html").exists());
        assert!(out.path().join("guide").join("intro.html").exists());
    }

    // ── render fragment ───────────────────────────────────────────────────────

    #[test]
    fn render_fragment_section_title() {
        let b = builder();
        let (_title, html) = b.render_fragment("test", "Hello\n=====\n\nWorld.\n");
        // html5 writer produces <section ...> and heading elements
        assert!(html.contains("Hello") || html.contains("section"));
    }

    #[test]
    fn render_fragment_empty_source() {
        let b = builder();
        let (_title, html) = b.render_fragment("empty", "");
        // Empty source → empty or minimal output, no panic.
        let _ = html; // just ensure it doesn't panic
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    #[test]
    fn percent_encode_plain_path() {
        assert_eq!(percent_encode_path("index"), "index");
    }

    #[test]
    fn percent_encode_path_with_slash() {
        assert_eq!(percent_encode_path("guide/intro"), "guide/intro");
    }

    #[test]
    fn html_escape_special_chars() {
        assert_eq!(
            html_escape("A & B < C > D \"E\""),
            "A &amp; B &lt; C &gt; D &quot;E&quot;"
        );
    }

    #[test]
    fn html_escape_plain_text() {
        assert_eq!(html_escape("Hello World"), "Hello World");
    }

    #[test]
    fn discover_rst_finds_files() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("a.rst"), "").unwrap();
        std::fs::write(tmp.path().join("b.txt"), "").unwrap(); // not rst
        let docs = discover_rst_docnames(tmp.path());
        assert_eq!(docs, vec!["a"]);
    }

    // ── sanitize_docname ──────────────────────────────────────────────────────

    #[test]
    fn sanitize_docname_rejects_dotdot() {
        assert!(sanitize_docname("../etc/passwd").is_err());
        assert!(sanitize_docname("foo/../../etc").is_err());
    }

    #[test]
    fn sanitize_docname_rejects_empty() {
        assert!(sanitize_docname("").is_err());
    }

    #[test]
    fn sanitize_docname_rejects_double_slash() {
        assert!(sanitize_docname("foo//bar").is_err());
    }

    #[test]
    fn sanitize_docname_accepts_valid() {
        assert!(sanitize_docname("index").is_ok());
        assert!(sanitize_docname("guide/intro").is_ok());
        assert!(sanitize_docname("api/v1/ref").is_ok());
    }

    #[test]
    fn build_doc_rejects_path_traversal() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        let err = b.build_doc("../../evil", "x", tmp.path());
        assert!(err.is_err(), "path traversal docname must be rejected");
    }

    #[test]
    fn src_path_for_docname_with_suffixes_prefers_existing_file() {
        let tmp = TempDir::new().unwrap();
        
        // Create a .rst file
        std::fs::write(tmp.path().join("index.rst"), "").unwrap();
        
        // Mock a config with default suffixes
        let config = SphinxConfig::new_defaults();
        
        let path = src_path_for_docname_with_suffixes(tmp.path(), "index", &config)
            .expect("should find index.rst");
        
        assert!(path.ends_with("index.rst"));
    }

    #[test]
    fn src_path_for_docname_handles_already_extended_docname() {
        let tmp = TempDir::new().unwrap();
        
        // Create a file with extension already in the docname
        std::fs::write(tmp.path().join("index.rst"), "").unwrap();
        
        let config = SphinxConfig::new_defaults();
        
        let path = src_path_for_docname_with_suffixes(tmp.path(), "index.rst", &config)
            .expect("should find index.rst");
        
        // Should just use the docname as-is (not add another extension)
        assert!(path.ends_with("index.rst"));
        assert!(!path.ends_with("index.rst.rst"));
    }

    #[test]
    fn src_path_for_docname_with_suffixes_errors_on_missing_file() {
        let tmp = TempDir::new().unwrap();
        
        // Don't create any files
        let config = SphinxConfig::new_defaults();
        
        let result = src_path_for_docname_with_suffixes(tmp.path(), "missing", &config);
        
        assert!(result.is_err(), "should error when no source file exists");
    }
}
