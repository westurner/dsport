//! `sphinxdocrs::domains::py_sig` — `ruff_python_parser`-backed signature
//! parsing for the `py` domain (**H3b**).
//!
//! Rather than hand-roll paren/annotation splitting for
//! `.. py:function::`/`.. py:class::` argument lines, this wraps the
//! whole line as a synthetic `def`/`class` statement and reuses the same
//! `ruff_python_ast`/`ruff_python_parser` machinery `crate::autodoc`
//! already ports `sphinx.ext.autodoc`'s signature formatting with. This
//! gets real Python-grammar handling of nested parens, string/numeric
//! defaults, and `->` return annotations for free, instead of a
//! regex/manual paren-balance scanner.

use ruff_python_ast::Stmt;
use ruff_python_parser::parse_module;

use crate::autodoc::{format_signature, render_expr};

/// Parse a `.. py:function::`/`.. py:method::`/`.. py:classmethod::`/
/// `.. py:staticmethod::` argument line (`"name(args) -> ret"`) into
/// `(name, rendered_signature)`.
///
/// Falls back to a naive `(`/`)` split when the line isn't valid Python
/// (e.g. Sphinx's own docs sometimes use `[...]` optional-argument
/// bracket notation, which isn't real Python syntax) — the fallback
/// signature is then just the raw text between the outermost parens,
/// unparsed/unnormalized.
pub(super) fn parse_function_signature(raw: &str) -> (String, String) {
    let raw = raw.trim();
    if raw.is_empty() {
        return (String::new(), String::new());
    }
    if !raw.contains('(') {
        // No parameter list at all: treat the whole thing as a bare name
        // (e.g. a no-arg function documented without `()`).
        return (raw.to_string(), String::new());
    }
    let wrapped = format!("def {raw}: pass");
    if let Ok(parsed) = parse_module(&wrapped)
        && let Some(Stmt::FunctionDef(f)) = parsed.suite().first()
    {
        return (f.name.to_string(), format_signature(&f.parameters));
    }
    fallback_paren_split(raw)
}

/// Parse a `.. py:class::`/`.. py:exception::` argument line
/// (`"Name(Base, ...)"` or just `"Name"`) into `(name, rendered_bases)`.
pub(super) fn parse_class_signature(raw: &str) -> (String, String) {
    let raw = raw.trim();
    if raw.is_empty() {
        return (String::new(), String::new());
    }
    if !raw.contains('(') {
        return (raw.to_string(), String::new());
    }
    let wrapped = format!("class {raw}: pass");
    if let Ok(parsed) = parse_module(&wrapped)
        && let Some(Stmt::ClassDef(c)) = parsed.suite().first()
    {
        let bases = c
            .arguments
            .as_ref()
            .map(|args| {
                args.args
                    .iter()
                    .map(render_expr)
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();
        return (c.name.to_string(), bases);
    }
    fallback_paren_split(raw)
}

/// Best-effort fallback when `raw` isn't valid Python: take the name as
/// everything before the first `(`, and the signature as the raw text
/// between the outermost matching parens (unparsed/unnormalized).
fn fallback_paren_split(raw: &str) -> (String, String) {
    let Some(open) = raw.find('(') else {
        return (raw.trim().to_string(), String::new());
    };
    let name = raw[..open].trim().to_string();
    let inner = match raw.rfind(')') {
        Some(close) if close > open => raw[open + 1..close].trim().to_string(),
        _ => raw[open + 1..].trim().to_string(),
    };
    (name, inner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_function_signature() {
        let (name, sig) = parse_function_signature("greet(name, loud=False)");
        assert_eq!(name, "greet");
        assert_eq!(sig, "name, loud=False");
    }

    #[test]
    fn parses_function_with_return_annotation() {
        let (name, sig) = parse_function_signature("add(a, b) -> int");
        assert_eq!(name, "add");
        assert_eq!(sig, "a, b");
    }

    #[test]
    fn parses_nested_parens_in_default() {
        // Nested parens inside a tuple default used to defeat a naive
        // paren-balance scanner; the real Python parser handles it
        // correctly regardless. `render_expr` now has full `Expr::Tuple`
        // support (**H9c**), so the default renders literally instead of
        // being elided to `...`.
        let (name, sig) = parse_function_signature("configure(opts=(1, 2))");
        assert_eq!(name, "configure");
        assert_eq!(sig, "opts=(1, 2)");
    }

    #[test]
    fn bare_name_has_no_signature() {
        let (name, sig) = parse_function_signature("greet");
        assert_eq!(name, "greet");
        assert_eq!(sig, "");
    }

    #[test]
    fn parses_class_with_bases() {
        let (name, sig) = parse_class_signature("Widget(Base, Mixin)");
        assert_eq!(name, "Widget");
        assert_eq!(sig, "Base, Mixin");
    }

    #[test]
    fn parses_class_without_bases() {
        let (name, sig) = parse_class_signature("Widget");
        assert_eq!(name, "Widget");
        assert_eq!(sig, "");
    }

    #[test]
    fn falls_back_on_invalid_python() {
        // `[loud]` optional-argument bracket notation isn't valid Python.
        let (name, sig) = parse_function_signature("greet(name[, loud])");
        assert_eq!(name, "greet");
        assert_eq!(sig, "name[, loud]");
    }
}
