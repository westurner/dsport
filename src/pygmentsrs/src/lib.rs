//! pygmentsrs — Rust port of Pygments.
//!
//! ## Lexer backends
//!
//! Three dispatch modes via [`Backend`] / the `backend=` parameter:
//!
//! * `Auto` (default): try the native Rust lexer first; if pygmentsrs
//!   has no implementation for the alias, fall back to the PyO3
//!   bridge to vendored upstream `pygments` (requires `python-bridge`
//!   feature, which is enabled by default).
//! * `Rust`: native only; returns `None` if there's no Rust lexer.
//! * `Python`: skip the native path entirely and call upstream
//!   `pygments.lex(...)` via PyO3 (requires `python-bridge` feature).
//!
//! Build without Python / CPython dependency:
//! ```sh
//! cargo build -p pygmentsrs --no-default-features
//! ```

#[cfg(feature = "python-bridge")]
use pyo3::prelude::*;

#[cfg(feature = "python-bridge")]
pub mod bridge;

#[cfg(not(feature = "python-bridge"))]
/// Stub bridge used when the `python-bridge` feature is disabled.
/// All functions return `None`; the `Auto` backend behaves like `Rust`.
pub mod bridge {
    use crate::token::TokenType;

    pub fn lex(_alias: &str, _code: &str) -> Option<Vec<(String, String)>> {
        None
    }
    pub fn format(_name: &str, _tokens: &[(String, String)]) -> Option<String> {
        None
    }
    pub fn alias_is_known(_alias: &str) -> bool {
        false
    }
    pub fn formatter_is_known(_name: &str) -> bool {
        false
    }
}

pub mod formatters;
pub mod lexer;
pub mod lexers;
pub mod regexopt;
pub mod token;

/// Crate version string. Mirrors `Cargo.toml` `[package].version`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Coarse capability flags advertised by the Rust port.
pub fn features() -> &'static [&'static str] {
    &[
        "token:hierarchy",
        "lexer:regex_engine",
        "engine:fancy_regex",
        "lexers:text",
        "lexers:python",
        "lexers:json",
        "lexers:diff",
        "lexers:transpiled",
        "formatters:html",
        "bridge:pygments",
    ]
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

#[cfg(feature = "python-bridge")]
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

/// Backend-aware variant of [`highlight`]. Lexer and formatter stages
/// dispatch independently per `backend`:
///
/// * `Auto`: native if registered, else fall through to the pygments
///   bridge — applied separately to the lexer and the formatter.
/// * `Rust`: native only for both stages; `None` if either is missing.
/// * `Python`: bridge only for both stages.
pub fn highlight_with_backend(
    code: &str,
    alias: &str,
    formatter: &str,
    backend: Backend,
) -> Option<String> {
    let raw = lex_with_backend(alias, code, backend)?;
    match backend {
        Backend::Python => bridge::format(formatter, &raw),
        Backend::Rust => {
            let tokens = to_native_tokens(raw);
            formatters::registry::format_native(formatter, &tokens)
        }
        Backend::Auto => {
            if formatters::registry::has_native(formatter) {
                let tokens = to_native_tokens(raw);
                formatters::registry::format_native(formatter, &tokens)
            } else {
                bridge::format(formatter, &raw)
            }
        }
    }
}

fn to_native_tokens(raw: Vec<(String, String)>) -> Vec<(token::TokenType, String)> {
    raw.into_iter()
        .map(|(name, v)| {
            let t = token::from_dotted(&name).unwrap_or(token::TOKEN);
            (t, v)
        })
        .collect()
}

#[cfg(feature = "python-bridge")]
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
#[cfg(feature = "python-bridge")]
#[pyfunction(name = "native_aliases")]
fn py_native_aliases() -> Vec<&'static str> {
    lexers::registry::native_aliases().to_vec()
}

#[cfg(feature = "python-bridge")]
#[pyfunction(name = "has_native_lexer")]
fn py_has_native_lexer(alias: &str) -> bool {
    lexers::registry::get_lexer_by_name(alias).is_some()
}

/// Names of formatters with a native Rust implementation.
#[cfg(feature = "python-bridge")]
#[pyfunction(name = "native_formatters")]
fn py_native_formatters() -> Vec<&'static str> {
    formatters::registry::native_names().to_vec()
}

#[cfg(feature = "python-bridge")]
#[pyfunction(name = "has_native_formatter")]
fn py_has_native_formatter(name: &str) -> bool {
    formatters::registry::has_native(name)
}

#[cfg(feature = "python-bridge")]
#[pymodule]
fn pygmentsrs(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    m.add_function(wrap_pyfunction!(py_features, m)?)?;
    m.add_function(wrap_pyfunction!(py_supports, m)?)?;
    m.add_function(wrap_pyfunction!(py_lex, m)?)?;
    m.add_function(wrap_pyfunction!(py_highlight, m)?)?;
    m.add_function(wrap_pyfunction!(py_native_aliases, m)?)?;
    m.add_function(wrap_pyfunction!(py_has_native_lexer, m)?)?;
    m.add_function(wrap_pyfunction!(py_native_formatters, m)?)?;
    m.add_function(wrap_pyfunction!(py_has_native_formatter, m)?)?;
    m.add_function(wrap_pyfunction!(py_main, m)?)?;
    Ok(())
}

#[cfg(feature = "python-bridge")]
#[pyfunction(name = "version")]
fn py_version() -> &'static str {
    version()
}

#[cfg(feature = "python-bridge")]
#[pyfunction(name = "features")]
fn py_features() -> Vec<&'static str> {
    features().to_vec()
}

#[cfg(feature = "python-bridge")]
#[pyfunction(name = "supports")]
fn py_supports(feature: &str) -> bool {
    features().contains(&feature)
}

#[cfg(feature = "python-bridge")]
#[pyfunction(name = "main")]
fn py_main() {
    println!("stub running");
}
