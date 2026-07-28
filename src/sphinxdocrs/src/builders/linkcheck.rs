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
//! | `HyperlinkAvailabilityChecker.check` | [`check_uri`] | HTTP check via [`crate::http_client`] |
//! | `linkcheck_ignore` | [`LinkcheckConfig::from_config`] | regex allowlist of URIs to skip |
//! | `linkcheck_anchors` / `linkcheck_anchors_ignore` | [`check_uri`] | `#fragment` existence check |
//! | `linkcheck_allowed_redirects` | [`check_uri`] | redirect classified `working` vs `redirected` |
//! | `linkcheck_timeout` / `linkcheck_retries` / `linkcheck_rate_limit_timeout` | [`check_uri`] | timeouts, retry count, HTTP 429 backoff |
//!
//! **Accepted deviations**: upstream validates `linkcheck_*` regex patterns
//! at config-read time and raises `ConfigError`; here, an invalid pattern is
//! silently dropped from the corresponding list rather than failing the
//! build. Anchor existence is checked with a permissive `id="..."` /
//! `name="..."` regex scan rather than a full HTML parse.
//!
//! ## Security
//!
//! Only `http`/`https` URLs are checked over the network, via
//! [`crate::http_client`] — the same hardened backend (curl by default,
//! optional in-process `reqwest`) used by [`crate::intersphinx`]: restricted
//! schemes, bounded redirects, time and body-size limits, and URIs passed
//! after `--` (curl backend) so a URI beginning with `-` cannot be treated
//! as a flag.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use docutilsrs::doctree::{Doctree, NodeKind};
use docutilsrs::parse_rst_with_source;
use regex::Regex;

use super::{BuildError, BuildResult, Builder};
use crate::config::SphinxConfig;
use crate::environment::BuildEnvironment;
use crate::http_client;

/// Status of a single hyperlink check.  Mirrors the string values written by
/// upstream Sphinx to `output.txt` / `output.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkStatus {
    /// Link resolved successfully (HTTP < 400, and any redirect target was
    /// either identical to the request or explicitly allowed).
    Working,
    /// Link redirected to a different URI not covered by
    /// `linkcheck_allowed_redirects`.
    Redirected,
    /// Link failed (network error, HTTP >= 400, rate-limited with retries
    /// exhausted, or a required anchor was not found).
    Broken,
    /// Non-network scheme (e.g. `mailto:`), or matched `linkcheck_ignore` —
    /// recorded but not fetched.
    Ignored,
}

impl LinkStatus {
    /// Canonical lowercase name used in `output.txt` / `output.json`.
    pub fn as_str(self) -> &'static str {
        match self {
            LinkStatus::Working => "working",
            LinkStatus::Redirected => "redirected",
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

/// Resolved `linkcheck_*` settings for a single build, sourced from
/// [`SphinxConfig`]. Regex patterns are pre-compiled once rather than
/// per-check.
#[derive(Debug, Clone)]
pub struct LinkcheckConfig {
    /// `linkcheck_timeout` — seconds to wait for a response.
    pub timeout_secs: u32,
    /// `linkcheck_retries` — retry attempts for failed/rate-limited requests.
    pub retries: u32,
    /// `linkcheck_rate_limit_timeout` — overall seconds to keep retrying a
    /// rate-limited host before giving up.
    pub rate_limit_timeout_secs: f64,
    /// `linkcheck_anchors` — whether to verify `#fragment` anchors.
    pub check_anchors: bool,
    ignore: Vec<Regex>,
    anchors_ignore: Vec<Regex>,
    allowed_redirects: Vec<(Regex, Regex)>,
}

impl Default for LinkcheckConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            retries: 1,
            rate_limit_timeout_secs: 300.0,
            check_anchors: true,
            ignore: Vec::new(),
            anchors_ignore: compile_patterns(&["^!".to_string()]),
            allowed_redirects: Vec::new(),
        }
    }
}

