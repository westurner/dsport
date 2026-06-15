//! HTML/XML escape routines.
//!
//! Mirrors the C-level `markupsafe._speedups` module that Python MarkupSafe
//! ships for performance.  This Rust implementation achieves the same goal
//! without a C extension: the inner loop is branchless on the common "no
//! escaping needed" fast-path.

use crate::markup::Markup;

/// Map a character to its HTML entity string, or `None` if no escaping needed.
///
/// Matches the five entities that Python MarkupSafe escapes:
///
/// | char | entity |
/// |------|--------|
/// | `&`  | `&amp;`  |
/// | `<`  | `&lt;`   |
/// | `>`  | `&gt;`   |
/// | `"`  | `&#34;`  |
/// | `'`  | `&#39;`  |
#[inline]
pub fn html_escape_char(c: char) -> Option<&'static str> {
    match c {
        '&'  => Some("&amp;"),
        '<'  => Some("&lt;"),
        '>'  => Some("&gt;"),
        '"'  => Some("&#34;"),
        '\'' => Some("&#39;"),
        _    => None,
    }
}

/// Escape `src` into `dst`, appending the result.
///
/// Only allocates when an escapeable character is encountered; the fast path
/// is a single `push_str` per contiguous run of safe characters.
pub fn escape_to(src: &str, dst: &mut String) {
    let mut last = 0;
    for (i, c) in src.char_indices() {
        if let Some(entity) = html_escape_char(c) {
            dst.push_str(&src[last..i]);
            dst.push_str(entity);
            last = i + c.len_utf8();
        }
    }
    dst.push_str(&src[last..]);
}

/// Escape `s` and return the result as a [`Markup`].
///
/// Free-function equivalent of [`Markup::escape`].  Mirrors the Python
/// `markupsafe.escape(s)` function.
///
/// If `s` is already a `Markup` (marked safe), this is a no-op clone.
///
/// ```rust
/// use markupsafers::escape;
/// let safe = escape("<b>user input</b>");
/// assert_eq!(safe.as_str(), "&lt;b&gt;user input&lt;/b&gt;");
/// ```
#[inline]
pub fn escape(s: &str) -> Markup {
    Markup::escape(s)
}

/// Escape a `Markup` value: because it is already safe, return a clone.
///
/// Mirrors the `__html__()` protocol: if the object already has an `__html__`
/// method (i.e. is a `Markup`), `escape()` returns it unchanged.
///
/// ```rust
/// use markupsafers::{escape_value, Markup};
/// let safe = Markup::from_safe("<b>already safe</b>");
/// let again = escape_value(&safe);
/// assert_eq!(again.as_str(), "<b>already safe</b>");
/// ```
#[inline]
pub fn escape_value(m: &Markup) -> Markup {
    m.clone()
}

/// Silently escape `None` to an empty `Markup`.
///
/// Mirrors `markupsafe.escape_silent(s)` from Python.
///
/// ```rust
/// use markupsafers::escape_silent;
/// assert_eq!(escape_silent(None).as_str(), "");
/// assert_eq!(escape_silent(Some("<b>")).as_str(), "&lt;b&gt;");
/// ```
#[inline]
pub fn escape_silent(s: Option<&str>) -> Markup {
    match s {
        Some(s) => escape(s),
        None => Markup::from_safe(""),
    }
}

/// Convert to string without escaping if the value is already `Markup`.
///
/// Mirrors `markupsafe.soft_str(s)` — returns a reference to the inner `str`
/// without any escaping.  Intended for cases where a function accepts either
/// `&str` or `Markup` and should not double-escape.
///
/// In Rust this is simply `Deref` — the function is provided for API parity.
///
/// ```rust
/// use markupsafers::{soft_str, Markup};
/// let m = Markup::from_safe("<b>ok</b>");
/// assert_eq!(soft_str(&m), "<b>ok</b>");
/// ```
#[inline]
pub fn soft_str(m: &Markup) -> &str {
    m.as_str()
}

// ── MarkupEscapeWriter ────────────────────────────────────────────────────────

/// A `fmt::Write` adapter that escapes every byte written into the inner
/// `Markup`.
///
/// Useful for implementing `Markup`-aware `Display` formatting.
///
/// ```rust
/// use std::fmt::Write as _;
/// use markupsafers::{MarkupEscapeWriter, Markup};
///
/// let mut w = MarkupEscapeWriter::new();
/// write!(w, "{}", "<untrusted>").unwrap();
/// assert_eq!(w.into_markup().as_str(), "&lt;untrusted&gt;");
/// ```
pub struct MarkupEscapeWriter {
    buf: String,
}

impl MarkupEscapeWriter {
    /// Create a new empty writer.
    pub fn new() -> Self {
        MarkupEscapeWriter { buf: String::new() }
    }

    /// Create a writer with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        MarkupEscapeWriter { buf: String::with_capacity(cap) }
    }

    /// Consume the writer and return the accumulated `Markup`.
    pub fn into_markup(self) -> Markup {
        Markup::from_safe(self.buf)
    }
}

impl Default for MarkupEscapeWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Write for MarkupEscapeWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        escape_to(s, &mut self.buf);
        Ok(())
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_all_five_chars() {
        assert_eq!(escape("&").as_str(), "&amp;");
        assert_eq!(escape("<").as_str(), "&lt;");
        assert_eq!(escape(">").as_str(), "&gt;");
        assert_eq!(escape("\"").as_str(), "&#34;");
        assert_eq!(escape("'").as_str(), "&#39;");
    }

    #[test]
    fn escape_mixed() {
        assert_eq!(
            escape("<script>alert('xss')</script>").as_str(),
            "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"
        );
    }

    #[test]
    fn escape_clean_string_is_unchanged() {
        let s = "Hello, World!";
        assert_eq!(escape(s).as_str(), s);
    }

    #[test]
    fn escape_silent_none() {
        assert_eq!(escape_silent(None).as_str(), "");
    }

    #[test]
    fn escape_silent_some() {
        assert_eq!(escape_silent(Some("<b>")).as_str(), "&lt;b&gt;");
    }

    #[test]
    fn soft_str_returns_inner() {
        let m = Markup::from_safe("<b>");
        assert_eq!(soft_str(&m), "<b>");
    }

    #[test]
    fn markup_escape_writer() {
        use std::fmt::Write as _;
        let mut w = MarkupEscapeWriter::new();
        write!(w, "{}", "<a> & </a>").unwrap();
        assert_eq!(w.into_markup().as_str(), "&lt;a&gt; &amp; &lt;/a&gt;");
    }

    #[test]
    fn unicode_passthrough() {
        let s = "日本語テスト";
        assert_eq!(escape(s).as_str(), s);
    }
}
