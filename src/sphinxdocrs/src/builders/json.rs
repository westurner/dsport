//! `sphinxdocrs::builders::json` — Rust port of
//! `sphinxcontrib.serializinghtml.JSONHTMLBuilder`.
//!
//! Reads source files, parses them with `docutilsrs`,
//! renders to HTML5 fragments via `docutilsrs::html5`, then writes a
//! per-page `.fjson` file containing the serialized page context dict.
//! A `globalcontext.json` file is written by [`JsonBuilder::build_all`].
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `JSONHTMLBuilder.name` | `"json"` | constant |
//! | `JSONHTMLBuilder.format` | `"json"` | constant |
//! | `JSONHTMLBuilder.out_suffix` | `".fjson"` | constant |
//! | `JSONHTMLBuilder.globalcontext_filename` | `"globalcontext.json"` | constant |
//! | `JSONHTMLBuilder.searchindex_filename` | `"searchindex.json"` | constant |
//! | `SerializingHTMLBuilder.get_target_uri` | [`JsonBuilder::get_target_uri`] | `docname + "/"` (matches Python SEP) |
//! | `SerializingHTMLBuilder.handle_page` | [`JsonBuilder::build_doc`] | parse source → HTML5 body → `PageContext` → `.fjson` |
//! | `SerializingHTMLBuilder.handle_finish` | [`JsonBuilder::build_all`] | write `globalcontext.json` after all pages |
//! | `conf.py` `source_suffix` (list/dict) | [`JsonBuilder::source_suffixes`] | multiple extensions; mirrors Sphinx multi-suffix discovery |
//!
//! **Deferred**: Jinja2 templates, TOC tree, search index, CSS/JS assets,
//! image handling, domain indices, i18n, `_sources` copy.
//!
//! ## Multi-format projects
//!
//! [`source_suffixes`] defaults to `[".rst"]` but can be set to any list of
//! extensions, mirroring Sphinx's `source_suffix` config option.  `build_all`
//! discovers files matching **any** listed extension and records the actual
//! extension in each page's `sourcename` field.
//!
//! ```rust
//! use sphinxdocrs::builders::json::JsonBuilder;
//! let b = JsonBuilder::with_source_suffixes(vec![".rst".into(), ".md".into()]);
//! assert_eq!(b.source_suffixes, vec![".rst", ".md"]);
//! ```
//!
//! [`source_suffixes`]: JsonBuilder::source_suffixes

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use docutilsrs::cli::{CommonOptions, Html5Options};
use docutilsrs::{TitlePromotion, html5, parse_rst_with_options};
use serde::{Deserialize, Serialize};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// A [`serde_json::ser::Formatter`] matching Python's `json.dump(obj)`
/// default output exactly: a single line, `", "` between array/object
/// items and `": "` after each key, no indentation. `serde_json`'s built-in
/// `CompactFormatter` omits those spaces, and `PrettyFormatter` uses
/// multi-line indentation — neither matches real Sphinx's serializing
/// builders, which just call `json.dump` with no `indent`/`separators`
/// argument (verified against a real `sphinx-build -b json` run).
struct PyCompactFormatter;

impl serde_json::ser::Formatter for PyCompactFormatter {
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(if first { b"" } else { b", " })
    }

    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(if first { b"" } else { b", " })
    }

    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b": ")
    }
}

/// Serialize `value` as compact, single-line JSON using Python's default
/// `", "`/`": "` separators (see [`PyCompactFormatter`]).
fn to_py_json_writer<W: io::Write, T: Serialize + ?Sized>(
    writer: W,
    value: &T,
) -> serde_json::Result<()> {
    let mut ser = serde_json::Serializer::with_formatter(writer, PyCompactFormatter);
    value.serialize(&mut ser)
}

// ── Output data model ─────────────────────────────────────────────────────────

/// Per-page context written to `<docname>.fjson`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageContext {
    pub body: String,
    pub title: String,
    pub toc: String,
    pub display_toc: bool,
    pub current_page_name: String,
    #[serde(default)]
    pub parents: Vec<RelatedDoc>,
    pub prev: Option<RelatedDoc>,
    pub next: Option<RelatedDoc>,
    /// The copied-source link name (e.g. `"index.rst.txt"`), or `""` when
    /// `html_copy_source` is disabled. Mirrors upstream
    /// `StandaloneHTMLBuilder.write_doc`'s `sourcename` local exactly:
    /// `docname + source_suffix`, plus `html_sourcelink_suffix` appended
    /// unless it already equals `source_suffix`; empty when
    /// `html_copy_source` is `False`.
    pub sourcename: String,
    /// The document's raw source suffix (e.g. `".rst"`), independent of
    /// `html_copy_source`/`sourcename`. Mirrors upstream's `page_source_suffix`.
    pub page_source_suffix: String,
}

