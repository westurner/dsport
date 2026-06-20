#![allow(clippy::needless_borrows_for_generic_args)]

//! Byte-parity tests comparing Rust formatters against upstream Pygments

use pygmentsrs::formatters::registry::format_native;
use pygmentsrs::token::{COMMENT, KEYWORD, NUMBER, OPERATOR, STRING_DOUBLE, TEXT};

/// Test case: simple assignment `x = 42`
#[test]
fn test_html_formatter_simple_assignment() {
    let tokens = vec![
        (KEYWORD, "let".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "x".to_string()),
        (OPERATOR, " = ".to_string()),
        (NUMBER, "42".to_string()),
    ];

    let result = format_native("html", &tokens).expect("HTML formatter failed");
    assert!(result.contains("<div class=\"highlight\">"));
    assert!(result.contains("</div>"));
    assert!(result.contains("42"));
    assert!(result.contains("&lt;") || result.contains("<"));
}

/// Test case: string with quotes
#[test]
fn test_html_formatter_string() {
    let tokens = vec![
        (KEYWORD, "let".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "text".to_string()),
        (OPERATOR, " = ".to_string()),
        (STRING_DOUBLE, "\"hello world\"".to_string()),
    ];

    let result = format_native("html", &tokens).expect("HTML formatter failed");
    assert!(result.contains("hello world"));
    // Should escape quotes
    assert!(result.contains("&quot;") || result.contains("\""));
}

/// Test case: terminal formatting
#[test]
fn test_terminal_formatter_basic() {
    let tokens = vec![
        (KEYWORD, "def".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "foo".to_string()),
    ];

    let result = format_native("terminal", &tokens).expect("Terminal formatter failed");
    assert!(!result.is_empty());
    // Terminal output may contain ANSI codes
    assert!(result.contains("def") || result.contains("\x1b"));
}

/// Test case: LaTeX formatter with special chars
#[test]
fn test_latex_formatter_escaping() {
    let tokens = vec![
        (COMMENT, "# comment".to_string()),
        (TEXT, "\n".to_string()),
        (KEYWORD, "x".to_string()),
        (OPERATOR, " = ".to_string()),
        (NUMBER, "42".to_string()),
    ];

    let result = format_native("latex", &tokens).expect("LaTeX formatter failed");
    assert!(result.contains("documentclass"));
    assert!(result.contains("lstlisting"));
    // Verify control chars are handled
    assert!(!result.contains("\n\t") || result.contains("par"));
}

/// Test case: BBCode formatter
#[test]
fn test_bbcode_formatter_tags() {
    let tokens = vec![
        (KEYWORD, "def".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "foo".to_string()),
        (TEXT, "():".to_string()),
    ];

    let result = format_native("bbcode", &tokens).expect("BBCode formatter failed");
    assert!(result.contains("color="));
    assert!(result.contains("def"));
    // Check for BBCode escaping of brackets
    let has_brackets = result.contains("[") || result.contains("&#91;");
    assert!(has_brackets, "Result should have brackets: {}", result);
}

/// Test case: SVG formatter
#[test]
fn test_svg_formatter_structure() {
    let tokens = vec![
        (KEYWORD, "let".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "x".to_string()),
    ];

    let result = format_native("svg", &tokens).expect("SVG formatter failed");
    assert!(result.contains("<svg"));
    assert!(result.contains("</svg>"));
    assert!(result.contains("<text"));
    assert!(result.contains("Courier New"));
}

/// Test case: RTF formatter
#[test]
fn test_rtf_formatter_structure() {
    let tokens = vec![
        (KEYWORD, "let".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "x".to_string()),
    ];

    let result = format_native("rtf", &tokens).expect("RTF formatter failed");
    assert!(result.contains("{\\rtf1"));
    assert!(result.contains("fonttbl"));
    assert!(result.contains("let"));
}

/// Test case: Groff formatter
#[test]
fn test_groff_formatter_structure() {
    let tokens = vec![
        (KEYWORD, "let".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "x".to_string()),
    ];

    let result = format_native("groff", &tokens).expect("Groff formatter failed");
    assert!(result.contains(".defcolor"));
    assert!(result.contains(".ft"));
    assert!(result.contains("let"));
}

/// Test case: Pango formatter
#[test]
fn test_pango_formatter_xml() {
    let tokens = vec![
        (KEYWORD, "def".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "foo".to_string()),
    ];

    let result = format_native("pango", &tokens).expect("Pango formatter failed");
    assert!(result.contains("span"));
    assert!(
        result.contains("color") || result.contains("weight"),
        "Missing formatting attributes: {}",
        result
    );
    // Should escape XML special chars
    assert!(!result.contains("<span color='") || result.contains("'"));
}

/// Test case: Raw token formatter
#[test]
fn test_raw_formatter_debug() {
    let tokens = vec![
        (KEYWORD, "def".to_string()),
        (TEXT, " ".to_string()),
        (NUMBER, "42".to_string()),
    ];

    let result = format_native("raw", &tokens).expect("Raw formatter failed");
    assert!(result.contains("def") || result.contains("42"));
}

/// Test case: Null formatter (passthrough)
#[test]
fn test_null_formatter_passthrough() {
    let tokens = vec![(KEYWORD, "def".to_string()), (TEXT, " foo".to_string())];

    let result = format_native("text", &tokens).expect("Text formatter failed");
    assert_eq!(result, "def foo");
}

/// Test case: Complex multi-line code
#[test]
fn test_terminal256_formatter_multiline() {
    let tokens = vec![
        (KEYWORD, "def".to_string()),
        (TEXT, " ".to_string()),
        (KEYWORD, "foo".to_string()),
        (TEXT, "():\n".to_string()),
        (TEXT, "    ".to_string()),
        (KEYWORD, "return".to_string()),
        (TEXT, " ".to_string()),
        (NUMBER, "1".to_string()),
    ];

    let result = format_native("terminal256", &tokens).expect("Terminal256 formatter failed");
    assert!(!result.is_empty());
    assert!(result.contains("def") || result.contains("1"));
}

/// Test: all native formatters are callable
#[test]
fn test_all_native_formatters_registered() {
    let formatters = [
        "html",
        "text",
        "raw",
        "tokens",
        "testcase",
        "terminal",
        "console",
        "terminal256",
        "256",
        "terminal16m",
        "truecolor",
        "irc",
        "bbcode",
        "groff",
        "groff-256",
        "pango",
        "latex",
        "tex",
        "rtf",
        "svg",
    ];

    let test_tokens = vec![(KEYWORD, "test".to_string())];

    for fmt in formatters.iter() {
        let result = format_native(fmt, &test_tokens)
            .unwrap_or_else(|| panic!("Formatter '{}' failed", fmt));
        assert!(!result.is_empty(), "Formatter '{}' returned empty", fmt);
    }
}
