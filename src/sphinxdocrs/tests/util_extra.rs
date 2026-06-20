//! Integration tests for `sphinxdocrs::util_uri`, `sphinxdocrs::util_lines`,
//! and `sphinxdocrs::util_docstrings`.
//!
//! Mirrors the upstream test suites:
//! - `sphinx/tests/test_util/test_util_uri.py`
//! - `sphinx/tests/test_util/test_util_lines.py`
//! - `sphinx/tests/test_util/test_util_docstrings.py`

use rstest::*;
use sphinxdocrs::util_docstrings::{prepare_commentdoc, prepare_docstring, separate_metadata};
use sphinxdocrs::util_lines::parse_line_num_spec;
use sphinxdocrs::util_uri::{encode_uri, is_url};

// ═══════════════════════════════════════════════════════════════════════════════
// util_uri — mirrors test_util_uri.py
// ═══════════════════════════════════════════════════════════════════════════════

// ── is_url ────────────────────────────────────────────────────────────────────

#[rstest]
#[case("https://example.com", true)]
#[case("ftp://files.example.org/a.txt", true)]
#[case("http://localhost:8080", true)]
#[case("", false)]
#[case("example.com", false)]
#[case("docs/index.rst", false)]
#[case("/absolute/path", false)]
fn is_url_cases(#[case] url: &str, #[case] expected: bool) {
    assert_eq!(is_url(url), expected, "is_url({url:?})");
}

// ── encode_uri ────────────────────────────────────────────────────────────────

#[test]
fn encode_uri_cyrillic_path() {
    // Mirrors test_encode_uri() — Cyrillic path encoding
    let uri = "https://ru.wikipedia.org/wiki/Система_управления_базами_данных";
    let encoded = encode_uri(uri);
    let expected = concat!(
        "https://ru.wikipedia.org/wiki/",
        "%D0%A1%D0%B8%D1%81%D1%82%D0%B5%D0%BC%D0%B0_",
        "%D1%83%D0%BF%D1%80%D0%B0%D0%B2%D0%BB%D0%B5%D0%BD%D0%B8%D1%8F_",
        "%D0%B1%D0%B0%D0%B7%D0%B0%D0%BC%D0%B8_%D0%B4%D0%B0%D0%BD%D0%BD%D1%8B%D1%85"
    );
    assert_eq!(encoded, expected);
}

#[test]
fn encode_uri_already_encoded_query_unchanged() {
    // Mirrors test_encode_uri() — pre-encoded query passes through unchanged.
    let uri = concat!(
        "https://github.com/search?",
        "utf8=%E2%9C%93&",
        "q=is%3Aissue+is%3Aopen+is%3Asprint-friendly+user%3Ajupyter&",
        "type=Issues&ref=searchresults"
    );
    assert_eq!(encode_uri(uri), uri);
}

#[test]
fn encode_uri_plain_ascii_unchanged() {
    let uri = "https://example.com/docs/index.html";
    assert_eq!(encode_uri(uri), uri);
}

