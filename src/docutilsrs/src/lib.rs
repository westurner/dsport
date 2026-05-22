//! docutilsrs — Rust port of docutils.
//!
//! Phase 1 surface: a paragraph + inline-markup parser slice, with a
//! pseudo-XML writer for parity comparison against vendored docutils.

use pyo3::prelude::*;

pub mod doctree;
pub mod html5_writer;
pub mod parser;
mod python;
pub mod transforms;
pub mod writer;

pub use doctree::{Doctree, NodeKind};
pub use html5_writer::html5;
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

/// Parse rST `source` and return a minimal HTML5 fragment.
///
/// This output is not parity-tested against docutils; it is a small,
/// predictable subset suitable for downstream rendering. See
/// `docs/compat.md`.
#[pyfunction(name = "parse_to_html5", signature = (source, source_path = "<string>"))]
fn py_parse_to_html5(source: &str, source_path: &str) -> String {
    let tree = parse_rst_with_source(source, source_path);
    html5(&tree)
}

/// List of feature flags supported by the Rust port at runtime.
///
/// Used by the hybrid wrapper to decide whether to dispatch a given input
/// to the Rust pipeline. Each name corresponds to a coarse compatibility
/// claim documented in `docs/compat.md`.
pub fn features() -> &'static [&'static str] {
    &[
        "writer:pseudoxml",
        "writer:html5",
        "parser:paragraphs",
        "parser:inline",
        "parser:bullet_list",
        "parser:enumerated_list",
        "parser:definition_list",
        "parser:field_list",
        "parser:sections",
        "parser:transitions",
        "parser:block_quote",
        "parser:literal_block",
        "parser:comment",
        "parser:admonition",
        "parser:image",
        "parser:figure",
        "parser:code_block",
        "parser:raw",
        "parser:role",
        "parser:substitution",
        "parser:table",
        "parser:reference_named",
        "parser:reference_anonymous",
        "parser:reference_phrase",
        "parser:reference_embedded_uri",
        "parser:footnote_numbered",
        "parser:citation_named",
        "transform:resolve_references",
        "transform:promote_document_title",
        "transform:promote_docinfo",
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
fn docutilsrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    m.add_function(wrap_pyfunction!(py_parse_to_pseudoxml, m)?)?;
    m.add_function(wrap_pyfunction!(py_parse_to_html5, m)?)?;
    m.add_function(wrap_pyfunction!(py_features, m)?)?;
    m.add_function(wrap_pyfunction!(py_supports, m)?)?;
    m.add_function(wrap_pyfunction!(python::py_parse_rst, m)?)?;
    m.add_class::<python::PyDoctree>()?;
    m.add_class::<python::PyNode>()?;
    Ok(())
}
