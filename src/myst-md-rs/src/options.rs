//! MyST directive option-list parser.
//!
//! Implements the YAML-subset tokenizer used by
//! `myst_parser.parsers.options.options_to_items`, including quoted and
//! multiline scalars, block/folded values, comments, and ordered output.

use std::fmt;

/// One `(key, value)` entry from a directive option block.
pub type OptionItem = (String, String);

#[derive(Debug, Default, Clone, Copy)]
pub struct ParseState {
    pub has_comments: bool,
}

#[derive(Debug, Clone)]
pub enum OptionsError {
    /// Triggered when a structured value cannot be represented as a scalar
    /// option pair.
    Unimplemented(String),
    /// Triggered on syntax errors recognized by the option tokenizer.
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
    let state = ParseState {
        has_comments: contains_comment(input),
    };
    validate_top_level(input)?;
    let normalized = normalize_multiline_quotes(input);
    let mapping: serde_yaml::Mapping = serde_yaml::from_str(&normalized).map_err(|error| {
        let (line, column) = error
            .location()
            .map(|location| {
                (
                    location.line().saturating_sub(1),
                    location.column().saturating_sub(1),
                )
            })
            .unwrap_or((0, 0));
        OptionsError::Syntax {
            message: error
                .to_string()
                .lines()
                .next()
                .unwrap_or("invalid YAML")
                .to_string(),
            line,
            column,
        }
    })?;

    let mut items = Vec::with_capacity(mapping.len());
    for (key, value) in mapping {
        let Some(key) = yaml_scalar_to_string(&key) else {
            return Err(OptionsError::Unimplemented("non-scalar option key".into()));
        };
        let value = yaml_value_to_string(&value);
        items.push((key, value));
    }
    Ok((items, state))
}

fn normalize_multiline_quotes(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut quote: Option<u8> = None;
    let mut escaped = false;
    for character in input.chars() {
        if character == '\n' && quote.is_some() {
            output.push(' ');
            escaped = false;
            continue;
        }
        output.push(character);
        if quote == Some(b'"') && escaped {
            escaped = false;
            continue;
        }
        match (quote, character) {
            (None, '\'' | '"') => quote = Some(character as u8),
            (Some(b'"'), '\\') => escaped = true,
            (Some(b'"'), '"') => quote = None,
            (Some(b'\''), '\'') => quote = None,
            _ => {}
        }
    }
    output
}

fn validate_top_level(input: &str) -> Result<(), OptionsError> {
    let mut saw_key = false;
    let mut block_scalar_indent: Option<usize> = None;
    for (line, raw) in input.lines().enumerate() {
        let trimmed = raw.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let indent = raw.len() - trimmed.len();
        if let Some(required) = block_scalar_indent {
            if indent >= required {
                continue;
            }
            block_scalar_indent = None;
        }
        if indent > 0 && !saw_key && block_scalar_indent.is_none() {
            return Err(OptionsError::Syntax {
                message: "expected key to start at column 0".into(),
                line,
                column: indent,
            });
        }
        if indent > 0 {
            continue;
        }
        saw_key = true;
        let value = trimmed
            .split_once(':')
            .map(|(_, value)| value.trim())
            .unwrap_or_default();
        if value.starts_with('|') || value.starts_with('>') {
            block_scalar_indent = Some(indent + 1);
        }
    }
    Ok(())
}

fn contains_comment(input: &str) -> bool {
    input.lines().any(|line| {
        let mut quote = None;
        for (index, byte) in line.as_bytes().iter().enumerate() {
            match (quote, *byte) {
                (None, b'\'' | b'"') => quote = Some(*byte),
                (Some(q), byte) if byte == q => quote = None,
                (None, b'#') if index == 0 || line.as_bytes()[index - 1].is_ascii_whitespace() => {
                    return true;
                }
                _ => {}
            }
        }
        false
    })
}

fn yaml_scalar_to_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::Null => Some(String::new()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        serde_yaml::Value::String(value) => Some(value.clone()),
        serde_yaml::Value::Sequence(_) | serde_yaml::Value::Mapping(_) => None,
        serde_yaml::Value::Tagged(value) => yaml_scalar_to_string(&value.value),
    }
}

fn yaml_value_to_string(value: &serde_yaml::Value) -> String {
    if let Some(value) = yaml_scalar_to_string(value) {
        return value;
    }
    match value {
        serde_yaml::Value::Sequence(values) => format!(
            "[{}]",
            values
                .iter()
                .map(yaml_value_to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        serde_yaml::Value::Mapping(values) => format!(
            "{{{}}}",
            values
                .iter()
                .map(|(key, value)| format!(
                    "{}: {}",
                    yaml_value_to_string(key),
                    yaml_value_to_string(value)
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        serde_yaml::Value::Tagged(value) => yaml_value_to_string(&value.value),
        _ => String::new(),
    }
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
