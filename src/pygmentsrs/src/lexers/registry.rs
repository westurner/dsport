//! Lexer alias → lexer instance resolution.
//!
//! Mirrors `pygments.lexers.get_lexer_by_name`. New lexers register
//! their aliases by extending both [`get_lexer_by_name`] and
//! [`native_aliases`] (the latter advertises which aliases the
//! native path handles, so callers can decide whether to short-circuit
//! a PyO3 hop).

use crate::lexer::Lexer;
use crate::lexers::diff::DiffLexer;
use crate::lexers::json::JsonLexer;
use crate::lexers::python::PythonLexer;
use crate::lexers::text::TextLexer;

pub fn get_lexer_by_name(alias: &str) -> Option<Box<dyn Lexer>> {
    match alias {
        "text" | "plain" | "" => Some(Box::new(TextLexer)),
        "python" | "py" | "python3" | "py3" => Some(Box::new(PythonLexer)),
        "json" | "json-object" => Some(Box::new(JsonLexer)),
        "diff" | "udiff" => Some(Box::new(DiffLexer)),
        _ => None,
    }
}

/// Aliases for which a native Rust lexer exists. Sorted by primary
/// name first so the list is stable for snapshotting.
pub fn native_aliases() -> &'static [&'static str] {
    &[
        "text",
        "plain",
        "",
        "python",
        "py",
        "python3",
        "py3",
        "json",
        "json-object",
        "diff",
        "udiff",
    ]
}