/// A related document link.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedDoc {
    pub link: String,
    pub title: String,
}

/// Global project context written to `globalcontext.json`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GlobalContext {
    pub project: String,
    pub copyright: String,
    pub release: String,
    pub version: String,
    pub builder: String,
    /// `None` unless `html_last_updated_fmt` is configured — matches
    /// upstream's default of not showing a last-updated date at all.
    pub last_updated: Option<String>,
    pub titles: HashMap<String, String>,
}

// ── JsonBuilder ───────────────────────────────────────────────────────────────

/// JSON serializing builder.
///
/// Mirrors `sphinxcontrib.serializinghtml.JSONHTMLBuilder` with multi-format
/// support: set [`source_suffixes`] to `[".rst", ".md"]` (or any combination)
/// to build mixed projects.
///
/// [`source_suffixes`]: JsonBuilder::source_suffixes
pub struct JsonBuilder {
    /// Per-page output suffix (`.fjson`). Mirrors `JSONHTMLBuilder.out_suffix`.
    pub out_suffix: String,
    /// Mirrors `JSONHTMLBuilder.globalcontext_filename`.
    pub globalcontext_filename: String,
    /// Mirrors `JSONHTMLBuilder.searchindex_filename`.
    pub searchindex_filename: String,
    /// Source file extensions to discover and build, each with a leading dot.
    ///
    /// Mirrors Sphinx's `source_suffix` conf.py option.
    /// `build_all` discovers files matching **any** suffix in this list and
    /// records the actual matched extension in each page's `sourcename`.
    /// `build_doc` (single-file API) uses the **first** suffix.
    ///
    /// Default: `[".rst"]`.
    pub source_suffixes: Vec<String>,
    html5_options: Html5Options,
    common_options: CommonOptions,
}

impl JsonBuilder {
    /// Construct with default RST-only settings.
    pub fn new() -> Self {
        Self {
            out_suffix: ".fjson".into(),
            globalcontext_filename: "globalcontext.json".into(),
            searchindex_filename: "searchindex.json".into(),
            source_suffixes: vec![".rst".into()],
            html5_options: Html5Options::default(),
            common_options: CommonOptions::default(),
        }
    }

    /// Construct with an explicit list of source suffixes.
    ///
    /// ```rust
    /// use sphinxdocrs::builders::json::JsonBuilder;
    /// let b = JsonBuilder::with_source_suffixes(vec![".rst".into(), ".md".into()]);
    /// assert_eq!(b.source_suffixes, vec![".rst", ".md"]);
    /// ```
    pub fn with_source_suffixes(suffixes: Vec<String>) -> Self {
        assert!(!suffixes.is_empty(), "source_suffixes must not be empty");
        Self {
            source_suffixes: suffixes,
            ..Self::new()
        }
    }

    fn render_body(&self, docname: &str, source: &str) -> String {
        let tree = parse_rst_with_options(source, docname, TitlePromotion::Preserve);
        html5(&tree, &self.html5_options, &self.common_options)
    }

    fn extract_title(docname: &str, source: &str) -> String {
        let lines: Vec<&str> = source.lines().collect();
        for i in 0..lines.len().saturating_sub(1) {
            let candidate = lines[i].trim();
            let underline = lines[i + 1].trim();
            if !candidate.is_empty()
                && !underline.is_empty()
                && underline.chars().all(|c| "=-~^\"'`#+*:.<>_".contains(c))
                && underline.len() >= candidate.len()
            {
                return candidate.to_string();
            }
        }
        docname.to_string()
    }

    /// Build a minimal TOC `<ul>` from section headings found in `source`.
    ///
    /// Full toctree support is deferred; this covers the single-page case.
    fn build_toc(source: &str, target_uri: &str) -> (String, bool) {
        let lines: Vec<&str> = source.lines().collect();
        let mut entries: Vec<String> = Vec::new();
        for i in 0..lines.len().saturating_sub(1) {
            let candidate = lines[i].trim();
            let underline = lines[i + 1].trim();
            if !candidate.is_empty()
                && !underline.is_empty()
                && underline.chars().all(|c| "=-~^\"'`#+*:.<>_".contains(c))
                && underline.len() >= candidate.len()
            {
                entries.push(format!(
                    r#"<li><a href="{}">{}</a></li>"#,
                    html_escape(target_uri),
                    html_escape(candidate),
                ));
            }
        }
        let display_toc = entries.len() > 1;
        let toc = if entries.is_empty() {
            String::new()
        } else {
            format!("<ul>\n{}\n</ul>", entries.join("\n"))
        };
        (toc, display_toc)
    }

