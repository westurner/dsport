//! `jinja2rs::loaders` — template source loaders.
//!
//! Ports the Jinja2 loader classes used by Sphinx:
//! - [`FileSystemLoader`] — searches one or more directories for templates.
//! - [`SphinxFileSystemLoader`] — extends `FileSystemLoader` with legacy
//!   `_t` suffix support (`.jinja` → `_t`, matching `sphinx.jinja2glue`).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

use crate::errors::Jinja2Error;

/// Read a template source from a list of search paths.
///
/// Returns `Ok(Some(source))` when found, `Ok(None)` if the template does not
/// exist in any search path, or `Err` on I/O failure.
pub fn load_from_paths(paths: &[PathBuf], name: &str) -> Result<Option<String>, Jinja2Error> {
    // Also try legacy Sphinx `_t` suffix when the template ends in `.jinja`.
    let legacy_name: Option<String> = if name.ends_with(".jinja") {
        Some(format!("{}_t", &name[..name.len() - 6]))
    } else {
        None
    };

    for base in paths {
        let candidate = base.join(name);
        if candidate.exists() {
            let source = fs::read_to_string(&candidate)?;
            return Ok(Some(source));
        }
        if let Some(ref legacy) = legacy_name {
            let legacy_path = base.join(legacy);
            if legacy_path.exists() {
                let source = fs::read_to_string(&legacy_path)?;
                return Ok(Some(source));
            }
        }
    }
    Ok(None)
}

/// A filesystem loader that searches one or more directories for templates.
///
/// Mirrors `jinja2.FileSystemLoader`.
pub struct FileSystemLoader {
    pub(crate) paths: Arc<Vec<PathBuf>>,
}

impl FileSystemLoader {
    /// Create a loader rooted at a single directory.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            paths: Arc::new(vec![path.into()]),
        }
    }

    /// Create a loader searching multiple directories in order.
    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self {
            paths: Arc::new(paths),
        }
    }

    /// Load a template source by name, used as a minijinja loader closure.
    pub fn load_source(paths: &Arc<Vec<PathBuf>>, name: &str) -> Option<String> {
        load_from_paths(paths, name).ok().flatten()
    }

    /// Return a minijinja-compatible loader closure for this instance.
    pub fn into_minijinja_loader(self) -> impl Fn(&str) -> Result<Option<String>, minijinja::Error> + Send + Sync + 'static {
        let paths = self.paths.clone();
        move |name: &str| Ok(Self::load_source(&paths, name))
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

    /// Return template source, attempting both `.jinja` and `_t` suffix.
    pub fn get_source(&self, name: &str) -> Result<Option<String>, Jinja2Error> {
        load_from_paths(&self.inner.paths, name)
    }

    /// Return a minijinja-compatible loader closure.
    pub fn into_minijinja_loader(self) -> impl Fn(&str) -> Result<Option<String>, minijinja::Error> + Send + Sync + 'static {
        let paths = self.inner.paths.clone();
        move |name: &str| {
            load_from_paths(&paths, name).map_err(|e| {
                minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string())
            })
        }
    }
}
