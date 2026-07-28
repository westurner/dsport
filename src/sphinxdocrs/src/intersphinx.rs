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
//! | `_load.fetch_inventory` | [`fetch_inventories`] | HTTP GET via [`crate::http_client`] |
//! | `util.inventory.InventoryFile.loads` | [`Inventory::loads`] | v1 and v2 payloads |
//! | `util.inventory.InventoryFile.dump` | [`dumps`] | body writer; caller supplies the rows |
//! | `util.inventory._Inventory` | [`Inventory`] | `type → name → item` lookup |
//! | `util.inventory._InventoryItem` | [`InventoryItem`] | project, version, uri, display name |
//! | cache dir `__intersphinx_cache__` | same path | matches Python sphinx |
//! | cache file `{name}_objects.inv` | same name | matches Python sphinx |
//!
//! **Deferred**: wiring [`Inventory`] into cross-reference resolution, so
//! `:ref:` to external projects still does not resolve.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::config::SphinxConfig;
use crate::http_client;

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
        eprintln!(
            "Warning: intersphinx: cannot create cache dir {}: {e}",
            cache_dir.display()
        );
        return Vec::new();
    }

    let mut results = Vec::new();

    for (name, base_url, inv_url) in &mapping {
        // ── Security: sanitize the mapping key before using it in a path ──────
        // The key comes from `conf.py` and is used to build the cache filename.
        // Reject anything that could escape the cache directory (path
        // separators, parent refs, empty, or non-alphanumeric junk).
        let Some(safe_name) = sanitize_project_name(name) else {
            eprintln!("Warning: intersphinx: skipping mapping entry with unsafe name {name:?}");
            continue;
        };

        let cache_file = cache_dir.join(format!("{safe_name}_objects.inv"));

        // Defense in depth: confirm the resolved path is still inside cache_dir.
        if !cache_file.starts_with(&cache_dir) {
            eprintln!("Warning: intersphinx: refusing to write outside cache dir for {name:?}");
            continue;
        }

        // Already cached — reuse without a network request.
        if cache_file.exists() {
            results.push(InvCache {
                name: name.clone(),
                path: cache_file,
            });
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
        // Prevents the HTTP backend from being coerced into reading local
        // files (`file://`), hitting internal services over other protocols
        // (`scp://`, `smb://`, `gopher://`, …), or similar SSRF vectors.
        if !http_client::is_http_url(&url) {
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

        // `download_to_file` uses the same security-hardened backend as
        // `builders::linkcheck` (curl by default, or in-process `reqwest`
        // when built with `--features http-reqwest` and selected via
        // `SPHINXDOCRS_HTTP_CLIENT=reqwest`): restricted schemes, bounded
        // redirects, a 30s time limit, and a 50 MiB size cap (anti disk-fill).
        match http_client::download_to_file(&url, &tmp_file, 30, 52_428_800) {
            Ok(()) => {
                // Atomically move the completed download into place.
                if let Err(e) = std::fs::rename(&tmp_file, &cache_file) {
                    eprintln!("Warning: intersphinx: cannot finalize cache file for {name:?}: {e}");
                    let _ = std::fs::remove_file(&tmp_file);
                    continue;
                }
                results.push(InvCache {
                    name: name.clone(),
                    path: cache_file,
                });
            }
            Err(e) => {
                let _ = std::fs::remove_file(&tmp_file);
                eprintln!("Warning: intersphinx: failed to fetch {url}: {e}");
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
/// paths, etc. — is rejected. See [`crate::http_client::is_http_url`] for the
/// canonical implementation (tests below call it via this alias).
#[cfg(test)]
use http_client::is_http_url;

// ── objects.inv parsing ───────────────────────────────────────────────────────

/// A single object exported by an external project.
///
/// Mirrors `sphinx.util.inventory._InventoryItem`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryItem {
    /// `# Project:` header value of the inventory the item came from.
    pub project_name: String,
    /// `# Version:` header value of the inventory the item came from.
    pub project_version: String,
    /// Absolute URI, already joined against the inventory's base `uri`.
    pub uri: String,
    /// Display name, or `"-"` when it equals the object name.
    pub display_name: String,
}

/// A parsed `objects.inv` file: `type → name → item`.
///
/// Mirrors `sphinx.util.inventory._Inventory`. `type` is the qualified
/// `{domain}:{objtype}` string, e.g. `"py:class"` or `"std:label"`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Inventory {
    data: BTreeMap<String, BTreeMap<String, InventoryItem>>,
    ambiguities: Vec<String>,
}

/// Failure modes of [`Inventory::loads`].
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InventoryError {
    /// The first line is not a recognised `# Sphinx inventory version …` banner.
    #[error("invalid inventory header: {0}")]
    InvalidHeader(String),
    /// The banner announces a version this port cannot read.
    #[error("unknown or unsupported inventory version: {0:?}")]
    UnsupportedVersion(String),
    /// The `# Project:` / `# Version:` lines are missing.
    #[error("invalid inventory header: missing project name or version")]
    MissingProjectInfo,
    /// The v2 check line does not mention `zlib`.
    #[error("invalid inventory header (not compressed): {0}")]
    NotCompressed(String),
    /// The zlib payload could not be inflated.
    #[error("could not decompress inventory payload: {0}")]
    Decompress(String),
    /// The payload is not valid UTF-8.
    #[error("inventory payload is not valid UTF-8")]
    NotUtf8,
}

/// Regex mirroring upstream's entry pattern. Names may contain spaces, so the
/// first group is lazy and the priority field anchors the split.
const ENTRY_PATTERN: &str = r"^(.+?)\s+(\S+)\s+(-?\d+)\s+?(\S*)\s+(.*)$";

impl Inventory {
    /// Parse the raw bytes of an `objects.inv` file.
    ///
    /// `uri` is the base URL of the external project; every entry's location
    /// is joined against it. Mirrors `InventoryFile.loads`.
    pub fn loads(content: &[u8], uri: &str) -> Result<Self, InventoryError> {
        let (format_line, rest) = match content.iter().position(|b| *b == b'\n') {
            Some(index) => (&content[..index], &content[index + 1..]),
            None => (content, &content[content.len()..]),
        };
        let format_line = trim_ascii_end(format_line);

        const BANNER: &[u8] = b"# Sphinx inventory version ";
        match format_line {
            b"# Sphinx inventory version 2" => Self::loads_v2(rest, uri),
            b"# Sphinx inventory version 1" => {
                let text = std::str::from_utf8(rest).map_err(|_| InventoryError::NotUtf8)?;
                Self::loads_v1(text, uri)
            }
            line if line.starts_with(BANNER) => Err(InventoryError::UnsupportedVersion(
                String::from_utf8_lossy(&line[BANNER.len()..]).into_owned(),
            )),
            line => Err(InventoryError::InvalidHeader(
                String::from_utf8_lossy(line).into_owned(),
            )),
        }
    }

    /// Read and parse an `objects.inv` file from disk.
    pub fn load_file(path: impl AsRef<Path>, uri: &str) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        Self::loads(&bytes, uri)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Parse a version 1 (uncompressed) inventory body.
    fn loads_v1(body: &str, uri: &str) -> Result<Self, InventoryError> {
        let mut lines = body.lines();
        let (Some(project_line), Some(version_line)) = (lines.next(), lines.next()) else {
            return Err(InventoryError::MissingProjectInfo);
        };
        let project_name = header_value(project_line);
        let project_version = header_value(version_line);

        let mut inv = Inventory::default();
        for line in lines {
            // `split(None, maxsplit=2)`: name, type, location.
            let mut parts = line.trim_end().splitn(3, char::is_whitespace);
            let (Some(name), Some(item_type), Some(location)) =
                (parts.next(), parts.next(), parts.next())
            else {
                continue;
            };
            let location = posix_join(uri, location.trim_start());
            // Version 1 did not record anchors, so they are synthesised here.
            let (item_type, location) = if item_type == "mod" {
                ("py:module".to_owned(), format!("{location}#module-{name}"))
            } else {
                (format!("py:{item_type}"), format!("{location}#{name}"))
            };
            inv.insert(
                item_type,
                name.to_owned(),
                InventoryItem {
                    project_name: project_name.clone(),
                    project_version: project_version.clone(),
                    uri: location,
                    display_name: "-".to_owned(),
                },
            );
        }
        Ok(inv)
    }

    /// Parse a version 2 (zlib-compressed) inventory body.
    fn loads_v2(body: &[u8], uri: &str) -> Result<Self, InventoryError> {
        let mut cursor = body;
        let mut header = Vec::with_capacity(3);
        for _ in 0..3 {
            let Some(index) = cursor.iter().position(|b| *b == b'\n') else {
                return Err(InventoryError::MissingProjectInfo);
            };
            header.push(&cursor[..index]);
            cursor = &cursor[index + 1..];
        }
        let [project_line, version_line, check_line] = header[..] else {
            return Err(InventoryError::MissingProjectInfo);
        };

        if !check_line.windows(4).any(|w| w == b"zlib") {
            return Err(InventoryError::NotCompressed(
                String::from_utf8_lossy(check_line).into_owned(),
            ));
        }

        let project_name = header_value(&String::from_utf8_lossy(project_line));
        let project_version = header_value(&String::from_utf8_lossy(version_line));

        let mut decompressed = Vec::new();
        flate2::read::ZlibDecoder::new(cursor)
            .read_to_end(&mut decompressed)
            .map_err(|e| InventoryError::Decompress(e.to_string()))?;
        let decompressed = String::from_utf8(decompressed).map_err(|_| InventoryError::NotUtf8)?;

        let pattern = regex::Regex::new(ENTRY_PATTERN).expect("ENTRY_PATTERN is a valid regex");
        let mut inv = Inventory::default();
        // definition (case-folded) → (priority, location, display name)
        let mut potential: BTreeMap<String, (String, String, String)> = BTreeMap::new();
        let mut ambiguous: std::collections::BTreeSet<String> = Default::default();

        for line in decompressed.lines() {
            let Some(caps) = pattern.captures(line.trim_end()) else {
                continue;
            };
            let name = &caps[1];
            let item_type = &caps[2];
            let priority = &caps[3];
            let location = &caps[4];
            let display_name = &caps[5];

            // A well-formed type is `{domain}:{objtype}`. Upstream checks this
            // in code rather than in the regex to avoid a ReDoS; so do we.
            if !item_type.contains(':') {
                continue;
            }
            // Sphinx ≤ 1.1 wrote two entries per Python module; the first wins.
            if item_type == "py:module" && inv.get(item_type, name).is_some() {
                continue;
            }
            // `std:label` and `std:term` match case-insensitively, so clashing
            // case variants have to be reported.
            if item_type == "std:label" || item_type == "std:term" {
                let definition = format!("{item_type}:{name}");
                let content = (
                    priority.to_owned(),
                    location.to_owned(),
                    display_name.to_owned(),
                );
                match potential.get(&definition.to_lowercase()) {
                    Some(existing) if *existing != content => {
                        ambiguous.insert(definition);
                    }
                    Some(_) => {}
                    None => {
                        potential.insert(definition.to_lowercase(), content);
                    }
                }
            }

            // A trailing `$` abbreviates an anchor that repeats the name.
            let location = match location.strip_suffix('$') {
                Some(prefix) => format!("{prefix}{name}"),
                None => location.to_owned(),
            };
            inv.insert(
                item_type.to_owned(),
                name.to_owned(),
                InventoryItem {
                    project_name: project_name.clone(),
                    project_version: project_version.clone(),
                    uri: posix_join(uri, &location),
                    display_name: display_name.to_owned(),
                },
            );
        }

        inv.ambiguities = ambiguous.into_iter().collect();
        Ok(inv)
    }

    fn insert(&mut self, item_type: String, name: String, item: InventoryItem) {
        self.data.entry(item_type).or_default().insert(name, item);
    }

    /// Look up an object by qualified type and name.
    pub fn get(&self, item_type: &str, name: &str) -> Option<&InventoryItem> {
        self.data.get(item_type)?.get(name)
    }

    /// All objects of one qualified type, keyed by name.
    pub fn objects_of_type(&self, item_type: &str) -> Option<&BTreeMap<String, InventoryItem>> {
        self.data.get(item_type)
    }

    /// Every qualified type present in the inventory, in sorted order.
    pub fn types(&self) -> impl Iterator<Item = &str> {
        self.data.keys().map(String::as_str)
    }

    /// Iterate over every `(type, name, item)` triple.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, &InventoryItem)> {
        self.data.iter().flat_map(|(item_type, objects)| {
            objects
                .iter()
                .map(move |(name, item)| (item_type.as_str(), name.as_str(), item))
        })
    }

    /// Total number of objects across all types.
    pub fn len(&self) -> usize {
        self.data.values().map(BTreeMap::len).sum()
    }

    /// Return `true` when the inventory has no objects.
    pub fn is_empty(&self) -> bool {
        self.data.values().all(BTreeMap::is_empty)
    }

    /// `std:label` / `std:term` definitions that differ only by case and carry
    /// conflicting targets. Upstream reports these at `info` level.
    pub fn ambiguities(&self) -> &[String] {
        &self.ambiguities
    }
}

/// One row of an inventory body, as written by `InventoryFile.dump`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryEntry {
    /// Fully-qualified object name.
    pub name: String,
    /// Qualified `{domain}:{objtype}` string.
    pub item_type: String,
    /// Search priority; `-1` hides the object from search.
    pub priority: i32,
    /// Target URI relative to the project root, including any `#anchor`.
    pub uri: String,
    /// Human-readable name.
    pub display_name: String,
}

/// Serialise entries as a version 2 `objects.inv` payload.
///
/// Mirrors the body of `InventoryFile.dump`, including its two size
/// optimisations: an anchor that ends with the object name is abbreviated to
/// `$`, and a display name equal to the object name is written as `-`.
pub fn dumps(project: &str, version: &str, entries: &[InventoryEntry]) -> std::io::Result<Vec<u8>> {
    use std::io::Write as _;

    let escape = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");

    let mut out = format!(
        "# Sphinx inventory version 2\n\
         # Project: {}\n\
         # Version: {}\n\
         # The remainder of this file is compressed using zlib.\n",
        escape(project),
        escape(version),
    )
    .into_bytes();

    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::new(9));
    for entry in entries {
        let uri = match entry.uri.strip_suffix(&entry.name) {
            Some(prefix) if prefix.contains('#') => format!("{prefix}$"),
            _ => entry.uri.clone(),
        };
        let display_name = if entry.display_name == entry.name {
            "-"
        } else {
            &entry.display_name
        };
        writeln!(
            encoder,
            "{} {} {} {} {}",
            entry.name, entry.item_type, entry.priority, uri, display_name
        )?;
    }
    out.extend_from_slice(&encoder.finish()?);
    Ok(out)
}