    /// Write `globalcontext.json` into `outdir`.
    fn write_globalcontext(
        &self,
        outdir: &Path,
        env: &BuildEnvironment,
        titles: HashMap<String, String>,
    ) -> Result<(), BuildError> {
        let ctx = GlobalContext {
            project: env.config.project(),
            copyright: env
                .config
                .get("copyright")
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default(),
            release: env.config.release(),
            version: env.config.version(),
            builder: "json".into(),
            last_updated: env.config.html_last_updated_fmt(),
            titles,
        };
        let mut value = serde_json::to_value(&ctx)
            .map_err(|e| BuildError::Other(format!("globalcontext serialization failed: {e}")))?;
        if let serde_json::Value::Object(fields) = &mut value {
            fields.insert("embedded".into(), false.into());
            fields.insert("master_doc".into(), env.config.root_doc().into());
            fields.insert("root_doc".into(), env.config.root_doc().into());
            fields.insert(
                "use_opensearch".into(),
                env.config.html_use_opensearch().into(),
            );
            fields.insert("docstitle".into(), env.config.html_title().into());
            fields.insert("shorttitle".into(), env.config.html_short_title().into());
            fields.insert(
                "show_copyright".into(),
                env.config.html_show_copyright().into(),
            );
            fields.insert(
                "show_search_summary".into(),
                env.config.html_show_search_summary().into(),
            );
            fields.insert("show_sphinx".into(), env.config.html_show_sphinx().into());
            fields.insert("has_source".into(), env.config.html_copy_source().into());
            fields.insert(
                "show_source".into(),
                env.config.html_show_sourcelink().into(),
            );
            fields.insert(
                "sourcelink_suffix".into(),
                env.config.html_sourcelink_suffix().into(),
            );
            fields.insert("file_suffix".into(), self.out_suffix.clone().into());
            fields.insert("link_suffix".into(), ".html".into());
            fields.insert("language".into(), env.config.language().into());
            fields.insert("sphinx_version".into(), "9.1.1+/29cde4fd3".into());
            fields.insert(
                "sphinx_version_tuple".into(),
                serde_json::json!([9, 1, 1, "beta", 0]),
            );
            fields.insert(
                "docutils_version_info".into(),
                serde_json::json!([0, 22, 4, "final", 0]),
            );
            let (css_files, script_files) = static_asset_names(outdir);
            let styles = css_files
                .iter()
                .filter(|name| !name.ends_with("pygments.css"))
                .map(|name| name.trim_start_matches("_static/").to_string())
                .collect::<Vec<_>>();
            fields.insert("styles".into(), serde_json::json!(styles));
            fields.insert("parents".into(), serde_json::json!([]));
            fields.insert("css_files".into(), serde_json::json!(css_files));
            fields.insert("script_files".into(), serde_json::json!(script_files));
            fields.insert("logo_url".into(), "".into());
            fields.insert(
                "logo_alt".into(),
                format!("Logo of {}", env.config.project()).into(),
            );
            fields.insert("favicon_url".into(), "".into());
            fields.insert("html5_doctype".into(), true.into());
            fields.insert(
                "rellinks".into(),
                if env.config.html_use_index() {
                    serde_json::json!([["genindex", "General Index", "I", "index"]])
                } else {
                    serde_json::json!([])
                },
            );
            if let Some((_, theme_options, _, _)) = crate::theme_static::resolve_theme_templates(
                &env.config.html_theme(),
                &env.srcdir,
                &env.config.html_theme_path(),
                env.config.registered_themes(),
            ) {
                for (key, option) in theme_options {
                    fields.insert(key, option.into());
                }
            }
            fields.insert("theme_nosidebar".into(), "false".into());
        }

        let path = outdir.join(&self.globalcontext_filename);
        let file = std::fs::File::create(&path).map_err(BuildError::Io)?;
        to_py_json_writer(file, &value)
            .map_err(|e| BuildError::Other(format!("globalcontext serialization failed: {e}")))?;
        Ok(())
    }

    /// Core per-page writer.  `source_suffix` is recorded verbatim in
    /// `page_source_suffix`; `sourcename` follows upstream's
    /// `html_copy_source`-gated formula (see [`PageContext::sourcename`]).
    fn write_page(
        &self,
        docname: &str,
        source: &str,
        outdir: &Path,
        source_suffix: &str,
        config: &crate::config::SphinxConfig,
        extra: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), BuildError> {
        let body = self.render_body(docname, source);
        let title = Self::extract_title(docname, source);
        let target_uri = self.get_target_uri(docname);
        let (toc, display_toc) = Self::build_toc(source, &target_uri);

        let sourcename = if config.html_copy_source() {
            let sourcelink_suffix = config.html_sourcelink_suffix();
            let mut name = format!("{docname}{source_suffix}");
            if source_suffix != sourcelink_suffix {
                name.push_str(&sourcelink_suffix);
            }
            name
        } else {
            String::new()
        };

        let ctx = PageContext {
            body,
            title: html_escape(&title),
            toc,
            display_toc,
            current_page_name: docname.to_string(),
            parents: Vec::new(),
            prev: None,
            next: None,
            sourcename,
            page_source_suffix: source_suffix.to_string(),
        };

        let rel: PathBuf = docname
            .split('/')
            .collect::<PathBuf>()
            .with_extension("fjson");
        let out_path = outdir.join(rel);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut value = serde_json::to_value(&ctx)
            .map_err(|e| BuildError::Other(format!("page serialization failed: {e}")))?;
        if let serde_json::Value::Object(fields) = &mut value {
            fields.extend(extra.clone());
        }
        let file = std::fs::File::create(&out_path).map_err(BuildError::Io)?;
        to_py_json_writer(file, &value)
            .map_err(|e| BuildError::Other(format!("page serialization failed: {e}")))?;
        Ok(())
    }

