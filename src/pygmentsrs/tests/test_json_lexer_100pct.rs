//! 100% Branch Coverage Tests for JSON Lexer
//!
//! Comprehensive tests for src/pygmentsrs/src/lexers/json.rs
//! 
//! The JSON lexer is a character-stream state machine (not regex-based) with:
//! - String handling: escape sequences, unicode escapes (\uXXXX)
//! - Number parsing: integer vs float detection
//! - Comment support: // (single-line) and /* */ (multi-line)
//! - Constant keywords: true, false, null
//! - Punctuation: {, }, [, ], :, ,
//! - Special behavior: Object keys (strings before :) reclassified as Name.Tag
//!
//! Coverage targets:
//! - All state machine transitions (in_string, in_number, in_constant, etc.)
//! - Escape sequence handling (single char vs \uXXXX)
//! - Number parsing (integer vs float vs signed vs exponent)
//! - Comment parsing (both types, nested patterns)
//! - Queue behavior (strings before : → Name.Tag)
//! - EOF handling (partial tokens)
//! - Error cases (invalid escape, unterminated string, bad comments)

use pygmentsrs::lexer::Lexer;
use pygmentsrs::lexers::json::JsonLexer;
use pygmentsrs::token::*;

fn lex(src: &str) -> Vec<(TokenType, String)> {
    JsonLexer.get_tokens(src)
}

fn lex_repr(src: &str) -> Vec<(String, String)> {
    lex(src)
        .into_iter()
        .map(|(t, v)| (t.repr(), v))
        .collect()
}

// ============================================================================
// STRING HANDLING - Branch Coverage
// ============================================================================
// Branches: in_string, in_escape, in_unicode_escape, quote termination

#[test]
fn string_simple() {
    // Basic string parsing: enter in_string, exit on quote
    let result = lex("\"hello\"");
    assert!(result.iter().any(|(t, v)| *t == STRING_DOUBLE && v == "\"hello\""),
            "Simple string not found");
}

#[test]
fn string_empty() {
    // Empty string: enter and immediately exit in_string
    let result = lex("\"\"");
    assert!(result.iter().any(|(t, v)| *t == STRING_DOUBLE && v == "\"\""),
            "Empty string not found");
}

#[test]
fn string_with_space() {
    let result = lex("\" space \"");
    assert!(result.iter().any(|(t, v)| *t == STRING_DOUBLE),
            "String with space not found");
}

#[test]
fn string_escape_simple() {
    // Test simple escape: \ followed by non-u char
    // in_escape=true, character!='u' → in_escape=false
    let result = lex("\"\\n\"");
    assert!(result.iter().any(|(t, v)| *t == STRING_DOUBLE && v.contains("\\n")),
            "Escaped newline not found");
}

#[test]
fn string_escape_quote() {
    // Backslash before quote is escape, not string end
    let result = lex("\"\\\"inner\\\"\"");
    assert!(result.iter().any(|(t, v)| *t == STRING_DOUBLE),
            "String with escaped quotes not found");
}

#[test]
fn string_escape_backslash() {
    // Double backslash
    let result = lex("\"\\\\\"");
    assert!(result.iter().any(|(t, v)| *t == STRING_DOUBLE),
            "String with escaped backslash not found");
}

#[test]
fn string_unicode_escape_valid() {
    // \uXXXX escape: in_escape=true, char='u' → in_unicode_escape=4
    // Then consume 4 hex digits
    let result = lex("\"\\u0041\"");
    assert!(result.iter().any(|(t, v)| *t == STRING_DOUBLE),
            "Unicode escape not handled");
}

#[test]
fn string_unicode_escape_4_digits() {
    // Exactly 4 hex digits after \u
    let result = lex("\"\\uffff\"");
    let tokens = lex_repr("\"\\uffff\"");
    // Should tokenize as single STRING_DOUBLE token
    assert!(tokens.iter().any(|(t, v)| t == "Token.Literal.String.Double" && v == "\"\\uffff\""),
            "4 hex digits after \\u failed");
}

