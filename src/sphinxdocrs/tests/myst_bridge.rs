//! W7/H13 acceptance coverage for the native MyST-to-Sphinx bridge.

use std::collections::HashMap;

use sphinxdocrs::application::SphinxApp;
use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::html::HtmlBuilder;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};
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

    let single_output = TempDir::new().unwrap();
    let single_doctrees = TempDir::new().unwrap();
    let mut single_env = BuildEnvironment::new(
        app.config.clone(),
        EnvProject::new(project.path(), &[(".rst", "restructuredtext")]),
        project.path(),
        single_doctrees.path(),
    );
    single_env.find_files().unwrap();
    HtmlBuilder::new()
        .build_all(project.path(), single_output.path(), &single_env)
        .unwrap();
    let single_phase_html =
        std::fs::read_to_string(single_output.path().join("guide.html")).unwrap();

    let result = app.build().unwrap();
    assert_eq!(result.written, 2);
    let html = std::fs::read_to_string(output.path().join("guide.html")).unwrap();
    assert_eq!(html, single_phase_html);
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
