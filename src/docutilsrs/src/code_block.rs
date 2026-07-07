//! Syntax-highlight a `code`/`code-block`/`sourcecode` body into the
//! list of `(class, text)` token spans the parser hands to
//! `Block::LiteralBlock`.
//!
//! Strategy (Path A+native hybrid from `docs/handoff/pygments.md`):
//!
//! 1. If `pygmentsrs` has a Rust lexer registered for `lang`, lex
//!    natively (no GIL hop).
//! 2. Otherwise, fall back to `docutils.utils.code_analyzer.Lexer`
//!    via PyO3 — preserves byte-parity for every language docutils
//!    itself can handle.
//! 3. On any failure (no language, `lang == "text"`, pygments
//!    missing, unknown language with `with_pygments=False`), return
//!    `None` so the parser emits the flat `<literal_block>` shape.
//!
//! Token-class normalization mirrors
//! `docutils.utils.code_analyzer.Lexer` with `tokennames="long"`:
//!
//! - `str(tokentype).lower().split('.')` (so `Token.Name.Function`
//!   → `["token", "name", "function"]`)
//! - drop entries in `unstyled_tokens = {"token", "text", ""}`
//! - join the rest with a space (`"name function"`)
//! - empty result → emit as bare text (no `<inline>` wrapper),
//!   represented as a `None` class
//!
//! The very last token value has a single trailing `\n` stripped
//! (pygments appends one; upstream's `merge()` strips it).
//! Adjacent same-type tokens are merged. Both behaviors are inherited
//! when we lex through `code_analyzer`; the native path applies them
//! here.

use pyo3::prelude::*;
use pyo3::types::PyList;

/// A normalized `(class, value)` pair.
/// `class = None` means emit as bare text inside the `<literal_block>`
/// (Token / Token.Text / unstyled — no `<inline>` wrapper).
pub type Span = (Option<String>, String);

/// Try to tokenize `code` as `lang`. Returns `None` when:
/// - `lang` is empty or `"text"` (caller emits flat text);
/// - both pygmentsrs and the Python bridge are unavailable;
/// - the language is not recognized by either backend.
pub fn tokenize(lang: &str, code: &str) -> Option<Vec<Span>> {
    let lang = lang.trim();
    if lang.is_empty() || lang.eq_ignore_ascii_case("text") {
        return None;
    }
    #[cfg(feature = "syntax-highlighting")]
    if let Some(spans) = tokenize_native(lang, code) {
        return Some(spans);
    }
    tokenize_bridge(lang, code)
}

#[cfg(feature = "syntax-highlighting")]
fn tokenize_native(lang: &str, code: &str) -> Option<Vec<Span>> {
    // `Backend::Auto` already does pygmentsrs-native-first then upstream
    // `pygments.lex(...)` via PyO3; the secondary `tokenize_bridge`
    // below is kept as a last-resort path to `docutils.utils.code_analyzer`
    // (which respects user-disabled syntax highlighting).
    let raw = pygmentsrs::lex_with_backend(lang, code, pygmentsrs::Backend::Auto)?;
    Some(normalize_long(raw))
}

fn tokenize_bridge(lang: &str, code: &str) -> Option<Vec<Span>> {
    Python::try_attach(|py| -> Option<Vec<Span>> {
        let analyzer = py.import("docutils.utils.code_analyzer").ok()?;
        let with_pygments: bool = analyzer
            .getattr("with_pygments")
            .ok()?
            .extract()
            .unwrap_or(false);
        if !with_pygments {
            return None;
        }
        let lexer_cls = analyzer.getattr("Lexer").ok()?;
        let lexer = lexer_cls.call1((code, lang, "long")).ok()?;
        let iter = lexer.try_iter().ok()?;
        let mut out: Vec<Span> = Vec::new();
        for item in iter {
            let item = item.ok()?;
            let tuple = item.cast::<pyo3::types::PyTuple>().ok()?;
            let classes_obj = tuple.get_item(0).ok()?;
            let value: String = tuple.get_item(1).ok()?.extract().ok()?;
            let classes_list: Vec<String> = classes_obj
                .cast::<PyList>()
                .ok()?
                .extract()
                .unwrap_or_default();
            let class = if classes_list.is_empty() {
                None
            } else {
                Some(classes_list.join(" "))
            };
            out.push((class, value));
        }
        Some(merge_adjacent(out))
    })?
}

