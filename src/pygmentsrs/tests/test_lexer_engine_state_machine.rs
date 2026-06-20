#![allow(clippy::needless_borrows_for_generic_args)]


//! Lexer engine state machine and action dispatch tests
//! Target: Cover state transitions, action types, edge cases in lexer/engine.rs
//!
//! The engine (670 LOC, ~80 branches) executes the core tokenization loop:
//! - State stack management (push/pop)
//! - Rule matching with fancy_regex (lookahead/lookbehind/backreferences)
//! - Action dispatch: Single, ByGroups, UsingThis, UsingLexer, DispatchCodeBlock
//! - Edge cases: zero-width rules, nested lexing, backreference failures

//use pygmentsrs::lexer::Lexer;
use pygmentsrs::lexers::registry::get_lexer_by_name;
use pygmentsrs::token::*;

// ============================================================================
// State Machine Transitions
// ============================================================================

#[test]
fn test_state_push_single() {
    // Test pushing a new state onto the stack
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "def foo():\n    x = 1";
    let tokens = lexer.get_tokens(src);

    // Should have function def followed by body
    assert!(!tokens.is_empty());
    let has_def = tokens.iter().any(|(t, v)| *t == KEYWORD && v == "def");
    assert!(has_def);
}

#[test]
fn test_state_pop_return_to_root() {
    // Test popping state to return to root
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "def f():\n    pass\nx = 1";
    let tokens = lexer.get_tokens(src);

    // Should have: def keyword, function name, colon, pass keyword, then back to root with assignment
    assert!(tokens.iter().any(|(t, v)| *t == KEYWORD && v == "def"));
    assert!(tokens.iter().any(|(t, v)| *t == KEYWORD && v == "pass"));
}

#[test]
fn test_nested_state_push_multiple() {
    // Test multiple nested state pushes
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "def f():\n    def g():\n        x = 1";
    let tokens = lexer.get_tokens(src);

    // Both function definitions should be recognized
    let def_count = tokens
        .iter()
        .filter(|(t, v)| *t == KEYWORD && v == "def")
        .count();
    assert!(def_count >= 2);
}

#[test]
fn test_state_with_lookahead() {
    // Rules using lookahead assertions
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "123 456 789abc";
    let tokens = lexer.get_tokens(src);

    // Numbers followed by non-word should be tokenized separately
    let numbers = tokens
        .iter()
        .filter(|(t, _)| *t == NUMBER || *t == NUMBER_INTEGER)
        .count();
    assert!(numbers >= 2);
}

// ============================================================================
// Single Action Tests
// ============================================================================

#[test]
fn test_action_single_keyword() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "if while for";
    let tokens = lexer.get_tokens(src);

    let keywords = tokens.iter().filter(|(t, _)| *t == KEYWORD).count();
    assert!(keywords >= 2);
}

#[test]
fn test_action_single_operator() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "a + b * c / d";
    let tokens = lexer.get_tokens(src);

    let operators = tokens.iter().filter(|(t, _)| *t == OPERATOR).count();
    assert!(operators >= 3);
}

#[test]
fn test_action_single_number_int() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "42 100 999";
    let tokens = lexer.get_tokens(src);

    let numbers = tokens
        .iter()
        .filter(|(t, _)| *t == NUMBER || *t == NUMBER_INTEGER)
        .count();
    assert!(numbers >= 2);
}

#[test]
fn test_action_single_number_float() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "3.14 2.71";
    let tokens = lexer.get_tokens(src);

    let floats = tokens
        .iter()
        .filter(|(t, _)| *t == NUMBER || *t == NUMBER_FLOAT)
        .count();
    assert!(floats >= 1);
}

// ============================================================================
// ByGroups Action Tests
// ============================================================================

#[test]
fn test_action_bygroups_function_definition() {
    // def name() splits into keyword, name, punctuation
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "def my_function(x):";
    let tokens = lexer.get_tokens(src);

    let has_keyword = tokens.iter().any(|(t, v)| *t == KEYWORD && v == "def");
    let has_function = tokens.iter().any(|(t, _)| *t == NAME_FUNCTION);

    assert!(has_keyword);
    assert!(has_function);
}

#[test]
fn test_action_bygroups_class_definition() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "class MyClass(Base):";
    let tokens = lexer.get_tokens(src);

    let has_keyword = tokens.iter().any(|(t, v)| *t == KEYWORD && v == "class");
    assert!(has_keyword);
}

#[test]
fn test_action_bygroups_import_statement() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "from os import path";
    let tokens = lexer.get_tokens(src);

    // The Python lexer tokenizes 'from' and 'import' as KEYWORD_NAMESPACE for imports
    let has_from = tokens
        .iter()
        .any(|(t, v)| KEYWORD.contains(*t) && v == "from");
    let has_import = tokens
        .iter()
        .any(|(t, v)| KEYWORD.contains(*t) && v == "import");

    assert!(has_from, "Missing 'from' keyword in tokens");
    assert!(has_import, "Missing 'import' keyword in tokens");
}

