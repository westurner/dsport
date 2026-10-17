//! W7/H13 acceptance coverage for the native MyST-to-Sphinx bridge.

use std::collections::HashMap;

use sphinxdocrs::application::SphinxApp;
use tempfile::TempDir;

fn write_mixed_project() -> TempDir {
    let project = TempDir::new().expect("temporary project");
    std::fs::write(
        project.path().join("conf.py"),
        "project = 'MyST bridge'\nsource_suffix = {'.rst': 'restructuredtext', '.md': 'myst'}\nmaster_doc = 'index'\nexclude_patterns = ['included.md']\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("index.rst"),
        "Bridge\n======\n\nThe project also contains a native Markdown page.\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("guide.md"),
        "# Markdown guide {#guide-title .guide}\n\nThis is **native** MyST with a [link](https://example.com).\n\n:::note\nRead the guide.\n:::\n\n:::include included.md\n:::\n\n```{eval-rst}\n**eval bold**\n```\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\nTerm\n: Definition\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("included.md"),
        "Included **Markdown**.\n",
    )
    .unwrap();
    project
}

fn system_message_text(tree: &docutilsrs::Doctree, needle: &str) -> Option<(Option<u32>, String)> {
    (0..tree.nodes_len()).find_map(|id| {
        let node = tree.node(id);
        let docutilsrs::NodeKind::SystemMessage { line, .. } = node.kind else {
            return None;
        };
        let message = node
            .children
            .iter()
            .filter_map(|child| match &tree.node(*child).kind {
                docutilsrs::NodeKind::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<String>();
        message.contains(needle).then_some((line, message))
    })
}

fn system_message_kind(
    tree: &docutilsrs::Doctree,
    needle: &str,
) -> Option<(Option<u32>, u32, &'static str)> {
    (0..tree.nodes_len()).find_map(|id| {
        let node = tree.node(id);
        let docutilsrs::NodeKind::SystemMessage {
            line, level, ty, ..
        } = &node.kind
        else {
            return None;
        };
        let message = node
            .children
            .iter()
            .filter_map(|child| match &tree.node(*child).kind {
                docutilsrs::NodeKind::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<String>();
        message.contains(needle).then_some((*line, *level, *ty))
    })
}

#[test]
fn mixed_rst_and_myst_build_uses_persisted_doctree() {
    let project = write_mixed_project();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    assert_eq!(
        env.parser_for_path(&project.path().join("guide.md")),
        "myst"
    );
    assert!(env.found_docs().contains("guide"));
    assert!(env.has_stored_doctree("guide"));
    let tree = env.get_doctree("guide").unwrap();
    let xml = docutilsrs::to_xml(&tree);
    assert!(xml.contains("source=\"") && xml.contains("guide.md"));
    assert!(xml.contains("ids=\"guide-title\""));
    assert!(xml.contains("<strong>"));
    assert!(xml.contains("<reference"));
    assert!(xml.contains("<note>"));
    assert!(xml.contains("Included"));
    assert!(xml.contains("eval bold"));
    assert!(xml.contains("<table>"));
    assert!(xml.contains("<definition_list>"));
    drop(env);

    let result = app.build().unwrap();
    assert_eq!(result.written, 2);
    let html = std::fs::read_to_string(output.path().join("guide.html")).unwrap();
    assert!(html.contains("Markdown guide"));
    assert!(html.contains("id=\"guide-title\""));
    assert!(html.contains("native"));
    assert!(html.contains("Read the guide."));
    assert!(html.contains("Included"));
    assert!(html.contains("eval bold"));
    assert!(html.contains("<table>"));
    assert!(html.contains("Definition"));
}

#[test]
fn myst_include_ignores_included_front_matter() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        ":::include included.md\n:::\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("included.md"),
        "---\ntitle: Included metadata\n---\nIncluded body.\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let xml = docutilsrs::to_xml(&tree);
    assert!(xml.contains("Included body."));
    assert!(!xml.contains("Included metadata"));
}

#[test]
fn myst_include_reports_unknown_role_from_included_source() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        ":::include included.md\n:::\n",
    )
    .unwrap();
    std::fs::write(project.path().join("included.md"), "Before.\n\n{a}`b`\n").unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let (line, message) = system_message_text(&tree, "Unknown interpreted text role")
        .expect("unknown interpreted role warning");
    assert_eq!(line, Some(3), "{message}");
    assert!(message.contains("a"));
    assert!(message.contains("included.md"));
}

#[test]
fn myst_unknown_directive_warning_survives_sphinx_read_path() {
    let project = write_mixed_project();
    std::fs::write(project.path().join("guide.md"), "```{unknown}\nbody\n```\n").unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let (line, message) =
        system_message_text(&tree, "Unknown directive type").expect("unknown-directive warning");
    assert_eq!(line, Some(1));
    assert!(message.contains("unknown"));
    assert!(message.contains("[myst.directive_unknown]"));
}

#[test]
fn myst_include_accepts_a_registered_extension_role() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        ":::include included.md\n:::\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("included.md"),
        "{fixture-role}`value`\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.registry
        .borrow_mut()
        .add_role("fixture-role", "FixtureRole");
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    assert!(system_message_text(&tree, "Unknown interpreted text role").is_none());
}