    /// Resolve effective suffixes: `env.project.source_suffix` takes priority.
    fn effective_suffixes<'a>(&'a self, env: &'a BuildEnvironment) -> Vec<&'a str> {
        if !env.project.source_suffix.is_empty() {
            env.project
                .source_suffix
                .iter()
                .map(|(ext, _)| ext.as_str())
                .collect()
        } else {
            self.source_suffixes.iter().map(String::as_str).collect()
        }
    }
}

impl Default for JsonBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for JsonBuilder {
    fn name(&self) -> &str {
        "json"
    }
    fn format(&self) -> &str {
        "json"
    }
    fn out_suffix(&self) -> &str {
        &self.out_suffix
    }

    /// Return the output URI for `docname`.
    ///
    /// ```rust
    /// use sphinxdocrs::builders::{Builder, json::JsonBuilder};
    /// let b = JsonBuilder::new();
    /// assert_eq!(b.get_target_uri("index"), "");
    /// assert_eq!(b.get_target_uri("guide/index"), "guide/");
    /// assert_eq!(b.get_target_uri("guide/intro"), "guide/intro/");
    /// ```
    fn get_target_uri(&self, docname: &str) -> String {
        if docname == "index" {
            String::new()
        } else if docname.ends_with("/index") {
            docname[..docname.len() - 5].to_string()
        } else {
            format!("{}/", docname)
        }
    }

    /// Write a single page using the first configured suffix for `sourcename`.
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let suffix = self
            .source_suffixes
            .first()
            .map(String::as_str)
            .unwrap_or(".rst");
        self.write_page(
            docname,
            source,
            outdir,
            suffix,
            &crate::config::SphinxConfig::new_defaults(),
            &serde_json::Map::new(),
        )
    }

    /// Build all source documents, honouring all configured `source_suffixes`.
    ///
    /// Discovery finds files matching **any** suffix in order; if both
    /// `index.rst` and `index.md` exist, the first configured suffix wins.
    /// Each page's `sourcename` reflects its actual file extension.
    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let suffixes = self.effective_suffixes(env);
        let mut result = BuildResult::default();

        let mut docs: Vec<(String, String)> = if !env.all_docs.is_empty() {
            env.all_docs
                .keys()
                .map(|docname| {
                    let suffix = suffixes
                        .iter()
                        .find(|&&ext| src_path_for_docname(srcdir, docname, ext).exists())
                        .copied()
                        .unwrap_or_else(|| suffixes.first().copied().unwrap_or(".rst"));
                    (docname.clone(), suffix.to_string())
                })
                .collect()
        } else {
            discover_sources(srcdir, &suffixes)
        };
        docs.sort_by(|left, right| left.0.cmp(&right.0));

        std::fs::create_dir_all(outdir)?;

        // JSONHTMLBuilder inherits the HTML asset pipeline. Keep the
        // artifact set useful to consumers even though no HTML page is
        // rendered by this builder.
        super::html::write_static_files(outdir, super::html::PathStyle::Flat, false)?;
        super::html::copy_html_static_path(srcdir, outdir, &env.config)?;
        crate::theme_static::copy_theme_static_files_for_builder(
            &env.config,
            outdir,
            srcdir,
            "json",
        )
        .map_err(BuildError::Io)?;

        // The embedded stylesheet is only a fallback for native HTML pages;
        // it is not part of the upstream JSON builder's asset set.
        if outdir.join("_static/basic.css").exists() {
            let _ = std::fs::remove_file(outdir.join("_static/sphinxdocrs.css"));
        }

        let mut titles: HashMap<String, String> = HashMap::new();
        let mut sources = Vec::with_capacity(docs.len());
        for (docname, suffix) in &docs {
            let src_path = src_path_for_docname(srcdir, docname, suffix);
            let source =
                crate::environment::read_source_file(&src_path, &env.config.source_encoding())
                    .map_err(|e| {
                        BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
                    })?;
            let title = Self::extract_title(docname, &source);
            titles.insert(docname.clone(), html_escape(&title));
            sources.push(source);
        }

        let navigation_docs = navigation_docnames(env, &docs);
        for (index, (docname, suffix)) in docs.iter().enumerate() {
            let source = &sources[index];
            let extra = page_context_extras(docname, &navigation_docs, &titles);
            self.write_page(docname, source, outdir, suffix, &env.config, &extra)?;
            result.written += 1;
        }

        self.write_globalcontext(outdir, env, titles)?;

        let docnames: Vec<String> = docs.iter().map(|(name, _)| name.clone()).collect();
        if let Err(error) =
            crate::search::SearchIndex::build_and_write_json_with_env(env, srcdir, outdir)
        {
            return Err(BuildError::Other(format!(
                "searchindex.json serialization failed: {error}"
            )));
        }
        write_json_inventory(env, outdir, &docnames)?;
        write_search_page(outdir)?;
        std::fs::File::create(outdir.join("last_build"))?;
        Ok(result)
    }
}

