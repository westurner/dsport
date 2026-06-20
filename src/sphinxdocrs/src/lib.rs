//! sphinxdocrs — Rust port of Sphinx.
//!
//! Phase 4 surface:
//!
//! * [`errors`] — `SphinxError` and its subclasses, exposed as proper
//!   Python `Exception` types via `pyo3::create_exception!`.
//! * [`events`] — `EventManager`, mirroring `sphinx.events.EventManager`
//!   for the core-event surface (connect / disconnect / emit /
//!   emit_firstresult, priority ordering, `allowed_exceptions`,
//!   `app.pdb` short-circuit, `ExtensionError` wrapping).
//! * [`project`] — minimal `Project` with `path2doc` / `doc2path`.
//!
//! See [docs/sphinx-port-inventory.md](../../../docs/sphinx-port-inventory.md)
//! for the full Phase 4 port plan and per-test triage.

use pyo3::prelude::*;

pub mod apidoc;
pub mod assets;
pub mod build;
pub mod cli;
pub mod config;
pub mod errors;
pub mod events;
pub mod extension;
pub mod project;
pub mod quickstart;
pub mod util_console;
pub mod util_matching;

/// Crate version string. Mirrors `Cargo.toml` `[package].version`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction(name = "version")]
fn py_version() -> &'static str {
    version()
}

/// Coarse capability flags advertised by the Rust port. Mirrors
/// docutilsrs' `features()` pattern so a hybrid wrapper can probe
/// without importing internals.
pub fn features() -> &'static [&'static str] {
    &[
        "errors:sphinx_hierarchy",
        "events:event_manager",
        "events:emit_firstresult",
        "project:path2doc",
        "project:doc2path",
        "project:discover",
        "extension:wrapper",
        "extension:verify_needs_extensions",
        "util:matching",
        "util:console",
        "config:read_conf_py",
        "config:mathjax_path",
        "config:imgmath",
        "math:mathjax",
        "math:imgmath",
        "math:ratex",
        "assets:fetch_and_cache",
        "assets:sri_hash",
        "assets:sri_hash_file",
        "assets:fetch_with_integrity",
    ]
}

#[pyfunction(name = "features")]
fn py_features() -> Vec<&'static str> {
    features().to_vec()
}

#[pyfunction(name = "supports")]
fn py_supports(feature: &str) -> bool {
    features().contains(&feature)
}

#[pymodule]
fn sphinxdocrs(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    m.add_function(wrap_pyfunction!(py_features, m)?)?;
    m.add_function(wrap_pyfunction!(py_supports, m)?)?;
    m.add_class::<events::EventManager>()?;
    m.add_class::<project::Project>()?;
    m.add_class::<extension::Extension>()?;
    m.add_function(wrap_pyfunction!(extension::py_verify_needs_extensions, m)?)?;
    util_matching::register(m)?;
    util_console::register(m)?;
    m.add_function(wrap_pyfunction!(config::py_read_conf_py, m)?)?;
    m.add_function(wrap_pyfunction!(assets::py_cache_path_for, m)?)?;
    m.add_function(wrap_pyfunction!(assets::py_fetch_and_cache, m)?)?;
    m.add_function(wrap_pyfunction!(assets::py_sri_hash, m)?)?;
    m.add_function(wrap_pyfunction!(assets::py_sri_hash_file, m)?)?;
    m.add_function(wrap_pyfunction!(assets::py_fetch_with_integrity, m)?)?;

    // Exception types
    m.add("SphinxError", py.get_type::<errors::SphinxError>())?;
    m.add("SphinxWarning", py.get_type::<errors::SphinxWarning>())?;
    m.add(
        "ApplicationError",
        py.get_type::<errors::ApplicationError>(),
    )?;
    m.add("ExtensionError", py.get_type::<errors::ExtensionError>())?;
    m.add(
        "BuildEnvironmentError",
        py.get_type::<errors::BuildEnvironmentError>(),
    )?;
    m.add("ConfigError", py.get_type::<errors::ConfigError>())?;
    m.add("DocumentError", py.get_type::<errors::DocumentError>())?;
    m.add("ThemeError", py.get_type::<errors::ThemeError>())?;
    m.add(
        "VersionRequirementError",
        py.get_type::<errors::VersionRequirementError>(),
    )?;
    m.add(
        "SphinxParallelError",
        py.get_type::<errors::SphinxParallelError>(),
    )?;
    m.add("PycodeError", py.get_type::<errors::PycodeError>())?;
    m.add("NoUri", py.get_type::<errors::NoUri>())?;
    m.add(
        "FiletypeNotFoundError",
        py.get_type::<errors::FiletypeNotFoundError>(),
    )?;
    m.add_function(wrap_pyfunction!(py_main, m)?)?;
    m.add_function(wrap_pyfunction!(py_sphinx_quickstart, m)?)?;
    m.add_function(wrap_pyfunction!(py_sphinx_apidoc, m)?)?;
    m.add_function(wrap_pyfunction!(py_sphinx_autogen, m)?)?;
    Ok(())
}

#[pyfunction(name = "main")]
fn py_main() {
    println!("stub running");
}
#[pyfunction(name = "sphinx_quickstart")]
fn py_sphinx_quickstart() {
    println!("stub running");
}
#[pyfunction(name = "sphinx_apidoc")]
fn py_sphinx_apidoc() {
    println!("stub running");
}
#[pyfunction(name = "sphinx_autogen")]
fn py_sphinx_autogen() {
    println!("stub running");
}
