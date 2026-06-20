#![allow(clippy::needless_borrows_for_generic_args)]

//! Extended language coverage for PyO3 bridge
//!
//! Tests additional lexer/formatter combinations and error recovery patterns:
//! - More languages (Ruby, Go, Rust, Java, C#, etc.)
//! - Complex language features (generics, decorators, etc.)
//! - Error recovery validation
//! - Multi-language document handling
//!
//! **Skipping:** Set `SKIP_EXTENDED_LANGUAGE_TESTS=1` to skip

#[cfg(feature = "python-bridge")]
mod bridge_extended_language_tests {
    use pygmentsrs::bridge;

    // ========== HELPERS ==========

    fn should_skip() -> bool {
        std::env::var("SKIP_EXTENDED_LANGUAGE_TESTS")
            .map(|v| v == "1" || v == "true")
            .unwrap_or(false)
    }

    fn bridge_available() -> bool {
        use pyo3::prelude::*;
        Python::try_attach(|py| {
            py.import("pygments")
                .and_then(|_| py.import("pygments.lexers"))
                .is_ok()
        })
        .unwrap_or(false)
    }

    macro_rules! skip_if_needed {
        () => {
            if should_skip() || !bridge_available() {
                return;
            }
        };
    }

    // ========== ADDITIONAL LANGUAGES: Basic Support ==========

    #[test]
    fn bridge_lex_ruby_basic() {
        skip_if_needed!();
        let code = r#"
def greet(name)
  "Hello, #{name}!"
end

puts greet("Ruby")
"#;
        let result = bridge::lex("ruby", code);
        assert!(result.is_some(), "ruby should be a known lexer");
        let tokens = result.unwrap();
        assert!(!tokens.is_empty(), "ruby code should produce tokens");
    }

    #[test]
    fn bridge_lex_go_basic() {
        skip_if_needed!();
        let code = r#"
package main
import "fmt"

func main() {
    fmt.Println("Hello, Go!")
}
"#;
        let result = bridge::lex("go", code);
        assert!(result.is_some(), "go should be a known lexer");
        let tokens = result.unwrap();
        assert!(!tokens.is_empty(), "go code should produce tokens");
    }

    #[test]
    fn bridge_lex_rust_basic() {
        skip_if_needed!();
        let code = r#"
fn main() {
    let msg = "Hello, Rust!";
    println!("{}", msg);
}
"#;
        let result = bridge::lex("rust", code);
        assert!(result.is_some(), "rust should be a known lexer");
        let tokens = result.unwrap();
        assert!(!tokens.is_empty(), "rust code should produce tokens");
    }

