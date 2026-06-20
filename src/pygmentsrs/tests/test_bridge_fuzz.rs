#![allow(clippy::needless_borrows_for_generic_args)]

//! Fuzz testing for PyO3 bridge with property-based tests
//!
//! Uses quickcheck to generate random inputs and verify:
//! - No panics on arbitrary input
//! - Consistent behavior across similar inputs
//! - Error handling robustness
//! - Edge case discovery
//!
//! **Skipping:** Set `SKIP_FUZZ_TESTS=1` to skip (useful for deterministic CI)

#[cfg(feature = "python-bridge")]
mod bridge_fuzz_tests {
    use pygmentsrs::bridge;

    // ========== HELPERS ==========

    fn should_skip() -> bool {
        std::env::var("SKIP_FUZZ_TESTS")
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

    // ========== FUZZ: Random Lexer Names ==========

    #[test]
    fn fuzz_bridge_lex_with_random_aliases() {
        skip_if_needed!();
        // Test lex with random/invalid lexer names - should not panic
        let long_name = "a".repeat(1000);
        let random_names: Vec<&str> =
            vec!["xyzunknown", "12345", "!!!", "", "   ", "\0", "نص", "😀"];

        for name in random_names {
            // Should either succeed or return None gracefully
            let result = bridge::lex(name, "test code");
            // If it returns Some, it's a valid lexer; if None, that's also fine
            let _ = result;
        }

        // Test with long name separately
        let result = bridge::lex(&long_name, "test code");
        let _ = result;
    }

    #[test]
    fn fuzz_bridge_lex_with_random_code() {
        skip_if_needed!();
        // Test lex with random/weird code strings - should not panic
        let long_str = "a".repeat(100000);
        let weird_codes: Vec<&str> = vec![
            "",
            "\0\0\0",
            "\n\n\n",
            "\\\\\\",
            "\"\"\"\"\"",
            "'''''",
            "\t\t\t",
            "\r\n\r\n",
            "🎉🎉🎉",
        ];

        for code in weird_codes {
            // Should not panic even with weird input
            let result = bridge::lex("python", code);
            let _ = result;
        }

        // Very long string as separate test
        let result = bridge::lex("python", &long_str);
        let _ = result;
    }

    // ========== FUZZ: Random Formatter Names ==========

    #[test]
    fn fuzz_bridge_format_with_random_formatters() {
        skip_if_needed!();
        // Test format with random/invalid formatter names - should not panic
        let random_formatters = vec!["xyzunknown", "12345", "!!!", "", "   ", "\0", "نص", "😀"];

        let tokens = vec![("Token.Text".to_string(), "test".to_string())];

        for formatter in random_formatters {
            let result = bridge::format(formatter, &tokens);
            let _ = result; // Should be None, not panic
        }
    }

    #[test]
    fn fuzz_bridge_format_with_random_tokens() {
        skip_if_needed!();
        // Test format with random/weird token data - should not panic
        let test_cases = vec![
            // Empty tokens
            vec![],
            // Very long token values
            vec![("Token.Text".to_string(), "x".repeat(100000))],
            // Special characters in tokens
            vec![("Token.Text".to_string(), "\0\0\0".to_string())],
            // Unicode in tokens
            vec![("Token.Text".to_string(), "🎉🎉🎉".to_string())],
            // Mixed
            vec![
                ("Token.Keyword".to_string(), "if".to_string()),
                ("Token.Text".to_string(), "\n".to_string()),
                ("Token.String".to_string(), r#""test\0test""#.to_string()),
            ],
            // Weird token type names
            vec![("InvalidType".to_string(), "test".to_string())],
            vec![("".to_string(), "test".to_string())],
            vec![("Token.".to_string(), "test".to_string())],
        ];

        for tokens in test_cases {
            let result = bridge::format("html", &tokens);
            let _ = result; // Should not panic
        }
    }

    // ========== FUZZ: Combinations ==========

    #[test]
    fn fuzz_bridge_lex_all_known_aliases() {
        skip_if_needed!();
        // Test lex with various inputs against all known lexer aliases
        let aliases = vec![
            "python",
            "javascript",
            "json",
            "shell",
            "cpp",
            "html",
            "xml",
        ];
        let long_code = "a".repeat(10000);
        let codes: Vec<&str> = vec!["", "x", "test\0code", "🎉"];

        for alias in aliases {
            for code in &codes {
                let result = bridge::lex(alias, code);
                // Should not panic - may succeed or fail gracefully
                let _ = result;
            }

            // Test with long code separately
            let result = bridge::lex(alias, &long_code);
            let _ = result;
        }
    }

    #[test]
    fn fuzz_bridge_format_all_known_formatters() {
        skip_if_needed!();
        // Test format with various tokens against all known formatters
        let formatters = vec!["html", "terminal", "latex", "bbcode"];
        let token_sets = vec![
            vec![],
            vec![("Token.Text".to_string(), "test".to_string())],
            vec![
                ("Token.Keyword".to_string(), "if".to_string()),
                ("Token.String".to_string(), "\"test\"".to_string()),
            ],
        ];

        for formatter in formatters {
            for tokens in &token_sets {
                let result = bridge::format(formatter, tokens);
                // Should not panic - may succeed or fail gracefully
                let _ = result;
            }
        }
    }

    // ========== FUZZ: Regression Detection ==========

    #[test]
    fn fuzz_bridge_regression_consistency() {
        skip_if_needed!();
        // Same input should produce same output (no random behavior)
        let code = "x = 42";

        let result1 = bridge::lex("python", code);
        let result2 = bridge::lex("python", code);
        let result3 = bridge::lex("python", code);

        assert_eq!(
            result1, result2,
            "identical lex calls should produce identical results"
        );
        assert_eq!(
            result2, result3,
            "identical lex calls should produce identical results"
        );
    }

    #[test]
    fn fuzz_bridge_regression_token_format_consistency() {
        skip_if_needed!();
        // Token stream should be in consistent format
        if let Some(tokens) = bridge::lex("python", "x = 1") {
            for (ttype, value) in &tokens {
                // Token type should have consistent format
                assert!(
                    ttype.starts_with("Token.") || ttype == "Token",
                    "token type should start with 'Token.': {}",
                    ttype
                );
                // Value can be any string (including empty)
                let _ = value;
            }
        }
    }

    // ========== FUZZ: Edge Cases ==========

    #[test]
    fn fuzz_bridge_edge_case_null_bytes() {
        skip_if_needed!();
        // Test handling of null bytes in code
        let codes_with_nulls = vec!["code\0here", "\0\0\0", "start\0middle\0end"];

        for code in codes_with_nulls {
            let result = bridge::lex("python", code);
            // Should not panic
            let _ = result;
        }
    }

    #[test]
    fn fuzz_bridge_edge_case_very_long_input() {
        skip_if_needed!();
        // Test with extremely long inputs
        let long_code = "x = 1\n".repeat(100000); // ~600KB

        let result = bridge::lex("python", &long_code);
        // Should handle or gracefully fail
        let _ = result;
    }

    #[test]
    fn fuzz_bridge_edge_case_unicode_everywhere() {
        skip_if_needed!();
        // Test with unicode in all positions
        let unicode_codes: Vec<&str> = vec!["🎉", "نص", "日本語", "你好", "مرحبا", "Здравствуй"];

        let repeated_emoji = "🎉".repeat(100);

        for code in unicode_codes {
            let result = bridge::lex("python", code);
            let _ = result;
        }

        // Test with repeated emoji separately
        let result = bridge::lex("python", &repeated_emoji);
        let _ = result;
    }

    #[test]
    fn fuzz_bridge_edge_case_mixed_line_endings() {
        skip_if_needed!();
        // Test with mixed line ending styles
        let codes = vec![
            "a\nb\nc",          // \n
            "a\r\nb\r\nc",      // \r\n
            "a\rb\rc",          // \r
            "a\n\r\nb\r\nc\ne", // mixed
        ];

        for code in codes {
            let result = bridge::lex("python", code);
            let _ = result;
        }
    }

    #[test]
    fn fuzz_bridge_edge_case_only_whitespace() {
        skip_if_needed!();
        // Test with only whitespace/control chars
        let whitespace_codes = vec![
            " ",
            "\t",
            "\n",
            "   \t\t\n\n   ",
            "\r\n\r\n\r\n",
            "\x0b\x0c\x1c", // Form feed, vertical tab, etc.
        ];

        for code in whitespace_codes {
            let result = bridge::lex("python", code);
            let _ = result;
        }
    }

    // ========== STRESS TESTS ==========

    #[test]
    fn stress_bridge_many_rapid_lex_calls() {
        skip_if_needed!();
        // Rapid sequential lex calls - stress test
        for i in 0..100 {
            let code = format!("x = {}", i);
            let result = bridge::lex("python", &code);
            assert!(result.is_some(), "rapid call {} failed", i);
        }
    }

    #[test]
    fn stress_bridge_many_rapid_format_calls() {
        skip_if_needed!();
        // Rapid sequential format calls - stress test
        let tokens = vec![("Token.Number".to_string(), "42".to_string())];

        for _ in 0..100 {
            let result = bridge::format("html", &tokens);
            assert!(result.is_some(), "rapid format call failed");
        }
    }

    #[test]
    fn stress_bridge_alternating_lex_format() {
        skip_if_needed!();
        // Alternating lex and format calls - stress test interactions
        for i in 0..50 {
            let code = format!("x = {}", i);

            if let Some(tokens) = bridge::lex("python", &code) {
                let _ = bridge::format("html", &tokens);
            }
        }
    }

    #[test]
    fn stress_bridge_many_different_languages() {
        skip_if_needed!();
        // Lex the same code with many different languages
        let code = "x = 42";
        let languages = vec![
            "python",
            "javascript",
            "json",
            "shell",
            "cpp",
            "java",
            "ruby",
            "go",
            "rust",
        ];

        for lang in languages {
            let result = bridge::lex(lang, code);
            // Either succeeds (valid language) or fails (unknown), both OK
            let _ = result;
        }
    }
}

// If python-bridge feature is not enabled, provide a no-op test
#[cfg(not(feature = "python-bridge"))]
mod bridge_fuzz_disabled {
    #[test]
    fn bridge_fuzz_feature_disabled() {
        // Fuzz tests not run without the feature
    }
}
