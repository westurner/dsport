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
        let cache_file = cache_dir.join(format!("{name}_objects.inv"));

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

        eprintln!("sphinxdocrs: intersphinx: fetching {url}");

        // Use `curl` — available in the devcontainer and most CI environments.
        // Flags: --silent (no progress), --fail (non-zero on HTTP errors),
        //        --location (follow redirects), -o <dest>.
        let status = Command::new("curl")
            .args([
                "--silent",
                "--fail",
                "--location",
                "-o",
                cache_file.to_str().unwrap_or_default(),
                &url,
            ])
            .status();

        match status {
            Ok(s) if s.success() => {
                results.push(InvCache { name: name.clone(), path: cache_file });
            }
            Ok(s) => {
                eprintln!(
                    "Warning: intersphinx: curl exited {} fetching {url}",
                    s.code().unwrap_or(-1)
                );
            }
            Err(e) => {
                eprintln!("Warning: intersphinx: failed to run curl for {url}: {e}");
            }
        }
    }

    results
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
}
