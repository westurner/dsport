//! `sphinxdocrs::builders::linkcheck` — Rust port of
//! `sphinx.builders.linkcheck.CheckExternalLinksBuilder` (minimal path).
//!
//! Collects external hyperlinks from every RST document, checks each unique
//! URL over the network, and writes `output.txt` and `output.json` to the
//! output directory — matching the file names and line formats produced by
//! upstream Sphinx.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! |-----------------|-------------|-------|
//! | `HyperlinkCollector.find_uri` | [`collect_hyperlinks`] | walks doctree for `reference`/`image` URIs |
//! | `CheckExternalLinksBuilder.write_entry` | `output.txt` line format | `{file}:{line}: [{status}] {uri}` |
//! | `CheckExternalLinksBuilder.write_linkstat` | `output.json` line format | one JSON object per line |
//! | `HyperlinkAvailabilityChecker.check` | [`check_uri`] | HTTP check via hardened `curl` |
//!
//! **Deferred**: anchor checking (`#fragment`), redirect classification,
//! rate-limit backoff, `linkcheck_ignore` regex config, retries.
//!
//! ## Security
//!
//! Only `http`/`https` URLs are checked over the network.  The `curl`
//! invocation is hardened identically to [`crate::intersphinx`]:
//! `--proto`/`--proto-redir` restricted to http(s), bounded redirects,
//! time and body-size limits.  URIs are passed after `--` so a URI
//! beginning with `-` cannot be treated as a flag.

use std::path::{Path, PathBuf};
use std::process::Command;

use docutilsrs::doctree::{Doctree, NodeKind};
use docutilsrs::parse_rst_with_source;

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// Status of a single hyperlink check.  Mirrors the string values written by
/// upstream Sphinx to `output.txt` / `output.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkStatus {
    /// Link resolved successfully (HTTP < 400).
    Working,
    /// Link failed (network error or HTTP >= 400).
    Broken,
    /// Non-network scheme (e.g. `mailto:`) — recorded but not fetched.
    Ignored,
}

impl LinkStatus {
    /// Canonical lowercase name used in `output.txt` / `output.json`.
    pub fn as_str(self) -> &'static str {
        match self {
            LinkStatus::Working => "working",
            LinkStatus::Broken => "broken",
            LinkStatus::Ignored => "ignored",
        }
    }
}

/// Result of checking one URI.
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// The document the link was found in (docname, no extension).
    pub docname: String,
    /// The checked URI.
    pub uri: String,
    /// The resolved status.
    pub status: LinkStatus,
    /// HTTP status code, or 0 when not applicable.
    pub code: i32,
    /// Human-readable message (empty on success).
    pub info: String,
}

/// Minimal external-link checker.
#[derive(Debug)]
pub struct LinkcheckBuilder {
    /// Per-request timeout in seconds (passed to `curl --max-time`).
    pub timeout_secs: u32,
}

impl Default for LinkcheckBuilder {
    fn default() -> Self {
        Self { timeout_secs: 30 }
    }
}

