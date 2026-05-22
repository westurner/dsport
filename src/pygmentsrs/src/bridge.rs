//! PyO3 bridge to vendored upstream `pygments`.
//!
//! Used as the fallback path when pygmentsrs has no native Rust lexer
//! for a given alias, and as the implementation of the explicit
//! `backend="python"` request from [`crate::lex_with_backend`].
//!
//! Returns the same `(repr(ttype), value)` shape as the native
//! `pygmentsrs::lex` so callers can use either interchangeably.
//!
//! Bridge failures (Python not embedded, `pygments` not installed,
//! unknown alias) are surfaced as `None`. The caller decides whether
//! to treat that as an error or to fall through to another path.

use pyo3::prelude::*;

/// Lex `code` using upstream `pygments.lexers.get_lexer_by_name(alias)`.
/// Returns the `(repr(ttype), value)` token stream as a Rust `Vec`.
/// `None` if pygments is unavailable or the alias is unknown.
pub fn lex(alias: &str, code: &str) -> Option<Vec<(String, String)>> {
    Python::try_attach(|py| -> Option<Vec<(String, String)>> {
        let lexers_mod = py.import("pygments.lexers").ok()?;
        let lexer = lexers_mod
            .getattr("get_lexer_by_name")
            .ok()?
            .call1((alias,))
            .ok()?;
        let pyg = py.import("pygments").ok()?;
        let stream = pyg.getattr("lex").ok()?.call1((code, lexer)).ok()?;
        let iter = stream.try_iter().ok()?;
        let mut out: Vec<(String, String)> = Vec::new();
        for item in iter {
            let item = item.ok()?;
            let tuple = item.cast::<pyo3::types::PyTuple>().ok()?;
            let ttype_obj = tuple.get_item(0).ok()?;
            let value: String = tuple.get_item(1).ok()?.extract().ok()?;
            let ttype_repr: String = ttype_obj.repr().ok()?.extract().ok()?;
            out.push((ttype_repr, value));
        }
        Some(out)
    })?
}

/// Does upstream `pygments.lexers.get_lexer_by_name(alias)` resolve?
/// Useful for the dispatcher to decide whether the alias is even
/// known to upstream before reaching for the bridge.
pub fn alias_is_known(alias: &str) -> bool {
    Python::try_attach(|py| {
        py.import("pygments.lexers")
            .and_then(|m| m.getattr("get_lexer_by_name"))
            .and_then(|f| f.call1((alias,)))
            .is_ok()
    })
    .unwrap_or(false)
}
