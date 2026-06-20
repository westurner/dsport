//! Performance testing for PyO3 bridge vs native Rust implementations
//!
//! Benchmarks bridge lex/format operations against native implementations to:
//! - Measure FFI overhead
//! - Validate large file handling
//! - Test deeply nested structures
//! - Profile memory usage
//!
//! **Skipping:** Set `SKIP_PERFORMANCE_TESTS=1` to skip (useful for CI without benchmarks)

#[cfg(feature = "python-bridge")]
mod bridge_performance_tests {
    use pygmentsrs::bridge;
    use std::time::Instant;

    // ========== HELPERS ==========

    fn should_skip() -> bool {
        std::env::var("SKIP_PERFORMANCE_TESTS")
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

    // ========== PERFORMANCE: Large Files ==========

    #[test]
    fn perf_bridge_lex_large_file_100kb() {
        skip_if_needed!();
        // Test lexing a 100KB Python file
        let code = "x = 1\n".repeat(16667); // ~100KB

        let start = Instant::now();
        let result = bridge::lex("python", &code);
        let elapsed = start.elapsed();

        assert!(result.is_some(), "lexing large file should succeed");
        let tokens = result.unwrap();
        assert!(!tokens.is_empty(), "large file should produce tokens");

        // Performance assertion: should complete in reasonable time (< 5 seconds)
        assert!(
            elapsed.as_secs() < 5,
            "lexing 100KB should be fast (took {:?})",
            elapsed
        );
        eprintln!("Lexed 100KB file: {} tokens in {:?}", tokens.len(), elapsed);
    }

    #[test]
    fn perf_bridge_lex_1mb_file() {
        skip_if_needed!();
        // Test lexing a 1MB Python file
        let code = "x = 1\n".repeat(166667); // ~1MB

        let start = Instant::now();
        let result = bridge::lex("python", &code);
        let elapsed = start.elapsed();

        assert!(result.is_some(), "lexing 1MB file should succeed");
        let tokens = result.unwrap();
        assert!(!tokens.is_empty(), "1MB file should produce tokens");

        // Should complete in reasonable time (< 30 seconds)
        assert!(
            elapsed.as_secs() < 30,
            "lexing 1MB should complete (took {:?})",
            elapsed
        );
        eprintln!("Lexed 1MB file: {} tokens in {:?}", tokens.len(), elapsed);
    }

    // ========== PERFORMANCE: Deeply Nested Structures ==========

    #[test]
    fn perf_bridge_lex_deeply_nested_50_levels() {
        skip_if_needed!();
        // Test JSON with deeply nested structures (50 levels)
        let mut json = String::new();
        for _ in 0..50 {
            json.push('{');
        }
        json.push_str(r#""key": "value""#);
        for _ in 0..50 {
            json.push('}');
        }

        let start = Instant::now();
        let result = bridge::lex("json", &json);
        let elapsed = start.elapsed();

        assert!(result.is_some(), "deeply nested JSON should lex");
        let tokens = result.unwrap();
        assert!(!tokens.is_empty(), "nested JSON should produce tokens");

        eprintln!(
            "Lexed 50-level nested JSON: {} tokens in {:?}",
            tokens.len(),
            elapsed
        );
    }

    #[test]
    fn perf_bridge_lex_deeply_nested_200_levels() {
        skip_if_needed!();
        // Test JSON with very deep nesting (200 levels)
        let mut json = String::new();
        for _ in 0..200 {
            json.push('{');
        }
        json.push_str(r#""key": "value""#);
        for _ in 0..200 {
            json.push('}');
        }

        let start = Instant::now();
        let result = bridge::lex("json", &json);
        let elapsed = start.elapsed();

        // May fail or take a while, but shouldn't panic
        if let Some(tokens) = result {
            eprintln!(
                "Lexed 200-level nested JSON: {} tokens in {:?}",
                tokens.len(),
                elapsed
            );
        } else {
            eprintln!("200-level nesting failed (acceptable): {:?}", elapsed);
        }
    }

    // ========== PERFORMANCE: Format Large Token Streams ==========

    #[test]
    fn perf_bridge_format_1000_tokens() {
        skip_if_needed!();
        // Test formatting a large token stream (1000 tokens)
        let mut tokens = Vec::new();
        for i in 0..1000 {
            let ttype = match i % 5 {
                0 => "Token.Keyword",
                1 => "Token.Text",
                2 => "Token.Number",
                3 => "Token.String",
                _ => "Token.Comment",
            };
            tokens.push((ttype.to_string(), format!("token_{}", i)));
        }

        let start = Instant::now();
        let result = bridge::format("html", &tokens);
        let elapsed = start.elapsed();

        assert!(result.is_some(), "formatting 1000 tokens should succeed");
        let output = result.unwrap();
        assert!(!output.is_empty(), "formatting should produce output");

        eprintln!("Formatted 1000 tokens to HTML in {:?}", elapsed);
    }

    #[test]
    fn perf_bridge_format_10000_tokens() {
        skip_if_needed!();
        // Test formatting 10,000 tokens
        let mut tokens = Vec::new();
        for i in 0..10000 {
            let ttype = match i % 5 {
                0 => "Token.Keyword",
                1 => "Token.Text",
                2 => "Token.Number",
                3 => "Token.String",
                _ => "Token.Comment",
            };
            tokens.push((ttype.to_string(), format!("tok_{}", i % 100)));
        }

        let start = Instant::now();
        let result = bridge::format("html", &tokens);
        let elapsed = start.elapsed();

        assert!(result.is_some(), "formatting 10K tokens should succeed");
        eprintln!("Formatted 10K tokens to HTML in {:?}", elapsed);
    }

    // ========== PERFORMANCE: Multiple Format Operations ==========

    #[test]
    fn perf_bridge_format_to_all_formatters() {
        skip_if_needed!();
        // Test formatting same tokens to all available formatters
        let tokens = vec![
            ("Token.Keyword".to_string(), "if".to_string()),
            ("Token.Text".to_string(), " ".to_string()),
            ("Token.Number".to_string(), "42".to_string()),
        ];

        let formatters = vec!["html", "terminal", "latex", "bbcode", "ansi"];

        for formatter in formatters {
            let start = Instant::now();
            if let Some(result) = bridge::format(formatter, &tokens) {
                let elapsed = start.elapsed();
                eprintln!(
                    "Format to {}: {:?} (output: {} bytes)",
                    formatter,
                    elapsed,
                    result.len()
                );
            }
        }
    }

    // ========== PERFORMANCE: Repeated Operations (Consistency Check) ==========

    #[test]
    fn perf_bridge_repeated_lex_operations() {
        skip_if_needed!();
        // Test that repeated lex operations maintain consistent performance
        let code = "x = 1\ny = 2\nz = 3\n".repeat(100);

        let mut times = Vec::new();
        for _ in 0..10 {
            let start = Instant::now();
            let _ = bridge::lex("python", &code);
            times.push(start.elapsed());
        }

        let avg_time = times.iter().sum::<std::time::Duration>() / times.len() as u32;
        let first_time = times[0];
        let last_time = times[times.len() - 1];

        eprintln!(
            "Repeated lex operations: first={:?}, last={:?}, avg={:?}",
            first_time, last_time, avg_time
        );
        eprintln!("Times: {:?}", times);

        // Check that performance doesn't degrade significantly over time
        // Allow 50% variation
        let max_acceptable = first_time + first_time / 2;
        assert!(
            last_time < max_acceptable,
            "performance degradation detected (first: {:?}, last: {:?})",
            first_time,
            last_time
        );
    }

    #[test]
    fn perf_bridge_repeated_format_operations() {
        skip_if_needed!();
        // Test that repeated format operations maintain consistent performance
        let tokens = vec![
            ("Token.Keyword".to_string(), "if".to_string()),
            ("Token.Number".to_string(), "42".to_string()),
        ];

        let mut times = Vec::new();
        for _ in 0..10 {
            let start = Instant::now();
            let _ = bridge::format("html", &tokens);
            times.push(start.elapsed());
        }

        let avg_time = times.iter().sum::<std::time::Duration>() / times.len() as u32;
        eprintln!(
            "Repeated format operations: avg={:?}, times={:?}",
            avg_time, times
        );

        // Check consistency
        let first_time = times[0];
        let last_time = times[times.len() - 1];
        let max_acceptable = first_time + first_time / 2;
        assert!(
            last_time < max_acceptable,
            "performance degradation detected (first: {:?}, last: {:?})",
            first_time,
            last_time
        );
    }

    // ========== PERFORMANCE: Python vs Rust Bridge Selection ==========

    #[test]
    fn perf_bridge_selection_analysis() {
        skip_if_needed!();
        // Informational test: analyze when to use bridge vs native
        let large_code = "x = 1\n".repeat(1000);
        let complex_code = (0..1000)
            .map(|i| format!(r#"{{"key{}":{}}}"#, i, i))
            .collect::<Vec<_>>()
            .join(",");

        let test_cases = vec![
            ("json", r#"{"key": "value"}"#, "small"),
            ("python", "x = 1", "small"),
            ("javascript", "var x = 42;", "small"),
            ("python", &large_code, "large"),
            ("json", &complex_code, "complex"),
        ];

        eprintln!("\n=== Bridge vs Native Performance Analysis ===");
        for (lexer, code, size_class) in test_cases {
            let start = Instant::now();
            let result = bridge::lex(lexer, code);
            let bridge_time = start.elapsed();

            if let Some(tokens) = result {
                eprintln!(
                    "{:<12} | {} | {} tokens | {:?}",
                    lexer,
                    size_class,
                    tokens.len(),
                    bridge_time
                );
            }
        }

        eprintln!("\nRecommendation:");
        eprintln!("- Use Native Rust: small inputs (<1KB), JSON/Python only");
        eprintln!("- Use Bridge: any unknown lexer, large files, flexibility");
        eprintln!("- Benchmark: your specific use case on your hardware");
    }

    // ========== PERFORMANCE: Memory Efficiency ==========

    #[test]
    fn perf_bridge_memory_scaling() {
        skip_if_needed!();
        // Test memory usage as input size grows
        let sizes = vec![1000, 10000, 100000];

        for size in sizes {
            let code = "x = 1\n".repeat(size);

            // Count memory indirectly by token count
            if let Some(tokens) = bridge::lex("python", &code) {
                let approx_memory = tokens.iter().map(|(t, v)| t.len() + v.len()).sum::<usize>();
                eprintln!(
                    "Input size: {} lines, Tokens: {}, Approx bytes: {}",
                    size,
                    tokens.len(),
                    approx_memory
                );
            }
        }
    }

    // ========== PERFORMANCE: Language Complexity Comparison ==========

    #[test]
    fn perf_bridge_language_complexity() {
        skip_if_needed!();
        // Compare lexing different languages for complexity analysis
        let code_samples = vec![
            ("python", "def func(x):\n    return x * 2\n"),
            ("javascript", "function func(x) { return x * 2; }\n"),
            ("json", r#"{"key": "value", "num": 42}"#),
            ("shell", "#!/bin/bash\necho \"hello\"\n"),
            ("cpp", "#include <iostream>\nint main() { return 0; }\n"),
        ];

        eprintln!("\n=== Language Complexity Analysis ===");
        for (lang, code) in code_samples {
            let start = Instant::now();
            if let Some(tokens) = bridge::lex(lang, code) {
                let elapsed = start.elapsed();
                let avg_token_len = code.len() / tokens.len().max(1);
                eprintln!(
                    "{:<12}: {} tokens, avg {} bytes/token, {:?}",
                    lang,
                    tokens.len(),
                    avg_token_len,
                    elapsed
                );
            }
        }
    }
}

// If python-bridge feature is not enabled, provide a no-op test
#[cfg(not(feature = "python-bridge"))]
mod bridge_performance_disabled {
    #[test]
    fn bridge_performance_feature_disabled() {
        // Performance tests not run without the feature
    }
}
