//! `jinja2rs::loaders` — template source loaders.
//!
//! Ports the Jinja2 loader classes used by Sphinx:
//! - [`FileSystemLoader`] — searches one or more directories for templates.
//! - [`SphinxFileSystemLoader`] — extends `FileSystemLoader` with legacy
//!   `_t` suffix support (`.jinja` → `_t`, matching `sphinx.jinja2glue`).
//! - [`DictLoader`] — loads templates from an in-memory map (useful for
//!   testing and simple use cases).
//! - [`ChoiceLoader`] — tries multiple loaders in order until one succeeds.

use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};  // {, Path}
use std::sync::Arc;
//use std::time::SystemTime;

use crate::errors::Jinja2Error;

/// Read a template source from a list of search paths.
///
/// Returns `Ok(Some(source))` when found, `Ok(None)` if the template does not
/// exist in any search path, or `Err` on I/O failure.
///
/// # Security
///
/// This loader is hardened against the most common path-traversal patterns:
///
/// - `name` containing a NUL byte is rejected outright.
/// - The candidate is `canonicalize()`d (resolving symlinks and `..`) and
///   confirmed to live inside the canonicalized base directory. Symlinks that
///   *escape* the base are blocked; symlinks that stay inside it are allowed
///   (matching Jinja2 behavior).
/// - The subsequent read uses the canonical path, eliminating the simplest
///   string-level TOCTOU window.
/// - By default, only regular files are read. Sockets, FIFOs, and devices are
///   rejected unless `allow_special_files=true`. Directories are *always*
///   rejected, regardless of that flag.
///
/// **Residual risk:** between `canonicalize()` and `open(2)` the kernel
/// re-resolves the path string. An attacker with write access to a parent
/// directory can swap a component for a symlink in that window and redirect
/// the read. Eliminating this fully requires
/// `openat2(..., RESOLVE_BENEATH)` (Linux 5.6+) or directory-fd-relative
/// `*at` syscalls. This matches the upstream Jinja2 risk profile; sandboxed
/// deployments should use `crate::sandbox_config` for stronger guarantees.
fn load_from_paths_impl(
    paths: &[PathBuf],
    name: &str,
    allow_special_files: bool,
) -> Result<Option<String>, Jinja2Error> {
    // Reject null bytes — they can truncate paths on some platforms.
    if name.contains('\0') {
        return Ok(None);
    }

    // Also try legacy Sphinx `_t` suffix when the template ends in `.jinja`.
    let legacy_name: Option<String> = if name.ends_with(".jinja") {
        Some(format!("{}_t", &name[..name.len() - 6]))
    } else {
        None
    };

    for base in paths {
        // Canonicalize the base once per search root.  Skip roots that cannot
        // be resolved (non-existent or permission-denied directories).
        let Ok(canonical_base) = base.canonicalize() else { continue; };

        // Security: resolve `candidate` to its real, symlink-free path and
        // confirm it lives inside `canonical_base`.
        //
        // Reading from the *canonical* path (not the original `candidate`)
        // prevents TOCTOU races where a symlink is swapped between the check
        // and the read.
        //
        // When `canonicalize` fails the path does not exist (or is
        // inaccessible); we return `None` rather than falling back to an
        // unresolved `starts_with` check — `Path::starts_with` does not
        // normalise `..` components and so cannot be used as a security
        // boundary on un-canonicalized paths.
        let safe_read = |sub: PathBuf| -> Result<Option<String>, Jinja2Error> {
            match sub.canonicalize() {
                Ok(canonical) if canonical.starts_with(&canonical_base) => {
                    // Directories are *always* rejected: reading one would
                    // surface an `IsADirectory` error, and `name` values like
                    // "", ".", or ".." resolve to the base directory itself.
                    if canonical.is_dir() {
                        return Ok(None);
                    }
                    // Other non-regular files (sockets, FIFOs, devices) are
                    // rejected by default and only read when explicitly opted in.
                    if !allow_special_files && !canonical.is_file() {
                        return Ok(None);
                    }
                    Ok(Some(fs::read_to_string(&canonical)?))
                }
                Ok(_) => Ok(None),  // resolved outside the base directory
                Err(_) => Ok(None), // path does not exist or is inaccessible
            }
        };

        if let Some(source) = safe_read(base.join(name))? {
            return Ok(Some(source));
        }
        if let Some(ref legacy) = legacy_name {
            if let Some(source) = safe_read(base.join(legacy))? {
                return Ok(Some(source));
            }
        }
    }
    Ok(None)
}