#[test]
fn encode_uri_preserves_fragment() {
    let uri = "https://example.com/page#section";
    let encoded = encode_uri(uri);
    assert!(encoded.ends_with("#section"), "got: {encoded}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// util_lines — mirrors test_util_lines.py
// ═══════════════════════════════════════════════════════════════════════════════

// All cases from test_parse_line_num_spec.

#[test]
fn lines_single_numbers() {
    assert_eq!(parse_line_num_spec("1,2,3", 10).unwrap(), vec![0, 1, 2]);
}

#[test]
fn lines_with_spaces() {
    assert_eq!(parse_line_num_spec("4, 5, 6", 10).unwrap(), vec![3, 4, 5]);
}

#[test]
fn lines_left_open() {
    assert_eq!(parse_line_num_spec("-4", 10).unwrap(), vec![0, 1, 2, 3]);
}

#[test]
fn lines_closed_range() {
    assert_eq!(parse_line_num_spec("7-9", 10).unwrap(), vec![6, 7, 8]);
}

#[test]
fn lines_right_open() {
    assert_eq!(parse_line_num_spec("7-", 10).unwrap(), vec![6, 7, 8, 9]);
}

#[test]
fn lines_mixed() {
    assert_eq!(
        parse_line_num_spec("1,7-", 10).unwrap(),
        vec![0, 6, 7, 8, 9]
    );
}

#[test]
fn lines_single_element_range() {
    assert_eq!(parse_line_num_spec("7-7", 10).unwrap(), vec![6]);
}

#[test]
fn lines_right_open_beyond_total() {
    assert_eq!(parse_line_num_spec("11-", 10).unwrap(), vec![10]);
}

#[rstest]
#[case("1-2-3")]
#[case("abc-def")]
#[case("-")]
#[case("3-1")]
fn lines_invalid_spec(#[case] spec: &str) {
    let err = parse_line_num_spec(spec, 10).unwrap_err();
    assert!(
        err.contains("invalid line number spec"),
        "spec={spec:?}, err={err}"
    );
    assert!(err.contains(spec), "spec={spec:?}, err={err}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// util_docstrings — mirrors test_util_docstrings.py
// ═══════════════════════════════════════════════════════════════════════════════

// ── prepare_docstring ─────────────────────────────────────────────────────────

#[test]
fn docstring_multiline_common_indent() {
    // Mirrors the long multiline test from test_prepare_docstring
    let docstring = "multiline docstring

                Lorem ipsum dolor sit amet, consectetur adipiscing elit,
                sed do eiusmod tempor incididunt ut labore et dolore magna
                aliqua::

                  Ut enim ad minim veniam, quis nostrud exercitation
                    ullamco laboris nisi ut aliquip ex ea commodo consequat.
                ";

    let result = prepare_docstring(docstring, 8);
    assert_eq!(result[0], "multiline docstring");
    assert_eq!(result[1], "");
    assert_eq!(
        result[2],
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit,"
    );
    assert_eq!(
        result[3],
        "sed do eiusmod tempor incididunt ut labore et dolore magna"
    );
    assert_eq!(result[4], "aliqua::");
    assert_eq!(result[5], "");
    assert_eq!(
        result[6],
        "  Ut enim ad minim veniam, quis nostrud exercitation"
    );
    assert_eq!(
        result[7],
        "    ullamco laboris nisi ut aliquip ex ea commodo consequat."
    );
    assert_eq!(result[8], "");
}

#[test]
fn docstring_leading_blank_lines() {
    let docstring =
        "\n\n                multiline docstring with leading empty lines\n                ";
    let result = prepare_docstring(docstring, 8);
    assert_eq!(
        result,
        vec!["multiline docstring with leading empty lines", ""]
    );
}

#[test]
fn docstring_single_line() {
    assert_eq!(
        prepare_docstring("single line docstring", 8),
        vec!["single line docstring", ""]
    );
}

// ── prepare_commentdoc ────────────────────────────────────────────────────────

#[test]
fn commentdoc_plain_text() {
    assert_eq!(prepare_commentdoc("hello world"), Vec::<String>::new());
}

#[test]
fn commentdoc_simple() {
    assert_eq!(
        prepare_commentdoc("#: hello world"),
        vec!["hello world", ""]
    );
}

#[test]
fn commentdoc_extra_space_preserved() {
    assert_eq!(
        prepare_commentdoc("#:  hello world"),
        vec![" hello world", ""]
    );
}

#[test]
fn commentdoc_multiline() {
    assert_eq!(
        prepare_commentdoc("#: hello\n#: world\n"),
        vec!["hello", "world", ""]
    );
}

// ── separate_metadata ─────────────────────────────────────────────────────────

#[test]
fn metadata_only_field_list() {
    let text = ":meta foo: bar\n:meta baz:\n";
    let (docstring, meta) = separate_metadata(Some(text));
    let doc = docstring.unwrap_or_default();
    // The cleaned docstring should be essentially empty (just whitespace/newlines)
    assert!(
        doc.trim().is_empty(),
        "docstring should be empty, got: {doc:?}"
    );
    assert_eq!(meta.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(meta.get("baz").map(String::as_str), Some(""));
}

#[test]
fn metadata_with_non_meta_field() {
    let text = ":meta foo: bar\n:param baz:\n";
    let (docstring, meta) = separate_metadata(Some(text));
    let doc = docstring.unwrap_or_default();
    assert!(doc.contains(":param baz:"), "doc={doc:?}");
    assert_eq!(meta.get("foo").map(String::as_str), Some("bar"));
    assert!(!meta.contains_key("baz"));
}

#[test]
fn metadata_after_paragraph_not_extracted() {
    // Field list items after a paragraph are NOT treated as metadata.
    let text = "blah blah blah\n:meta foo: bar\n:meta baz:\n";
    let (docstring, meta) = separate_metadata(Some(text));
    assert!(
        meta.is_empty(),
        "no metadata should be extracted, got: {meta:?}"
    );
    let doc = docstring.unwrap_or_default();
    assert!(doc.contains("blah blah blah"), "doc={doc:?}");
}

#[test]
fn metadata_after_blank_line_paragraph_extracted() {
    // Blank line separates paragraph from field list → metadata extracted.
    let text = "blah blah blah\n\n:meta foo: bar\n:meta baz:\n";
    let (docstring, meta) = separate_metadata(Some(text));
    assert_eq!(meta.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(meta.get("baz").map(String::as_str), Some(""));
    let doc = docstring.unwrap_or_default();
    assert!(doc.contains("blah blah blah"), "doc={doc:?}");
}

#[test]
fn metadata_none_input() {
    let (result, meta) = separate_metadata(None);
    assert!(result.is_none());
    assert!(meta.is_empty());
}
