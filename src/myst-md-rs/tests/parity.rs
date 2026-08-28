//! Cross-wave parity aggregator.
//!
//! Prints one line per ported test fixture so CI can scrape a single
//! summary. Doesn't fail unless an *unexpected* regression appears in
//! one of the already-green waves (currently W1 commonmark spec).
//!
//! Per-fixture allowlists live in the relevant module; this file just
//! aggregates the counts.

#[path = "common/mod.rs"]
mod common;

use std::sync::atomic::{AtomicUsize, Ordering};

use myst_md_rs::directives::{
    DirectiveError, DirectiveSpec, OptionKind, OptionValue, parse_directive_text,
};
use serde_yaml::Value;

/// Run a single fixture set and return `(passed, allowed, failed,
/// total)`. `case_runner` returns `Ok(true)` on pass, `Ok(false)` on
/// allowed deviation, and `Err(detail)` on unexpected failure.
fn run_fixture<F>(
    label: &str,
    cases: &[common::ParamCase],
    mut case_runner: F,
) -> (usize, usize, usize)
where
    F: FnMut(&common::ParamCase) -> Result<bool, String>,
{
    let mut passed = 0;
    let mut allowed = 0;
    let mut failed = 0;
    for case in cases {
        match case_runner(case) {
            Ok(true) => passed += 1,
            Ok(false) => allowed += 1,
            Err(_detail) => failed += 1,
        }
    }
    eprintln!(
        "[parity] {label}: {passed}/{total} passing ({allowed} pending, {failed} unexpected)",
        total = cases.len(),
    );
    (passed, allowed, failed)
}

#[test]
fn aggregate_parity() {
    let unexpected = AtomicUsize::new(0);

    // W1: CommonMark spec. The dedicated `commonmark.rs` integration
    // test already enforces parity; here we just surface the count.
    eprintln!(
        "[parity] commonmark: see `cargo test -p myst-md-rs --test commonmark` (623/646 + 23 allowed + 3 skipped of 649)"
    );

    // W3a: option block parsing.
    {
        let raw = include_str!("data/option_parsing.yaml");
        let cases = common::parse_yaml_format(raw);
        let (_, _, f) = run_fixture("options (parse)", &cases, |case| {
            myst_md_rs::options::options_to_items(&case.content)
                .map(|_| true)
                .map_err(|error| error.to_string())
        });
        unexpected.fetch_add(f, Ordering::Relaxed);
    }

    // W3b: option block error reporting.
    {
        let raw = include_str!("data/option_parsing_errors.yaml");
        let cases = common::parse_yaml_format(raw);
        let (_, _, f) = run_fixture("options (errors)", &cases, |case| {
            match myst_md_rs::options::options_to_items(&case.content) {
                Err(_) => Ok(true),
                Ok(_) => Err("invalid option fixture was accepted".into()),
            }
        });
        unexpected.fetch_add(f, Ordering::Relaxed);
    }

    // W3c: directive text parsing.
    {
        let raw = include_str!("data/directive_parsing.txt");
        let cases = common::parse_dot_format(raw);
        let (_, _, f) = run_fixture("directives", &cases, compare_directive_case);
        unexpected.fetch_add(f, Ordering::Relaxed);
    }

    let n = unexpected.load(Ordering::Relaxed);
    assert_eq!(n, 0, "{n} unexpected parity failures");
}

#[derive(Debug, serde::Deserialize)]
struct ExpectedDirective {
    #[serde(default)]
    arguments: Vec<String>,
    #[serde(default)]
    body: Vec<String>,
    #[serde(default)]
    content_offset: usize,
    #[serde(default)]
    options: std::collections::BTreeMap<String, Value>,
    #[serde(default)]
    warnings: Vec<Value>,
    error: Option<String>,
}

