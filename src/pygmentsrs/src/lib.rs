//! pygmentsrs — Rust port of Pygments.
//!
//! Phase 0 surface: `version()` + `features()`. Phase 1 adds the
//! token tree, the `RegexLexer` engine, the Python lexer, and the
//! HTML formatter (see `docs/handoff/pygments.md` in the dsport
//! repo for the full plan).
//!
//! ## Lexer backends
//!
//! Three dispatch modes via [`Backend`] / the `backend=` parameter:
//!
//! * `Auto` (default): try the native Rust lexer first; if pygmentsrs
//!   has no implementation for the alias, fall back to the PyO3
//!   bridge to vendored upstream `pygments`.
//! * `Rust`: native only; returns `None` if there's no Rust lexer.
//! * `Python`: skip the native path entirely and call upstream
//!   `pygments.lex(...)` via PyO3.

use pyo3::prelude::*;

pub mod bridge;
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
        "bridge:pygments",
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

/// Which lexer implementation to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Backend {
    /// Native Rust if available, else upstream pygments via PyO3.
    Auto,
    /// Native Rust only; `None` if no Rust lexer is registered.
    Rust,
    /// Always upstream pygments via PyO3 bridge.
    Python,
}

impl Backend {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(Self::Auto),
            "rust" | "native" | "pygmentsrs" => Some(Self::Rust),
            "python" | "pygments" | "bridge" => Some(Self::Python),
            _ => None,
        }
    }
}

/// Return the (token-name, value) stream for `code` using the named
/// lexer alias. Token names use the same `repr()` form as
/// `pygments.token` (e.g. `"Token.Literal.String"`), so callers can
/// compare directly against `pygments.lex(...)`.
///
/// Returns `None` if the alias is not registered in pygmentsrs.
/// Equivalent to [`lex_with_backend`] with [`Backend::Rust`].
pub fn lex(alias: &str, code: &str) -> Option<Vec<(String, String)>> {
    lex_with_backend(alias, code, Backend::Rust)
}

/// Backend-aware variant of [`lex`]. See [`Backend`] for dispatch rules.
pub fn lex_with_backend(
    alias: &str,
    code: &str,
    backend: Backend,
) -> Option<Vec<(String, String)>> {
    match backend {
        Backend::Rust => lex_rust(alias, code),
        Backend::Python => bridge::lex(alias, code),
        Backend::Auto => lex_rust(alias, code).or_else(|| bridge::lex(alias, code)),
    }
}

fn lex_rust(alias: &str, code: &str) -> Option<Vec<(String, String)>> {
    let lexer = lexers::registry::get_lexer_by_name(alias)?;
    Some(
        lexer
            .get_tokens(code)
            .into_iter()
            .map(|(t, v)| (t.repr(), v))
            .collect(),
    )
}

#[pyfunction(name = "lex", signature = (alias, code, backend = "auto"))]
fn py_lex(alias: &str, code: &str, backend: &str) -> PyResult<Option<Vec<(String, String)>>> {
    let b = Backend::parse(backend).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "unknown backend {backend:?}; expected one of: auto, rust, python"
        ))
    })?;
    Ok(lex_with_backend(alias, code, b))
}

/// Highlight `code` using `alias` and the named formatter.
/// Currently only `"html"` is supported. Returns `None` if either
/// alias or formatter is unknown.
pub fn highlight(code: &str, alias: &str, formatter: &str) -> Option<String> {
    highlight_with_backend(code, alias, formatter, Backend::Auto)
}

/// Backend-aware variant of [`highlight`]. With [`Backend::Auto`] or
/// [`Backend::Python`], the token stream comes from upstream pygments
/// when there's no Rust lexer; the formatter stage is always the Rust
/// implementation.
pub fn highlight_with_backend(
    code: &str,
    alias: &str,
    formatter: &str,
    backend: Backend,
) -> Option<String> {
    let raw = lex_with_backend(alias, code, backend)?;
    let tokens: Vec<(token::TokenType, String)> = raw
        .into_iter()
        .map(|(name, v)| {
            let t = token::from_dotted(&name).unwrap_or(token::TOKEN);
            (t, v)
        })
        .collect();
    match formatter {
        "html" => Some(formatters::html::HtmlFormatter.format(&tokens)),
        _ => None,
    }
}

#[pyfunction(name = "highlight", signature = (code, alias, formatter = "html", backend = "auto"))]
fn py_highlight(
    code: &str,
    alias: &str,
    formatter: &str,
    backend: &str,
) -> PyResult<Option<String>> {
    let b = Backend::parse(backend).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "unknown backend {backend:?}; expected one of: auto, rust, python"
        ))
    })?;
    Ok(highlight_with_backend(code, alias, formatter, b))
}

/// Aliases natively implemented by pygmentsrs. Helpers for callers
/// that want to pre-flight a dispatch decision.
#[pyfunction(name = "native_aliases")]
fn py_native_aliases() -> Vec<&'static str> {
    lexers::registry::native_aliases().to_vec()
}

#[pyfunction(name = "has_native_lexer")]
fn py_has_native_lexer(alias: &str) -> bool {
    lexers::registry::get_lexer_by_name(alias).is_some()
}

#[pymodule]
fn pygmentsrs(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    m.add_function(wrap_pyfunction!(py_features, m)?)?;
    m.add_function(wrap_pyfunction!(py_supports, m)?)?;
    m.add_function(wrap_pyfunction!(py_lex, m)?)?;
    m.add_function(wrap_pyfunction!(py_highlight, m)?)?;
    m.add_function(wrap_pyfunction!(py_native_aliases, m)?)?;
    m.add_function(wrap_pyfunction!(py_has_native_lexer, m)?)?;
    Ok(())
}
