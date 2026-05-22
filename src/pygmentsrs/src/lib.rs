//! pygmentsrs — Rust port of Pygments.
//!
//! Phase 0 surface: `version()` + `features()`. Phase 1 adds the
//! token tree, the `RegexLexer` engine, the Python lexer, and the
//! HTML formatter (see `docs/handoff/pygments.md` in the dsport
//! repo for the full plan).

use pyo3::prelude::*;

pub mod formatters;
pub mod lexer;
pub mod lexers;
pub mod regexopt;
pub mod token;

/// Crate version string. Mirrors `Cargo.toml` `[package].version`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction(name = "version")]
fn py_version() -> &'static str {
    version()
}

/// Coarse capability flags advertised by the Rust port. Mirrors the
/// `docutilsrs`/`sphinxdocrs` `features()` pattern so a hybrid
/// wrapper can probe without importing internals.
pub fn features() -> &'static [&'static str] {
    &[
        "token:hierarchy",
        "lexer:regex_engine",
        "lexers:python",
        "lexers:text",
        "formatters:html",
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

/// Return the (token-name, value) stream for `code` using the named
/// lexer alias. Token names use the same `repr()` form as
/// `pygments.token` (e.g. `"Token.Literal.String"`), so callers can
/// compare directly against `pygments.lex(...)`.
///
/// Returns `None` if the alias is not registered in pygmentsrs.
pub fn lex(alias: &str, code: &str) -> Option<Vec<(String, String)>> {
    let lexer = lexers::registry::get_lexer_by_name(alias)?;
    Some(
        lexer
            .get_tokens(code)
            .into_iter()
            .map(|(t, v)| (t.repr(), v))
            .collect(),
    )
}

#[pyfunction(name = "lex")]
fn py_lex(alias: &str, code: &str) -> Option<Vec<(String, String)>> {
    lex(alias, code)
}

/// Highlight `code` using `alias` and the named formatter.
/// Currently only `"html"` is supported. Returns `None` if either
/// alias or formatter is unknown.
pub fn highlight(code: &str, alias: &str, formatter: &str) -> Option<String> {
    let lexer = lexers::registry::get_lexer_by_name(alias)?;
    let tokens = lexer.get_tokens(code);
    match formatter {
        "html" => Some(formatters::html::HtmlFormatter.format(&tokens)),
        _ => None,
    }
}

#[pyfunction(name = "highlight", signature = (code, alias, formatter = "html"))]
fn py_highlight(code: &str, alias: &str, formatter: &str) -> Option<String> {
    highlight(code, alias, formatter)
}

#[pymodule]
fn pygmentsrs(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    m.add_function(wrap_pyfunction!(py_features, m)?)?;
    m.add_function(wrap_pyfunction!(py_supports, m)?)?;
    m.add_function(wrap_pyfunction!(py_lex, m)?)?;
    m.add_function(wrap_pyfunction!(py_highlight, m)?)?;
    Ok(())
}