/// Read a template source from a list of search paths (default: regular files only).
///
/// By default, only regular files are read. Use `load_from_paths_with_special`
/// to allow reading from sockets, FIFOs, or other special files.
pub fn load_from_paths(paths: &[PathBuf], name: &str) -> Result<Option<String>, Jinja2Error> {
    load_from_paths_impl(paths, name, false)
}

/// Read a template source from a list of search paths, optionally allowing special files.
///
/// When `allow_special_files` is true, reads from sockets, FIFOs, and other
/// non-regular files are allowed. Default (false) only reads regular files.
pub fn load_from_paths_with_special(
    paths: &[PathBuf],
    name: &str,
    allow_special_files: bool,
) -> Result<Option<String>, Jinja2Error> {
    load_from_paths_impl(paths, name, allow_special_files)
}

/// A filesystem loader that searches one or more directories for templates.
///
/// Mirrors `jinja2.FileSystemLoader`.
pub struct FileSystemLoader {
    pub(crate) paths: Arc<Vec<PathBuf>>,
    pub(crate) allow_special_files: bool,
}

impl FileSystemLoader {
    /// Create a loader rooted at a single directory.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            paths: Arc::new(vec![path.into()]),
            allow_special_files: false,
        }
    }

    /// Create a loader searching multiple directories in order.
    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self {
            paths: Arc::new(paths),
            allow_special_files: false,
        }
    }

    /// Allow reading from non-regular files (sockets, FIFOs, devices).
    ///
    /// By default, only regular files are read. Set this to true to also
    /// read from special files. This can be useful for reading from named pipes
    /// or device files, but should be used with caution in untrusted contexts.
    pub fn with_special_files(mut self, allow: bool) -> Self {
        self.allow_special_files = allow;
        self
    }

    /// Check if reading from non-regular files is allowed.
    pub fn allows_special_files(&self) -> bool {
        self.allow_special_files
    }

    /// Load a template source by name, used as a minijinja loader closure.
    pub fn load_source(paths: &Arc<Vec<PathBuf>>, name: &str) -> Option<String> {
        load_from_paths(paths, name).ok().flatten()
    }

    /// Return a minijinja-compatible loader closure for this instance.
    pub fn into_minijinja_loader(self) -> impl Fn(&str) -> Result<Option<String>, minijinja::Error> + Send + Sync + 'static {
        let paths = self.paths.clone();
        let allow_special = self.allow_special_files;
        move |name: &str| Ok(load_from_paths_with_special(&paths, name, allow_special).ok().flatten())
    }
}

/// Sphinx-specific filesystem loader.
///
/// Extends [`FileSystemLoader`] with the Sphinx convention of treating
/// templates ending in `.jinja` as potentially having a `_t`-suffixed legacy
/// fallback (e.g., `layout.html.jinja` → tries `layout.html_t` as well).
///
/// Mirrors `sphinx.jinja2glue.SphinxFileSystemLoader`.
pub struct SphinxFileSystemLoader {
    inner: FileSystemLoader,
}

impl SphinxFileSystemLoader {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            inner: FileSystemLoader::new(path),
        }
    }

    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self {
            inner: FileSystemLoader::with_paths(paths),
        }
    }

    /// Allow reading from non-regular files (sockets, FIFOs, devices).
    pub fn with_special_files(mut self, allow: bool) -> Self {
        self.inner = self.inner.with_special_files(allow);
        self
    }

    /// Return template source, attempting both `.jinja` and `_t` suffix.
    pub fn get_source(&self, name: &str) -> Result<Option<String>, Jinja2Error> {
        load_from_paths_with_special(&self.inner.paths, name, self.inner.allow_special_files)
    }

    /// Return a minijinja-compatible loader closure.
    pub fn into_minijinja_loader(self) -> impl Fn(&str) -> Result<Option<String>, minijinja::Error> + Send + Sync + 'static {
        let paths = self.inner.paths.clone();
        let allow_special = self.inner.allow_special_files;
        move |name: &str| {
            load_from_paths_with_special(&paths, name, allow_special).map_err(|e| {
                minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string())
            })
        }
    }
}

/// In-memory dictionary loader.
///
/// Mirrors `jinja2.DictLoader`. Useful for testing and simple applications
/// where templates are built into the binary or loaded once at startup.
///
/// Example:
/// ```ignore
/// let mut loader = DictLoader::new();
/// loader.insert("greeting.html", "Hello {{ name }}!");
/// env.add_template("greeting.html", "Hello {{ name }}!").unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct DictLoader {
    templates: Arc<HashMap<String, String>>,
}

