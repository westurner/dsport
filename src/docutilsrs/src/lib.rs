//! docutilsrs — Rust port of docutils.
//!
//! Phase 1 surface: a paragraph + inline-markup parser slice, with a
//! pseudo-XML writer for parity comparison against vendored docutils.

use pyo3::prelude::*;

pub mod doctree;
pub mod html5_writer;
pub mod latex_writer;
pub mod manpage_writer;
pub mod parser;
pub mod plugins;
mod python;
pub mod transforms;
pub mod writer;

pub use doctree::{Doctree, NodeKind};
pub use html5_writer::html5;
pub use latex_writer::latex;
pub use manpage_writer::manpage;
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

/// Parse rST `source` and return a minimal LaTeX document.
///
/// Not parity-tested against `docutils.writers.latex2e`; produces a
/// small, predictable subset suitable for downstream rendering. See
/// `docs/compat.md`.
#[pyfunction(name = "parse_to_latex", signature = (source, source_path = "<string>"))]
fn py_parse_to_latex(source: &str, source_path: &str) -> String {
    let tree = parse_rst_with_source(source, source_path);
    latex(&tree)
}

/// Parse rST `source` and return a minimal manpage (troff) document.
///
/// Not parity-tested against `docutils.writers.manpage`. See
/// `docs/compat.md`.
#[pyfunction(name = "parse_to_manpage", signature = (source, source_path = "<string>"))]
fn py_parse_to_manpage(source: &str, source_path: &str) -> String {
    let tree = parse_rst_with_source(source, source_path);
    manpage(&tree)
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
        "writer:latex",
        "writer:manpage",
        "parser:table_colspan",
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
        "plugin:python_directives",
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
    m.add_function(wrap_pyfunction!(py_parse_to_latex, m)?)?;
    m.add_function(wrap_pyfunction!(py_parse_to_manpage, m)?)?;
    m.add_function(wrap_pyfunction!(py_features, m)?)?;
    m.add_function(wrap_pyfunction!(py_supports, m)?)?;
    m.add_function(wrap_pyfunction!(python::py_parse_rst, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_register_directive, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_unregister_directive, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_registered_directives, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_clear_directives, m)?)?;
    m.add_class::<python::PyDoctree>()?;
    m.add_class::<python::PyNode>()?;
    Ok(())
}
