//! `sphinxdocrs::intersphinx` — minimal port of `sphinx.ext.intersphinx`.
//!
//! Downloads external `objects.inv` inventory files listed in
//! `intersphinx_mapping` and caches them under
//! `<doctreedir>/__intersphinx_cache__/<name>_objects.inv`, mirroring
//! `sphinx.ext.intersphinx._load`'s cache layout.
//!
//! ## What is ported
//!
//! | upstream | Rust target | notes |
//! |----------|-------------|-------|
//! | `_load.fetch_inventory` | [`fetch_inventories`] | HTTP GET via `curl` |
//! | cache dir `__intersphinx_cache__` | same path | matches Python sphinx |
//! | cache file `{name}_objects.inv` | same name | matches Python sphinx |
//!
//! **Deferred**: parsing the zlib-compressed inventory entries for
//! cross-reference resolution (needed for `:ref:` to external projects).

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::SphinxConfig;

/// Inventory cache entry: project name and the on-disk path.
#[derive(Debug, Clone)]
pub struct InvCache {
    /// Project name (key from `intersphinx_mapping`).
    pub name: String,
    /// Absolute path to the cached `objects.inv` file.
    pub path: PathBuf,
}

/// Fetch all `intersphinx_mapping` inventories and cache them under
/// `doctreedir/__intersphinx_cache__/`.
///
/// Mirrors the cache-population step in
/// `sphinx.ext.intersphinx._load.load_mappings`.
///
/// - Skips entries whose cache file already exists (no re-download).
/// - Emits a `Warning:` message to stderr and continues when a download fails.
/// - Returns only the successfully cached entries.
///
/// # Extension guard
///
/// Returns an empty vec immediately when `sphinx.ext.intersphinx` is not in
/// `config.extensions()`, so this is safe to call unconditionally.
pub fn fetch_inventories(config: &SphinxConfig, doctreedir: &Path) -> Vec<InvCache> {
    // Only run when the extension is enabled.
    if !config
        .extensions()
        .iter()
        .any(|e| e == "sphinx.ext.intersphinx")
    {
        return Vec::new();
    }

    let mapping = config.intersphinx_mapping();
    if mapping.is_empty() {
        return Vec::new();
    }

    let cache_dir = doctreedir.join("__intersphinx_cache__");
    if let Err(e) = std::fs::create_dir_all(&cache_dir) {
        eprintln!("Warning: intersphinx: cannot create cache dir {}: {e}", cache_dir.display());
        return Vec::new();
    }

    let mut results = Vec::new();

    for (name, base_url, inv_url) in &mapping {
        // ── Security: sanitize the mapping key before using it in a path ──────
        // The key comes from `conf.py` and is used to build the cache filename.
        // Reject anything that could escape the cache directory (path
        // separators, parent refs, empty, or non-alphanumeric junk).
        let Some(safe_name) = sanitize_project_name(name) else {
            eprintln!(
                "Warning: intersphinx: skipping mapping entry with unsafe name {name:?}"
            );
            continue;
        };

        let cache_file = cache_dir.join(format!("{safe_name}_objects.inv"));

        // Defense in depth: confirm the resolved path is still inside cache_dir.
        if !cache_file.starts_with(&cache_dir) {
            eprintln!(
                "Warning: intersphinx: refusing to write outside cache dir for {name:?}"
            );
            continue;
        }

        // Already cached — reuse without a network request.
        if cache_file.exists() {
            results.push(InvCache { name: name.clone(), path: cache_file });
            continue;
        }

        // Resolve the inventory URL: explicit `inv_url` or `<base>/objects.inv`.
        let url = match inv_url {
            Some(u) => u.clone(),
            None => {
                let base = base_url.trim_end_matches('/');
                format!("{base}/objects.inv")
            }
        };

        // ── Security: only allow http(s) inventory URLs ──────────────────────
        // Prevents `curl` from being coerced into reading local files
        // (`file://`), hitting internal services over other protocols
        // (`scp://`, `smb://`, `gopher://`, …), or similar SSRF vectors.
        if !is_http_url(&url) {
            eprintln!(
                "Warning: intersphinx: refusing non-http(s) inventory URL {url:?} for {name:?}"
            );
            continue;
        }

        eprintln!("sphinxdocrs: intersphinx: fetching {url}");

        // Download to a temporary file first, then rename atomically on success.
        // This prevents a partially-written file (from an interrupted download)
        // from poisoning the cache and being reused on the next run.
        let tmp_file = cache_dir.join(format!("{safe_name}_objects.inv.tmp"));

        // Security-hardened curl invocation:
        //   --proto '=https,http'    restrict the initial request to http(s)
        //   --proto-redir '=https,http'  restrict redirect targets too
        //   --max-redirs 10          bound redirect chains
        //   --max-time 30            bound total time (anti-hang / DoS)
        //   --max-filesize 52428800  cap download at 50 MiB (anti disk-fill)
        //   --fail                   non-zero exit on HTTP >= 400 (no body file)
        //   --silent                 no progress noise
        let status = Command::new("curl")
            .args([
                "--silent",
                "--fail",
                "--location",
                "--proto",
                "=https,http",
                "--proto-redir",
                "=https,http",
                "--max-redirs",
                "10",
                "--max-time",
                "30",
                "--max-filesize",
                "52428800",
                "-o",
                tmp_file.to_str().unwrap_or_default(),
                "--",
                &url,
            ])
            .status();

        match status {
            Ok(s) if s.success() => {
                // Atomically move the completed download into place.
                if let Err(e) = std::fs::rename(&tmp_file, &cache_file) {
                    eprintln!(
                        "Warning: intersphinx: cannot finalize cache file for {name:?}: {e}"
                    );
                    let _ = std::fs::remove_file(&tmp_file);
                    continue;
                }
                results.push(InvCache { name: name.clone(), path: cache_file });
            }
            Ok(s) => {
                let _ = std::fs::remove_file(&tmp_file);
                eprintln!(
                    "Warning: intersphinx: curl exited {} fetching {url}",
                    s.code().unwrap_or(-1)
                );
            }
            Err(e) => {
                let _ = std::fs::remove_file(&tmp_file);
                eprintln!("Warning: intersphinx: failed to run curl for {url}: {e}");
            }
        }
    }

    results
}

