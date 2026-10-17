use serde::Deserialize;
use std::collections::BTreeMap;

#[path = "common/mod.rs"]
mod common;

use common::parse_dot_format;

#[derive(Debug, Deserialize)]
struct Expected {
    dict: Vec<(String, String)>,
    comments: bool,
}

#[derive(Debug, Deserialize)]
struct Case {
    content: String,
    expected: String,
}

#[test]
fn option_fixtures_match_ordered_pairs_and_comment_state() {
    let cases: serde_yaml::Mapping =
        serde_yaml::from_str(include_str!("data/option_parsing.yaml")).unwrap();
    for (title, value) in cases {
        let title = title.as_str().unwrap();
        let case: Case = serde_yaml::from_value(value).unwrap();
        let expected: Expected = serde_json::from_str(&case.expected).unwrap();
        let (actual, state) = myst_md_rs::options::options_to_items(&case.content)
            .unwrap_or_else(|error| panic!("{title}: {error}"));
        assert_eq!(actual, expected.dict, "{title}");
        assert_eq!(state.has_comments, expected.comments, "{title}");
    }
}

#[test]
fn option_error_fixtures_are_rejected() {
    let cases: serde_yaml::Mapping =
        serde_yaml::from_str(include_str!("data/option_parsing_errors.yaml")).unwrap();
    for (title, value) in cases {
        let title = title.as_str().unwrap();
        let case: Case = serde_yaml::from_value(value).unwrap();
        assert!(
            myst_md_rs::options::options_to_items(&case.content).is_err(),
            "{title} should be rejected"
        );
    }
}

#[test]
fn upstream_directive_options_fixture_matches_parser_contract() {
    let cases = parse_dot_format(include_str!(
        "../../MyST-Parser/tests/test_renderers/fixtures/directive_options.md"
    ));
    let mut option_spec = BTreeMap::new();
    option_spec.insert(
        "option1".to_owned(),
        myst_md_rs::directives::OptionKind::String,
    );
    option_spec.insert(
        "option2".to_owned(),
        myst_md_rs::directives::OptionKind::String,
    );
    let spec = myst_md_rs::directives::DirectiveSpec {
        optional_arguments: 1,
        has_content: true,
        final_argument_whitespace: true,
        option_spec,
        ..myst_md_rs::directives::DirectiveSpec::default()
    };

    for case in cases {
        if case.title == "Unknown Directive:" {
            continue;
        }
        let lines = case.content.lines().collect::<Vec<_>>();
        let opening = lines
            .first()
            .unwrap_or_else(|| panic!("{}: no opening fence", case.title));
        let first_line = opening
            .strip_prefix("```{restructuredtext-test-directive}")
            .unwrap_or_else(|| panic!("{}: unexpected directive fence", case.title))
            .trim_start();
        assert_eq!(
            lines.last(),
            Some(&"```"),
            "{}: no closing fence",
            case.title
        );
        let body = &lines[1..lines.len() - 1];
        let parsed = myst_md_rs::directives::parse_directive_text(&spec, first_line, body);

        if case.title == "Test Directive Options Error:" {
            assert!(
                matches!(
                    parsed,
                    Err(myst_md_rs::directives::DirectiveError::Options(_))
                ),
                "{} should reject malformed options",
                case.title
            );
            continue;
        }

        let parsed = parsed.unwrap_or_else(|error| panic!("{}: {error:?}", case.title));
        let expected_arguments: Vec<&str> = match case.title.as_str() {
            "Test Directive 3:" | "Test Directive 4:" => vec!["foo"],
            "Test Directive 5:" | "Test Directive 6:" => vec!["foo bar"],
            _ => Vec::new(),
        };
        assert_eq!(parsed.arguments, expected_arguments, "{}", case.title);
        let expected_options = if case.title.starts_with("Test Directive Options") {
            vec![
                ("option1".to_owned(), "a".to_owned()),
                ("option2".to_owned(), "b".to_owned()),
            ]
        } else {
            Vec::new()
        };
        assert_eq!(parsed.options, expected_options, "{}", case.title);
        let expected_body: Vec<String> = match case.title.as_str() {
            "Test Directive 2:" | "Test Directive 7:" => vec!["foo".into()],
            "Test Directive 4:" => vec!["bar".into()],
            "Test Directive 6:" => vec!["baz".into()],
            "Test Directive Options 1:" | "Test Directive Options 2:" => vec!["foo".into()],
            _ => Vec::new(),
        };
        assert_eq!(parsed.body, expected_body, "{}", case.title);
    }
}