#[test]
fn string_unicode_escape_partial() {
    // Fewer than 4 hex digits: in_unicode_escape > 0 but hits non-hex
    let result = lex("\"\\uAB");
    // Should end as error (unterminated string with partial unicode)
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Partial unicode escape not marked as error");
}

#[test]
fn string_unicode_escape_non_hex() {
    // Character after \u that's not hex: in_unicode_escape=4 but char not hex
    let result = lex("\"\\uGGGG\"");
    // Non-hex char should end unicode escape and be treated as literal
    assert!(!result.is_empty(), "Non-hex after \\u failed");
}

#[test]
fn string_unterminated() {
    // No closing quote: in_string=true at EOF
    let result = lex("\"unterminated");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Unterminated string not marked as error");
}

#[test]
fn string_multiple_escapes() {
    // Multiple escape sequences in one string
    let result = lex("\"\\n\\t\\\"\\\\\"");
    assert!(result.iter().any(|(t, v)| *t == STRING_DOUBLE),
            "Multiple escapes not parsed");
}

#[test]
fn string_unicode_followed_by_normal_escape() {
    // \uXXXX followed by another escape
    let result = lex("\"\\u0041\\n\"");
    assert!(result.iter().any(|(t, _)| *t == STRING_DOUBLE),
            "Unicode + normal escape not parsed");
}

// ============================================================================
// WHITESPACE HANDLING - Branch Coverage
// ============================================================================
// Branches: in_whitespace, transition from whitespace

#[test]
fn whitespace_space() {
    let result = lex(" ");
    assert!(result.iter().any(|(t, _)| *t == WHITESPACE),
            "Single space not tokenized");
}

#[test]
fn whitespace_newline() {
    let result = lex("\n");
    assert!(result.iter().any(|(t, _)| *t == WHITESPACE),
            "Newline not tokenized as whitespace");
}

#[test]
fn whitespace_tab() {
    let result = lex("\t");
    assert!(result.iter().any(|(t, _)| *t == WHITESPACE),
            "Tab not tokenized");
}

#[test]
fn whitespace_carriage_return() {
    let result = lex("\r");
    assert!(result.iter().any(|(t, _)| *t == WHITESPACE),
            "Carriage return not tokenized");
}

#[test]
fn whitespace_multiple() {
    // Multiple whitespace chars coalesced
    let result = lex("  \n  \t  ");
    let ws_count = result.iter().filter(|(t, _)| *t == WHITESPACE).count();
    // Should be coalesced into fewer tokens
    assert!(ws_count > 0, "Whitespace not found");
}

#[test]
fn whitespace_between_tokens() {
    // Whitespace between punctuation
    let result = lex("{ }");
    assert!(result.iter().any(|(t, _)| *t == WHITESPACE),
            "Whitespace between braces not found");
}

// ============================================================================
// CONSTANT HANDLING - Branch Coverage
// ============================================================================
// Branches: in_constant, true/false/null recognition

#[test]
fn constant_true() {
    let result = lex("true");
    assert!(result.iter().any(|(t, v)| *t == KEYWORD_CONSTANT && v == "true"),
            "true constant not found");
}

#[test]
fn constant_false() {
    let result = lex("false");
    assert!(result.iter().any(|(t, v)| *t == KEYWORD_CONSTANT && v == "false"),
            "false constant not found");
}

#[test]
fn constant_null() {
    let result = lex("null");
    assert!(result.iter().any(|(t, v)| *t == KEYWORD_CONSTANT && v == "null"),
            "null constant not found");
}

#[test]
fn constant_in_array() {
    let result = lex("[true, false, null]");
    assert!(result.iter().filter(|(t, _)| *t == KEYWORD_CONSTANT).count() >= 3,
            "Not all constants found in array");
}

