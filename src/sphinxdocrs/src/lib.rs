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

pub mod errors;
pub mod events;
pub mod project;

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
    Ok(())
}
