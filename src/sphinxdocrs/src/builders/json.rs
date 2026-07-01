//! `sphinxdocrs::builders::json` — Rust port of
//! `sphinxcontrib.serializinghtml.JSONHTMLBuilder`.
//!
//! Reads source files, parses them with `docutilsrs::parse_rst_with_source`,
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
use std::path::{Path, PathBuf};

use docutilsrs::cli::{CommonOptions, Html5Options};
use docutilsrs::{html5, parse_rst_with_source};
use serde::{Deserialize, Serialize};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

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
    /// Source file name including extension (e.g. `"index.rst"`, `"guide.md"`).
    pub sourcename: String,
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
    pub last_updated: String,
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
        let tree = parse_rst_with_source(source, docname);
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
            last_updated: current_date_utc(),
            titles,
        };
        let path = outdir.join(&self.globalcontext_filename);
        let file = std::fs::File::create(&path).map_err(BuildError::Io)?;
        serde_json::to_writer_pretty(file, &ctx)
            .map_err(|e| BuildError::Other(format!("globalcontext serialization failed: {e}")))?;
        Ok(())
    }

    /// Core per-page writer.  `source_suffix` is recorded verbatim in `sourcename`.
    fn write_page(
        &self,
        docname: &str,
        source: &str,
        outdir: &Path,
        source_suffix: &str,
    ) -> Result<(), BuildError> {
        let body = self.render_body(docname, source);
        let title = Self::extract_title(docname, source);
        let target_uri = self.get_target_uri(docname);
        let (toc, display_toc) = Self::build_toc(source, &target_uri);

        let ctx = PageContext {
            body,
            title: html_escape(&title),
            toc,
            display_toc,
            current_page_name: docname.to_string(),
            parents: Vec::new(),
            prev: None,
            next: None,
            sourcename: format!("{docname}{source_suffix}"),
        };

        let rel: PathBuf = docname
            .split('/')
            .collect::<PathBuf>()
            .with_extension("fjson");
        let out_path = outdir.join(rel);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = std::fs::File::create(&out_path).map_err(BuildError::Io)?;
        serde_json::to_writer_pretty(file, &ctx)
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
        self.write_page(docname, source, outdir, suffix)
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

        let docs: Vec<(String, String)> = if !env.all_docs.is_empty() {
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

        std::fs::create_dir_all(outdir)?;

        let mut titles: HashMap<String, String> = HashMap::new();
        for (docname, suffix) in &docs {
            let src_path = src_path_for_docname(srcdir, docname, suffix);
            let source = std::fs::read_to_string(&src_path).map_err(|e| {
                BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
            })?;
            let title = Self::extract_title(docname, &source);
            titles.insert(docname.clone(), html_escape(&title));
            self.write_page(docname, &source, outdir, suffix)?;
            result.written += 1;
        }

        self.write_globalcontext(outdir, env, titles)?;
        Ok(result)
    }
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

fn current_date_utc() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    epoch_secs_to_date(secs)
}

fn epoch_secs_to_date(secs: u64) -> String {
    let days = (secs / 86400) as i64;
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", y, m, d)
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
        assert_eq!(ctx.sourcename, "index.rst");
    }

    #[test]
    fn build_doc_sourcename_uses_md_when_configured() {
        let tmp = TempDir::new().unwrap();
        JsonBuilder::with_source_suffixes(vec![".md".into()])
            .build_doc("readme", "Title\n=====\n\nContent.\n", tmp.path())
            .unwrap();
        let raw = std::fs::read_to_string(tmp.path().join("readme.fjson")).unwrap();
        let ctx: PageContext = serde_json::from_str(&raw).unwrap();
        assert_eq!(ctx.sourcename, "readme.md");
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

    // ── epoch_secs_to_date ────────────────────────────────────────────────────

    #[test]
    fn epoch_unix_zero_is_1970_01_01() {
        assert_eq!(epoch_secs_to_date(0), "1970-01-01");
    }
    #[test]
    fn epoch_known_date() {
        assert_eq!(epoch_secs_to_date(1_704_067_200), "2024-01-01");
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
            sourcename: "index.rst".into(),
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
            last_updated: "2024-01-01".into(),
            titles: HashMap::from([("index".into(), "Welcome".into())]),
        };
        let back: GlobalContext =
            serde_json::from_str(&serde_json::to_string(&ctx).unwrap()).unwrap();
        assert_eq!(ctx, back);
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
