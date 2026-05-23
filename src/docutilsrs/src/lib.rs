//! docutilsrs — Rust port of docutils.
//!
//! Phase 1 surface: a paragraph + inline-markup parser slice, with a
//! pseudo-XML writer for parity comparison against vendored docutils.

use pyo3::prelude::*;
use pyo3::types::PyBytes;

pub mod code_block;
pub mod doctree;
pub mod html5_writer;
pub mod latex_writer;
pub mod manpage_writer;
pub mod odt_writer;
pub mod parser;
pub mod plugins;
mod python;
pub mod transforms;
pub mod writer;
pub mod zip_writer;

pub use doctree::{Doctree, NodeKind};
pub use html5_writer::html5;
pub use latex_writer::latex;
pub use manpage_writer::manpage;
pub use odt_writer::odt;
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

/// Parse rST `source` and return a binary ODT (`.odt`) document.
///
/// By default this uses the native Rust writer
/// (`docutilsrs::odt_writer`), which produces a valid `.odt` ZIP
/// container with a minimal style set. It is structurally correct but
/// *not* byte-for-byte identical to the vendored Python writer.
///
/// Pass `compat=True` to delegate to the vendored
/// `docutils.writers.odf_odt` writer instead. In compat mode the output
/// is byte-for-byte identical to upstream (gated by
/// `tests/test_writer_odt_parity.py`, which mirrors upstream's
/// `content.xml`-after-`ET.tostring` normalization).
#[pyfunction(name = "parse_to_odt", signature = (source, source_path = "<string>", *, compat = false, settings_overrides = None))]
fn py_parse_to_odt<'py>(
    py: Python<'py>,
    source: &str,
    source_path: &str,
    compat: bool,
    settings_overrides: Option<Bound<'py, pyo3::types::PyDict>>,
) -> PyResult<Bound<'py, PyBytes>> {
    if compat {
        let core = py.import("docutils.core")?;
        let odf = py.import("docutils.writers.odf_odt")?;
        let writer = odf.getattr("Writer")?.call0()?;
        let overrides = pyo3::types::PyDict::new(py);
        overrides.set_item("_disable_config", true)?;
        overrides.set_item("language_code", "en-US")?;
        if let Some(user) = settings_overrides {
            for (k, v) in user.iter() {
                overrides.set_item(k, v)?;
            }
        }
        let kwargs = pyo3::types::PyDict::new(py);
        kwargs.set_item("source", source.as_bytes())?;
        kwargs.set_item("source_path", source_path)?;
        kwargs.set_item("writer", writer)?;
        kwargs.set_item("settings_overrides", overrides)?;
        let result = core
            .getattr("publish_string")?
            .call((), Some(&kwargs))?
            .extract::<Vec<u8>>()?;
        return Ok(PyBytes::new(py, &result));
    }
    let tree = parse_rst_with_source(source, source_path);
    let bytes = odt(&tree);
    Ok(PyBytes::new(py, &bytes))
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
        "writer:odt",
        "writer:odt_compat",
        "parser:table_colspan",
        "parser:table_rowspan",
        "parser:table_multipara_cells",
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
        "plugin:python_transforms",
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
    m.add_function(wrap_pyfunction!(py_parse_to_odt, m)?)?;
    m.add_function(wrap_pyfunction!(py_features, m)?)?;
    m.add_function(wrap_pyfunction!(py_supports, m)?)?;
    m.add_function(wrap_pyfunction!(python::py_parse_rst, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_register_directive, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_unregister_directive, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_registered_directives, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_clear_directives, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_register_transform, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_unregister_transform, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_registered_transforms, m)?)?;
    m.add_function(wrap_pyfunction!(plugins::py_clear_transforms, m)?)?;
    m.add_class::<python::PyDoctree>()?;
    m.add_class::<python::PyNode>()?;
    m.add_function(wrap_pyfunction!(py_main, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2html, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2html4, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2html5, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2latex, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2man, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2odt, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2pseudoxml, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2s5, m)?)?;
    m.add_function(wrap_pyfunction!(py_rst2xetex, m)?)?;
    Ok(())
}

#[pyfunction(name = "main")]
fn py_main() {
    println!("stub running");
}
#[pyfunction(name = "rst2html")]
fn py_rst2html() {
    println!("stub running");
}
#[pyfunction(name = "rst2html4")]
fn py_rst2html4() {
    println!("stub running");
}
#[pyfunction(name = "rst2html5")]
fn py_rst2html5() {
    println!("stub running");
}
#[pyfunction(name = "rst2latex")]
fn py_rst2latex() {
    println!("stub running");
}
#[pyfunction(name = "rst2man")]
fn py_rst2man() {
    println!("stub running");
}
#[pyfunction(name = "rst2odt")]
fn py_rst2odt() {
    println!("stub running");
}
#[pyfunction(name = "rst2pseudoxml")]
fn py_rst2pseudoxml() {
    println!("stub running");
}
#[pyfunction(name = "rst2s5")]
fn py_rst2s5() {
    println!("stub running");
}
#[pyfunction(name = "rst2xetex")]
fn py_rst2xetex() {
    println!("stub running");
}
