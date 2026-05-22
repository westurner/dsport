//! MyST directive option-list parser.
//!
//! Stub surface — matches the public signature of
//! `myst_parser.parsers.options.options_to_items` so test fixtures from
//! `tests/data/option_parsing*.yaml` can be wired in immediately. The
//! actual YAML-subset tokenizer is a port-in-progress; today this
//! implementation only handles a narrow happy path (plain block
//! mapping with optional continuation lines) and surfaces every other
//! input as `Err(OptionsError::Unimplemented)`. Failing fixtures are
//! tracked in `tests/parity.rs` and `docs/compat.md`.

use std::fmt;

/// One `(key, value)` entry from a directive option block.
pub type OptionItem = (String, String);

#[derive(Debug, Default, Clone, Copy)]
pub struct ParseState {
    pub has_comments: bool,
}

#[derive(Debug, Clone)]
pub enum OptionsError {
    /// Triggered when the input uses a YAML feature this stub does not
    /// yet implement (quoted scalars, block scalars, flow style, etc.).
    Unimplemented(String),
    /// Triggered on syntax errors the stub does recognise.
    Syntax {
        message: String,
        line: usize,
        column: usize,
    },
}

impl fmt::Display for OptionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsError::Unimplemented(s) => write!(f, "unimplemented: {s}"),
            OptionsError::Syntax {
                message,
                line,
                column,
            } => write!(f, "{message}\nat line {line}, column {column}"),
        }
    }
}

impl std::error::Error for OptionsError {}

/// Parse a directive option block. Returns the ordered list of
/// `(key, value)` pairs plus a `ParseState` recording whether any
/// `#` comments were stripped.
pub fn options_to_items(input: &str) -> Result<(Vec<OptionItem>, ParseState), OptionsError> {
    let mut state = ParseState::default();
    let mut items: Vec<OptionItem> = Vec::new();

    for (idx, raw) in input.lines().enumerate() {
        let line = raw;
        let trimmed = line.trim_start();
        // Comment / blank.
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('#') {
            state.has_comments = true;
            continue;
        }

        let leading = line.len() - trimmed.len();
        if leading > 0 {
            // Continuation line for the previous key.
            let Some(last) = items.last_mut() else {
                return Err(OptionsError::Syntax {
                    message: "expected key to start at column 0".to_string(),
                    line: idx,
                    column: leading,
                });
            };
            if last.1.is_empty() {
                last.1 = trimmed.trim().to_string();
            } else {
                last.1.push(' ');
                last.1.push_str(trimmed.trim());
            }
            continue;
        }

        // Refuse anything that looks like quoted / flow / block-scalar
        // YAML so we don't silently mis-parse it. Future work.
        if matches!(trimmed.as_bytes().first(), Some(b'"' | b'\'' | b'{' | b'[')) {
            return Err(OptionsError::Unimplemented(
                "quoted scalars / flow style".into(),
            ));
        }

        let Some(colon) = find_top_level_colon(trimmed) else {
            return Err(OptionsError::Syntax {
                message: "expected ':' after key".to_string(),
                line: idx,
                column: line.len(),
            });
        };
        let key = trimmed[..colon].trim().to_string();
        let rest = trimmed[colon + 1..].trim();
        if rest == "|" || rest == ">" || rest.starts_with("| ") || rest.starts_with("> ") {
            return Err(OptionsError::Unimplemented("block scalars".into()));
        }
        items.push((key, rest.to_string()));
    }

    Ok((items, state))
}

fn find_top_level_colon(s: &str) -> Option<usize> {
    // Only colons followed by space or end-of-line count as separators
    // (so `http://x` stays as a value).
    let bytes = s.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        if *b == b':' && (i + 1 == bytes.len() || bytes[i + 1] == b' ' || bytes[i + 1] == b'\t') {
            return Some(i);
        }
    }
    // Allow `key:` with no value at end of line.
    if bytes.last() == Some(&b':') {
        return Some(bytes.len() - 1);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_key_values() {
        let input = "key1:\nkey2: val2\nkey3:\n    val3\nkey4: val4.1\n    val4.2\n";
        let (items, state) = options_to_items(input).unwrap();
        assert_eq!(
            items,
            vec![
                ("key1".into(), "".into()),
                ("key2".into(), "val2".into()),
                ("key3".into(), "val3".into()),
                ("key4".into(), "val4.1 val4.2".into()),
            ]
        );
        assert!(!state.has_comments);
    }

    #[test]
    fn comments_set_flag() {
        let input = "# heading\nkey: value\n";
        let (items, state) = options_to_items(input).unwrap();
        assert_eq!(items, vec![("key".into(), "value".into())]);
        assert!(state.has_comments);
    }

    #[test]
    fn missing_colon_errors() {
        let err = options_to_items("key1\n").unwrap_err();
        assert!(matches!(err, OptionsError::Syntax { .. }));
    }
}