impl LinkcheckConfig {
    /// Build a `LinkcheckConfig` from the `linkcheck_*` options registered on
    /// `config`.
    pub fn from_config(config: &SphinxConfig) -> Self {
        Self {
            timeout_secs: config.linkcheck_timeout(),
            retries: config.linkcheck_retries(),
            rate_limit_timeout_secs: config.linkcheck_rate_limit_timeout(),
            check_anchors: config.linkcheck_anchors(),
            ignore: compile_patterns(&config.linkcheck_ignore()),
            anchors_ignore: compile_patterns(&config.linkcheck_anchors_ignore()),
            allowed_redirects: config
                .linkcheck_allowed_redirects()
                .into_iter()
                .filter_map(|(from, to)| Some((Regex::new(&from).ok()?, Regex::new(&to).ok()?)))
                .collect(),
        }
    }

    fn is_ignored(&self, uri: &str) -> bool {
        self.ignore.iter().any(|re| re.is_match(uri))
    }

    fn is_anchor_ignored(&self, anchor: &str) -> bool {
        self.anchors_ignore.iter().any(|re| re.is_match(anchor))
    }

    fn is_allowed_redirect(&self, from: &str, to: &str) -> bool {
        self.allowed_redirects
            .iter()
            .any(|(f, t)| f.is_match(from) && t.is_match(to))
    }