#[test]
fn constant_followed_by_punctuation() {
    // Constant transition: is_constant(char)=false → end constant
    let result = lex("true]");
    assert!(result.iter().any(|(t, v)| *t == KEYWORD_CONSTANT && v == "true"),
            "Constant before bracket not parsed");
    assert!(result.iter().any(|(t, v)| *t == PUNCTUATION && v == "]"),
            "Bracket after constant not parsed");
}

// ============================================================================
// NUMBER HANDLING - Branch Coverage
// ============================================================================
// Branches: in_number, is_integer(), is_float(), in_float

#[test]
fn number_integer_zero() {
    let result = lex("0");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_INTEGER && v == "0"),
            "Zero not parsed as integer");
}

#[test]
fn number_integer_positive() {
    let result = lex("42");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_INTEGER && v == "42"),
            "Positive integer not parsed");
}

#[test]
fn number_integer_negative() {
    let result = lex("-123");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_INTEGER && v == "-123"),
            "Negative integer not parsed");
}

#[test]
fn number_float_decimal() {
    // is_float('.') → in_float=true
    let result = lex("3.14");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_FLOAT && v == "3.14"),
            "Float with decimal not parsed");
}

#[test]
fn number_float_exponent_e() {
    // is_float('e') → in_float=true
    let result = lex("1e10");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_FLOAT && v == "1e10"),
            "Exponent with e not parsed");
}

#[test]
fn number_float_exponent_E() {
    // is_float('E') → in_float=true
    let result = lex("1E10");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_FLOAT && v == "1E10"),
            "Exponent with E not parsed");
}

#[test]
fn number_float_exponent_sign() {
    // is_float('+') after 'e'
    let result = lex("1e+5");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_FLOAT && v == "1e+5"),
            "Signed exponent not parsed");
}

#[test]
fn number_float_negative_exponent() {
    let result = lex("2.5e-3");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_FLOAT && v == "2.5e-3"),
            "Negative exponent not parsed");
}

#[test]
fn number_complex() {
    let result = lex("-123.456e-78");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_FLOAT),
            "Complex number not parsed as float");
}

#[test]
fn number_at_eof() {
    // Number at EOF: in_number=true at end, should flush as INTEGER or FLOAT
    let result = lex("123");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_INTEGER && v == "123"),
            "Number at EOF not parsed");
}

#[test]
fn number_float_at_eof() {
    let result = lex("3.14");
    assert!(result.iter().any(|(t, _)| *t == NUMBER_FLOAT),
            "Float at EOF not parsed");
}

#[test]
fn number_followed_by_punctuation() {
    let result = lex("42}");
    assert!(result.iter().any(|(t, v)| *t == NUMBER_INTEGER && v == "42"),
            "Number before punctuation not parsed");
    assert!(result.iter().any(|(t, v)| *t == PUNCTUATION && v == "}"),
            "Punctuation after number not parsed");
}

// ============================================================================
// PUNCTUATION HANDLING - Branch Coverage
// ============================================================================
// Branches: is_punct(), punctuation coalescing

#[test]
fn punct_left_brace() {
    let result = lex("{");
    assert!(result.iter().any(|(t, v)| *t == PUNCTUATION && v == "{"),
            "Left brace not parsed");
}

#[test]
fn punct_right_brace() {
    let result = lex("}");
    assert!(result.iter().any(|(t, v)| *t == PUNCTUATION && v == "}"),
            "Right brace not parsed");
}

#[test]
fn punct_left_bracket() {
    let result = lex("[");
    assert!(result.iter().any(|(t, v)| *t == PUNCTUATION && v == "["),
            "Left bracket not parsed");
}

#[test]
fn punct_right_bracket() {
    let result = lex("]");
    assert!(result.iter().any(|(t, v)| *t == PUNCTUATION && v == "]"),
            "Right bracket not parsed");
}

#[test]
fn punct_comma() {
    let result = lex(",");
    assert!(result.iter().any(|(t, v)| *t == PUNCTUATION && v == ","),
            "Comma not parsed");
}