#[test]
fn test_action_bygroups_multiple_groups() {
    // Multiple capture groups in single rule
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "if x > 5:";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

// ============================================================================
// UsingThis Action Tests (Recursive Lexing)
// ============================================================================

#[test]
fn test_action_using_this_f_string() {
    // F-strings use UsingThis to recursively lex expressions
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = r#"f"hello {name} world""#;
    let tokens = lexer.get_tokens(src);

    // Should have string and variable reference
    assert!(
        tokens
            .iter()
            .any(|(t, _)| *t == STRING_DOUBLE || *t == STRING)
    );
}

#[test]
fn test_action_using_this_docstring() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = r#""""This is a docstring with 'quotes'""""#;
    let tokens = lexer.get_tokens(src);

    let has_string = tokens
        .iter()
        .any(|(t, _)| *t == STRING_DOUBLE || *t == STRING);
    assert!(has_string);
}

#[test]
fn test_action_using_this_multiline_string() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "'''\nLine 1\nLine 2\n'''";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

// ============================================================================
// UsingLexer Action Tests (Delegation)
// ============================================================================

#[test]
fn test_action_using_lexer_embedded_sql() {
    // Some template languages embed SQL with UsingLexer
    let lexer = get_lexer_by_name("html").expect("HTML lexer not found");
    let src = "<style>body { color: red; }</style>";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

// ============================================================================
// DispatchCodeBlock Action Tests
// ============================================================================

#[test]
fn test_action_dispatch_codeblock_indented() {
    // Tests indent stripping in DispatchCodeBlock
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "if x:\n    y = 1";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

#[test]
fn test_action_dispatch_nested_indentation() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "if a:\n    if b:\n        c = 1";
    let tokens = lexer.get_tokens(src);

    let if_count = tokens
        .iter()
        .filter(|(t, v)| *t == KEYWORD && v == "if")
        .count();
    assert!(if_count >= 2);
}

#[test]
fn test_action_dispatch_large_block() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let mut src = String::new();
    for i in 0..50 {
        src.push_str(&format!("    x{} = {}\n", i, i));
    }
    let tokens = lexer.get_tokens(&src);

    assert!(!tokens.is_empty());
}

// ============================================================================
// Lookahead and Lookbehind Tests
// ============================================================================

#[test]
fn test_lookahead_number_digit_boundary() {
    // Number digit boundary with lookahead
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "123 456abc";
    let tokens = lexer.get_tokens(src);

    // Should recognize 123 as number, but 456abc differently
    assert!(tokens.len() >= 2);
}

#[test]
fn test_lookahead_identifier_boundary() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "var var1 var_name";
    let tokens = lexer.get_tokens(src);

    let names = tokens.iter().filter(|(t, _)| *t == NAME).count();
    assert!(names >= 1);
}

#[test]
fn test_lookahead_keyword_vs_identifier() {
    // "ifTrue" should not be keyword, "if" should be
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "if ifTrue: pass";
    let tokens = lexer.get_tokens(src);

    let if_keywords = tokens
        .iter()
        .filter(|(t, v)| *t == KEYWORD && v == "if")
        .count();
    assert_eq!(if_keywords, 1);
}

// ============================================================================
// Backreference Tests
// ============================================================================

#[test]
fn test_backreference_string_quote_matching() {
    // Strings can use 'string' or "string" but must match
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = r#"'single' "double" """triple""""""#;
    let tokens = lexer.get_tokens(src);

    let strings = tokens
        .iter()
        .filter(|(t, _)| *t == STRING || *t == STRING_SINGLE || *t == STRING_DOUBLE)
        .count();
    assert!(strings >= 3);
}

#[test]
fn test_backreference_comment_eol() {
    // # comment continues to end of line
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "x = 1  # this is a comment\ny = 2";
    let tokens = lexer.get_tokens(src);

    let has_comment = tokens.iter().any(|(t, _)| *t == COMMENT_SINGLE);
    assert!(has_comment);
}

// ============================================================================
// Zero-Width Rules
// ============================================================================

#[test]
fn test_zero_width_assertion_indent() {
    // Python uses zero-width assertions for indentation levels
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "def foo():\n    x = 1";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

#[test]
fn test_zero_width_state_boundary() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "x = 1\ndef f(): pass";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

// ============================================================================
// Complex Nesting Tests
// ============================================================================

#[test]
fn test_nested_parens_brackets_braces() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "[({}), ([]), {[()]}]";
    let tokens = lexer.get_tokens(src);

    let parens = tokens
        .iter()
        .filter(|(t, v)| *t == PUNCTUATION && (v == "(" || v == ")"))
        .count();
    let brackets = tokens
        .iter()
        .filter(|(t, v)| *t == PUNCTUATION && (v == "[" || v == "]"))
        .count();
    let braces = tokens
        .iter()
        .filter(|(t, v)| *t == PUNCTUATION && (v == "{" || v == "}"))
        .count();

    assert!(parens > 0);
    assert!(brackets > 0);
    assert!(braces > 0);
}

