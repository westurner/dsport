use serde::Deserialize;

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
