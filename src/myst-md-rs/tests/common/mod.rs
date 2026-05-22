//! Helpers for the upstream `pytest-param-files` fixture formats used
//! across the MyST-Parser test suite.
//!
//! Two flavours are supported:
//!
//! 1. **Dot-delimited text** (`*.md`, `*.txt`) — each case looks like
//!    ```text
//!    Title line
//!    optional description line(s)
//!    .
//!    content
//!    .
//!    expected
//!    .
//!    ```
//!    Cases are separated by blank lines. See e.g.
//!    `MyST-Parser/tests/test_renderers/fixtures/dollarmath.md`.
//!
//! 2. **YAML mapping** (`*.yaml`) — top-level mapping `title -> {content,
//!    expected}`. See e.g.
//!    `MyST-Parser/tests/test_renderers/fixtures/option_parsing.yaml`.
//!
//! Both loaders return `Vec<ParamCase>` so test bodies can iterate
//! uniformly.

use serde::Deserialize;

/// One parsed fixture case.
#[derive(Debug, Clone)]
pub struct ParamCase {
    pub title: String,
    pub description: String,
    pub content: String,
    pub expected: String,
}

/// Parse upstream dot-delimited param-file text.
///
/// Lenient about trailing newlines and CRLF; case bodies are returned
/// without their trailing `\n` so callers can re-add one if needed.
pub fn parse_dot_format(input: &str) -> Vec<ParamCase> {
    let normalized = input.replace("\r\n", "\n");
    let mut cases = Vec::new();

    // Split into case blocks by blank lines, but only blank lines that
    // *separate* cases (i.e. between a closing `.` and the next title).
    // We do that by scanning line-by-line with a tiny state machine.
    let mut iter = normalized.lines().peekable();
    while iter.peek().is_some() {
        // Skip blank separator lines.
        while let Some(&line) = iter.peek() {
            if line.trim().is_empty() {
                iter.next();
            } else {
                break;
            }
        }
        if iter.peek().is_none() {
            break;
        }

        // Header: until first `.` line.
        let mut header = Vec::new();
        while let Some(&line) = iter.peek() {
            if line == "." {
                iter.next();
                break;
            }
            header.push(line);
            iter.next();
        }
        if header.is_empty() {
            continue;
        }

        // Content: until next `.` line.
        let mut content = Vec::new();
        while let Some(&line) = iter.peek() {
            if line == "." {
                iter.next();
                break;
            }
            content.push(line);
            iter.next();
        }

        // Expected: until next `.` line.
        let mut expected = Vec::new();
        while let Some(&line) = iter.peek() {
            if line == "." {
                iter.next();
                break;
            }
            expected.push(line);
            iter.next();
        }

        let title = header[0].trim().to_string();
        let description = if header.len() > 1 {
            header[1..].join("\n")
        } else {
            String::new()
        };

        cases.push(ParamCase {
            title,
            description,
            content: content.join("\n"),
            expected: expected.join("\n"),
        });
    }

    cases
}

#[derive(Debug, Deserialize)]
struct YamlCase {
    content: String,
    expected: String,
    #[serde(default)]
    description: String,
}

/// Parse upstream YAML param-file mapping.
pub fn parse_yaml_format(input: &str) -> Vec<ParamCase> {
    // Use IndexMap-like preservation via serde_yaml::Mapping to keep
    // file order, which matches upstream pytest ordering.
    let mapping: serde_yaml::Mapping =
        serde_yaml::from_str(input).expect("valid yaml fixture mapping");

    let mut cases = Vec::with_capacity(mapping.len());
    for (k, v) in mapping {
        let title = k.as_str().expect("string title key").to_string();
        let case: YamlCase = serde_yaml::from_value(v).expect("valid yaml case body");
        cases.push(ParamCase {
            title,
            description: case.description,
            content: case.content,
            expected: case.expected,
        });
    }
    cases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_two_dot_cases() {
        let raw = "first:\n.\nfoo\n.\nbar\n.\n\nsecond:\n.\nbaz\n.\nqux\n.\n";
        let cases = parse_dot_format(raw);
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].title, "first:");
        assert_eq!(cases[0].content, "foo");
        assert_eq!(cases[0].expected, "bar");
        assert_eq!(cases[1].title, "second:");
        assert_eq!(cases[1].content, "baz");
        assert_eq!(cases[1].expected, "qux");
    }

    #[test]
    fn dot_format_preserves_description_lines() {
        let raw = "title:\ndescription line one\ndescription line two\n.\ninput\n.\noutput\n.\n";
        let cases = parse_dot_format(raw);
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].title, "title:");
        assert_eq!(
            cases[0].description,
            "description line one\ndescription line two"
        );
    }

    #[test]
    fn parses_yaml_cases_in_order() {
        let raw = "alpha:\n  content: |-\n    a\n  expected: |-\n    A\nbeta:\n  content: |-\n    b\n  expected: |-\n    B\n";
        let cases = parse_yaml_format(raw);
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].title, "alpha");
        assert_eq!(cases[0].content, "a");
        assert_eq!(cases[0].expected, "A");
        assert_eq!(cases[1].title, "beta");
    }
}
