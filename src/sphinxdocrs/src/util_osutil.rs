//! `sphinxdocrs::util_osutil` — Rust port of `sphinx.util.osutil`.
//!
//! Operating-system path and file utilities used by multiple Sphinx subsystems.
//! All functions here are **pure** (no I/O) or have explicit I/O via
//! `std::fs` so they remain easily testable.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `SEP` | [`SEP`] | canonical path separator `'/'` |
//! | `os_path(canonical)` | [`os_path`] | canonical → OS-native separators |
//! | `canon_path(native)` | [`canon_path`] | OS-native → canonical (forward slashes) |
//! | `path_stabilize(path)` | [`path_stabilize`] | `canon_path` + NFC normalise |
//! | `relative_uri(base, to)` | [`relative_uri`] | compute a relative URL |
//! | `ensuredir(path)` | [`ensuredir`] | `mkdir -p` |
//! | `make_filename(s)` | [`make_filename`] | strip non-`[a-zA-Z0-9_-]` chars |
//! | `make_filename_from_project(project)` | [`make_filename_from_project`] | project → safe filename |
//! | `FileAvoidWrite` | [`FileAvoidWrite`] | buffer + write-only-if-changed |
//!
//! **Deferred** (not yet needed or requires OS-level file operations outside
//! the P2 scope): `_last_modified_time`, `_copy_times`, `copyfile`, `relpath`,
//! `_relative_path`, `rmtree`.

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use unicode_normalization::UnicodeNormalization;

// ── SEP ───────────────────────────────────────────────────────────────────────

/// Canonical path separator used throughout Sphinx.
///
/// Mirrors `SEP = '/'` in `sphinx.util.osutil`.
pub const SEP: char = '/';

// ── path conversions ──────────────────────────────────────────────────────────

/// Convert a canonical Sphinx path (forward slashes) to the OS-native
/// path separator.
///
/// Mirrors `sphinx.util.osutil.os_path`.
///
/// On POSIX this is a no-op.  On Windows, `/` → `\`.
pub fn os_path(canonical_path: &str) -> String {
    #[cfg(windows)]
    return canonical_path.replace('/', std::path::MAIN_SEPARATOR_STR);
    #[cfg(not(windows))]
    canonical_path.to_string()
}

/// Convert a native OS path to canonical form (forward slashes).
///
/// Mirrors `sphinx.util.osutil.canon_path`.
pub fn canon_path(native_path: &(impl AsRef<Path> + ?Sized)) -> String {
    native_path
        .as_ref()
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/")
}

/// Normalize path separator and apply Unicode NFC normalization.
///
/// Mirrors `sphinx.util.osutil.path_stabilize`.
pub fn path_stabilize(filepath: &(impl AsRef<Path> + ?Sized)) -> String {
    let canonical = canon_path(filepath);
    canonical.nfc().collect()
}

// ── relative_uri ─────────────────────────────────────────────────────────────

/// Compute a relative URL from `base` to `to`.
///
/// If `to` starts with `/` it is returned as-is (absolute URL).
///
/// Mirrors `sphinx.util.osutil.relative_uri`.
///
/// ```rust
/// use sphinxdocrs::util_osutil::relative_uri;
/// assert_eq!(relative_uri("a/b/c.html", "a/b/d.html"), "d.html");
/// assert_eq!(relative_uri("a/b/index.html", "a/c/d.html"), "../c/d.html");
/// assert_eq!(relative_uri("f/index.html", "f/index.html"), "");
/// assert_eq!(relative_uri("f/index.html", "f/"), "./");
/// assert_eq!(relative_uri("x.html", "/abs.html"), "/abs.html");
/// ```
pub fn relative_uri(base: &str, to: &str) -> String {
    if to.starts_with('/') {
        return to.to_string();
    }
    let base_path = base.split('#').next().unwrap_or(base);
    let to_path = to.split('#').next().unwrap_or(to);

    let mut b2: Vec<&str> = base_path.split('/').collect();
    let mut t2: Vec<&str> = to_path.split('/').collect();

    // remove common leading segments (except the last segment of each)
    loop {
        if b2.len() <= 1 || t2.len() <= 1 {
            break;
        }
        if b2[0] == t2[0] {
            b2.remove(0);
            t2.remove(0);
        } else {
            break;
        }
    }

    if b2 == t2 {
        // special case: same path
        return String::new();
    }

    if b2.len() == 1 && t2 == [""] {
        // special case: relative_uri('f/index.html','f/') → './'
        return format!(".{SEP}");
    }

    let up_count = b2.len().saturating_sub(1);
    let prefix = format!("..{SEP}").repeat(up_count);
    format!("{}{}", prefix, t2.join(&SEP.to_string()))
}