#[test]
fn unknown_source_parser_fails_during_read() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("conf.py"),
        "project = 'Unknown parser'\nsource_suffix = {'.md': 'not-a-parser'}\nmaster_doc = 'guide'\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);

    let error = app.read().expect_err("unknown parser should be rejected");
    assert!(error.to_string().contains("unknown source parser"));
}

#[test]
fn myst_substitution_diagnostic_survives_sphinx_read_path() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "---\ntitle: Example\n---\nUse {{ missing }} here.\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let diagnostic = (0..tree.nodes_len())
        .find(|id| {
            matches!(
                tree.node(*id).kind,
                docutilsrs::NodeKind::SystemMessage {
                    line: Some(4),
                    ty: "WARNING",
                    ..
                }
            )
        })
        .expect("source-located undefined-substitution warning");
    let message =
        tree.node(diagnostic)
            .children
            .iter()
            .find_map(|child| match &tree.node(*child).kind {
                docutilsrs::NodeKind::Text(text) => Some(text.as_str()),
                _ => None,
            });
    let message = message.expect("diagnostic message text");
    assert!(message.starts_with("undefined substitution"));
    assert!(message.contains("missing"));
}

#[test]
fn myst_front_matter_warnings_survive_sphinx_read_path() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "---\nmyst:\n  unknown: true\n  title_to_header: 1\n  url_schemes: [1]\n---\nBody.\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    for needle in ["Unknown field: unknown", "title_to_header", "url_schemes"] {
        let (line, _) = system_message_text(&tree, needle).expect("front matter warning");
        assert_eq!(line, Some(1), "warning for {needle}");
    }
}

#[test]
fn myst_html_meta_invalid_attribute_emits_error() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "---\nmyst:\n  html_meta:\n    name noequals: value\n---\nBody.\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let (line, level, ty) = system_message_kind(&tree, "Error parsing meta tag attribute")
        .expect("malformed html_meta error");
    assert_eq!(line, Some(1));
    assert_eq!(level, 3);
    assert_eq!(ty, "ERROR");
    let (_, message) = system_message_text(&tree, "Error parsing meta tag attribute").unwrap();
    let normalized = message
        .replace('\u{2018}', "'")
        .replace('\u{2019}', "'")
        .replace('\u{201c}', "\"")
        .replace('\u{201d}', "\"");
    assert_eq!(
        normalized,
        "Error parsing meta tag attribute \"name noequals\": no '=' in noequals."
    );
}

#[test]
fn myst_directive_option_and_missing_content_diagnostics_survive_read_path() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "```{note}\n:class: [1]\n```\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let normalize_smart_quotes = |message: &str| {
        message
            .replace('\u{2018}', "'")
            .replace('\u{2019}', "'")
            .replace('\u{201c}', "\"")
            .replace('\u{201d}', "\"")
    };
    let (option_line, option_warning) =
        system_message_text(&tree, "Invalid option value for").expect("invalid option warning");
    assert_eq!(option_line, Some(1));
    assert_eq!(
        normalize_smart_quotes(&option_warning),
        "'note': Invalid option value for 'class': [1]: cannot make \"[1]\" into a class name [myst.directive_option]"
    );
    let (line, content_error) =
        system_message_text(&tree, "Content block expected").expect("missing-content error");
    assert_eq!(line, Some(1));
    assert_eq!(
        normalize_smart_quotes(&content_error),
        "Content block expected for the \"note\" directive; none found."
    );
}

#[test]
fn myst_heading_warning_survives_sphinx_read_path() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "# First\n\n### Skipped level\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let (line, message) = system_message_text(&tree, "Non-consecutive header level increase")
        .expect("heading level warning");
    assert_eq!(line, Some(3));
    assert!(message.contains("H1 to H3"));
}

#[test]
fn myst_duplicate_reference_warning_survives_sphinx_read_path() {
    let project = write_mixed_project();
    std::fs::write(project.path().join("guide.md"), "[a]: first\n[a]: second\n").unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let (line, message) =
        system_message_text(&tree, "Duplicate reference definition").expect("duplicate warning");
    assert_eq!(line, Some(2));
    assert!(message.contains("A [myst.duplicate_def]"));
}

#[test]
fn myst_missing_local_link_reports_an_error() {
    let project = write_mixed_project();
    std::fs::write(project.path().join("guide.md"), "[label](missing)\n").unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let (line, message) =
        system_message_text(&tree, "Unknown target name").expect("missing local link diagnostic");
    assert_eq!(line, Some(1));
    assert!(message.contains("missing"));
}

#[test]
fn myst_existing_local_link_does_not_report_an_error() {
    let project = write_mixed_project();
    std::fs::write(project.path().join("guide.md"), "[included](included.md)\n").unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    assert!(system_message_text(&tree, "Unknown target name").is_none());
}