/// Strip a `# Project: ` / `# Version: ` prefix, mirroring upstream's
/// fixed-width `[11:]` slice.
fn header_value(line: &str) -> String {
    let line = line.trim_end();
    line.char_indices()
        .nth(11)
        .map_or(String::new(), |(offset, _)| line[offset..].to_owned())
}

/// Trailing-whitespace trim for a byte slice (`bytes.rstrip()`).
fn trim_ascii_end(bytes: &[u8]) -> &[u8] {
    let mut end = bytes.len();
    while end > 0 && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    &bytes[..end]
}

/// `posixpath.join(base, path)` for the subset used by inventories: an
/// absolute `path` replaces `base`, otherwise the two are joined with `/`.
fn posix_join(base: &str, path: &str) -> String {
    if path.starts_with('/') || base.is_empty() {
        return path.to_owned();
    }
    if base.ends_with('/') {
        format!("{base}{path}")
    } else {
        format!("{base}/{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ConfigVal, SphinxConfig};
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn config_with_intersphinx(mapping: Vec<(String, ConfigVal)>) -> SphinxConfig {
        let mut raw = HashMap::new();
        raw.insert(
            "extensions".into(),
            ConfigVal::List(vec![ConfigVal::Str("sphinx.ext.intersphinx".into())]),
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
        assert!(
            cache_dir.exists(),
            "cache dir should be created even on fetch failure"
        );
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
        assert!(
            !tmp.path()
                .parent()
                .unwrap()
                .join("evil_objects.inv")
                .exists()
        );
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
        let cache_file = tmp
            .path()
            .join("__intersphinx_cache__")
            .join("localfile_objects.inv");
        assert!(!cache_file.exists(), "must not have fetched via file://");
    }
}
