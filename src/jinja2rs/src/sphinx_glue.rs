//! `jinja2rs::sphinx_glue` — Rust port of `sphinx.jinja2glue`.
//!
//! This module provides the [`BuiltinTemplateLoader`], a direct translation of
//! `sphinx.jinja2glue.BuiltinTemplateLoader` from Python to Rust.
//!
//! The key difference from the Python version is that template rendering runs
//! entirely within the Rust process — no Python interpreter is involved in the
//! hot path.  This eliminates GIL contention and serde serialisation overhead
//! that `minijinja-py` would otherwise incur.
//!
//! # Integration with sphinxdocrs
//!
//! `sphinxdocrs` should replace its `jinja2glue` Python import with a direct
//! call to `jinja2rs::sphinx_glue::BuiltinTemplateLoader`.  Template contexts
//! are passed as `serde_json::Value` or any `Serialize` type.

//use std::collections::HashMap;
use std::path::PathBuf; // {, Path}

use serde::Serialize;
//use serde_json::Value as JsonValue;

use crate::errors::Jinja2Error;
use crate::filters;
use crate::globals::{AccessKey, IdGen};
use crate::loaders::SphinxFileSystemLoader;
use crate::sandbox::SandboxedEnvironment;

/// Sphinx's built-in template loader and renderer.
///
/// Mirrors `sphinx.jinja2glue.BuiltinTemplateLoader`:
/// - Accepts a chain of theme + template directories.
/// - Wraps [`SandboxedEnvironment`] pre-configured with all Sphinx filters and
///   globals.
/// - Exposes [`render`] for single-page template rendering.
pub struct BuiltinTemplateLoader {
    env: SandboxedEnvironment,
    /// Ordered search paths (theme dirs first, then loader dirs).
    path_chain: Vec<PathBuf>,
}

impl BuiltinTemplateLoader {
    /// Initialise the loader for a Sphinx builder.
    ///
    /// `theme_dirs` — the theme's own directory chain (from `theme.get_theme_dirs()`).
    /// `template_paths` — explicit `templates_path` from `conf.py`.
    pub fn new(theme_dirs: Vec<PathBuf>, template_paths: Vec<PathBuf>) -> Self {
        // Loader chain: explicit template paths first, then theme dirs, then
        // parent directories of theme dirs (for theme inheritance).
        let mut loader_chain: Vec<PathBuf> = Vec::new();
        loader_chain.extend(template_paths.iter().cloned());
        loader_chain.extend(theme_dirs.iter().cloned());
        loader_chain.extend(
            theme_dirs
                .iter()
                .filter_map(|p| p.parent().map(PathBuf::from)),
        );

        let sphinx_loader = SphinxFileSystemLoader::with_paths(loader_chain.clone());
        let loader_fn = sphinx_loader.into_minijinja_loader();

        let mut env = SandboxedEnvironment::new();
        env.inner_mut().inner.set_loader(loader_fn);

        // Register Sphinx filters
        env.inner_mut().inner.add_filter("tobool", filters::tobool);
        env.inner_mut().inner.add_filter("toint", filters::toint);
        env.inner_mut().inner.add_filter("todim", filters::todim);
        env.inner_mut()
            .inner
            .add_filter("filesizeformat", filters::filesizeformat);
        env.inner_mut()
            .inner
            .add_filter("slice_index", filters::slice_index);

        // Register Sphinx globals
        env.inner_mut()
            .inner
            .add_global("idgen", minijinja::Value::from_object(IdGen::new()));
        env.inner_mut()
            .inner
            .add_global("accesskey", minijinja::Value::from_object(AccessKey::new()));

        Self {
            env,
            path_chain: loader_chain,
        }
    }

    /// Render `template_name` with the given context.
    ///
    /// Mirrors `BuiltinTemplateLoader.render(template, context)`.
    pub fn render<S: Serialize>(
        &self,
        template_name: &str,
        context: S,
    ) -> Result<String, Jinja2Error> {
        let tmpl = self.env.get_template(template_name)?;
        tmpl.render(context)
    }

    /// Return the ordered list of search directories (for debugging).
    pub fn path_chain(&self) -> &[PathBuf] {
        &self.path_chain
    }

    /// Return the mtime of the most-recently-modified template file.
    ///
    /// Mirrors `BuiltinTemplateLoader.newest_template_mtime()`.
    pub fn newest_template_mtime(&self) -> Option<std::time::SystemTime> {
        self.path_chain
            .iter()
            .flat_map(|dir| {
                std::fs::read_dir(dir).ok().into_iter().flat_map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .filter(|e| {
                            let p = e.path();
                            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                            name.ends_with(".html")
                                || name.ends_with(".jinja")
                                || name.ends_with("_t")
                        })
                        .filter_map(|e| e.metadata().ok()?.modified().ok())
                        .collect::<Vec<_>>()
                })
            })
            .max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    //use std::io::Write;

    #[test]
    fn test_render_from_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let tmpl_path = dir.path().join("page.html");
        fs::write(&tmpl_path, "Page: {{ title }}").unwrap();

        let loader = BuiltinTemplateLoader::new(vec![dir.path().to_path_buf()], vec![]);
        let out = loader
            .render("page.html", serde_json::json!({"title": "Hello World"}))
            .unwrap();
        assert_eq!(out, "Page: Hello World");
    }

    #[test]
    fn test_path_chain_ordering() {
        let theme_dir = tempfile::tempdir().expect("theme tempdir");
        let tmpl_dir = tempfile::tempdir().expect("templates tempdir");
        let loader = BuiltinTemplateLoader::new(
            vec![theme_dir.path().to_path_buf()],
            vec![tmpl_dir.path().to_path_buf()],
        );
        // template_paths come first
        assert_eq!(loader.path_chain()[0], tmpl_dir.path());
    }
}