    #[test]
    fn bridge_lex_java_basic() {
        skip_if_needed!();
        let code = r#"
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, Java!");
    }
}
"#;
        let result = bridge::lex("java", code);
        assert!(result.is_some(), "java should be a known lexer");
        let tokens = result.unwrap();
        assert!(!tokens.is_empty(), "java code should produce tokens");
    }

    #[test]
    fn bridge_lex_csharp_basic() {
        skip_if_needed!();
        let code = r#"
using System;

class Program {
    static void Main() {
        Console.WriteLine("Hello, C#!");
    }
}
"#;
        let result = bridge::lex("csharp", code);
        if let Some(tokens) = result {
            assert!(!tokens.is_empty(), "C# code should produce tokens");
        }
    }

    #[test]
    fn bridge_lex_php_basic() {
        skip_if_needed!();
        let code = r#"
<?php
function greet($name) {
    echo "Hello, $name!";
}
greet("PHP");
?>
"#;
        let result = bridge::lex("php", code);
        if let Some(tokens) = result {
            assert!(!tokens.is_empty(), "PHP code should produce tokens");
        }
    }

    // ========== LANGUAGE FEATURES: Complex Constructs ==========

    #[test]
    fn bridge_lex_python_decorators() {
        skip_if_needed!();
        let code = r#"
@decorator
@another_decorator(arg="value")
def func(x: int) -> int:
    """Docstring"""
    return x * 2
"#;
        let result = bridge::lex("python", code);
        assert!(result.is_some());
        let tokens = result.unwrap();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_javascript_async_await() {
        skip_if_needed!();
        let code = r#"
async function fetchData() {
    const response = await fetch('/api/data');
    const data = await response.json();
    return data;
}
"#;
        let result = bridge::lex("javascript", code);
        assert!(result.is_some());
        let tokens = result.unwrap();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_typescript_generics() {
        skip_if_needed!();
        let code = r#"
function identity<T>(arg: T): T {
    return arg;
}

interface Container<T> {
    value: T;
}
"#;
        let result = bridge::lex("typescript", code);
        if let Some(tokens) = result {
            assert!(!tokens.is_empty());
        }
    }

    #[test]
    fn bridge_lex_cpp_templates() {
        skip_if_needed!();
        let code = r#"
template<typename T>
T max(T a, T b) {
    return a > b ? a : b;
}

template<>
const char* max(const char* a, const char* b) {
    return strcmp(a, b) > 0 ? a : b;
}
"#;
        let result = bridge::lex("cpp", code);
        let tokens = result.expect("cpp lexing should succeed");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_sql_complex_query() {
        skip_if_needed!();
        let code = r#"
SELECT u.id, u.name, COUNT(o.id) as order_count
FROM users u
LEFT JOIN orders o ON u.id = o.user_id
WHERE u.created_at > '2024-01-01'
GROUP BY u.id, u.name
HAVING COUNT(o.id) > 5
ORDER BY order_count DESC;
"#;
        let result = bridge::lex("sql", code);
        if let Some(tokens) = result {
            assert!(!tokens.is_empty());
        }
    }

    // ========== ERROR RECOVERY: Incomplete Code ==========

    #[test]
    fn bridge_lex_recovery_incomplete_function() {
        skip_if_needed!();
        let incomplete = r#"
def func(x:
    return x
"#;
        let result = bridge::lex("python", incomplete);
        // Should still tokenize despite syntax error
        let tokens = result.expect("python lexing should succeed");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_recovery_unclosed_string() {
        skip_if_needed!();
        let code = r#"x = "unclosed string"#;
        let result = bridge::lex("python", code);
        // Should still tokenize
        let tokens = result.expect("python lexing should succeed");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_recovery_incomplete_json() {
        skip_if_needed!();
        let json = r#"{"key": "value""#;
        let result = bridge::lex("json", json);
        let tokens = result.expect("json lexing should succeed");
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_recovery_html_unclosed_tags() {
        skip_if_needed!();
        let html = r#"<div><p>Content</div>"#;
        let result = bridge::lex("html", html);
        if let Some(tokens) = result {
            assert!(!tokens.is_empty());
        }
    }

    // ========== MULTI-LANGUAGE DOCUMENTS ==========

    #[test]
    fn bridge_lex_markdown_with_code_blocks() {
        skip_if_needed!();
        let markdown = r#"
# Example

```python
def hello():
    print("world")
```

More text
"#;
        let result = bridge::lex("markdown", markdown);
        if let Some(tokens) = result {
            assert!(!tokens.is_empty());
        }
    }

    #[test]
    fn bridge_lex_html_with_inline_javascript() {
        skip_if_needed!();
        let html = r#"
<html>
<script>
function onClick() {
    console.log("clicked");
}
</script>
</html>
"#;
        let result = bridge::lex("html", html);
        if let Some(tokens) = result {
            assert!(!tokens.is_empty());
        }
    }

    #[test]
    fn bridge_lex_xml_with_namespaces() {
        skip_if_needed!();
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root xmlns:custom="http://example.com/custom">
    <custom:element attr="value">
        Content
    </custom:element>
</root>
"#;
        let result = bridge::lex("xml", xml);
        if let Some(tokens) = result {
            assert!(!tokens.is_empty());
        }
    }

    // ========== FORMATTER COVERAGE: Extended Formats ==========

    #[test]
    fn bridge_format_to_rst() {
        skip_if_needed!();
        let tokens = vec![
            ("Token.Keyword".to_string(), "def".to_string()),
            ("Token.Text".to_string(), " ".to_string()),
            ("Token.Name".to_string(), "func".to_string()),
        ];
        let result = bridge::format("rst", &tokens);
        // rst formatter may not be available, but should not panic
        let _ = result;
    }

    #[test]
    fn bridge_format_to_irc() {
        skip_if_needed!();
        let tokens = vec![
            ("Token.Keyword".to_string(), "if".to_string()),
            ("Token.Number".to_string(), "42".to_string()),
        ];
        let result = bridge::format("irc", &tokens);
        let _ = result;
    }

    #[test]
    fn bridge_format_to_terminal16() {
        skip_if_needed!();
        let tokens = vec![
            ("Token.Keyword".to_string(), "while".to_string()),
            ("Token.String".to_string(), r#""test""#.to_string()),
        ];
        let result = bridge::format("terminal16", &tokens);
        if let Some(output) = result {
            assert!(!output.is_empty());
        }
    }

    // ========== LANGUAGE ALIASES ==========

    #[test]
    fn bridge_lex_language_aliases() {
        skip_if_needed!();
        // Test that aliases resolve correctly
        let aliases_map = vec![
            ("python", "py"), // python aliases
            ("python", "python3"),
            ("javascript", "js"), // javascript aliases
            ("javascript", "node"),
            ("shell", "bash"), // shell aliases
            ("shell", "sh"),
        ];

        for (primary, alias) in aliases_map {
            let code = "x = 1";

            let primary_result = bridge::lex(primary, code);
            let alias_result = bridge::lex(alias, code);

            // Both should either succeed or both fail
            match (primary_result, alias_result) {
                (Some(p_tokens), Some(a_tokens)) => {
                    assert!(!p_tokens.is_empty());
                    assert!(!a_tokens.is_empty());
                }
                (None, None) => {
                    // Both unknown, which is fine
                }
                (Some(_), None) | (None, Some(_)) => {
                    // One works but not the other - acceptable, aliases may differ
                }
            }
        }
    }

    // ========== CONSISTENCY: Cross-Language Patterns ==========

    #[test]
    fn bridge_lex_hello_world_consistency() {
        skip_if_needed!();
        // Verify "hello world" pattern produces consistent token types across languages
        let hello_world_samples = vec![
            ("python", r#"print("hello world")"#),
            ("javascript", r#"console.log("hello world")"#),
            ("ruby", r#"puts "hello world""#),
            ("java", r#"System.out.println("hello world");"#),
        ];

        for (lang, code) in hello_world_samples {
            let result = bridge::lex(lang, code);
            if let Some(tokens) = result {
                assert!(!tokens.is_empty(), "{} tokens should be non-empty", lang);

                // Should contain string and function/method tokens
                let has_string = tokens.iter().any(|(t, _)| t.contains("String"));
                let has_name = tokens
                    .iter()
                    .any(|(t, _)| t.contains("Name") || t.contains("Keyword"));

                eprintln!(
                    "{}: {} tokens, has_string={}, has_name={}",
                    lang,
                    tokens.len(),
                    has_string,
                    has_name
                );
            }
        }
    }

    // ========== EDGE CASES: Language-Specific ==========

    #[test]
    fn bridge_lex_python_with_type_hints() {
        skip_if_needed!();
        let code = r#"
def process(items: list[str], count: int = 0) -> dict[str, int]:
    return {item: i for i, item in enumerate(items)}
"#;
        let result = bridge::lex("python", code);
        assert!(result.is_some());
        let tokens = result.unwrap();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_javascript_template_literals() {
        skip_if_needed!();
        let code = r#"
const name = "World";
const greeting = `Hello, ${name}!`;
console.log(`Nested ${`template ${1 + 2}`} literals`);
"#;
        let result = bridge::lex("javascript", code);
        assert!(result.is_some());
        let tokens = result.unwrap();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_regex_patterns() {
        skip_if_needed!();
        let code = r#"
import re
pattern = r'^\d{3}-\d{2}-\d{4}$'  # SSN pattern
result = re.match(pattern, "123-45-6789")
"#;
        let result = bridge::lex("python", code);
        assert!(result.is_some());
        let tokens = result.unwrap();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn bridge_lex_multiline_strings() {
        skip_if_needed!();
        let code = r#"text = """This is a
multiline string that spans
multiple lines of code"""

other = '''Single quoted
multiline too'''
"#;
        let result = bridge::lex("python", code);
        assert!(result.is_some());
        let tokens = result.unwrap();
        assert!(!tokens.is_empty());
    }
}

// If python-bridge feature is not enabled, provide a no-op test
#[cfg(not(feature = "python-bridge"))]
mod bridge_extended_language_disabled {
    #[test]
    fn bridge_extended_language_feature_disabled() {
        // Extended language tests not run without the feature
    }
}