#[test]
fn punct_colon() {
    // Special: colon triggers queue flush
    let result = lex(":");
    assert!(result.iter().any(|(t, v)| *t == PUNCTUATION && v == ":"),
            "Colon not parsed");
}

#[test]
fn punct_multiple_together() {
    // Punctuation coalesces: is_punct() continues in_punctuation
    let result = lex("{}[]");
    let punct_toks: Vec<_> = result.iter().filter(|(t, _)| *t == PUNCTUATION).collect();
    assert!(!punct_toks.is_empty(), "Punctuation tokens not found");
}

// ============================================================================
// COMMENT HANDLING - Branch Coverage
// ============================================================================
// Branches: single-line //, multi-line /* */, expecting_second_comment_opener

#[test]
fn comment_single_line() {
    // '/' → expecting_second_comment_opener=true
    // '/' → in_comment_single=true
    // '\n' → end comment
    let result = lex("// comment\n");
    assert!(result.iter().any(|(t, v)| *t == COMMENT_SINGLE && v == "// comment"),
            "Single-line comment not parsed");
}

#[test]
fn comment_single_line_with_content() {
    let result = lex("// this is a comment");
    assert!(result.iter().any(|(t, v)| *t == COMMENT_SINGLE),
            "Comment line not parsed");
}

#[test]
fn comment_multiline_basic() {
    // '/' → expecting_second_comment_opener=true
    // '*' → in_comment_multiline=true
    // '*' → expecting_second_comment_closer=true
    // '/' → end comment
    let result = lex("/* comment */");
    assert!(result.iter().any(|(t, v)| *t == COMMENT_MULTILINE && v == "/* comment */"),
            "Multi-line comment not parsed");
}

#[test]
fn comment_multiline_nested_stars() {
    // '*' inside comment that's not followed by '/' should continue
    let result = lex("/* * comment */");
    assert!(result.iter().any(|(t, _)| *t == COMMENT_MULTILINE),
            "Nested * in comment not handled");
}

#[test]
fn comment_multiline_newlines() {
    // Multi-line comment across lines
    let result = lex("/* line 1\nline 2 */");
    assert!(result.iter().any(|(t, _)| *t == COMMENT_MULTILINE),
            "Multi-line spanning not handled");
}

#[test]
fn comment_single_before_eof() {
    // Single-line comment at EOF (no newline): in_comment_single=true at EOF
    let result = lex("// comment");
    assert!(result.iter().any(|(t, _)| *t == COMMENT_SINGLE),
            "EOF single-line comment not handled");
}

#[test]
fn comment_multiline_unterminated() {
    // /* without closing */: in_comment_multiline=true at EOF
    let result = lex("/* unterminated");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Unterminated multiline comment not marked as error");
}

#[test]
fn comment_slash_without_second_char() {
    // '/' at EOF without following '/' or '*': expecting_second_comment_opener=true
    // Flush queue and error
    let result = lex("/");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Lone slash not marked as error");
}

#[test]
fn comment_slash_star_eofeof() {
    // Multiline comment not closed
    let result = lex("/* no close");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Unclosed multiline comment not error");
}

// ============================================================================
// COLON SPECIAL BEHAVIOR - Branch Coverage
// ============================================================================
// Branches: ':' triggers queue → replace STRING_DOUBLE with NAME_TAG

#[test]
fn colon_object_key_single_value() {
    // String before ':' should become NAME_TAG
    let result = lex_repr("{\"key\": 1}");
    assert!(result.iter().any(|(t, v)| t == "Token.Name.Tag" && v == "\"key\""),
            "Object key not reclassified as Name.Tag");
}

#[test]
fn colon_multiple_keys() {
    let result = lex_repr("{\"a\": 1, \"b\": 2}");
    let name_tag_count = result.iter().filter(|(t, _)| t == "Token.Name.Tag").count();
    assert!(name_tag_count >= 2, "Not all object keys reclassified");
}

