//! `sphinxdocrs::util_docstrings` — Rust port of `sphinx.util.docstrings`.
//!
//! Docstring processing utilities: indentation stripping, comment-doc extraction,
//! and metadata separation.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `prepare_docstring(s, tabsize)` | [`prepare_docstring`] | strips common indentation; returns Vec of lines |
//! | `prepare_commentdoc(s)` | [`prepare_commentdoc`] | extracts `#:` comment lines |
//! | `separate_metadata(s)` | [`separate_metadata`] | splits `:meta …:` field list items from the rest |

use regex::Regex;

// ── prepare_docstring ─────────────────────────────────────────────────────────

/// Convert a docstring into lines of parseable reST.
///
/// Removes common leading indentation (ignoring the first line), strips
/// leading blank lines, and ensures the result ends with a single empty
/// string (so it can be inserted into a docutils ViewList with a separator).
///
/// `tabsize` defaults to 8 (matching Python's `str.expandtabs`).
///
/// Mirrors `sphinx.util.docstrings.prepare_docstring`.
///
/// ```rust
/// use sphinxdocrs::util_docstrings::prepare_docstring;
/// assert_eq!(prepare_docstring("single line docstring", 8), vec!["single line docstring", ""]);
/// ```
pub fn prepare_docstring(s: &str, tabsize: usize) -> Vec<String> {
    let expanded = expand_tabs(s, tabsize);
    let mut lines: Vec<String> = expanded.lines().map(String::from).collect();

    // Find minimum indentation of any non-blank lines AFTER line 0.
    let margin = lines[1..]
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(usize::MAX);

    // Remove indentation from line 0 (strip left).
    if !lines.is_empty() {
        lines[0] = lines[0].trim_start().to_string();
    }

    // Remove common indentation from the remaining lines.
    if margin < usize::MAX {
        for line in lines.iter_mut().skip(1) {
            if line.len() >= margin {
                *line = line[margin..].to_string();
            }
        }
    }

    // Remove leading blank lines.
    while !lines.is_empty() && lines[0].trim().is_empty() {
        lines.remove(0);
    }

    // Ensure a trailing empty line.
    if !lines.is_empty() && !lines.last().is_none_or(|l| l.is_empty()) {
        lines.push(String::new());
    }

    lines
}

/// Expand tabs in `s` using `tabsize` spaces.
fn expand_tabs(s: &str, tabsize: usize) -> String {
    let mut result = String::with_capacity(s.len());
    let mut col = 0usize;
    for c in s.chars() {
        if c == '\t' {
            let spaces = tabsize - (col % tabsize);
            result.extend(std::iter::repeat_n(' ', spaces));
            col += spaces;
        } else {
            if c == '\n' {
                col = 0;
            } else {
                col += 1;
            }
            result.push(c);
        }
    }
    result
}

// ── prepare_commentdoc ────────────────────────────────────────────────────────

/// Extract documentation comment lines (starting with `#:`) and return them
/// as a list of lines.
///
/// Returns an empty `Vec` if no `#:` comments are found.  If any are found,
/// a trailing empty string is appended (so the block can be used as a
/// docutils ViewList separator).
///
/// Mirrors `sphinx.util.docstrings.prepare_commentdoc`.
///
/// ```rust
/// use sphinxdocrs::util_docstrings::prepare_commentdoc;
/// assert_eq!(prepare_commentdoc("hello world"), Vec::<String>::new());
/// assert_eq!(prepare_commentdoc("#: hello world"), vec!["hello world", ""]);
/// assert_eq!(prepare_commentdoc("#:  hello world"), vec![" hello world", ""]);
/// ```
pub fn prepare_commentdoc(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    for raw_line in s.expand_tabs_iter(8) {
        let line = raw_line.trim();
        if let Some(rest) = line.strip_prefix("#:") {
            // The first space after the comment is ignored.
            let text = rest.strip_prefix(' ').unwrap_or(rest);
            result.push(text.to_string());
        }
    }
    if !result.is_empty() && !result.last().is_none_or(|l: &String| l.is_empty()) {
        result.push(String::new());
    }
    result
}

trait ExpandTabsIter {
    fn expand_tabs_iter(&self, tabsize: usize) -> Vec<String>;
}

impl ExpandTabsIter for str {
    fn expand_tabs_iter(&self, tabsize: usize) -> Vec<String> {
        expand_tabs(self, tabsize)
            .lines()
            .map(String::from)
            .collect()
    }
}

// ── separate_metadata ─────────────────────────────────────────────────────────