fn page_context_extras(
    docname: &str,
    docnames: &[String],
    titles: &HashMap<String, String>,
) -> serde_json::Map<String, serde_json::Value> {
    let related = |name: &str| {
        serde_json::json!({
            "link": JsonBuilder::new().get_target_uri(name),
            "title": titles.get(name).cloned().unwrap_or_else(|| name.to_string()),
        })
    };
    let mut extra = serde_json::Map::new();
    extra.insert("meta".into(), serde_json::Value::Null);
    extra.insert(
        "metatags".into(),
        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n".into(),
    );
    extra.insert("has_maths_elements".into(), false.into());
    extra.insert("rellinks".into(), serde_json::json!([]));
    extra.insert(
        "sidebars".into(),
        serde_json::json!([
            "localtoc.html",
            "relations.html",
            "sourcelink.html",
            "searchbox.html"
        ]),
    );
    let index = docnames.iter().position(|name| name == docname);
    let mut rellinks = Vec::new();
    if let Some(index) = index {
        if let Some(name) = index.checked_sub(1).and_then(|i| docnames.get(i)) {
            rellinks.push(serde_json::json!([
                name,
                titles.get(name).cloned().unwrap_or_else(|| name.clone()),
                "P",
                "previous"
            ]));
        }
        if let Some(name) = docnames.get(index + 1) {
            rellinks.push(serde_json::json!([
                name,
                titles.get(name).cloned().unwrap_or_else(|| name.clone()),
                "N",
                "next"
            ]));
        }
    }
    extra.insert("rellinks".into(), serde_json::Value::Array(rellinks));
    extra.insert("parents".into(), serde_json::json!([]));
    extra.insert(
        "prev".into(),
        index
            .and_then(|i| i.checked_sub(1))
            .and_then(|i| docnames.get(i))
            .map_or(serde_json::Value::Null, |name| related(name)),
    );
    extra.insert(
        "next".into(),
        index
            .and_then(|i| docnames.get(i + 1))
            .map_or(serde_json::Value::Null, |name| related(name)),
    );
    extra
}

fn navigation_docnames(env: &BuildEnvironment, docs: &[(String, String)]) -> Vec<String> {
    let mut ordered = Vec::with_capacity(docs.len());
    let root = env.config.root_doc();
    if docs.iter().any(|(docname, _)| docname == &root) {
        ordered.push(root);
    }
    let mut rest = docs
        .iter()
        .map(|(docname, _)| docname.clone())
        .filter(|docname| !ordered.contains(docname))
        .collect::<Vec<_>>();
    rest.sort();
    ordered.extend(rest);
    ordered
}

fn static_asset_names(outdir: &Path) -> (Vec<String>, Vec<String>) {
    let mut css = Vec::new();
    let mut js = Vec::new();
    let Ok(entries) = std::fs::read_dir(outdir.join("_static")) else {
        return (css, js);
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if path.is_file() && name.ends_with(".css") {
            css.push(format!("_static/{name}"));
        } else if path.is_file() && name.ends_with(".js") {
            js.push(format!("_static/{name}"));
        }
    }
    css.sort_by_key(|name| (!name.ends_with("pygments.css"), name.clone()));
    let js_order = [
        "documentation_options.js",
        "doctools.js",
        "sphinx_highlight.js",
    ];
    js.retain(|name| js_order.iter().any(|expected| name.ends_with(expected)));
    js.sort_by_key(|name| {
        js_order
            .iter()
            .position(|expected| name.ends_with(expected))
            .unwrap_or(js_order.len())
    });
    (css, js)
}