// ── ensuredir ─────────────────────────────────────────────────────────────────

/// Create `file` and all parent directories, like `mkdir -p`.
///
/// Mirrors `sphinx.util.osutil.ensuredir`.
///
/// # Errors
/// Returns `io::Error` if the directory cannot be created.
pub fn ensuredir(path: impl AsRef<Path>) -> io::Result<()> {
    std::fs::create_dir_all(path)
}

// ── make_filename ─────────────────────────────────────────────────────────────

/// Strip characters not in `[a-zA-Z0-9_-]` from `string`.
///
/// Returns `"sphinx"` if the result would be empty.
///
/// Mirrors `sphinx.util.osutil.make_filename`.
///
/// ```rust
/// use sphinxdocrs::util_osutil::make_filename;
/// assert_eq!(make_filename("My Project!"), "MyProject");
/// assert_eq!(make_filename(""), "sphinx");
/// assert_eq!(make_filename("hello-world_2"), "hello-world_2");
/// ```
pub fn make_filename(string: &str) -> String {
    let out: String = string
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    if out.is_empty() {
        "sphinx".to_string()
    } else {
        out
    }
}

/// Convert a Sphinx project name to a safe filename (lowercase, stripped).
///
/// Removes a trailing `" Documentation"` suffix before calling
/// [`make_filename`], then lowercases the result.
///
/// Mirrors `sphinx.util.osutil.make_filename_from_project`.
///
/// ```rust
/// use sphinxdocrs::util_osutil::make_filename_from_project;
/// assert_eq!(make_filename_from_project("Sphinx Documentation"), "sphinx");
/// assert_eq!(make_filename_from_project("My Project"), "myproject");
/// ```
pub fn make_filename_from_project(project: &str) -> String {
    let trimmed = project.strip_suffix(" Documentation").unwrap_or(project);
    make_filename(trimmed).to_lowercase()
}

// ── FileAvoidWrite ────────────────────────────────────────────────────────────

/// Write-buffering wrapper that skips the write if file content is unchanged.
///
/// Mirrors `sphinx.util.osutil.FileAvoidWrite`.
///
/// Accumulate content via [`Write`] / [`std::fmt::Write`]; call
/// [`FileAvoidWrite::close`] (or drop) to commit.
#[derive(Debug)]
pub struct FileAvoidWrite {
    path: PathBuf,
    buf: Vec<u8>,
}

impl FileAvoidWrite {
    /// Create a new `FileAvoidWrite` for `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        FileAvoidWrite {
            path: path.into(),
            buf: Vec::new(),
        }
    }

    /// Return the buffered content as a string slice.
    pub fn get_value(&self) -> &[u8] {
        &self.buf
    }

    /// Commit: write the buffer to disk only if it differs from the current
    /// file content (or if the file does not exist).
    ///
    /// # Errors
    /// Returns `io::Error` on read or write failure.
    pub fn close(self) -> io::Result<()> {
        if let Ok(existing) = std::fs::read(&self.path) {
            if existing == self.buf {
                return Ok(());
            }
        }
        std::fs::write(&self.path, &self.buf)
    }
}

