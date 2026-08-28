//! MyST directive-text parser.
//!
//! Implements the core surface of
//! `myst_parser.parsers.directives.parse_directive_text`: arguments,
//! colon/YAML option blocks, typed option coercion, body offsets, and
//! non-fatal validation warnings. The complete built-in directive registry
//! remains a later compatibility layer.

use std::collections::BTreeMap;

use crate::options::{OptionsError, options_to_items};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionKind {
    String,
    Flag,
    Integer,
    Float,
    Bool,
    Class,
    Classes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue {
    String(String),
    Flag,
    Integer(i64),
    Float(f64),
    Bool(bool),
    Classes(Vec<String>),
}

#[derive(Debug, Clone, Default)]
pub struct DirectiveSpec {
    pub required_arguments: usize,
    pub optional_arguments: usize,
    pub has_content: bool,
    /// Final argument may contain whitespace.
    pub final_argument_whitespace: bool,
    pub option_spec: BTreeMap<String, OptionKind>,
}

#[derive(Debug, Clone, Default)]
pub struct ParsedDirective {
    pub arguments: Vec<String>,
    pub options: Vec<(String, String)>,
    pub coerced_options: BTreeMap<String, OptionValue>,
    pub body: Vec<String>,
    /// 0-based offset (in `body` lines) where the body starts relative
    /// to the directive opening line. Matches upstream's
    /// `content_offset`.
    pub content_offset: usize,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum DirectiveError {
    Options(OptionsError),
    Syntax(String),
}

impl From<OptionsError> for DirectiveError {
    fn from(e: OptionsError) -> Self {
        DirectiveError::Options(e)
    }
}

/// Parse the body of a fenced MyST directive. `first_line` is the
/// portion after the opening ` ```{name} ` on the fence line; `body`
/// is each subsequent line, in order, until the closing fence.
pub fn parse_directive_text(
    spec: &DirectiveSpec,
    first_line: &str,
    body: &[&str],
) -> Result<ParsedDirective, DirectiveError> {
    let mut out = ParsedDirective::default();

    let first_line = first_line.trim();
    if spec.required_arguments + spec.optional_arguments == 0 {
        if !first_line.is_empty() {
            out.body.push(first_line.to_string());
        }
    } else if !first_line.is_empty() {
        let mut arguments = if spec.final_argument_whitespace {
            vec![first_line.to_string()]
        } else {
            first_line
                .split_whitespace()
                .map(ToString::to_string)
                .collect()
        };
        if arguments.len() < spec.required_arguments {
            return Err(DirectiveError::Syntax(format!(
                "{} argument(s) required, {} supplied",
                spec.required_arguments,
                arguments.len()
            )));
        }
        if arguments.len() > spec.required_arguments + spec.optional_arguments {
            if spec.final_argument_whitespace {
                arguments.truncate(spec.required_arguments + spec.optional_arguments);
            } else {
                return Err(DirectiveError::Syntax(format!(
                    "maximum {} argument(s) allowed, {} supplied",
                    spec.required_arguments + spec.optional_arguments,
                    arguments.len()
                )));
            }
        }
        out.arguments = arguments;
    } else if spec.required_arguments > 0 {
        return Err(DirectiveError::Syntax(format!(
            "{} argument(s) required, 0 supplied",
            spec.required_arguments
        )));
    }

    // Split body into option-block + content. The option block is the
    // contiguous run of leading `:key: value` lines (and blank/comment
    // lines between them).
    let mut idx = 0usize;
    let mut option_lines: Vec<&str> = Vec::new();
    if body.first().is_some_and(|line| line.trim() == "---") {
        idx = 1;
        while idx < body.len() && body[idx].trim() != "---" {
            option_lines.push(body[idx]);
            idx += 1;
        }
        if idx < body.len() {
            idx += 1;
        }
    } else {
        while idx < body.len() {
            let line = body[idx];
            let trimmed = line.trim_start();
            if trimmed.starts_with(':') && trimmed[1..].contains(':') {
                // looks like `:key: value`
                option_lines.push(strip_leading_colon(line));
                idx += 1;
            } else if trimmed.is_empty() && !option_lines.is_empty() {
                // blank line ends the option block.
                idx += 1;
                break;
            } else if trimmed.starts_with('#') && !option_lines.is_empty() {
                option_lines.push(strip_leading_colon(line));
                idx += 1;
            } else {
                break;
            }
        }
    }

    if !option_lines.is_empty() {
        let joined = option_lines.join("\n");
        let (items, _state) = options_to_items(&joined)?;
        for (key, value) in items {
            if let Some(kind) = spec.option_spec.get(&key) {
                match coerce_option(kind, &value) {
                    Ok(coerced) => {
                        out.coerced_options.insert(key.clone(), coerced);
                    }
                    Err(error) => out.warnings.push(format!(
                        "Invalid option value for {key:?}: {value}: {error}"
                    )),
                }
            } else if !spec.option_spec.is_empty() {
                out.warnings.push(format!("Unknown option key: {key}"));
            }
            out.options.push((key, value));
        }
    }

    if spec.required_arguments + spec.optional_arguments == 0 && !first_line.is_empty() {
        out.content_offset = 0;
    } else {
        out.content_offset = idx;
    }
    while idx < body.len() {
        out.body.push(body[idx].to_string());
        idx += 1;
    }

    if out.body.first().is_some_and(|line| line.trim().is_empty()) {
        out.body.remove(0);
        out.content_offset += 1;
    }

    if !spec.has_content && !out.body.is_empty() {
        out.warnings
            .push("directive takes no content; ignoring body".into());
    }

    Ok(out)
}

fn coerce_option(kind: &OptionKind, value: &str) -> Result<OptionValue, String> {
    match kind {
        OptionKind::String => Ok(OptionValue::String(value.to_string())),
        OptionKind::Flag => Ok(OptionValue::Flag),
        OptionKind::Integer => value
            .parse::<i64>()
            .map(OptionValue::Integer)
            .map_err(|error| error.to_string()),
        OptionKind::Float => value
            .parse::<f64>()
            .map(OptionValue::Float)
            .map_err(|error| error.to_string()),
        OptionKind::Bool => match value.to_ascii_lowercase().as_str() {
            "true" | "yes" | "on" | "1" => Ok(OptionValue::Bool(true)),
            "false" | "no" | "off" | "0" => Ok(OptionValue::Bool(false)),
            _ => Err("expected a boolean".into()),
        },
        OptionKind::Class => {
            if value.split_whitespace().all(valid_class_name) {
                Ok(OptionValue::Classes(
                    value.split_whitespace().map(str::to_string).collect(),
                ))
            } else {
                Err("cannot make value into a class name".into())
            }
        }
        OptionKind::Classes => {
            let classes: Vec<_> = value.split_whitespace().map(str::to_string).collect();
            if classes.iter().all(|class| valid_class_name(class)) {
                Ok(OptionValue::Classes(classes))
            } else {
                Err("cannot make value into a class name".into())
            }
        }
    }
}

fn valid_class_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
}

/// Strip a single leading `:` from each option line so the run can
/// be fed into the option parser as a YAML block mapping.
fn strip_leading_colon(line: &str) -> &str {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed.strip_prefix(':') {
        rest
    } else {
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note() -> DirectiveSpec {
        DirectiveSpec {
            required_arguments: 0,
            optional_arguments: 1,
            has_content: true,
            final_argument_whitespace: true,
            option_spec: BTreeMap::new(),
        }
    }

    #[test]
    fn note_argument_only() {
        let r = parse_directive_text(&note(), "a", &[]).unwrap();
        assert_eq!(r.arguments, vec!["a".to_string()]);
        assert!(r.body.is_empty());
        assert!(r.options.is_empty());
    }

    #[test]
    fn note_body_only() {
        let r = parse_directive_text(&note(), "", &["a"]).unwrap();
        assert_eq!(r.body, vec!["a".to_string()]);
        assert_eq!(r.content_offset, 0);
    }

    #[test]
    fn note_options_then_body() {
        let r = parse_directive_text(&note(), "", &[":class: name", "a"]).unwrap();
        assert_eq!(r.options, vec![("class".into(), "name".into())]);
        assert_eq!(r.body, vec!["a".to_string()]);
        assert_eq!(r.content_offset, 1);
    }

    #[test]
    fn coerces_yaml_options_and_reports_invalid_values() {
        let mut option_spec = BTreeMap::new();
        option_spec.insert("class".into(), OptionKind::Class);
        option_spec.insert("flag".into(), OptionKind::Bool);
        option_spec.insert("count".into(), OptionKind::Integer);
        option_spec.insert("unknown_allowed".into(), OptionKind::String);
        let spec = DirectiveSpec {
            has_content: true,
            option_spec,
            ..note()
        };
        let parsed = parse_directive_text(
            &spec,
            "",
            &[
                "---",
                "class: tip",
                "flag: false",
                "count: 3",
                "---",
                "body",
            ],
        )
        .unwrap();
        assert_eq!(
            parsed.coerced_options["class"],
            OptionValue::Classes(vec!["tip".into()])
        );
        assert_eq!(parsed.coerced_options["flag"], OptionValue::Bool(false));
        assert_eq!(parsed.coerced_options["count"], OptionValue::Integer(3));
        assert_eq!(parsed.body, vec!["body"]);

        let invalid = parse_directive_text(&spec, "", &[":class: [1]"]).unwrap();
        assert!(invalid.coerced_options.is_empty());
        assert!(
            invalid
                .warnings
                .iter()
                .any(|warning| warning.contains("class"))
        );
    }

    #[test]
    fn no_argument_directive_moves_first_line_into_body() {
        let spec = DirectiveSpec {
            has_content: true,
            ..DirectiveSpec::default()
        };
        let parsed = parse_directive_text(&spec, "first line", &["second line"]).unwrap();
        assert_eq!(parsed.arguments, Vec::<String>::new());
        assert_eq!(parsed.body, vec!["first line", "second line"]);
        assert_eq!(parsed.content_offset, 0);
    }
}
