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
        load_from_paths(&self.paths, name)
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