#[test]
fn missing_myst_include_reports_a_source_located_error() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "Before.\n\n:::include missing.md\n:::\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let (line, message) = system_message_text(&tree, "file not found").unwrap();
    assert_eq!(line, Some(3));
    assert!(message.contains("missing.md"));
    assert_eq!(
        system_message_kind(&tree, "file not found"),
        Some((Some(3), 4, "SEVERE"))
    );
}

#[test]
fn myst_include_missing_argument_reports_an_error() {
    let project = write_mixed_project();
    std::fs::write(project.path().join("guide.md"), ":::include\n:::\n").unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    assert_eq!(
        system_message_kind(&tree, "1 argument(s) required, 0 supplied"),
        Some((Some(1), 3, "ERROR"))
    );
}

#[test]
fn myst_include_cannot_read_outside_the_source_root() {
    let project = write_mixed_project();
    let outside = TempDir::new().unwrap();
    std::fs::write(
        outside.path().join("outside.md"),
        "secret outside source root\n",
    )
    .unwrap();
    let outside_dir = outside.path().file_name().unwrap().to_string_lossy();
    std::fs::write(
        project.path().join("guide.md"),
        format!(":::include ../{outside_dir}/outside.md\n:::\n"),
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    assert!(!(0..tree.nodes_len()).any(|id| matches!(
        &tree.node(id).kind,
        docutilsrs::NodeKind::Text(text) if text.contains("secret outside source root")
    )));
    assert_eq!(
        system_message_text(&tree, "escapes source root").unwrap().0,
        Some(1)
    );
}

#[test]
fn recursive_myst_include_reports_the_include_chain() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        ":::include included.md\n:::\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("included.md"),
        ":::include guide.md\n:::\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let (line, message) =
        system_message_text(&tree, "recursive MyST include").expect("recursive include warning");
    assert_eq!(line, Some(1));
    assert!(message.contains("guide.md") && message.contains("included.md"));
}

#[test]
fn myst_include_literal_preserves_markup_as_text() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "```{include} included.md\n:literal:\n```\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("included.md"),
        "# Included\n\n**not parsed as markup**\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let literal = (0..tree.nodes_len())
        .find(|id| {
            matches!(
                tree.node(*id).kind,
                docutilsrs::NodeKind::LiteralBlock { .. }
            )
        })
        .expect("literal include block");
    let text = tree
        .node(literal)
        .children
        .iter()
        .find_map(|child| match &tree.node(*child).kind {
            docutilsrs::NodeKind::Text(text) => Some(text.as_str()),
            _ => None,
        });
    assert!(text.unwrap().contains("**not parsed as markup**"));
    assert!(
        !(0..tree.nodes_len()).any(|id| matches!(tree.node(id).kind, docutilsrs::NodeKind::Strong))
    );
}

#[test]
fn myst_include_code_selects_requested_line_range() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "```{include} included.md\n:code: text\n:start-line: 1\n:end-line: 2\n```\n",
    )
    .unwrap();
    std::fs::write(project.path().join("included.md"), "first\nsecond\nthird\n").unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let literal = (0..tree.nodes_len())
        .find(|id| {
            matches!(
                tree.node(*id).kind,
                docutilsrs::NodeKind::LiteralBlock { .. }
            )
        })
        .expect("code include block");
    assert!(matches!(
        &tree.node(literal).kind,
        docutilsrs::NodeKind::LiteralBlock { classes, .. } if classes == "code text"
    ));
    let text = tree
        .node(literal)
        .children
        .iter()
        .find_map(|child| match &tree.node(*child).kind {
            docutilsrs::NodeKind::Text(text) => Some(text.as_str()),
            _ => None,
        });
    assert_eq!(text, Some("second"));
}

#[test]
fn myst_include_code_selects_between_markers() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "```{include} included.md\n:code: text\n:start-after: START\n:end-before: END\n```\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("included.md"),
        "skip\nSTART\nkeep this line\nEND\nskip\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    let literal = (0..tree.nodes_len())
        .find(|id| {
            matches!(
                tree.node(*id).kind,
                docutilsrs::NodeKind::LiteralBlock { .. }
            )
        })
        .expect("code include block");
    let text = tree
        .node(literal)
        .children
        .iter()
        .find_map(|child| match &tree.node(*child).kind {
            docutilsrs::NodeKind::Text(text) => Some(text.as_str()),
            _ => None,
        });
    assert_eq!(text, Some("keep this line"));
}

#[test]
fn myst_include_reports_unsupported_and_invalid_options() {
    let project = write_mixed_project();
    std::fs::write(
        project.path().join("guide.md"),
        "```{include} included.md\n:number-lines: 1\n:start-line: invalid\n```\n",
    )
    .unwrap();
    let output = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    let mut app = SphinxApp::new(
        project.path(),
        output.path(),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();
    app.set_incremental_options(true, true);
    app.read().unwrap();

    let env = app.env.borrow();
    let tree = env.get_doctree("guide").unwrap();
    assert!(system_message_text(&tree, "Unknown option key: number-lines").is_some());
    let (_, invalid) = system_message_text(&tree, "Invalid option value for").unwrap();
    assert!(invalid.contains("start-line"));
}
