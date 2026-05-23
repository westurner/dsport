//! myst-md-rs — Rust port of MyST-Parser built on `pulldown-cmark`.
//!
//! Phase 0 surface mirrors `pygmentsrs` / `docutilsrs`: a `version()` +
//! `features()` pair, plus `parse_to_html()` for the end-to-end pipeline and
//! `parse_front_matter()` for just the YAML header.
//!
//! Pipeline:
//!
//! ```text
//! source ─▶ frontmatter::split ─▶ preprocess::preprocess ─▶ render::render
//!                  │
//!                  └─▶ Option<serde_yaml::Value>
//! ```
//!
//! MyST extensions covered in this phase:
//!
//! * YAML front matter (`---` fences).
//! * Colon fences `:::name … :::` → directive blocks.
//! * Inline roles `` {name}`content` `` → `<span class="myst-role">`.
//! * Inline math `$…$` and block math `$$…$$`.
//!
//! Larger features (substitutions, attrs, field lists, deflists, doctree
//! bridge) are tracked in `README.md`.

use pyo3::prelude::*;

pub mod directives;
pub mod frontmatter;
pub mod options;
pub mod preprocess;
pub mod render;
pub mod role;

/// Crate version string. Mirrors `Cargo.toml` `[package].version`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction(name = "version")]
fn py_version() -> &'static str {
    version()
}

/// Coarse capability flags advertised by the Rust port.
pub fn features() -> &'static [&'static str] {
    &[
        "core:commonmark",
        "core:gfm",
        "myst:front_matter",
        "myst:colon_fence",
        "myst:roles_inline",
        "myst:dollarmath_inline",
        "myst:dollarmath_block",
        "render:html",
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

/// Result of a full parse: HTML body plus optional front matter YAML
/// serialised back to a string (so the FFI surface stays string-only).
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub html: String,
    pub front_matter_yaml: Option<String>,
}

/// Parse a MyST source document to HTML.
pub fn parse_to_html(source: &str) -> ParseResult {
    parse_to_html_with(source, MathBackend::default())
}

/// Parse a MyST source document to HTML, choosing the math backend.
pub fn parse_to_html_with(source: &str, math_backend: MathBackend) -> ParseResult {
    let split = frontmatter::split(source);
    let preprocessed = preprocess::preprocess(split.body);
    let html = render::render_with(&preprocessed, math_backend);
    let fm = split.front_matter.as_ref().and_then(|v| serde_yaml::to_string(v).ok());
    ParseResult {
        html,
        front_matter_yaml: fm,
    }
}

/// Convenience: just the HTML body, ignoring front matter. Uses the
/// default math backend ([`MathBackend::Ratex`]).
pub fn render_html(source: &str) -> String {
    parse_to_html(source).html
}

/// Like [`render_html`] but picks the math backend explicitly.
pub fn render_html_with(source: &str, math_backend: MathBackend) -> String {
    parse_to_html_with(source, math_backend).html
}

/// Re-exported from [`mathrenderrs`] so downstream callers don't need to
/// depend on it directly to pick a math rendering backend.
pub use mathrenderrs::MathBackend;

/// Extract the front matter (if any), returning it as a YAML string.
pub fn parse_front_matter(source: &str) -> Option<String> {
    let split = frontmatter::split(source);
    split
        .front_matter
        .as_ref()
        .and_then(|v| serde_yaml::to_string(v).ok())
}

#[pyfunction(name = "render_html")]
fn py_render_html(source: &str) -> String {
    render_html(source)
}

#[pyfunction(name = "parse_to_html")]
fn py_parse_to_html<'py>(
    py: Python<'py>,
    source: &str,
) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
    let r = parse_to_html(source);
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("html", r.html)?;
    dict.set_item("front_matter_yaml", r.front_matter_yaml)?;
    Ok(dict)
}

#[pyfunction(name = "parse_front_matter")]
fn py_parse_front_matter(source: &str) -> Option<String> {
    parse_front_matter(source)
}

#[pymodule]
fn myst_md_rs(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    m.add_function(wrap_pyfunction!(py_features, m)?)?;
    m.add_function(wrap_pyfunction!(py_supports, m)?)?;
    m.add_function(wrap_pyfunction!(py_render_html, m)?)?;
    m.add_function(wrap_pyfunction!(py_parse_to_html, m)?)?;
    m.add_function(wrap_pyfunction!(py_parse_front_matter, m)?)?;
    Ok(())
}
