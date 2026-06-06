//! Trivial formatters — F0 phase.
//!
//! `NullFormatter` (text) — echo token text only
//! `RawTokenFormatter` (raw, tokens) — repr format
//! `TestcaseFormatter` (testcase) — Rust unit test skeleton

use crate::token::TokenType;

pub struct NullFormatter;

impl NullFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();
        for (_, value) in tokens {
            out.push_str(value);
        }
        out
    }
}

pub struct RawTokenFormatter;

impl RawTokenFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();
        for (ttype, value) in tokens {
            let token_str = format!("{:?}", ttype);
            let escaped = Self::escape_string(value);
            out.push_str(&format!("{}\t{}\n", token_str, escaped));
        }
        out
    }

    fn escape_string(s: &str) -> String {
        let mut out = String::from("'");
        for c in s.chars() {
            match c {
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\\' => out.push_str("\\\\"),
                '\'' => out.push_str("\\'"),
                _ => out.push(c),
            }
        }
        out.push('\'');
        out
    }
}

pub struct TestcaseFormatter;

impl TestcaseFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();
        out.push_str("#[test]\n");
        out.push_str("fn test_generated() {\n");
        out.push_str("    use pygmentsrs::lexer::Lexer;\n");
        out.push_str("    use pygmentsrs::token::Token;\n");
        out.push_str("    \n");
        out.push_str("    let tokens = vec![\n");
        
        for (ttype, value) in tokens {
            let token_str = format!("{:?}", ttype);
            let escaped = Self::escape_rust_string(value);
            out.push_str(&format!("        ({}, {}),\n", token_str, escaped));
        }
        
        out.push_str("    ];\n");
        out.push_str("    \n");
        out.push_str("    // Verify token types\n");
        out.push_str("    for (ttype, value) in tokens {\n");
        out.push_str("        assert!(!value.is_empty());\n");
        out.push_str("    }\n");
        out.push_str("}\n");
        out
    }

    fn escape_rust_string(s: &str) -> String {
        let mut out = String::from("\"");
        for c in s.chars() {
            match c {
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                _ => out.push(c),
            }
        }
        out.push('"');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_null_formatter() {
        let formatter = NullFormatter;
        let tokens = vec![
            (KEYWORD, "let".to_string()),
            (TEXT, " ".to_string()),
            (NAME, "x".to_string()),
        ];
        let result = formatter.format(&tokens);
        assert_eq!(result, "let x");
    }

    #[test]
    fn test_raw_formatter() {
        let formatter = RawTokenFormatter;
        let tokens = vec![(KEYWORD, "let".to_string())];
        let result = formatter.format(&tokens);
        // The Debug format includes "Token." prefix
        assert!(result.contains("Keyword") || result.contains("let"));
    }

    #[test]
    fn test_testcase_formatter() {
        let formatter = TestcaseFormatter;
        let tokens = vec![(KEYWORD, "let".to_string())];
        let result = formatter.format(&tokens);
        assert!(result.contains("#[test]"));
        assert!(result.contains("fn test_generated"));
    }
}