#[cfg(feature = "syntax-highlighting")]
/// Normalize pygmentsrs' `(token_repr, value)` stream into the
/// docutils long-name form: drop `Token` / `Token.Text` ancestors,
/// downcase, space-join, merge adjacent, strip a final `\n`.
fn normalize_long(raw: Vec<(String, String)>) -> Vec<Span> {
    let mut out: Vec<Span> = Vec::with_capacity(raw.len());
    for (ttype, value) in raw {
        let class = long_classes(&ttype);
        out.push((class, value));
    }
    let merged = merge_adjacent(out);
    strip_trailing_newline(merged)
}

#[cfg(feature = "syntax-highlighting")]
fn long_classes(ttype: &str) -> Option<String> {
    // `str(tokentype).lower().split('.')` — `Token.Name.Function`
    // → `["token", "name", "function"]`. The leading `Token` is
    // always present in pygments repr; drop both `token` and `text`
    // (and any literally empty entry — defensive).
    let parts: Vec<String> = ttype
        .to_ascii_lowercase()
        .split('.')
        .filter(|p| !matches!(*p, "token" | "text" | ""))
        .map(|p| p.to_string())
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn merge_adjacent(spans: Vec<Span>) -> Vec<Span> {
    let mut out: Vec<Span> = Vec::with_capacity(spans.len());
    for (class, value) in spans {
        if let Some(last) = out.last_mut()
            && last.0 == class
        {
            last.1.push_str(&value);
            continue;
        }
        out.push((class, value));
    }
    out
}

#[cfg(feature = "syntax-highlighting")]
fn strip_trailing_newline(mut spans: Vec<Span>) -> Vec<Span> {
    if let Some(last) = spans.last_mut() {
        if let Some(stripped) = last.1.strip_suffix('\n') {
            last.1 = stripped.to_string();
        }
        if last.1.is_empty() {
            spans.pop();
        }
    }
    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_classes_drops_token_and_text() {
        assert_eq!(long_classes("Token"), None);
        assert_eq!(long_classes("Token.Text"), None);
        assert_eq!(
            long_classes("Token.Text.Whitespace"),
            Some("whitespace".into())
        );
        assert_eq!(
            long_classes("Token.Name.Function"),
            Some("name function".into())
        );
        assert_eq!(
            long_classes("Token.Literal.Number.Integer"),
            Some("literal number integer".into())
        );
    }

    #[test]
    fn merge_collapses_adjacent_same_class() {
        let s = vec![
            (Some("a".into()), "x".into()),
            (Some("a".into()), "y".into()),
            (Some("b".into()), "z".into()),
            (None, "1".into()),
            (None, "2".into()),
        ];
        let m = merge_adjacent(s);
        assert_eq!(
            m,
            vec![
                (Some("a".into()), "xy".into()),
                (Some("b".into()), "z".into()),
                (None, "12".into()),
            ]
        );
    }

    #[test]
    fn trailing_newline_stripped_from_last_span() {
        let s = vec![
            (Some("a".into()), "x".into()),
            (Some("b".into()), "y\n".into()),
        ];
        let s = strip_trailing_newline(s);
        assert_eq!(
            s,
            vec![
                (Some("a".into()), "x".into()),
                (Some("b".into()), "y".into()),
            ]
        );
    }

    #[test]
    fn text_language_passes_through() {
        assert!(tokenize("text", "x = 1").is_none());
        assert!(tokenize("", "x = 1").is_none());
        assert!(tokenize("  TEXT  ", "x = 1").is_none());
    }
}