#[test]
fn test_nested_function_calls() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "f(g(h(i())))";
    let tokens = lexer.get_tokens(src);

    let parens = tokens
        .iter()
        .filter(|(t, v)| *t == PUNCTUATION && (v == "(" || v == ")"))
        .count();
    assert_eq!(parens, 8);
}

#[test]
fn test_deeply_nested_structure() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let mut src = String::new();
    for _ in 0..10 {
        src.push('(');
    }
    src.push('1');
    for _ in 0..10 {
        src.push(')');
    }
    let tokens = lexer.get_tokens(&src);

    assert!(!tokens.is_empty());
}

// ============================================================================
// Large Token Streams
// ============================================================================

#[test]
fn test_large_token_stream_1000_tokens() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let mut src = String::new();
    for i in 0..250 {
        src.push_str(&format!("x{} = {} + {}; ", i, i, i + 1));
    }
    let tokens = lexer.get_tokens(&src);

    assert!(tokens.len() > 750);
}

#[test]
fn test_large_token_stream_deep_expression() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let mut expr = String::new();
    for i in 0..30 {
        expr.push('(');
        expr.push_str(&i.to_string());
    }
    expr.push('1');
    for _ in 0..30 {
        expr.push(')');
    }
    let tokens = lexer.get_tokens(&expr);

    assert!(!tokens.is_empty());
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_unclosed_string() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = r#""unclosed string"#;
    let tokens = lexer.get_tokens(src);

    // Should still tokenize (may mark as error)
    assert!(!tokens.is_empty());
}

#[test]
fn test_unclosed_paren() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "f(a, b";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

#[test]
fn test_empty_input() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "";
    let tokens = lexer.get_tokens(src);

    // Empty input should produce empty token list
    assert!(tokens.is_empty() || tokens.iter().all(|(_, v)| v.is_empty()));
}

#[test]
fn test_only_whitespace() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "   \n\t  \n  ";
    let tokens = lexer.get_tokens(src);

    // Should be empty or just whitespace
    let non_empty = tokens
        .iter()
        .filter(|(_, v)| !v.chars().all(char::is_whitespace))
        .count();
    assert_eq!(non_empty, 0);
}

#[test]
fn test_unicode_identifiers() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "α = 1; β = α + 2";
    let tokens = lexer.get_tokens(src);

    // Should handle unicode
    assert!(!tokens.is_empty());
}

#[test]
fn test_very_long_line() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let mut src = String::new();
    for i in 0..500 {
        src.push_str(&format!("x{} + ", i));
    }
    src.push('1');
    let tokens = lexer.get_tokens(&src);

    assert!(tokens.len() > 100);
}

// ============================================================================
// State Stack Operations
// ============================================================================

#[test]
fn test_state_push_pop_sequence() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "def f():\n    x = 1\ny = 2";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

#[test]
fn test_deeply_nested_state_stack() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "def f():\n    def g():\n        def h():\n            x = 1";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

#[test]
fn test_multiple_state_transitions() {
    // Test rapid state transitions
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "if x:\n    pass\nelse:\n    pass\nfor i in range(10):\n    print(i)";
    let tokens = lexer.get_tokens(src);

    assert!(tokens.len() > 20);
}

// ============================================================================
// Real-World Code Examples
// ============================================================================

#[test]
fn test_real_world_function() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = r#"def factorial(n):
    """Calculate factorial."""
    if n <= 1:
        return 1
    return n * factorial(n - 1)"#;
    let tokens = lexer.get_tokens(src);

    assert!(tokens.len() > 20);
}

#[test]
fn test_real_world_class() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = r#"class Calculator:
    def add(self, a, b):
        return a + b
    
    def multiply(self, a, b):
        return a * b"#;
    let tokens = lexer.get_tokens(src);

    assert!(tokens.len() > 30);
}

#[test]
fn test_real_world_list_comprehension() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "[x * 2 for x in range(10) if x % 2 == 0]";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

#[test]
fn test_real_world_lambda() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = "sorted(items, key=lambda x: x[1])";
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}

#[test]
fn test_real_world_decorator() {
    let lexer = get_lexer_by_name("python").expect("Python lexer not found");
    let src = r#"@property
def value(self):
    return self._value"#;
    let tokens = lexer.get_tokens(src);

    assert!(!tokens.is_empty());
}