impl DictLoader {
    /// Create an empty dictionary loader.
    pub fn new() -> Self {
        Self {
            templates: Arc::new(HashMap::new()),
        }
    }

    /// Create a loader from an existing map (cloned internally).
    pub fn from_map(map: HashMap<String, String>) -> Self {
        Self {
            templates: Arc::new(map),
        }
    }

    /// Load a template source by name.
    pub fn get_source(&self, name: &str) -> Option<String> {
        self.templates.get(name).cloned()
    }

    /// Return a minijinja-compatible loader closure.
    pub fn into_minijinja_loader(self) -> impl Fn(&str) -> Result<Option<String>, minijinja::Error> + Send + Sync + 'static {
        let templates = self.templates.clone();
        move |name: &str| Ok(templates.get(name).cloned())
    }
}

impl Default for DictLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Choice loader — tries multiple loaders in order.
///
/// Mirrors `jinja2.ChoiceLoader`. Useful for layering loaders with fallback
/// semantics (e.g., try user templates first, then built-in templates).
///
/// # Example
///
/// ```ignore
/// let dict = DictLoader::from_map(map! {
///     "builtin.html" => "{{ name }}"
/// });
/// let fs = FileSystemLoader::new("/path/to/templates");
/// let choice = ChoiceLoader::new(vec![
///     Arc::new(dict),
///     Arc::new(fs),
/// ]);
/// ```
pub struct ChoiceLoader {
    loaders: Vec<Arc<dyn Loader>>,
}

/// Trait for custom template loaders.
///
/// Any type implementing this trait can be composed with `ChoiceLoader`.
pub trait Loader: Send + Sync {
    /// Return the template source, or `None` if not found.
    fn get_source(&self, name: &str) -> Result<Option<String>, Jinja2Error>;
}

impl Loader for FileSystemLoader {
    fn get_source(&self, name: &str) -> Result<Option<String>, Jinja2Error> {
        load_from_paths_with_special(&self.paths, name, self.allow_special_files)
    }
}

impl Loader for SphinxFileSystemLoader {
    fn get_source(&self, name: &str) -> Result<Option<String>, Jinja2Error> {
        self.get_source(name)
    }
}

impl Loader for DictLoader {
    fn get_source(&self, name: &str) -> Result<Option<String>, Jinja2Error> {
        Ok(self.get_source(name))
    }
}

impl ChoiceLoader {
    /// Create a choice loader with an ordered list of loaders.
    pub fn new(loaders: Vec<Arc<dyn Loader>>) -> Self {
        Self { loaders }
    }

    /// Load from the first loader that returns a template.
    pub fn get_source(&self, name: &str) -> Result<Option<String>, Jinja2Error> {
        for loader in &self.loaders {
            if let Ok(Some(source)) = loader.get_source(name) {
                return Ok(Some(source));
            }
        }
        Ok(None)
    }

    /// Return a minijinja-compatible loader closure.
    pub fn into_minijinja_loader(self) -> impl Fn(&str) -> Result<Option<String>, minijinja::Error> + Send + Sync + 'static {
        let loaders = self.loaders.clone();
        move |name: &str| {
            for loader in &loaders {
                if let Ok(Some(source)) = loader.get_source(name) {
                    return Ok(Some(source));
                }
            }
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict_loader_basic() {
        let mut map = HashMap::new();
        map.insert("greeting.html".to_string(), "Hello {{ name }}!".to_string());
        let loader = DictLoader::from_map(map);

        assert_eq!(
            loader.get_source("greeting.html"),
            Some("Hello {{ name }}!".to_string())
        );
        assert_eq!(loader.get_source("missing.html"), None);
    }

    #[test]
    fn test_choice_loader_fallback() {
        use std::sync::Arc;
        let mut dict_map = HashMap::new();
        dict_map.insert("user.html".to_string(), "USER".to_string());

        let dict = DictLoader::from_map(dict_map);
        let mut builtin_map = HashMap::new();
        builtin_map.insert("builtin.html".to_string(), "BUILTIN".to_string());
        builtin_map.insert("user.html".to_string(), "BUILTIN_USER".to_string());

        let builtin = DictLoader::from_map(builtin_map);

        let choice = ChoiceLoader::new(vec![Arc::new(dict), Arc::new(builtin)]);

        // user.html is in both; should get from first loader
        assert_eq!(choice.get_source("user.html").ok().flatten(), Some("USER".to_string()));
        // builtin.html only in builtin
        assert_eq!(choice.get_source("builtin.html").ok().flatten(), Some("BUILTIN".to_string()));
        // not in either
        assert_eq!(choice.get_source("missing.html").ok().flatten(), None);
    }
}
