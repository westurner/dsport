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
        "Nested Bullets",
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

#[test]
fn basic_include_fixture_matches_upstream_pseudo_xml() {
    let cases = parse_dot_format(include_str!(
        "../../MyST-Parser/tests/test_renderers/fixtures/mock_include.md"
    ));
    let basic_case = cases
        .iter()
        .find(|case| case.title == "Basic Include:")
        .expect("upstream Basic Include fixture");
    let front_matter_case = cases
        .iter()
        .find(|case| case.title == "Include with Front Matter (should be ignored):")
        .expect("upstream front matter include fixture");
    let literal_case = cases
        .iter()
        .find(|case| case.title == "Include Literal:")
        .expect("upstream Include Literal fixture");
    let literal_lines_case = cases
        .iter()
        .find(|case| case.title == "Include Literal, line range:")
        .expect("upstream literal line range fixture");
    let code_case = cases
        .iter()
        .find(|case| case.title == "Include code:")
        .expect("upstream code include fixture");
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "myst-include-parity-{}-{unique}",
        std::process::id()
    ));
    std::fs::create_dir(&root).unwrap();
    struct RemoveDir(std::path::PathBuf);
    impl Drop for RemoveDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = RemoveDir(root.clone());
    std::fs::write(root.join("other.md"), "a\nb\nc").unwrap();
    std::fs::write(root.join("fmatter.md"), "---\na: 1\n---\nb").unwrap();

    let mut options = myst_md_rs::DoctreeOptions::default();
    options.source_root = Some(root.clone());
    for case in [
        basic_case,
        front_matter_case,
        literal_case,
        literal_lines_case,
        code_case,
    ] {
        std::fs::write(root.join("test.md"), &case.content).unwrap();
        let source = root.join("test.md");
        let tree = myst_md_rs::parse_to_doctree(
            &case.content,
            source.to_string_lossy().into_owned(),
            &options,
        );
        let actual =
            docutilsrs::pseudo_xml(&tree).replace(&root.to_string_lossy().to_string(), "tmpdir");
        assert_eq!(
            actual.trim_end(),
            case.expected.trim_end(),
            "fixture {} did not match pseudo-XML",
            case.title
        );
    }
}
