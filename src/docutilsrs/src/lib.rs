//! docutilsrs — Rust port of docutils.
//!
//! M1 surface: a `version()` function exported to Python via PyO3,
//! sufficient to validate the build, packaging, and import loop.

use pyo3::prelude::*;

/// Crate version string. Mirrors `Cargo.toml` `[package].version`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction(name = "version")]
fn py_version() -> &'static str {
    version()
}

#[pymodule]
fn docutilsrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    Ok(())
}