fn compare_directive_case(case: &common::ParamCase) -> Result<bool, String> {
    if directive_case_is_allowed_deviation(case) {
        return Ok(false);
    }
    let expected: ExpectedDirective =
        serde_yaml::from_str(&case.expected).map_err(|error| error.to_string())?;
    let (name, first_line, body) = split_directive_fence(&case.content)?;
    let spec = directive_fixture_spec(&name);
    let actual = parse_directive_text(&spec, &first_line, &body);

    match (actual, expected.error.clone()) {
        (Err(error), Some(expected_message)) => {
            let actual_message = directive_error_message(error);
            if actual_message == expected_message {
                Ok(true)
            } else {
                Err(format!(
                    "{}: expected error {expected_message:?}, got {actual_message:?}",
                    case.title
                ))
            }
        }
        (Err(error), None) => Err(format!(
            "{}: unexpected parser error: {}",
            case.title,
            directive_error_message(error)
        )),
        (Ok(_), Some(expected_message)) => Err(format!(
            "{}: expected parser error {expected_message:?}",
            case.title
        )),
        (Ok(actual), None) => {
            let actual_options = actual
                .coerced_options
                .into_iter()
                .map(|(key, value)| (key, option_value_yaml(value)))
                .collect::<std::collections::BTreeMap<_, _>>();
            if actual.arguments != expected.arguments
                || actual.body != expected.body
                || actual.content_offset != expected.content_offset
                || actual_options != expected.options
                || actual.warnings.len() != expected.warnings.len()
            {
                return Err(format!(
                    "{}: expected {expected:?}, got arguments={:?}, body={:?}, content_offset={}, options={:?}, warnings={:?}",
                    case.title,
                    actual.arguments,
                    actual.body,
                    actual.content_offset,
                    actual_options,
                    actual.warnings
                ));
            }
            Ok(true)
        }
    }
}

fn split_directive_fence(content: &str) -> Result<(String, String, Vec<&str>), String> {
    let mut lines = content.lines();
    let opening = lines
        .next()
        .ok_or_else(|| "missing directive fence".to_string())?;
    let header = opening
        .strip_prefix("```")
        .and_then(|line| line.strip_prefix('{'))
        .ok_or_else(|| format!("invalid directive opening: {opening:?}"))?;
    let (name, first_line) = header
        .split_once('}')
        .map(|(name, first_line)| (name.to_string(), first_line.trim().to_string()))
        .unwrap_or_else(|| (header.to_string(), String::new()));
    let body = lines.take_while(|line| *line != "```").collect::<Vec<_>>();
    Ok((name, first_line, body))
}

fn directive_fixture_spec(name: &str) -> DirectiveSpec {
    let mut spec = DirectiveSpec {
        has_content: true,
        ..DirectiveSpec::default()
    };
    if name == "admonition" {
        spec.required_arguments = 1;
        spec.final_argument_whitespace = true;
    }
    spec.option_spec.insert("class".into(), OptionKind::Classes);
    spec.option_spec.insert("name".into(), OptionKind::String);
    if name == "code-block" {
        spec.option_spec.insert("force".into(), OptionKind::Flag);
        spec.option_spec.insert("linenos".into(), OptionKind::Flag);
        spec.option_spec
            .insert("lineno-start".into(), OptionKind::Integer);
    }
    spec
}

fn directive_case_is_allowed_deviation(case: &common::ParamCase) -> bool {
    matches!(
        case.title.as_str(),
        "note: comment in option"
            | "warning: bad yaml"
            | "warning: yaml not a dict"
            | "warning: unknown option name"
            | "warning: invalid option value"
    ) || (case.title == "note: content in first line and body"
        && case.content.contains(":class: tip"))
}

fn option_value_yaml(value: OptionValue) -> Value {
    match value {
        OptionValue::String(value) => Value::String(value),
        OptionValue::Flag => Value::Null,
        OptionValue::Integer(value) => Value::Number(value.into()),
        OptionValue::Float(value) => serde_yaml::to_value(value).unwrap_or(Value::Null),
        OptionValue::Bool(value) => Value::Bool(value),
        OptionValue::Classes(values) => {
            Value::Sequence(values.into_iter().map(Value::String).collect())
        }
    }
}

fn directive_error_message(error: DirectiveError) -> String {
    match error {
        DirectiveError::Syntax(message) => message,
        DirectiveError::Options(error) => error.to_string(),
    }
}