fn write_json_inventory(
    env: &BuildEnvironment,
    outdir: &Path,
    docnames: &[String],
) -> Result<(), BuildError> {
    let builder = JsonBuilder::new();
    let mut entries = env
        .domain_objects()
        .into_iter()
        .map(|(domain, object)| crate::intersphinx::InventoryEntry {
            name: object.name.clone(),
            item_type: format!("{domain}:{}", object.obj_type),
            priority: 0,
            uri: {
                let target = builder.get_target_uri(&object.docname);
                if object.anchor.is_empty() {
                    target
                } else {
                    format!("{target}#{}", object.anchor)
                }
            },
            display_name: object.name,
        })
        .collect::<Vec<_>>();
    entries.extend(
        docnames
            .iter()
            .map(|docname| crate::intersphinx::InventoryEntry {
                name: docname.clone(),
                item_type: "std:doc".into(),
                priority: -1,
                uri: builder.get_target_uri(docname),
                display_name: env.titles.get(docname).cloned().unwrap_or_default(),
            }),
    );
    entries.extend(
        [
            ("genindex", "genindex", "Index"),
            ("modindex", "py-modindex", "Module Index"),
            ("py-modindex", "py-modindex", "Python Module Index"),
            ("search", "search", "Search Page"),
        ]
        .into_iter()
        .map(
            |(name, docname, display_name)| crate::intersphinx::InventoryEntry {
                name: name.into(),
                item_type: "std:label".into(),
                priority: -1,
                uri: builder.get_target_uri(docname),
                display_name: display_name.into(),
            },
        ),
    );
    entries.sort_by(|left, right| {
        (&left.name, &left.item_type, &left.uri).cmp(&(&right.name, &right.item_type, &right.uri))
    });
    let content =
        crate::intersphinx::dumps(&env.config.project(), &env.config.version(), &entries)?;
    std::fs::write(outdir.join("objects.inv"), content)?;
    Ok(())
}

fn write_search_page(outdir: &Path) -> Result<(), BuildError> {
    let context = serde_json::json!({
        "current_page_name": "search",
        "sidebars": ["localtoc.html", "relations.html", "sourcelink.html", "searchbox.html"]
    });
    let file = std::fs::File::create(outdir.join("search.fjson"))?;
    to_py_json_writer(file, &context)
        .map_err(|e| BuildError::Other(format!("search page serialization failed: {e}")))?;
    Ok(())
}

// ── free helpers ──────────────────────────────────────────────────────────────

/// Walk `srcdir` and return `(docname, matched_suffix)` pairs for every file
/// whose extension is in `suffixes`.  If multiple suffixes match the same
/// docname (e.g. both `index.rst` and `index.md` exist), the suffix that
/// appears **earlier** in `suffixes` wins — regardless of filesystem walk order.
/// This mirrors Sphinx's `source_suffix` priority behaviour.
fn discover_sources(srcdir: &Path, suffixes: &[&str]) -> Vec<(String, String)> {
    let exts: Vec<&str> = suffixes.iter().map(|s| s.trim_start_matches('.')).collect();
    // Collect all (docname, suffix_index) without deduplication first.
    let mut raw: Vec<(String, usize)> = Vec::new();
    collect_sources(srcdir, srcdir, &exts, &mut raw);

    // Deduplicate: for each docname, keep the entry with the lowest suffix index.
    let mut best: HashMap<String, usize> = HashMap::new();
    for (docname, idx) in &raw {
        let entry = best.entry(docname.clone()).or_insert(*idx);
        if *idx < *entry {
            *entry = *idx;
        }
    }

    let mut results: Vec<(String, String)> = best
        .into_iter()
        .map(|(docname, idx)| (docname, suffixes[idx].to_string()))
        .collect();
    results.sort_by(|a, b| a.0.cmp(&b.0));
    results
}

fn collect_sources(root: &Path, dir: &Path, exts: &[&str], out: &mut Vec<(String, usize)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_sources(root, &path, exts, out);
        } else if let Some(file_ext) = path.extension().and_then(|s| s.to_str()) {
            if let Some(idx) = exts.iter().position(|&e| e == file_ext) {
                if let Ok(rel) = path.strip_prefix(root) {
                    let docname = rel.with_extension("").to_string_lossy().replace('\\', "/");
                    out.push((docname, idx));
                }
            }
        }
    }
}