impl Write for FileAvoidWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ── SEP ───────────────────────────────────────────────────────────────────

    #[test]
    fn sep_is_forward_slash() {
        assert_eq!(SEP, '/');
    }

    // ── os_path ───────────────────────────────────────────────────────────────

    #[test]
    fn os_path_posix_no_change() {
        // On POSIX, os_path('a/b/c') == 'a/b/c'
        #[cfg(not(windows))]
        assert_eq!(os_path("a/b/c"), "a/b/c");
    }

    // ── canon_path ────────────────────────────────────────────────────────────

    #[test]
    fn canon_path_string() {
        assert_eq!(canon_path("a/b/c"), "a/b/c");
    }

    #[test]
    fn canon_path_pathbuf() {
        let p = PathBuf::from("docs/api");
        assert_eq!(canon_path(&p), "docs/api");
    }

    // ── path_stabilize ────────────────────────────────────────────────────────

    #[test]
    fn path_stabilize_ascii() {
        assert_eq!(path_stabilize("a/b/c"), "a/b/c");
    }

    #[test]
    fn path_stabilize_nfc() {
        // NFC normalisation: precomposed 'é' (U+00E9) should remain as-is.
        assert_eq!(path_stabilize("caf\u{00e9}"), "caf\u{00e9}");
        // NFD form ('e' + combining acute) should be normalised to NFC.
        let nfd = "cafe\u{0301}"; // 'e' + COMBINING ACUTE ACCENT
        assert_eq!(path_stabilize(nfd), "caf\u{00e9}");
    }

    // ── relative_uri ─────────────────────────────────────────────────────────

    #[test]
    fn relative_uri_same_dir() {
        // Mirrors test_util.py: relative_uri('a/b/c.html', 'a/b/d.html') == 'd.html'
        assert_eq!(relative_uri("a/b/c.html", "a/b/d.html"), "d.html");
    }

    #[test]
    fn relative_uri_up_one() {
        // Mirrors: relative_uri('a/b/index.html', 'a/c/d.html') == '../c/d.html'
        assert_eq!(relative_uri("a/b/index.html", "a/c/d.html"), "../c/d.html");
    }

    #[test]
    fn relative_uri_up_two() {
        assert_eq!(relative_uri("a/b/c/d.html", "a/e.html"), "../../e.html");
    }

    #[test]
    fn relative_uri_same_file() {
        // Mirrors: relative_uri('f/index.html','f/index.html') == ''
        assert_eq!(relative_uri("f/index.html", "f/index.html"), "");
    }

    #[test]
    fn relative_uri_trailing_slash() {
        // Mirrors: relative_uri('f/index.html','f/') → './'
        assert_eq!(relative_uri("f/index.html", "f/"), "./");
    }

    #[test]
    fn relative_uri_absolute_to() {
        // If 'to' starts with '/', return it unchanged.
        assert_eq!(relative_uri("x.html", "/abs.html"), "/abs.html");
    }

    #[test]
    fn relative_uri_root_to_subdir() {
        assert_eq!(
            relative_uri("index.html", "api/module.html"),
            "api/module.html"
        );
    }

    // ── ensuredir ─────────────────────────────────────────────────────────────

    #[test]
    fn ensuredir_creates_nested() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("a").join("b").join("c");
        assert!(!path.exists());
        ensuredir(&path).unwrap();
        assert!(path.is_dir());
    }

    #[test]
    fn ensuredir_existing_is_ok() {
        let tmp = tempfile::tempdir().unwrap();
        ensuredir(tmp.path()).unwrap(); // already exists — should be a no-op
        assert!(tmp.path().is_dir());
    }

    // ── make_filename ─────────────────────────────────────────────────────────

    #[test]
    fn make_filename_strips_specials() {
        assert_eq!(make_filename("My Project!"), "MyProject");
    }

    #[test]
    fn make_filename_allows_safe_chars() {
        assert_eq!(make_filename("hello-world_2"), "hello-world_2");
    }

    #[test]
    fn make_filename_empty_returns_sphinx() {
        assert_eq!(make_filename(""), "sphinx");
        assert_eq!(make_filename("!!!"), "sphinx");
    }

    #[test]
    fn make_filename_alphanumeric_only() {
        assert_eq!(make_filename("abc123"), "abc123");
    }

    // ── make_filename_from_project ────────────────────────────────────────────

    #[test]
    fn make_filename_from_project_strips_suffix() {
        // Mirrors: make_filename_from_project('Sphinx Documentation') == 'sphinx'
        assert_eq!(make_filename_from_project("Sphinx Documentation"), "sphinx");
    }

    #[test]
    fn make_filename_from_project_no_suffix() {
        assert_eq!(make_filename_from_project("My Project"), "myproject");
    }

    #[test]
    fn make_filename_from_project_lowercase() {
        assert_eq!(make_filename_from_project("HelloWorld"), "helloworld");
    }

    // ── FileAvoidWrite ────────────────────────────────────────────────────────

    #[test]
    fn file_avoid_write_creates_new_file() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("output.txt");
        let mut w = FileAvoidWrite::new(&path);
        write!(w, "hello").unwrap();
        w.close().unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "hello");
    }

    #[test]
    fn file_avoid_write_skips_when_same() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("output.txt");
        std::fs::write(&path, "hello").unwrap();
        let mtime_before = std::fs::metadata(&path).unwrap().modified().ok();

        let mut w = FileAvoidWrite::new(&path);
        write!(w, "hello").unwrap();
        w.close().unwrap();

        // Content unchanged — mtime should not change (best-effort check).
        let mtime_after = std::fs::metadata(&path).unwrap().modified().ok();
        assert_eq!(
            mtime_before, mtime_after,
            "file should not have been rewritten"
        );
    }

    #[test]
    fn file_avoid_write_overwrites_when_different() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("output.txt");
        std::fs::write(&path, "old content").unwrap();

        let mut w = FileAvoidWrite::new(&path);
        write!(w, "new content").unwrap();
        w.close().unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "new content");
    }

    #[test]
    fn file_avoid_write_empty_buf_creates_empty_file() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("empty.txt");
        let w = FileAvoidWrite::new(&path);
        w.close().unwrap();
        // Should create an empty file (our impl writes empty byte slice).
        assert_eq!(std::fs::read(&path).unwrap(), b"");
    }
}
