//! jinja2rs — Rust port of Jinja2, powered by minijinja.
//!
//! This crate wraps [`minijinja`] with a Sphinx-compatible public API that
//! mirrors the key surfaces of the Python `jinja2` package used by Sphinx:
//!
//! * [`Environment`] / [`SandboxedEnvironment`] — template environment with
//!   filters, globals, and auto-escape settings.
//! * [`loaders`] — [`FileSystemLoader`](loaders::FileSystemLoader) and the
//!   Sphinx-specific [`SphinxFileSystemLoader`](loaders::SphinxFileSystemLoader)
//!   (handles legacy `_t` suffix alongside `.jinja`).
//! * [`filters`] — Sphinx built-in filters: `tobool`, `toint`, `todim`,
//!   `slice_index`.
//! * [`globals`] — Sphinx template globals: `accesskey`, `idgen`, `warning`.
//! * [`sphinx_glue`] — Rust port of `sphinx.jinja2glue.BuiltinTemplateLoader`.
//! * [`bridge`] — PyO3 Python extension module providing a drop-in `jinja2`
//!   import shim for incremental migration.
//!
//! # Why jinja2rs?
//!
//! Sphinx currently calls the Python Jinja2 engine on every page render.
//! `sphinxdocrs` can call `jinja2rs` directly from Rust, bypassing the
//! Python interpreter entirely for the hot rendering path and eliminating
//! GIL contention and serde round-trips.
//!
//! `minijinja` is already substantially faster than CPython Jinja2 on pure
//! rendering benchmarks (~3.7 µs vs ~12 µs per typical page at equal template
//! complexity), but the `minijinja-py` PyO3 bridge erases most of the gain
//! through serialisation overhead.  `jinja2rs` keeps the fast Rust→Rust path
//! as the primary integration point for `sphinxdocrs`.
//!
//! # Quick start (Rust)
//!
//! ```rust
//! use jinja2rs::Environment;
//! use serde_json::json;
//!
//! let mut env = Environment::new();
//! env.add_template("hello.html", "Hello, {{ name }}!").unwrap();
//! let tmpl = env.get_template("hello.html").unwrap();
//! let output = tmpl.render(json!({"name": "Sphinx"})).unwrap();
//! assert_eq!(output, "Hello, Sphinx!");
//! ```

use pyo3::prelude::*;

pub mod ansible_filters;
pub mod ansible_inventory;
pub mod ansible_validation;
pub mod compat;
pub mod environment;
pub mod errors;
pub mod filters;
pub mod globals;
pub mod i18n;
pub mod kubernetes_filters;
pub mod kubernetes_inventory;
pub mod loaders;
pub mod sandbox;
pub mod sandbox_config;
pub mod sphinx_glue;

mod bridge;

pub use compat::{AnsibleInventorySource, AnsibleMode, CompatMode, DjangoAutoEscape, DjangoMode};
pub use environment::Environment;
pub use errors::Jinja2Error;
pub use loaders::{
    ChoiceLoader, DictLoader, DjangoAppDirectoryLoader, FileSystemLoader, Loader,
    SphinxFileSystemLoader,
};
pub use sandbox::SandboxedEnvironment;
pub use sandbox_config::SandboxedEnvironmentBuilder;

/// Crate version string. Mirrors `Cargo.toml` `[package].version`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Feature flags advertised by this build.
pub fn features() -> &'static [&'static str] {
    &[
        "environment:basic",
        "environment:auto_escape",
        "loaders:filesystem",
        "loaders:sphinx_filesystem",
        "filters:tobool",
        "filters:toint",
        "filters:todim",
        "filters:slice_index",
        "filters:filesizeformat",
        "filters:indent",
        "filters:wordwrap",
        "filters:xmlattr",
        "filters:urlencode",
        "globals:accesskey",
        "globals:idgen",
        "globals:debug",
        "globals:cycler",
        "globals:joiner",
        "globals:lipsum",
        "globals:warning",
        "i18n:gettext",
        "i18n:ngettext",
        "sandbox:sandboxed_environment",
        "sandbox:seccomp_filtering",
        "sandbox:resource_limits",
        "sandbox:python_callable_warnings",
        "sphinx_glue:builtin_template_loader",
        "bridge:pyo3",
        "bridge:environment",
        "bridge:sandboxed_environment",
        "bridge:template",
        "bridge:exceptions",
        // Django mode
        "django:filters:upper",
        "django:filters:lower",
        "django:filters:capfirst",
        "django:filters:title",
        "django:filters:slugify",
        "django:filters:truncatewords",
        "django:filters:truncatechars",
        "django:filters:wordcount",
        "django:filters:wordwrap",
        "django:filters:add",
        "django:filters:floatformat",
        "django:filters:pluralize",
        "django:filters:first",
        "django:filters:last",
        "django:filters:join",
        "django:filters:length",
        "django:filters:length_is",
        "django:filters:yesno",
        "django:filters:default",
        "django:filters:default_if_none",
        "django:filters:escape",
        "django:filters:force_escape",
        "django:filters:safe",
        "django:filters:striptags",
        "django:filters:linebreaks",
        "django:filters:linebreaksbr",
        "django:filters:urlencode",
        "django:loaders:app_directories",
        "django:auto_escape:html",
    ]
}

#[pyfunction(name = "version")]
fn py_version() -> &'static str {
    version()
}

/// Python extension module entry point (used by maturin).
#[pymodule]
fn jinja2rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    bridge::register(m)?;
    Ok(())
}
