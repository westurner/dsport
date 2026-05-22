//! docutilsrs — Rust port of docutils.
//!
//! Phase 1 surface: a paragraph + inline-markup parser slice, with a
//! pseudo-XML writer for parity comparison against vendored docutils.

use pyo3::prelude::*;

pub mod doctree;
pub mod parser;
mod python;
pub mod writer;

pub use doctree::{Doctree, NodeKind};
pub use parser::{parse_rst, parse_rst_with_source};
pub use writer::pseudo_xml;

/// Crate version string. Mirrors `Cargo.toml` `[package].version`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction(name = "version")]
fn py_version() -> &'static str {
    version()
}

/// Parse rST `source` and return its pseudo-XML representation.
///
/// Only the phase 1 grammar slice is supported (paragraphs + inline
/// emphasis/strong/literal). See `docs/compat.md`.
#[pyfunction(name = "parse_to_pseudoxml", signature = (source, source_path = "<string>"))]
fn py_parse_to_pseudoxml(source: &str, source_path: &str) -> String {
    let tree = parse_rst_with_source(source, source_path);
    pseudo_xml(&tree)
}

#[pymodule]
fn docutilsrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    m.add_function(wrap_pyfunction!(py_parse_to_pseudoxml, m)?)?;
    m.add_function(wrap_pyfunction!(python::py_parse_rst, m)?)?;
    m.add_class::<python::PyDoctree>()?;
    m.add_class::<python::PyNode>()?;
    Ok(())
}
