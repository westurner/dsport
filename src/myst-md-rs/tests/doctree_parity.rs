#[path = "common/mod.rs"]
mod common;

use common::parse_dot_format;

#[test]
fn core_sphinx_doctree_fixtures_match_pseudo_xml_byte_for_byte() {
    let cases = parse_dot_format(include_str!(
        "../../MyST-Parser/tests/test_renderers/fixtures/sphinx_syntax_elements.md"
    ));
    let exact_titles = [
        "Raw",
        "Strong:",
        "Emphasis",
        "Heading:",
        "Heading Levels:",
        "Inline Code:",
        "Block Quote:",
    ];

    for case in cases
        .iter()
        .filter(|case| exact_titles.contains(&case.title.as_str()))
    {
        let tree = myst_md_rs::parse_to_doctree(
            &case.content,
            "<src>/index.md",
            &myst_md_rs::DoctreeOptions::default(),
        );
        let actual = docutilsrs::pseudo_xml(&tree);
        assert_eq!(
            actual.trim_end(),
            case.expected.trim_end(),
            "fixture {} did not match pseudo-XML",
            case.title
        );
    }
}
