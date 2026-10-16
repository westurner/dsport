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
fn missing_myst_include_reports_a_source_located_warning() {
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
    assert_eq!(
        system_message_text(&tree, "cannot resolve MyST include")
            .unwrap()
            .0,
        Some(3)
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
