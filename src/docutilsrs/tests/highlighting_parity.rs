use docutilsrs::code_block::{Span, tokenize};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HighlightCase {
    id: String,
    language: String,
    source: String,
    spans: Option<Vec<Span>>,
}

#[test]
fn normalized_highlighting_matches_fixture_matrix() {
    let cases: Vec<HighlightCase> =
        serde_json::from_str(include_str!("fixtures/highlighting/matrix.json")).unwrap();
    assert!(cases.len() >= 15);
    for case in cases {
        let actual = tokenize(&case.language, &case.source);
        assert_eq!(actual, case.spans, "highlighting mismatch for {}", case.id);
        if let Some(spans) = actual {
            assert_eq!(
                spans
                    .iter()
                    .map(|(_, text)| text.as_str())
                    .collect::<String>(),
                case.source.trim_end_matches('\n'),
                "span text mismatch for {}",
                case.id
            );
        }
    }
}