/// Separate docstring content into `:meta …:` metadata and the remaining text.
///
/// Scans the lines produced by [`prepare_docstring`] and extracts RST field
/// list items whose field name starts with `"meta "`.  Items in paragraphs
/// (after non-field-list lines) are left in the main content.
///
/// Returns `(cleaned_docstring, metadata_map)`.
/// If `s` is `None` or empty, returns it unchanged.
///
/// Mirrors `sphinx.util.docstrings.separate_metadata`.
pub fn separate_metadata(
    s: Option<&str>,
) -> (Option<String>, std::collections::HashMap<String, String>) {
    let mut metadata = std::collections::HashMap::new();
    let field_re = field_list_item_regex();

    let s = match s {
        None => return (None, metadata),
        Some(s) => s,
    };

    let mut lines: Vec<String> = Vec::new();
    let mut in_other_element = false;

    for line in prepare_docstring(s, 8) {
        if line.trim().is_empty() {
            in_other_element = false;
            lines.push(line);
        } else if let Some(caps) = field_re.find(&line) {
            if !in_other_element {
                // Extract field name from `:field_name:` at start of line.
                let field_text = &line[1..]; // skip leading ':'
                let field_name = field_text.split(':').next().unwrap_or("").trim();
                if let Some(meta_name) = field_name.strip_prefix("meta ") {
                    let value = line[caps.end()..].trim().to_string();
                    metadata.insert(meta_name.trim().to_string(), value);
                    // Don't push to lines — consumed as metadata.
                } else {
                    lines.push(line);
                }
            } else {
                lines.push(line);
            }
        } else {
            in_other_element = true;
            lines.push(line);
        }
    }

    let result = lines.join("\n");
    (Some(result), metadata)
}

/// Regex matching a docutils RST field list item at the start of a line.
///
/// Mirrors `Body.patterns['field_marker']` from docutils:
/// `r':(?![: ])([^:\\]|\\.|:(?![ `])\S)*(?<! ):( +|$)'`
fn field_list_item_regex() -> Regex {
    // Simplified: starts with ':', has field name, ends with ': '
    // This matches the common case without the full lookbehind.
    Regex::new(r"^:[^:]+:( |$)").unwrap()
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── prepare_docstring ─────────────────────────────────────────────────────

    #[test]
    fn single_line() {
        assert_eq!(
            prepare_docstring("single line docstring", 8),
            vec!["single line docstring", ""]
        );
    }

    #[test]
    fn multiline_common_indent() {
        let s = "multiline docstring

                Lorem ipsum dolor sit amet, consectetur adipiscing elit,
                sed do eiusmod tempor incididunt ut labore et dolore magna
                aliqua::

                  Ut enim ad minim veniam, quis nostrud exercitation
                    ullamco laboris nisi ut aliquip ex ea commodo consequat.
                ";
        let result = prepare_docstring(s, 8);
        assert_eq!(result[0], "multiline docstring");
        assert_eq!(result[1], "");
        assert_eq!(
            result[2],
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit,"
        );
        assert!(result.last().map_or(false, |l| l.is_empty()));
    }

    #[test]
    fn leading_blank_lines_stripped() {
        let s =
            "\n\n                multiline docstring with leading empty lines\n                ";
        let result = prepare_docstring(s, 8);
        assert_eq!(
            result,
            vec!["multiline docstring with leading empty lines", ""]
        );
    }

    // ── prepare_commentdoc ────────────────────────────────────────────────────

    #[test]
    fn no_comment_doc_returns_empty() {
        assert_eq!(prepare_commentdoc("hello world"), Vec::<String>::new());
    }

    #[test]
    fn simple_comment_doc() {
        assert_eq!(
            prepare_commentdoc("#: hello world"),
            vec!["hello world", ""]
        );
    }

    #[test]
    fn comment_doc_with_extra_space() {
        // '#:  hello world' → first space stripped → ' hello world'
        assert_eq!(
            prepare_commentdoc("#:  hello world"),
            vec![" hello world", ""]
        );
    }

    #[test]
    fn multi_line_comment_doc() {
        assert_eq!(
            prepare_commentdoc("#: hello\n#: world\n"),
            vec!["hello", "world", ""]
        );
    }

    // ── separate_metadata ─────────────────────────────────────────────────────

    #[test]
    fn metadata_only() {
        let (docstring, meta) = separate_metadata(Some(":meta foo: bar\n:meta baz:\n"));
        assert_eq!(docstring.as_deref().unwrap_or("").trim(), "");
        assert_eq!(meta.get("foo").map(String::as_str), Some("bar"));
        assert_eq!(meta.get("baz").map(String::as_str), Some(""));
    }

    #[test]
    fn metadata_mixed_with_non_meta() {
        let (docstring, meta) = separate_metadata(Some(":meta foo: bar\n:param baz:\n"));
        let doc = docstring.unwrap_or_default();
        assert!(doc.contains(":param baz:"), "doc={doc:?}");
        assert_eq!(meta.get("foo").map(String::as_str), Some("bar"));
        assert!(!meta.contains_key("baz"));
    }

    #[test]
    fn metadata_after_paragraph_not_extracted() {
        let text = "blah blah blah\n:meta foo: bar\n:meta baz:\n";
        let (docstring, meta) = separate_metadata(Some(text));
        assert_eq!(meta.len(), 0, "should be no metadata, got {meta:?}");
        let doc = docstring.unwrap_or_default();
        assert!(doc.contains("blah blah blah"), "doc={doc:?}");
    }

    #[test]
    fn metadata_after_blank_line_paragraph() {
        let text = "blah blah blah\n\n:meta foo: bar\n:meta baz:\n";
        let (docstring, meta) = separate_metadata(Some(text));
        assert_eq!(meta.get("foo").map(String::as_str), Some("bar"));
        assert_eq!(meta.get("baz").map(String::as_str), Some(""));
        let doc = docstring.unwrap_or_default();
        assert!(doc.contains("blah blah blah"), "doc={doc:?}");
    }

    #[test]
    fn none_input() {
        let (result, meta) = separate_metadata(None);
        assert!(result.is_none());
        assert!(meta.is_empty());
    }
}