fn src_path_for_docname(srcdir: &Path, docname: &str, source_suffix: &str) -> PathBuf {
    // Use string append, not with_extension — the latter strips any existing
    // dot in the final component (e.g. docname "changes/0.1" with ext ".rst"
    // would yield "changes/0.rst" instead of "changes/0.1.rst").
    let ext = source_suffix.trim_start_matches('.');
    srcdir.join(format!("{docname}.{ext}"))
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// ── inline unit tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ── builder meta ─────────────────────────────────────────────────────────

    #[test]
    fn builder_name_is_json() {
        assert_eq!(JsonBuilder::new().name(), "json");
    }
    #[test]
    fn builder_format_is_json() {
        assert_eq!(JsonBuilder::new().format(), "json");
    }
    #[test]
    fn builder_out_suffix_is_fjson() {
        assert_eq!(JsonBuilder::new().out_suffix(), ".fjson");
    }

    #[test]
    fn source_suffixes_default_is_rst() {
        assert_eq!(JsonBuilder::new().source_suffixes, vec![".rst"]);
    }

    #[test]
    fn with_source_suffixes_stores_all() {
        let b = JsonBuilder::with_source_suffixes(vec![".rst".into(), ".md".into(), ".txt".into()]);
        assert_eq!(b.source_suffixes, vec![".rst", ".md", ".txt"]);
    }

    // ── get_target_uri ────────────────────────────────────────────────────────

    #[test]
    fn get_target_uri_index() {
        assert_eq!(JsonBuilder::new().get_target_uri("index"), "");
    }
    #[test]
    fn get_target_uri_subdir_index() {
        assert_eq!(JsonBuilder::new().get_target_uri("guide/index"), "guide/");
    }
    #[test]
    fn get_target_uri_plain() {
        assert_eq!(
            JsonBuilder::new().get_target_uri("guide/intro"),
            "guide/intro/"
        );
    }

    // ── extract_title ─────────────────────────────────────────────────────────

    #[test]
    fn extract_title_from_rst() {
        assert_eq!(
            JsonBuilder::extract_title("index", "My Title\n========\n\nContent.\n"),
            "My Title"
        );
    }
    #[test]
    fn extract_title_fallback_to_docname() {
        assert_eq!(
            JsonBuilder::extract_title("guide/intro", "No underline here.\n"),
            "guide/intro"
        );
    }

    // ── build_toc ─────────────────────────────────────────────────────────────

    #[test]
    fn build_toc_empty_for_no_sections() {
        let (toc, display) = JsonBuilder::build_toc("No sections.", "index/");
        assert!(toc.is_empty());
        assert!(!display);
    }
    #[test]
    fn build_toc_single_section_not_display() {
        let (toc, display) = JsonBuilder::build_toc("Title\n=====\n", "");
        assert!(toc.contains("<ul>"));
        assert!(!display);
    }
    #[test]
    fn build_toc_multiple_sections_display() {
        let (toc, display) = JsonBuilder::build_toc("Title\n=====\n\nSection\n-------\n", "");
        assert!(display);
        assert!(toc.contains("<ul>"));
    }

    // ── build_doc sourcename ──────────────────────────────────────────────────

    #[test]
    fn build_doc_sourcename_uses_first_suffix() {
        let tmp = TempDir::new().unwrap();
        JsonBuilder::new()
            .build_doc("index", "Title\n=====\n\nContent.\n", tmp.path())
            .unwrap();
        let raw = std::fs::read_to_string(tmp.path().join("index.fjson")).unwrap();
        let ctx: PageContext = serde_json::from_str(&raw).unwrap();
        // Real Sphinx: sourcename = docname + source_suffix, plus
        // html_sourcelink_suffix (".txt" by default) appended unless it
        // already equals source_suffix.
        assert_eq!(ctx.sourcename, "index.rst.txt");
        assert_eq!(ctx.page_source_suffix, ".rst");
    }

    #[test]
    fn build_doc_sourcename_uses_md_when_configured() {
        let tmp = TempDir::new().unwrap();
        JsonBuilder::with_source_suffixes(vec![".md".into()])
            .build_doc("readme", "Title\n=====\n\nContent.\n", tmp.path())
            .unwrap();
        let raw = std::fs::read_to_string(tmp.path().join("readme.fjson")).unwrap();
        let ctx: PageContext = serde_json::from_str(&raw).unwrap();
        assert_eq!(ctx.sourcename, "readme.md.txt");
        assert_eq!(ctx.page_source_suffix, ".md");
    }

    #[test]
    fn write_page_sourcename_is_empty_when_html_copy_source_disabled() {
        let tmp = TempDir::new().unwrap();
        let mut raw_config = HashMap::new();
        raw_config.insert(
            "html_copy_source".to_string(),
            crate::config::ConfigVal::Bool(false),
        );
        let config = crate::config::SphinxConfig::new(raw_config, HashMap::new());
        JsonBuilder::new()
            .write_page(
                "index",
                "Title\n=====\n\nContent.\n",
                tmp.path(),
                ".rst",
                &config,
                &serde_json::Map::new(),
            )
            .unwrap();
        let raw = std::fs::read_to_string(tmp.path().join("index.fjson")).unwrap();
        let ctx: PageContext = serde_json::from_str(&raw).unwrap();
        assert_eq!(ctx.sourcename, "");
        assert_eq!(ctx.page_source_suffix, ".rst");
    }

    // ── discover_sources ──────────────────────────────────────────────────────

    fn write_file(dir: &Path, rel: &str, content: &str) {
        let path = dir.join(rel);
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn discover_sources_rst_only() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "index.rst", "");
        write_file(tmp.path(), "guide/intro.rst", "");
        let docs = discover_sources(tmp.path(), &[".rst"]);
        let map: HashMap<_, _> = docs.into_iter().collect();
        assert_eq!(map.get("index").map(String::as_str), Some(".rst"));
        assert_eq!(map.get("guide/intro").map(String::as_str), Some(".rst"));
    }

    #[test]
    fn discover_sources_md_only() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "index.md", "");
        write_file(tmp.path(), "notes.md", "");
        let docs = discover_sources(tmp.path(), &[".md"]);
        let map: HashMap<_, _> = docs.into_iter().collect();
        assert_eq!(map.get("index").map(String::as_str), Some(".md"));
        assert_eq!(map.get("notes").map(String::as_str), Some(".md"));
    }

    #[test]
    fn discover_sources_mixed_rst_and_md() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "index.rst", "");
        write_file(tmp.path(), "guide.md", "");
        write_file(tmp.path(), "api/module.rst", "");
        let docs = discover_sources(tmp.path(), &[".rst", ".md"]);
        let map: HashMap<_, _> = docs.into_iter().collect();
        assert_eq!(map.get("index").map(String::as_str), Some(".rst"));
        assert_eq!(map.get("guide").map(String::as_str), Some(".md"));
        assert_eq!(map.get("api/module").map(String::as_str), Some(".rst"));
    }

    #[test]
    fn discover_sources_first_suffix_wins_on_conflict() {
        let tmp = TempDir::new().unwrap();
        write_file(tmp.path(), "index.rst", "");
        write_file(tmp.path(), "index.md", "");
        let docs = discover_sources(tmp.path(), &[".rst", ".md"]);
        let map: HashMap<_, _> = docs.into_iter().collect();
        assert_eq!(
            map.get("index").map(String::as_str),
            Some(".rst"),
            "first suffix wins on docname conflict"
        );
    }

    // ── serde round-trips ─────────────────────────────────────────────────────

    #[test]
    fn page_context_round_trips() {
        let ctx = PageContext {
            body: "<p>Hello</p>".into(),
            title: "Hello".into(),
            toc: "<ul><li>Hello</li></ul>".into(),
            display_toc: false,
            current_page_name: "index".into(),
            parents: Vec::new(),
            prev: None,
            next: None,
            sourcename: "index.rst.txt".into(),
            page_source_suffix: ".rst".into(),
        };
        let back: PageContext =
            serde_json::from_str(&serde_json::to_string(&ctx).unwrap()).unwrap();
        assert_eq!(ctx, back);
    }

    #[test]
    fn global_context_round_trips() {
        let ctx = GlobalContext {
            project: "My Docs".into(),
            copyright: "2024 Author".into(),
            release: "1.0.0".into(),
            version: "1.0".into(),
            builder: "json".into(),
            last_updated: None,
            titles: HashMap::from([("index".into(), "Welcome".into())]),
        };
        let back: GlobalContext =
            serde_json::from_str(&serde_json::to_string(&ctx).unwrap()).unwrap();
        assert_eq!(ctx, back);
    }

    // ── PyCompactFormatter ────────────────────────────────────────────────────

    #[test]
    fn to_py_json_writer_matches_python_json_dump_style() {
        let ctx = PageContext {
            body: "<p>Hi</p>".into(),
            title: "Hi".into(),
            toc: String::new(),
            display_toc: false,
            current_page_name: "index".into(),
            parents: vec![RelatedDoc {
                link: "../".into(),
                title: "Up".into(),
            }],
            prev: None,
            next: None,
            sourcename: "index.rst.txt".into(),
            page_source_suffix: ".rst".into(),
        };
        let mut buf = Vec::new();
        to_py_json_writer(&mut buf, &ctx).unwrap();
        let out = String::from_utf8(buf).unwrap();
        // Single line: no embedded newlines, and Python-style `, `/`: ` separators.
        assert!(!out.contains('\n'));
        assert!(out.starts_with("{\"body\": \"<p>Hi</p>\", \"title\": \"Hi\""));
        assert!(out.contains("\"link\": \"../\", \"title\": \"Up\""));
    }

    // ── html_escape ───────────────────────────────────────────────────────────

    #[test]
    fn html_escape_ampersand() {
        assert_eq!(html_escape("a & b"), "a &amp; b");
    }
    #[test]
    fn html_escape_angle_brackets() {
        assert_eq!(html_escape("<em>"), "&lt;em&gt;");
    }
    #[test]
    fn html_escape_quotes() {
        assert_eq!(html_escape(r#"say "hi""#), "say &quot;hi&quot;");
    }
}