/// Sanitize an `intersphinx_mapping` project name for safe use as a path
/// component.
///
/// Returns `None` when the name is empty or contains anything other than
/// ASCII alphanumerics, `-`, `_`, or `.` — this blocks path separators
/// (`/`, `\`), parent references (`..`), and absolute-path markers that
/// could escape the cache directory.
fn sanitize_project_name(name: &str) -> Option<String> {
    if name.is_empty() || name.len() > 128 {
        return None;
    }
    // Reject `.` and `..` outright even though they'd pass the char check.
    if name == "." || name == ".." {
        return None;
    }
    if name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        Some(name.to_owned())
    } else {
        None
    }
}

/// Return `true` only for `http://` or `https://` URLs (case-insensitive
/// scheme).  Everything else — `file://`, `scp://`, `ftp://`, schemeless
/// paths, etc. — is rejected.
fn is_http_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::config::{ConfigVal, SphinxConfig};
    use tempfile::TempDir;

    fn config_with_intersphinx(mapping: Vec<(String, ConfigVal)>) -> SphinxConfig {
        let mut raw = HashMap::new();
        raw.insert(
            "extensions".into(),
            ConfigVal::List(vec![ConfigVal::Str(
                "sphinx.ext.intersphinx".into(),
            )]),
        );
        raw.insert("intersphinx_mapping".into(), ConfigVal::Map(mapping));
        SphinxConfig::new(raw, HashMap::new())
    }

    #[test]
    fn no_extension_returns_empty() {
        let cfg = SphinxConfig::new_defaults();
        let tmp = TempDir::new().unwrap();
        let result = fetch_inventories(&cfg, tmp.path());
        assert!(result.is_empty());
    }

    #[test]
    fn empty_mapping_returns_empty() {
        let cfg = config_with_intersphinx(vec![]);
        let tmp = TempDir::new().unwrap();
        let result = fetch_inventories(&cfg, tmp.path());
        assert!(result.is_empty());
    }

    #[test]
    fn creates_cache_dir() {
        // A mapping to a non-existent URL — curl will fail, but the cache dir
        // itself must be created.
        let cfg = config_with_intersphinx(vec![(
            "fakepkg".into(),
            ConfigVal::List(vec![
                ConfigVal::Str("http://127.0.0.1:0/".into()),
                ConfigVal::Null,
            ]),
        )]);
        let tmp = TempDir::new().unwrap();
        let cache_dir = tmp.path().join("__intersphinx_cache__");
        let _ = fetch_inventories(&cfg, tmp.path());
        assert!(cache_dir.exists(), "cache dir should be created even on fetch failure");
    }

    #[test]
    fn reuses_existing_cache_file() {
        let cfg = config_with_intersphinx(vec![(
            "mypkg".into(),
            ConfigVal::List(vec![
                ConfigVal::Str("http://127.0.0.1:0/".into()),
                ConfigVal::Null,
            ]),
        )]);
        let tmp = TempDir::new().unwrap();
        let cache_dir = tmp.path().join("__intersphinx_cache__");
        std::fs::create_dir_all(&cache_dir).unwrap();
        let cache_file = cache_dir.join("mypkg_objects.inv");
        std::fs::write(&cache_file, b"cached content").unwrap();

        let result = fetch_inventories(&cfg, tmp.path());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "mypkg");
        assert_eq!(result[0].path, cache_file);
    }

    #[test]
    fn intersphinx_mapping_round_trip() {
        let cfg = config_with_intersphinx(vec![
            (
                "python".into(),
                ConfigVal::List(vec![
                    ConfigVal::Str("https://docs.python.org/3/".into()),
                    ConfigVal::Null,
                ]),
            ),
            (
                "requests".into(),
                ConfigVal::List(vec![
                    ConfigVal::Str("https://requests.readthedocs.io/en/latest/".into()),
                    ConfigVal::Null,
                ]),
            ),
        ]);
        let mapping = cfg.intersphinx_mapping();
        assert_eq!(mapping.len(), 2);
        let py = mapping.iter().find(|(n, _, _)| n == "python").unwrap();
        assert_eq!(py.1, "https://docs.python.org/3/");
        assert!(py.2.is_none());
    }

    // ── security: sanitize_project_name ───────────────────────────────────────

    #[test]
    fn sanitize_accepts_normal_names() {
        assert_eq!(sanitize_project_name("python").as_deref(), Some("python"));
        assert_eq!(sanitize_project_name("py-3.12").as_deref(), Some("py-3.12"));
        assert_eq!(sanitize_project_name("my_proj").as_deref(), Some("my_proj"));
    }

    #[test]
    fn sanitize_rejects_path_traversal() {
        assert!(sanitize_project_name("..").is_none());
        assert!(sanitize_project_name(".").is_none());
        assert!(sanitize_project_name("../../etc/passwd").is_none());
        assert!(sanitize_project_name("foo/bar").is_none());
        assert!(sanitize_project_name("foo\\bar").is_none());
        assert!(sanitize_project_name("/abs/path").is_none());
    }

    #[test]
    fn sanitize_rejects_empty_and_oversized() {
        assert!(sanitize_project_name("").is_none());
        assert!(sanitize_project_name(&"a".repeat(129)).is_none());
    }

    #[test]
    fn sanitize_rejects_shell_metacharacters() {
        assert!(sanitize_project_name("a;b").is_none());
        assert!(sanitize_project_name("a b").is_none());
        assert!(sanitize_project_name("a$b").is_none());
        assert!(sanitize_project_name("a\0b").is_none());
    }

    // ── security: is_http_url ─────────────────────────────────────────────────

    #[test]
    fn http_url_accepts_http_and_https() {
        assert!(is_http_url("http://example.com/objects.inv"));
        assert!(is_http_url("https://example.com/objects.inv"));
        assert!(is_http_url("HTTPS://EXAMPLE.COM/objects.inv"));
    }

    #[test]
    fn http_url_rejects_other_schemes() {
        assert!(!is_http_url("file:///etc/passwd"));
        assert!(!is_http_url("ftp://example.com/objects.inv"));
        assert!(!is_http_url("scp://host/objects.inv"));
        assert!(!is_http_url("gopher://host/"));
        assert!(!is_http_url("/local/path/objects.inv"));
        assert!(!is_http_url("objects.inv"));
        assert!(!is_http_url(""));
    }

    #[test]
    fn unsafe_name_is_skipped_without_writing_outside_cache() {
        // A malicious mapping key must not produce a file outside the cache dir.
        let cfg = config_with_intersphinx(vec![(
            "../../evil".into(),
            ConfigVal::List(vec![
                ConfigVal::Str("https://example.com/".into()),
                ConfigVal::Null,
            ]),
        )]);
        let tmp = TempDir::new().unwrap();
        let result = fetch_inventories(&cfg, tmp.path());
        assert!(result.is_empty(), "unsafe name must be skipped");
        // Nothing should have been written above the cache dir.
        assert!(!tmp.path().join("evil_objects.inv").exists());
        assert!(!tmp.path().parent().unwrap().join("evil_objects.inv").exists());
    }

    #[test]
    fn non_http_url_is_skipped() {
        // A file:// inventory URL must be refused (no local file read).
        let cfg = config_with_intersphinx(vec![(
            "localfile".into(),
            ConfigVal::List(vec![
                ConfigVal::Str("file:///etc/passwd".into()),
                ConfigVal::Str("file:///etc/passwd".into()),
            ]),
        )]);
        let tmp = TempDir::new().unwrap();
        let result = fetch_inventories(&cfg, tmp.path());
        assert!(result.is_empty(), "non-http URL must be skipped");
        let cache_file = tmp.path()
            .join("__intersphinx_cache__")
            .join("localfile_objects.inv");
        assert!(!cache_file.exists(), "must not have fetched via file://");
    }
}