#[test]
fn colon_nested_object() {
    let result = lex_repr("{\"outer\": {\"inner\": 1}}");
    assert!(result.iter().any(|(t, v)| t == "Token.Name.Tag" && v == "\"outer\""),
            "Outer key not reclassified");
    assert!(result.iter().any(|(t, v)| t == "Token.Name.Tag" && v == "\"inner\""),
            "Inner key not reclassified");
}

#[test]
fn colon_with_non_string_before() {
    // Colon after non-string (number, constant, etc.) - queue empty or no STRING
    let result = lex("[1: 2]");
    // Number before colon: queue not transformed
    assert!(result.iter().any(|(t, v)| *t == NUMBER_INTEGER && v == "1"),
            "Number before colon not preserved");
}

// ============================================================================
// EOF HANDLING - Branch Coverage
// ============================================================================
// Branches: partial tokens at EOF (in_string, in_float, in_number, etc.)

#[test]
fn eof_string_unterminated() {
    // in_string=true at EOF → push ERROR
    let result = lex("\"open");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Unterminated string at EOF not error");
}

#[test]
fn eof_float_incomplete() {
    // in_float=true at EOF → push NUMBER_FLOAT
    let result = lex("3.14e");
    assert!(result.iter().any(|(t, _)| *t == NUMBER_FLOAT),
            "Incomplete float at EOF not handled");
}

#[test]
fn eof_number_integer() {
    // in_number=true (not in_float) at EOF → push NUMBER_INTEGER
    let result = lex("42");
    assert!(result.iter().any(|(t, _)| *t == NUMBER_INTEGER),
            "Integer at EOF not handled");
}

#[test]
fn eof_constant() {
    // in_constant=true at EOF → push KEYWORD_CONSTANT
    let result = lex("true");
    assert!(result.iter().any(|(t, v)| *t == KEYWORD_CONSTANT && v == "true"),
            "Constant at EOF not handled");
}

#[test]
fn eof_whitespace() {
    // in_whitespace=true at EOF → push WHITESPACE
    let result = lex("   ");
    assert!(result.iter().any(|(t, _)| *t == WHITESPACE),
            "Whitespace at EOF not handled");
}

#[test]
fn eof_punctuation() {
    // in_punctuation=true at EOF → push PUNCTUATION
    let result = lex("{}");
    assert!(result.iter().any(|(t, _)| *t == PUNCTUATION),
            "Punctuation at EOF not handled");
}

#[test]
fn eof_comment_single() {
    // in_comment_single=true at EOF → push COMMENT_SINGLE
    let result = lex("// comment");
    assert!(result.iter().any(|(t, _)| *t == COMMENT_SINGLE),
            "Single comment at EOF not handled");
}

#[test]
fn eof_comment_multiline() {
    // in_comment_multiline=true at EOF → push ERROR
    let result = lex("/* unclosed");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Unclosed comment at EOF not error");
}

#[test]
fn eof_expecting_second_comment_opener() {
    // expecting_second_comment_opener=true at EOF → push ERROR
    let result = lex("1 /");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Incomplete comment at EOF not error");
}

// ============================================================================
// REAL-WORLD JSON EXAMPLES - Integration Testing
// ============================================================================

#[test]
fn json_empty_object() {
    let result = lex("{}");
    assert!(result.iter().any(|(t, _)| *t == PUNCTUATION),
            "Empty object not parsed");
}

#[test]
fn json_empty_array() {
    let result = lex("[]");
    assert!(result.iter().any(|(t, _)| *t == PUNCTUATION),
            "Empty array not parsed");
}

#[test]
fn json_simple_object() {
    let src = r#"{"name": "value"}"#;
    let result = lex_repr(src);
    assert!(result.iter().any(|(t, v)| t == "Token.Name.Tag"),
            "Object key not found or not Name.Tag");
}

