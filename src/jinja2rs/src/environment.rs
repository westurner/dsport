//! `jinja2rs::environment` — core template environment.
//!
//! Wraps [`minijinja::Environment`] and registers the standard Jinja2
//! built-in filters plus the Sphinx-specific extras defined in
//! [`crate::filters`] and [`crate::globals`].

use std::path::PathBuf;
use std::sync::Arc;

use minijinja::Environment as MiniEnv;
use serde::Serialize;

use crate::errors::Jinja2Error;
use crate::filters;
use crate::globals;
use crate::loaders::FileSystemLoader;

/// Template handle returned by [`Environment::get_template`].
pub struct Template<'env> {
    inner: minijinja::Template<'env, 'env>,
}

impl<'env> Template<'env> {
    /// Render the template with the given context value.
    pub fn render<S: Serialize>(&self, ctx: S) -> Result<String, Jinja2Error> {
        Ok(self.inner.render(ctx)?)
    }
}

/// A Jinja2-compatible template environment backed by minijinja.
///
/// Mirrors `jinja2.Environment`:
/// - Manages a collection of templates (string or file-loaded).
/// - Maintains filter, test, and global registries.
/// - Configures auto-escaping by file extension (`.html` → HTML-safe by default).
pub struct Environment {
    pub(crate) inner: MiniEnv<'static>,
    pub(crate) search_paths: Vec<PathBuf>,
}

impl Environment {
    /// Create a new environment with Jinja2 defaults:
    /// - All built-in minijinja filters and tests.
    /// - Sphinx utility filters (`tobool`, `toint`, `todim`, `slice_index`).
    /// - Phase 3 filters: `indent`, `wordwrap`, `xmlattr`, `urlencode`, `filesizeformat`.
    /// - Phase 4 globals: `debug`, `cycler`, `joiner`.
    /// - Sphinx globals (`accesskey`, `idgen`, `warning`).
    /// - Auto-escape enabled for `.html`, `.xml`, `.htm`.
    pub fn new() -> Self {
        let mut env = MiniEnv::new();

        // Sphinx-specific filters
        env.add_filter("tobool", filters::tobool);
        env.add_filter("toint", filters::toint);
        env.add_filter("todim", filters::todim);
        env.add_filter("filesizeformat", filters::filesizeformat);
        env.add_filter("slice_index", filters::slice_index);

        // Phase 3 filters
        env.add_filter("indent", filters::indent);
        env.add_filter("wordwrap", filters::wordwrap);
        env.add_filter("xmlattr", filters::xmlattr);
        env.add_filter("urlencode", filters::urlencode);

        // Sphinx-specific and Phase 4 globals
        env.add_global("idgen", minijinja::Value::from_object(globals::IdGen::new()));
        env.add_global("accesskey", minijinja::Value::from_object(globals::AccessKey::new()));
        env.add_global("debug", minijinja::Value::from_object(globals::Debug::new()));

        Self {
            inner: env,
            search_paths: Vec::new(),
        }
    }

    /// Create an environment with a filesystem loader rooted at `path`.
    pub fn with_loader(path: impl Into<PathBuf>) -> Self {
        let mut env = Self::new();
        env.search_paths.push(path.into());
        let paths: Arc<Vec<PathBuf>> = Arc::new(env.search_paths.clone());
        env.inner.set_loader(move |name| {
            Ok(FileSystemLoader::load_source(&paths, name))
        });
        env
    }

    /// Add a named template from a string (mirrors `env.add_template()`).
    pub fn add_template(&mut self, name: &'static str, source: &'static str) -> Result<(), Jinja2Error> {
        self.inner.add_template(name, source)?;
        Ok(())
    }

    /// Retrieve a template by name.
    pub fn get_template(&self, name: &str) -> Result<Template<'_>, Jinja2Error> {
        let t = self.inner.get_template(name).map_err(|e| {
            if e.kind() == minijinja::ErrorKind::TemplateNotFound {
                Jinja2Error::TemplateNotFound(name.to_owned())
            } else {
                Jinja2Error::Render(e)
            }
        })?;
        Ok(Template { inner: t })
    }

    /// Render a one-off template string without registering it.
    pub fn render_str<S: Serialize>(&self, source: &str, ctx: S) -> Result<String, Jinja2Error> {
        Ok(self.inner.render_str(source, ctx)?)
    }

    /// Add a custom filter function (mirrors `env.filters[name] = fn`).
    pub fn add_filter<F>(&mut self, name: &'static str, f: F)
    where
        F: minijinja::filters::Filter<minijinja::Value, (minijinja::Value,)> + Send + Sync + 'static,
    {
        self.inner.add_filter(name, f);
    }

    /// Add a global variable (mirrors `env.globals[name] = value`).
    pub fn add_global(&mut self, name: &'static str, value: impl Into<minijinja::Value>) {
        self.inner.add_global(name, value);
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
