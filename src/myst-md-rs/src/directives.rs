//! MyST directive-text parser.
//!
//! Stub surface — matches the public signature of
//! `myst_parser.parsers.directives.parse_directive_text` so test
//! fixtures from `tests/data/directive_parsing.txt` can be wired in
//! immediately. Implements the simplest path (first-line argument +
//! body lines, no options) and defers the full option-spec coercion
//! to a later wave. Failing fixtures are tracked in
//! `tests/parity.rs` and `docs/compat.md`.

use crate::options::{OptionsError, options_to_items};

#[derive(Debug, Clone, Default)]
pub struct DirectiveSpec {
    pub required_arguments: usize,
    pub optional_arguments: usize,
    pub has_content: bool,
    /// Final argument may contain whitespace.
    pub final_argument_whitespace: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ParsedDirective {
    pub arguments: Vec<String>,
    pub options: Vec<(String, String)>,
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
    if !first_line.is_empty() {
        if spec.required_arguments + spec.optional_arguments == 0 {
            // Argument present where none is allowed → fold into body
            // (upstream just records a warning; do the same).
            out.warnings.push(format!(
                "directive takes no arguments; got {first_line:?}"
            ));
        } else if spec.final_argument_whitespace
            || (spec.required_arguments + spec.optional_arguments) == 1
        {
            out.arguments.push(first_line.to_string());
        } else {
            for arg in first_line.split_whitespace() {
                out.arguments.push(arg.to_string());
            }
        }
    }

    // Split body into option-block + content. The option block is the
    // contiguous run of leading `:key: value` lines (and blank/comment
    // lines between them).
    let mut idx = 0usize;
    let mut option_lines: Vec<&str> = Vec::new();
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

    if !option_lines.is_empty() {
        let joined = option_lines.join("\n");
        let (items, _state) = options_to_items(&joined)?;
        out.options = items;
    }

    out.content_offset = idx;
    while idx < body.len() {
        out.body.push(body[idx].to_string());
        idx += 1;
    }

    if !spec.has_content && !out.body.is_empty() {
        out.warnings
            .push("directive takes no content; ignoring body".into());
    }

    Ok(out)
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
}