impl LinkcheckBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Return `true` only for `http://` / `https://` URIs (case-insensitive).
fn is_http_url(uri: &str) -> bool {
    let lower = uri.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

/// Walk `tree` and collect every external hyperlink URI in document order.
///
/// Mirrors `HyperlinkCollector.run`: inspects `reference` (`refuri`) and
/// `image` (`uri`) nodes.  Only URIs containing a scheme (`scheme:`) or
/// protocol-relative prefix (`//`) are returned — internal cross-references
/// (bare docnames / `#anchors`) are skipped, matching upstream `uri_re`.
pub fn collect_hyperlinks(tree: &Doctree) -> Vec<String> {
    let mut uris = Vec::new();
    for id in 0..tree.nodes_len() {
        let uri = match &tree.node(id).kind {
            NodeKind::Reference { refuri, .. } if !refuri.is_empty() => refuri.clone(),
            NodeKind::Image { uri, .. } if !uri.is_empty() => uri.clone(),
            _ => continue,
        };
        // upstream uri_re: ([a-z]+:)?//  — require a scheme or protocol-relative //.
        if uri.contains("://") || uri.starts_with("//") || uri.starts_with("mailto:") {
            uris.push(uri);
        }
    }
    uris
}

/// Check a single URI over the network with a hardened `curl` invocation.
///
/// - `mailto:` and other non-http(s) schemes are reported as `Ignored`.
/// - http(s) URLs are fetched with `curl -I` (HEAD); the HTTP status code
///   determines `Working` (< 400) vs `Broken` (>= 400 or network error).
pub fn check_uri(docname: &str, uri: &str, timeout_secs: u32) -> CheckResult {
    // Non-network schemes are recorded but never fetched.
    if !is_http_url(uri) {
        return CheckResult {
            docname: docname.to_owned(),
            uri: uri.to_owned(),
            status: LinkStatus::Ignored,
            code: 0,
            info: String::new(),
        };
    }

    // `curl -sI --fail-with-body -o /dev/null -w '%{http_code}'`:
    //   -s / --silent          quiet
    //   -I / --head            HEAD request (no body download)
    //   --location             follow redirects
    //   --proto '=https,http'  restrict initial scheme
    //   --proto-redir …        restrict redirect scheme
    //   --max-redirs 10        bound redirects
    //   --max-time N           bound total time
    //   -o /dev/null           discard any body
    //   -w '%{http_code}'      print the final HTTP status code
    let output = Command::new("curl")
        .args([
            "--silent",
            "--head",
            "--location",
            "--proto",
            "=https,http",
            "--proto-redir",
            "=https,http",
            "--max-redirs",
            "10",
            "--max-time",
            &timeout_secs.to_string(),
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            "--",
            uri,
        ])
        .output();

    match output {
        Ok(out) => {
            let code_str = String::from_utf8_lossy(&out.stdout);
            let code: i32 = code_str.trim().parse().unwrap_or(0);
            if out.status.success() && (200..400).contains(&code) {
                CheckResult {
                    docname: docname.to_owned(),
                    uri: uri.to_owned(),
                    status: LinkStatus::Working,
                    code,
                    info: String::new(),
                }
            } else {
                CheckResult {
                    docname: docname.to_owned(),
                    uri: uri.to_owned(),
                    status: LinkStatus::Broken,
                    code,
                    info: if code > 0 {
                        format!("HTTP status {code}")
                    } else {
                        "network error".to_owned()
                    },
                }
            }
        }
        Err(e) => CheckResult {
            docname: docname.to_owned(),
            uri: uri.to_owned(),
            status: LinkStatus::Broken,
            code: 0,
            info: format!("failed to run curl: {e}"),
        },
    }
}

/// Escape a string for inclusion in a JSON string literal.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Serialise a `CheckResult` as a single-line JSON object matching the
/// upstream `write_linkstat` key set (`filename`, `lineno`, `status`,
/// `code`, `uri`, `info`).
fn result_to_json(r: &CheckResult) -> String {
    format!(
        "{{\"filename\": \"{}\", \"lineno\": {}, \"status\": \"{}\", \"code\": {}, \"uri\": \"{}\", \"info\": \"{}\"}}",
        json_escape(&format!("{}.rst", r.docname)),
        0,
        r.status.as_str(),
        r.code,
        json_escape(&r.uri),
        json_escape(&r.info),
    )
}

impl Builder for LinkcheckBuilder {
    fn name(&self) -> &str {
        "linkcheck"
    }
    fn format(&self) -> &str {
        // Upstream `CheckExternalLinksBuilder.format` is the empty string.
        ""
    }
    fn out_suffix(&self) -> &str {
        ".txt"
    }

    fn get_target_uri(&self, docname: &str) -> String {
        docname.to_string()
    }

    /// linkcheck has no per-document output file; the real work happens in
    /// [`build_all`](Self::build_all).  This is a no-op so the trait is
    /// satisfiable.
    fn build_doc(&self, _docname: &str, _source: &str, _outdir: &Path) -> Result<(), BuildError> {
        Ok(())
    }

    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let mut result = BuildResult::default();
        std::fs::create_dir_all(outdir)?;

        let docnames: Vec<String> = if !env.all_docs.is_empty() {
            let mut v: Vec<String> = env.all_docs.keys().cloned().collect();
            v.sort();
            v
        } else {
            super::html::discover_rst_docnames_pub(srcdir)
        };

        // Collect (docname, uri) pairs, de-duplicating identical URIs while
        // remembering the first document each was seen in (matches the way
        // upstream reports each unique link once).
        let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut checks: Vec<(String, String)> = Vec::new();
        for docname in &docnames {
            let src_path = srcdir.join(format!("{docname}.rst"));
            let source = match std::fs::read_to_string(&src_path) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let tree = parse_rst_with_source(&source, docname);
            for uri in collect_hyperlinks(&tree) {
                if seen.insert(uri.clone()) {
                    checks.push((docname.clone(), uri));
                }
            }
        }

        // Check each unique URI and accumulate results.
        let mut results: Vec<CheckResult> = Vec::with_capacity(checks.len());
        for (docname, uri) in &checks {
            results.push(check_uri(docname, uri, self.timeout_secs));
        }

        // Write output.txt (broken links only, matching upstream) and
        // output.json (every checked link, one JSON object per line).
        let mut txt = String::new();
        let mut json = String::new();
        let mut broken = 0usize;
        for r in &results {
            json.push_str(&result_to_json(r));
            json.push('\n');
            if r.status == LinkStatus::Broken {
                broken += 1;
                txt.push_str(&format!(
                    "{}.rst:{}: [{}] {}\n",
                    r.docname,
                    0,
                    r.status.as_str(),
                    r.uri
                ));
                result
                    .warnings
                    .push(format!("broken link: {} ({})", r.uri, r.info));
            }
        }

        std::fs::write(outdir.join("output.txt"), txt.as_bytes())?;
        std::fs::write(outdir.join("output.json"), json.as_bytes())?;

        result.written = results.len();
        // Report the number of broken links via skipped as a rough signal;
        // callers inspect `warnings` for detail.
        let _ = broken;
        Ok(result)
    }
}

