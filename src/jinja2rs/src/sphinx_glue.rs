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
        // Wrap the loader so every template loaded from disk passes through
        // the `{% trans %}` desugaring pass (see `crate::trans`) before
        // minijinja parses it.
        let loader_fn = move |name: &str| match loader_fn(name)? {
            Some(src) => Ok(Some(crate::trans::preprocess_trans(&src).into_owned())),
            None => Ok(None),
        };

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

    /// Install a `{% trans %}` translation provider (see [`crate::trans`]).
    ///
    /// Optional: without one, `{% trans %}` blocks still render (in the
    /// template's source language). `sphinxdocrs` may install a real
    /// `.mo`/`.po`-backed provider here, or leave it unset.
    pub fn set_trans_provider(&mut self, provider: crate::i18n::I18nProvider) {
        self.env.inner_mut().set_trans_provider(provider);
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
    use std::fmt;
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

    /// A stub `pathto(name, resource=False)` global: real per-page relative
    /// URL computation lives on the `sphinxdocrs` side (H6c); here it just
    /// proves the *template* calls it correctly.
    struct StubPathto;
    impl fmt::Debug for StubPathto {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "<StubPathto>")
        }
    }
    impl minijinja::value::Object for StubPathto {
        fn call(
            self: &std::sync::Arc<Self>,
            _state: &minijinja::State<'_, '_>,
            args: &[minijinja::Value],
        ) -> Result<minijinja::Value, minijinja::Error> {
            let name = args.first().map(|v| v.to_string()).unwrap_or_default();
            Ok(minijinja::Value::from(format!("{name}.html")))
        }
    }

    /// End-to-end smoke test: render **real, unmodified** upstream Sphinx
    /// `basic` theme sidebar partials straight from
    /// `src/sphinx/sphinx/themes/basic` through `jinja2rs` — proving `_()`,
    /// the `{% trans %}` desugaring pass, and ordinary Jinja2 constructs
    /// (`if`/`for`, `|e`) work together against genuine Sphinx template
    /// source, not a hand-written fixture.
    ///
    /// `pathto`/`hasdoc` are minimal stubs (real per-page implementations
    /// are `sphinxdocrs`'s responsibility, H6c) — this test only proves the
    /// jinja2rs-level template mechanics. See
    /// `renders_real_sphinx_basic_theme_full_layout` below for the full
    /// `layout.html`/`page.html` chain (which additionally exercises
    /// `{% block %}` tags nested inside `{% macro %}` bodies, e.g.
    /// `relbar()`'s `{% block rootrellink %}` — now supported by the
    /// vendored `minijinja` fork).
    #[test]
    fn renders_real_sphinx_basic_theme_sidebar_partials() {
        let theme_dir = PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../sphinx/sphinx/themes/basic"
        ));
        if !theme_dir.is_dir() {
            eprintln!("skipping: {theme_dir:?} not present in this checkout");
            return;
        }

        let mut loader = BuiltinTemplateLoader::new(vec![theme_dir], vec![]);
        loader
            .env
            .inner_mut()
            .inner
            .add_global("pathto", minijinja::Value::from_object(StubPathto));

        // localtoc.html — `{{ _('Table of Contents') }}`.
        let out = loader
            .render(
                "localtoc.html",
                serde_json::json!({"display_toc": true, "root_doc": "index", "toc": "<ul></ul>"}),
            )
            .unwrap();
        assert!(out.contains("Table of Contents"), "got:\n{out}");

        // relations.html — `{{ _('Previous topic') }}` / `{{ _('Next topic') }}`.
        let out = loader
            .render(
                "relations.html",
                serde_json::json!({
                    "prev": {"link": "a.html", "title": "A"},
                    "next": {"link": "b.html", "title": "B"},
                }),
            )
            .unwrap();
        assert!(out.contains("Previous topic"), "got:\n{out}");
        assert!(out.contains("Next topic"), "got:\n{out}");

        // sourcelink.html — `{{ _('Show Source') }}`.
        let out = loader
            .render(
                "sourcelink.html",
                serde_json::json!({
                    "show_source": true, "has_source": true, "sourcename": "index.rst.txt",
                }),
            )
            .unwrap();
        assert!(out.contains("Show Source"), "got:\n{out}");

        // searchbox.html — `{{ _('Quick search') }}` / `{{ _('Go') }}`.
        let out = loader
            .render(
                "searchbox.html",
                serde_json::json!({"pagename": "index", "builder": "html"}),
            )
            .unwrap();
        assert!(out.contains("Quick search"), "got:\n{out}");
        assert!(out.contains("value=\"Go\""), "got:\n{out}");
    }

    /// A stub `hasdoc(name)` global — always `false`, skipping the
    /// `linktags` block's optional `<link>` entries.
    struct StubHasdoc;
    impl fmt::Debug for StubHasdoc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "<StubHasdoc>")
        }
    }
    impl minijinja::value::Object for StubHasdoc {
        fn call(
            self: &std::sync::Arc<Self>,
            _state: &minijinja::State<'_, '_>,
            _args: &[minijinja::Value],
        ) -> Result<minijinja::Value, minijinja::Error> {
            Ok(minijinja::Value::from(false))
        }
    }

    /// End-to-end smoke test: render the **real, unmodified** upstream
    /// Sphinx `basic` theme's `page.html` \u2192 `layout.html` chain (which
    /// `{% extends %}`s and pulls in the `relbar()`/`sidebar()`/`css()`/
    /// `script()` macros \u2014 including `relbar()`'s `{% block rootrellink %}`
    /// nested inside the macro body) straight from
    /// `src/sphinx/sphinx/themes/basic`.
    ///
    /// This is the case that originally failed with minijinja's "block tags
    /// in macros are not allowed" restriction; the vendored `minijinja` fork
    /// now compiles and runs it (see `compiler/parser.rs::parse_block` and
    /// `vm/mod.rs::eval_macro`, which propagates the caller's block map into
    /// the macro's execution state instead of using an empty one).
    #[test]
    fn renders_real_sphinx_basic_theme_full_layout() {
        let theme_dir = PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../sphinx/sphinx/themes/basic"
        ));
        if !theme_dir.is_dir() {
            eprintln!("skipping: {theme_dir:?} not present in this checkout");
            return;
        }

        let mut loader = BuiltinTemplateLoader::new(vec![theme_dir], vec![]);
        loader
            .env
            .inner_mut()
            .inner
            .add_global("pathto", minijinja::Value::from_object(StubPathto));
        loader
            .env
            .inner_mut()
            .inner
            .add_global("hasdoc", minijinja::Value::from_object(StubHasdoc));

        let ctx = serde_json::json!({
            "embedded": false,
            "theme_nosidebar": false,
            "sidebars": [],
            "docstitle": "Test Docs",
            "title": "Welcome",
            "titlesuffix": "",
            "encoding": "utf-8",
            "metatags": "",
            "css_files": [],
            "script_files": [],
            "use_opensearch": false,
            "favicon_url": null,
            "pageurl": null,
            "root_doc": "index",
            "shorttitle": "Test Docs",
            "parents": [],
            "rellinks": [],
            "link": "index.html",
            "logo_url": null,
            "show_source": false,
            "has_source": false,
            "sourcename": "",
            "show_copyright": true,
            "copyright": "2026, Test",
            "last_updated": null,
            "language": "en",
            "content_root": "",
            "pagename": "index",
            "builder": "html",
            "display_toc": false,
            "toc": "",
            "next": null,
            "prev": null,
            "body": "<p>Hello, world!</p>",
            "html_tag": null,
            "show_sphinx": false,
            "sphinx_version": "0.0.0",
        });

        let out = loader.render("page.html", ctx).unwrap();
        assert!(out.contains("<!DOCTYPE html>"), "got:\n{out}");
        // `body` is passed as a plain (non-`Markup`-safe) JSON string here,
        // so it is HTML-escaped like any other auto-escaped `{{ }}` output —
        // in real `sphinxdocrs` usage `body` is already-rendered HTML5 and
        // would be passed through as a safe value instead. This assertion
        // only checks the text content survived the round trip.
        assert!(out.contains("Hello, world!"), "got:\n{out}");
        // `relbar()`'s `{% block rootrellink %}` — proves blocks-in-macros work.
        assert!(out.contains("nav-item-0"), "got:\n{out}");
        // `title|striptags|e` in `{% block htmltitle %}`.
        assert!(out.contains("<title>Welcome"), "got:\n{out}");
    }
}