    /// Set `linkcheck_ignore` patterns directly (builder-style; invalid
    /// regexes are silently dropped, matching [`Self::from_config`]).
    pub fn with_ignore(mut self, patterns: &[&str]) -> Self {
        self.ignore = compile_patterns(&patterns.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        self
    }

    /// Set `linkcheck_allowed_redirects` pairs directly (builder-style;
    /// invalid regexes are silently dropped, matching [`Self::from_config`]).
    pub fn with_allowed_redirects(mut self, pairs: &[(&str, &str)]) -> Self {
        self.allowed_redirects = pairs
            .iter()
            .filter_map(|(from, to)| Some((Regex::new(from).ok()?, Regex::new(to).ok()?)))
            .collect();
        self
    }
}

/// Compile every valid regex in `patterns`, silently dropping any that fail
/// to parse (see module-level "Accepted deviations").
fn compile_patterns(patterns: &[String]) -> Vec<Regex> {
    patterns.iter().filter_map(|p| Regex::new(p).ok()).collect()
}

/// Minimal external-link checker.
#[derive(Debug)]
pub struct LinkcheckBuilder {
    /// Per-request timeout in seconds, used only when no [`BuildEnvironment`]
    /// config is available (kept for direct/legacy construction).
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
    http_client::is_http_url(uri)
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

/// Split `uri` into `(request_url, decoded_anchor)`. The anchor is the
/// percent-decoded text after a `#`, or empty if there is none.
fn split_anchor(uri: &str) -> (&str, String) {
    match uri.split_once('#') {
        Some((base, frag)) if !frag.is_empty() => (base, percent_decode(frag)),
        Some((base, _)) => (base, String::new()),
        None => (uri, String::new()),
    }
}

/// Minimal percent-decoder (`%XX` → byte), used only for anchor names.
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Return `true` if `body` contains an element whose `id` or `name`
/// attribute equals `anchor` (case-insensitive attribute name, exact value
/// match). A permissive regex scan, not a full HTML parse.
fn contains_anchor(body: &[u8], anchor: &str) -> bool {
    if anchor.is_empty() {
        return true;
    }
    let text = String::from_utf8_lossy(body);
    let escaped = regex::escape(anchor);
    let pattern = format!(r#"(?i)\b(?:id|name)\s*=\s*["']{escaped}["']"#);
    Regex::new(&pattern)
        .map(|re| re.is_match(&text))
        .unwrap_or(false)
}

/// Check a single URI over the network via [`crate::http_client`].
///
/// - `mailto:` and other non-http(s) schemes, and any URI matching
///   `linkcheck_ignore`, are reported as `Ignored` without a network call.
/// - Otherwise the URI is fetched (`HEAD`, or `GET` when an anchor must be
///   checked). HTTP 429 responses are retried up to `cfg.retries` times,
///   honoring `Retry-After` when present, bounded overall by
///   `cfg.rate_limit_timeout_secs`. Network errors are retried the same way.
/// - A redirect whose final URL differs from the request is `Working` when
///   allowed by `linkcheck_allowed_redirects`, else `Redirected`.
/// - When `cfg.check_anchors` is set and the URI has a `#fragment` not
///   matched by `linkcheck_anchors_ignore`, the fetched body is scanned for
///   a matching `id`/`name` attribute; if absent, the link is `Broken`.
pub fn check_uri(docname: &str, uri: &str, cfg: &LinkcheckConfig) -> CheckResult {
    let ignored = |info: &str| CheckResult {
        docname: docname.to_owned(),
        uri: uri.to_owned(),
        status: LinkStatus::Ignored,
        code: 0,
        info: info.to_owned(),
    };

    if !is_http_url(uri) {
        return ignored("");
    }
    if cfg.is_ignored(uri) {
        return ignored("");
    }

    let (req_url, anchor) = split_anchor(uri);
    let want_anchor = cfg.check_anchors && !anchor.is_empty() && !cfg.is_anchor_ignored(&anchor);
    let method = if want_anchor {
        http_client::Method::Get
    } else {
        http_client::Method::Head
    };

    let mut attempts_left = cfg.retries;
    let deadline = Instant::now() + Duration::from_secs_f64(cfg.rate_limit_timeout_secs.max(0.0));

    loop {
        match http_client::request(req_url, method, cfg.timeout_secs) {
            Ok(resp) => {
                if resp.status == 429 {
                    if attempts_left > 0 && Instant::now() < deadline {
                        let wait = resp.retry_after_secs.unwrap_or(1).clamp(1, 60);
                        std::thread::sleep(Duration::from_secs(wait));
                        attempts_left -= 1;
                        continue;
                    }
                    return CheckResult {
                        docname: docname.to_owned(),
                        uri: uri.to_owned(),
                        status: LinkStatus::Broken,
                        code: 429,
                        info: "rate limited".to_owned(),
                    };
                }

                if resp.status >= 400 {
                    return CheckResult {
                        docname: docname.to_owned(),
                        uri: uri.to_owned(),
                        status: LinkStatus::Broken,
                        code: resp.status as i32,
                        info: format!("HTTP status {}", resp.status),
                    };
                }

                if want_anchor {
                    let found = resp
                        .body
                        .as_deref()
                        .map(|b| contains_anchor(b, &anchor))
                        .unwrap_or(false);
                    if !found {
                        return CheckResult {
                            docname: docname.to_owned(),
                            uri: uri.to_owned(),
                            status: LinkStatus::Broken,
                            code: resp.status as i32,
                            info: format!("Anchor '{anchor}' not found"),
                        };
                    }
                }

                if resp.final_url != req_url {
                    if cfg.is_allowed_redirect(req_url, &resp.final_url) {
                        return CheckResult {
                            docname: docname.to_owned(),
                            uri: uri.to_owned(),
                            status: LinkStatus::Working,
                            code: resp.status as i32,
                            info: String::new(),
                        };
                    }
                    return CheckResult {
                        docname: docname.to_owned(),
                        uri: uri.to_owned(),
                        status: LinkStatus::Redirected,
                        code: resp.status as i32,
                        info: resp.final_url.clone(),
                    };
                }

                return CheckResult {
                    docname: docname.to_owned(),
                    uri: uri.to_owned(),
                    status: LinkStatus::Working,
                    code: resp.status as i32,
                    info: String::new(),
                };
            }
            Err(e) => {
                if attempts_left > 0 {
                    attempts_left -= 1;
                    continue;
                }
                return CheckResult {
                    docname: docname.to_owned(),
                    uri: uri.to_owned(),
                    status: LinkStatus::Broken,
                    code: 0,
                    info: e.to_string(),
                };
            }
        }
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

        // Resolve linkcheck_* settings from the environment's config (falls
        // back to defaults, with `self.timeout_secs` overriding the default
        // timeout when the config left it unset, for direct/legacy callers).
        let mut cfg = LinkcheckConfig::from_config(&env.config);
        if cfg.timeout_secs == LinkcheckConfig::default().timeout_secs {
            cfg.timeout_secs = self.timeout_secs;
        }

        // Check each unique URI and accumulate results.
        let mut results: Vec<CheckResult> = Vec::with_capacity(checks.len());
        for (docname, uri) in &checks {
            results.push(check_uri(docname, uri, &cfg));
        }

        // Write output.txt (broken/redirected links, matching upstream) and
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
            } else if r.status == LinkStatus::Redirected {
                txt.push_str(&format!(
                    "{}.rst:{}: [{}] {} to {}\n",
                    r.docname,
                    0,
                    r.status.as_str(),
                    r.uri,
                    r.info
                ));
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
        let r = check_uri("index", "mailto:a@b.com", &LinkcheckConfig::default());
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