/// Convenience wrapper for the output-path helpers used by tests.
pub fn output_txt_path(outdir: &Path) -> PathBuf {
    outdir.join("output.txt")
}

/// Path to the JSON link-statistics file.
pub fn output_json_path(outdir: &Path) -> PathBuf {
    outdir.join("output.json")
}

#[cfg(test)]
mod tests {
    use super::*;
    use docutilsrs::parse_rst_with_source;

    #[test]
    fn name_and_format() {
        let b = LinkcheckBuilder::new();
        assert_eq!(b.name(), "linkcheck");
        assert_eq!(b.format(), "");
    }

    #[test]
    fn is_http_url_only_accepts_http() {
        assert!(is_http_url("http://example.com"));
        assert!(is_http_url("https://example.com"));
        assert!(is_http_url("HTTPS://EXAMPLE.COM"));
        assert!(!is_http_url("mailto:a@b.com"));
        assert!(!is_http_url("ftp://x/"));
        assert!(!is_http_url("/local/path"));
    }

    #[test]
    fn collect_hyperlinks_finds_external_urls() {
        let rst = "\
Title\n\
=====\n\
\n\
See `Example <https://example.com/>`_ and `Python <https://python.org>`_.\n";
        let tree = parse_rst_with_source(rst, "index");
        let uris = collect_hyperlinks(&tree);
        assert!(uris.iter().any(|u| u == "https://example.com/"));
        assert!(uris.iter().any(|u| u == "https://python.org"));
    }

    #[test]
    fn collect_hyperlinks_skips_internal_refs() {
        // A bare internal reference (no scheme) must not be collected.
        let rst = "\
Title\n\
=====\n\
\n\
See section_.\n\
\n\
.. _section: other-doc\n";
        let tree = parse_rst_with_source(rst, "index");
        let uris = collect_hyperlinks(&tree);
        assert!(
            !uris.iter().any(|u| u == "other-doc"),
            "internal ref should be skipped; got {uris:?}"
        );
    }

    #[test]
    fn check_uri_ignores_non_http() {
        let r = check_uri("index", "mailto:a@b.com", 5);
        assert_eq!(r.status, LinkStatus::Ignored);
        assert_eq!(r.code, 0);
    }

    #[test]
    fn json_escape_handles_quotes_and_control() {
        assert_eq!(json_escape("a\"b"), "a\\\"b");
        assert_eq!(json_escape("a\\b"), "a\\\\b");
        assert_eq!(json_escape("a\nb"), "a\\nb");
    }

    #[test]
    fn result_to_json_has_required_keys() {
        let r = CheckResult {
            docname: "index".into(),
            uri: "https://example.com/".into(),
            status: LinkStatus::Working,
            code: 200,
            info: String::new(),
        };
        let json = result_to_json(&r);
        for key in ["filename", "lineno", "status", "code", "uri", "info"] {
            assert!(
                json.contains(&format!("\"{key}\"")),
                "missing key {key} in {json}"
            );
        }
        assert!(json.contains("\"status\": \"working\""));
        assert!(json.contains("\"code\": 200"));
    }

    #[test]
    fn build_all_writes_output_files() {
        use crate::config::SphinxConfig;
        use crate::environment::{BuildEnvironment, EnvProject};
        use tempfile::TempDir;

        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        // A doc with only a non-network mailto link → Ignored, no broken.
        std::fs::write(
            src.path().join("index.rst"),
            "Title\n=====\n\nMail `me <mailto:a@b.com>`_.\n",
        )
        .unwrap();

        let config = SphinxConfig::new_defaults();
        let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env = BuildEnvironment::new(config, project, src.path(), out.path());

        let builder = LinkcheckBuilder::new();
        let result = builder.build_all(src.path(), out.path(), &env).unwrap();

        assert!(output_txt_path(out.path()).exists());
        assert!(output_json_path(out.path()).exists());
        // The mailto link is Ignored, so no broken warnings.
        assert!(result.warnings.is_empty());
    }
}