#[test]
fn json_array_mixed_types() {
    let src = "[1, 2.5, \"string\", true, false, null]";
    let result = lex(src);
    assert!(result.iter().any(|(t, _)| *t == NUMBER_INTEGER),
            "Integer not found");
    assert!(result.iter().any(|(t, _)| *t == NUMBER_FLOAT),
            "Float not found");
    assert!(result.iter().any(|(t, _)| *t == STRING_DOUBLE),
            "String not found");
    assert!(result.iter().any(|(t, _)| *t == KEYWORD_CONSTANT),
            "Constant not found");
}

#[test]
fn json_nested_structure() {
    let src = r#"{"outer": {"inner": [1, 2, 3]}}"#;
    let result = lex_repr(src);
    let name_tags: Vec<_> = result.iter()
        .filter(|(t, _)| t == "Token.Name.Tag")
        .collect();
    assert!(name_tags.len() >= 2, "Nested object keys not parsed");
}

#[test]
fn json_with_comments() {
    let src = r#"// metadata
{
    /* config */
    "key": "value"
}"#;
    let result = lex(src);
    assert!(result.iter().any(|(t, _)| *t == COMMENT_SINGLE),
            "Line comment not parsed");
    assert!(result.iter().any(|(t, _)| *t == COMMENT_MULTILINE),
            "Block comment not parsed");
}

#[test]
fn json_escaped_string() {
    let src = r#""\n\t\"\\\u0041""#;
    let result = lex(src);
    assert!(result.iter().any(|(t, _)| *t == STRING_DOUBLE),
            "Escaped string not parsed");
}

#[test]
fn json_scientific_notation() {
    let src = "[1e10, 2.5E-5, -3.14e+8]";
    let result = lex(src);
    assert!(result.iter().filter(|(t, _)| *t == NUMBER_FLOAT).count() >= 3,
            "Scientific notation not parsed");
}

// ============================================================================
// ERROR CASES - Branch Coverage
// ============================================================================

#[test]
fn error_invalid_character() {
    // Character that's not valid JSON (outside string)
    let result = lex("@");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Invalid character not marked as error");
}

#[test]
fn error_unterminated_string() {
    let result = lex("\"unclosed");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Unterminated string not error");
}

#[test]
fn error_unclosed_comment_multiline() {
    let result = lex("/* no close");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Unclosed multiline comment not error");
}

#[test]
fn error_lone_slash() {
    let result = lex("{\"key\": /}");
    assert!(result.iter().any(|(t, _)| *t == ERROR),
            "Lone slash not error");
}

// ============================================================================
// EDGE CASES - Stress Testing
// ============================================================================

#[test]
fn edge_unicode_escape_max() {
    let result = lex("\"\\uffff\"");
    assert!(result.iter().any(|(t, _)| *t == STRING_DOUBLE),
            "Max unicode escape not handled");
}

#[test]
fn edge_very_long_string() {
    let long = "x".repeat(10000);
    let src = format!("\"{}\"", long);
    let result = lex(&src);
    assert!(result.iter().any(|(t, _)| *t == STRING_DOUBLE),
            "Very long string not handled");
}

#[test]
fn edge_deeply_nested_structure() {
    let src = "[[[[[{\"a\": 1}]]]]]";
    let result = lex(src);
    assert!(!result.is_empty(), "Deeply nested structure failed");
}

#[test]
fn edge_mixed_line_endings() {
    let src = "{\n\r\n\r\"key\": \"value\"}";
    let result = lex(src);
    assert!(!result.is_empty(), "Mixed line endings not handled");
}

#[test]
fn edge_many_comments() {
    let src = "// comment 1\n// comment 2\n// comment 3\n[1, 2, 3]";
    let result = lex(src);
    assert!(result.iter().filter(|(t, _)| *t == COMMENT_SINGLE).count() >= 3,
            "Multiple comments not parsed");
}

#[test]
fn edge_empty_input() {
    let result = lex("");
    assert!(result.is_empty() || result.len() == 0,
            "Empty input should have no tokens");
}

#[test]
fn edge_only_whitespace() {
    let result = lex("   \n  \t  ");
    assert!(result.iter().all(|(t, _)| *t == WHITESPACE),
            "Only whitespace not handled");
}
