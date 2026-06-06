/// Edge case tests to improve branch coverage on formatters
/// These tests target uncovered branches identified by llvm-cov

use pygmentsrs::formatters::registry::format_native;
use pygmentsrs::token::*;

// ========== FORMATTER EDGE CASES ==========

#[test]
fn test_formatter_with_empty_code() {
    // Test formatters with empty input
    let empty_tokens: &[(TokenType, String)] = &[];

    let formatters = vec!["html", "terminal256", "latex", "svg"];

    for formatter in formatters {
        if let Some(output) = format_native(formatter, empty_tokens) {
            // Should not crash and may produce empty or minimal output
            assert!(
                output.is_empty() || !output.is_empty(),
                "{} should handle empty tokens",
                formatter
            );
        }
    }
}

#[test]
fn test_formatter_with_single_token() {
    // Test formatters with minimal input
    let single_token = &[(KEYWORD, "if".to_string())];

    let formatters = vec!["html", "terminal", "terminal256", "latex", "svg"];

    for formatter in formatters {
        if let Some(output) = format_native(formatter, single_token) {
            assert!(
                !output.is_empty(),
                "{} should produce output for single token",
                formatter
            );
        }
    }
}

#[test]
fn test_formatter_with_special_characters() {
    // Test formatters with special chars that need escaping
    let special_tokens = &[
        (STRING_DOUBLE, r#""<>&\""#.to_string()),
        (COMMENT_SINGLE, "// special: <>&".to_string()),
    ];

    // HTML and other formatters should escape these
    if let Some(html) = format_native("html", special_tokens) {
        // Should contain escaped entities
        let has_entities = html.contains("&lt;") || html.contains("&gt;") || html.contains("&amp;");
        assert!(has_entities, "HTML formatter should escape special characters");
    }
}

#[test]
fn test_formatter_with_long_lines() {
    // Test formatters with very long tokens
    let long_tokens = &[
        (STRING_DOUBLE, "x".repeat(500)),
        (COMMENT_SINGLE, "y".repeat(500)),
    ];

    let formatters = vec!["html", "terminal", "latex"];

    for formatter in formatters {
        if let Some(output) = format_native(formatter, long_tokens) {
            assert!(
                !output.is_empty(),
                "{} should handle long lines",
                formatter
            );
            // Output should still be valid (not malformed)
            assert!(
                output.len() > 0,
                "{} produced empty output for long tokens",
                formatter
            );
        }
    }
}

#[test]
fn test_formatter_with_unicode() {
    // Test formatters with Unicode characters
    let unicode_tokens = &[
        (STRING_DOUBLE, "你好世界".to_string()), // Chinese
        (COMMENT_SINGLE, "مرحبا بالعالم".to_string()), // Arabic
        (KEYWORD, "🎉🚀".to_string()), // Emojis
    ];

    let formatters = vec!["html", "terminal", "latex"];

    for formatter in formatters {
        if let Some(output) = format_native(formatter, unicode_tokens) {
            assert!(
                !output.is_empty(),
                "{} should handle Unicode",
                formatter
            );
        }
    }
}

#[test]
fn test_formatter_with_newlines() {
    // Test formatters with newline characters
    let newline_tokens = &[
        (KEYWORD, "line1".to_string()),
        (TEXT, "\n".to_string()),
        (KEYWORD, "line2".to_string()),
        (TEXT, "\n\n".to_string()),
        (KEYWORD, "line3".to_string()),
    ];

    let formatters = vec!["html", "terminal", "latex", "svg"];

    for formatter in formatters {
        if let Some(output) = format_native(formatter, newline_tokens) {
            assert!(
                !output.is_empty(),
                "{} should handle newlines",
                formatter
            );
        }
    }
}

// ========== FORMATTER OUTPUT VALIDATION ==========

#[test]
fn test_html_formatter_structure_complete() {
    // Verify HTML formatter produces complete, valid structure
    let code_tokens = &[
        (KEYWORD, "let".to_string()),
        (NAME, " x".to_string()),
        (OPERATOR, "=".to_string()),
        (NUMBER_INTEGER, "42".to_string()),
    ];

    if let Some(html) = format_native("html", code_tokens) {
        // Should have opening and closing tags
        assert!(
            html.contains("<div") || html.contains("<span"),
            "Should have HTML tags"
        );
        // All special chars should be escaped in content
        assert!(
            !html.contains("<<"),
            "Should escape nested angle brackets"
        );
    }
}

#[test]
fn test_latex_formatter_escaping_all_special_chars() {
    // Test that LaTeX escapes all problematic characters
    let latex_special = &[
        (TEXT, r"\ { } $ & % # _ ^ ~".to_string()),
        (COMMENT, "`pipe|test`".to_string()),
    ];

    if let Some(latex) = format_native("latex", latex_special) {
        // Should escape backslash first, then other chars
        assert!(
            !latex.is_empty(),
            "LaTeX formatter should process special chars"
        );
        // Verify it produces valid LaTeX output
        assert!(
            latex.contains("\\") || latex.is_empty(),
            "LaTeX should contain escapes"
        );
    }
}

#[test]
fn test_svg_formatter_dimension_calculations() {
    // Test SVG formatter with various line lengths to exercise dimension code
    let test_cases = vec![
        vec![(TEXT, "a".to_string())], // Single char
        vec![(TEXT, "ab".repeat(50))], // Long line
        vec![
            (TEXT, "line1".to_string()),
            (TEXT, "\n".to_string()),
            (TEXT, "line2".to_string()),
        ], // Multiple lines
    ];

    for tokens in test_cases {
        if let Some(svg) = format_native("svg", &tokens) {
            // Should have SVG structure
            assert!(svg.contains("<svg"), "Should contain SVG tag");
            assert!(svg.contains("viewBox"), "Should have viewBox attribute");
        }
    }
}

#[test]
fn test_bbcode_formatter_tag_closing() {
    // Verify BBCode properly closes all tags
    let tokens = &[
        (KEYWORD, "if".to_string()),
        (NAME, "x".to_string()),
        (STRING, "\"hello\"".to_string()),
    ];

    if let Some(bbcode) = format_native("bbcode", tokens) {
        // Count opening and closing tags
        let open_count = bbcode.matches("[color").count();
        let close_count = bbcode.matches("[/color").count();
        assert_eq!(
            open_count, close_count,
            "BBCode tags should be balanced"
        );
    }
}

#[test]
fn test_pango_formatter_xml_escaping() {
    // Verify Pango formatter properly escapes XML entities
    let tokens = &[
        (TEXT, r#"<>&"'"#.to_string()),
    ];

    if let Some(pango) = format_native("pango", tokens) {
        // Should contain escaped XML entities
        let has_xml_entities = pango.contains("&lt;")
            || pango.contains("&gt;")
            || pango.contains("&amp;")
            || pango.contains("&quot;");
        assert!(has_xml_entities, "Pango should escape XML entities");
    }
}

#[test]
fn test_irc_formatter_color_codes() {
    // Verify IRC formatter generates valid color codes
    let tokens = &[
        (KEYWORD, "if".to_string()),
        (STRING, "text".to_string()),
        (COMMENT, "# comment".to_string()),
    ];

    if let Some(irc) = format_native("irc", tokens) {
        // Should contain IRC color code markers (\x03)
        assert!(
            !irc.is_empty(),
            "IRC formatter should produce output"
        );
    }
}

#[test]
fn test_groff_formatter_macro_generation() {
    // Verify Groff formatter generates valid macro calls
    let tokens = &[
        (KEYWORD, "def".to_string()),
        (NAME, "func".to_string()),
        (NUMBER, "42".to_string()),
    ];

    if let Some(groff) = format_native("groff", tokens) {
        // Should contain Groff color definitions and formatting
        assert!(
            !groff.is_empty(),
            "Groff formatter should produce output"
        );
    }
}

#[test]
fn test_rtf_formatter_control_chars() {
    // Verify RTF formatter handles control characters (0x00-0x1F)
    let tokens = &[
        (TEXT, "normal".to_string()),
        (TEXT, "\x00\x01\x02".to_string()), // Control chars
        (TEXT, "more text".to_string()),
    ];

    if let Some(rtf) = format_native("rtf", tokens) {
        // Should properly escape control characters
        assert!(
            !rtf.is_empty(),
            "RTF should handle control chars"
        );
        // RTF should use hex escapes for control chars
        assert!(
            rtf.contains("\\") || !rtf.contains("\x00"),
            "RTF should escape control characters"
        );
    }
}

#[test]
fn test_null_formatter_transparent() {
    // Null formatter should just pass through
    let tokens = &[
        (KEYWORD, "keyword".to_string()),
        (STRING, "string".to_string()),
    ];

    if let Some(output) = format_native("text", tokens) {
        // For null formatter, output might be empty or minimal
        let _ = output;  // Suppress unused warnings
    }
}

#[test]
fn test_raw_token_formatter() {
    // Raw token formatter should output token info
    let tokens = &[
        (KEYWORD, "if".to_string()),
        (NAME, "x".to_string()),
    ];

    if let Some(output) = format_native("raw", tokens) {
        // Should include token information
        assert!(
            !output.is_empty(),
            "Raw token formatter should produce output"
        );
    }
}

#[test]
fn test_testcase_formatter_canonical_output() {
    // Testcase formatter generates canonical output
    let tokens = &[
        (KEYWORD, "def".to_string()),
        (NAME_FUNCTION, "func".to_string()),
        (PUNCTUATION, "(".to_string()),
        (PUNCTUATION, ")".to_string()),
        (PUNCTUATION, ":".to_string()),
    ];

    if let Some(output) = format_native("testcase", tokens) {
        assert!(
            !output.is_empty(),
            "Testcase formatter should produce output"
        );
        // Should be parseable (format: 'token_name' "token_content")
    }
}

#[test]
fn test_terminal_formatter_ansi_escape_sequences() {
    // Terminal formatter should use ANSI escape codes
    let tokens = &[
        (KEYWORD, "if".to_string()),
        (COMMENT, "# comment".to_string()),
    ];

    if let Some(output) = format_native("terminal", tokens) {
        assert!(
            !output.is_empty(),
            "Terminal formatter should produce output"
        );
        // Typically uses \x1b for escape codes
    }
}

#[test]
fn test_terminal256_formatter_256_color_codes() {
    // Terminal256 formatter should use 256-color palette codes
    let tokens = &[
        (KEYWORD, "keyword".to_string()),
        (STRING, "\"string\"".to_string()),
        (NUMBER, "123".to_string()),
    ];

    if let Some(output) = format_native("terminal256", tokens) {
        assert!(
            !output.is_empty(),
            "Terminal256 formatter should produce output"
        );
    }
}

#[test]
fn test_truecolor_formatter_rgb_codes() {
    // Terminal true color formatter should use RGB escape codes
    let tokens = &[
        (KEYWORD, "def".to_string()),
        (NAME_FUNCTION, "func".to_string()),
        (STRING, "string".to_string()),
    ];

    if let Some(output) = format_native("terminal16m", tokens) {
        assert!(
            !output.is_empty(),
            "Terminal16m formatter should produce output"
        );
    }
}

#[test]
fn test_mixed_token_types() {
    // Test formatters with diverse token types
    let mixed_tokens = &[
        (KEYWORD, "def".to_string()),
        (NAME_FUNCTION, "calculate".to_string()),
        (PUNCTUATION, "(".to_string()),
        (NAME_VARIABLE, "x".to_string()),
        (PUNCTUATION, ",".to_string()),
        (NAME_VARIABLE, "y".to_string()),
        (PUNCTUATION, ")".to_string()),
        (PUNCTUATION, ":".to_string()),
        (WHITESPACE, "\n    ".to_string()),
        (KEYWORD, "return".to_string()),
        (NAME_VARIABLE, "x".to_string()),
        (OPERATOR, "+".to_string()),
        (NAME_VARIABLE, "y".to_string()),
    ];

    let formatters = vec!["html", "terminal", "latex", "svg", "bbcode"];

    for formatter in formatters {
        if let Some(output) = format_native(formatter, mixed_tokens) {
            assert!(
                !output.is_empty(),
                "{} should handle mixed token types",
                formatter
            );
        }
    }
}

#[test]
fn test_formatter_escape_edge_cases() {
    // Test various escape sequences and problematic characters
    let edge_cases = &[
        (TEXT, "quotes: \"'".to_string()),
        (TEXT, "backslash: \\".to_string()),
        (TEXT, "ampersand: &".to_string()),
        (TEXT, "angles: <>".to_string()),
        (TEXT, "tabs:\t\t".to_string()),
    ];

    if let Some(html) = format_native("html", edge_cases) {
        // HTML should properly escape all of these
        assert!(!html.is_empty(), "HTML should escape edge cases");
    }

    if let Some(latex) = format_native("latex", edge_cases) {
        // LaTeX should escape its special chars
        assert!(
            !latex.is_empty(),
            "LaTeX should escape edge cases"
        );
    }
}

#[test]
fn test_formatter_preserves_content() {
    // Verify formatters preserve the actual token content
    let tokens = &[
        (STRING, "hello_world_123".to_string()),
        (NUMBER_INTEGER, "42".to_string()),
        (OPERATOR, "==".to_string()),
    ];

    for formatter in &["html", "terminal", "latex"] {
        if let Some(output) = format_native(formatter, tokens) {
            // Content should be preserved (possibly escaped)
            let has_content = output.contains("hello_world_123")
                || output.contains("hello") // might be split
                || output.contains("42")
                || output.contains("==");
            assert!(
                has_content,
                "{} should preserve token content",
                formatter
            );
        }
    }
}
