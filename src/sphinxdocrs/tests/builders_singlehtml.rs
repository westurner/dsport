//! Integration tests for
//! `sphinxdocrs::builders::singlehtml::SinglehtmlBuilder` (H7b).
//!
//! Complements the inline `#[cfg(test)]` unit tests in
//! `src/builders/singlehtml.rs` with a full `SphinxApp::build()` dispatch
//! and a multi-document, subdirectory-containing project.

use std::collections::HashMap;

use tempfile::TempDir;

use sphinxdocrs::application::SphinxApp;
use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::singlehtml::SinglehtmlBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn build_all_over_a_project_with_a_subdirectory_merges_every_doc() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::create_dir(src.path().join("guide")).unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("guide/intro.rst"),
        "Intro\n=====\n\nNested doc.\n",
    )
    .unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();
    let result = SinglehtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 2);

    // Exactly one page is written, at the output root.
    assert!(out.path().join("index.html").exists());
    assert!(!out.path().join("guide").exists());

    let combined = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(!combined.contains("id=\"document-index\""));
    assert!(combined.contains("id=\"document-guide/intro\""));
    assert!(combined.contains("Homepage."));
    assert!(combined.contains("Nested doc."));

    let inventory =
        sphinxdocrs::intersphinx::Inventory::load_file(out.path().join("objects.inv"), "").unwrap();
    let root_doc = inventory
        .get("std:doc", "index")
        .unwrap_or_else(|| panic!("missing root document row: {inventory:?}"));
    assert_eq!(root_doc.uri, "#document-index");
    let nested_doc = inventory
        .get("std:doc", "guide/intro")
        .unwrap_or_else(|| panic!("missing nested document row: {inventory:?}"));
    assert_eq!(nested_doc.uri, "#document-guide/intro");
}

#[test]
fn singlehtml_uses_real_theme_and_merged_page_anchors() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\n.. toctree::\n\n   guide\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("guide.rst"),
        "Guide\n=====\n\nGuide body.\n",
    )
    .unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(
        SphinxConfig::new_defaults(),
        project,
        src.path(),
        doctrees.path(),
    );
    env.config.set(
        "html_theme",
        sphinxdocrs::config::ConfigVal::Str("basic".into()),
    );
    env.find_files().unwrap();
    env.read_all().unwrap();

    SinglehtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(
        html.contains("<title>Project name not set  documentation</title>"),
        "{html}"
    );
    assert!(!out.path().join("_static/sphinxdocrs.css").exists());
    assert!(
        html.contains(
            "class=\"nav-item nav-item-0\"><a href=\"#\">Project name not set  documentation</a>"
        ),
        "{html}"
    );
    assert!(html.contains("class=\"sphinxsidebar\""), "{html}");
    assert!(html.contains("href=\"#document-guide\""), "{html}");
    assert!(html.contains("<li class=\"toctree-l1\">"), "{html}");
    assert!(html.contains("id=\"document-guide\""), "{html}");
    assert!(
        html.contains(
            "<div class=\"toctree-wrapper compound\">\n<span id=\"document-guide\"></span><section id=\"guide\">\n<h2>Guide"
        ),
        "{html}"
    );
    assert!(!html.contains("<div id=\"document-index\">"), "{html}");
    assert!(!html.contains("<div id=\"document-guide\">"), "{html}");
}

#[test]
fn sphinx_app_build_dispatches_to_singlehtml() {
    let src = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    std::fs::write(src.path().join("about.rst"), "About\n=====\n\nInfo.\n").unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();

    let mut app = SphinxApp::new(
        src.path(),
        out.path(),
        doctrees.path(),
        "singlehtml",
        HashMap::new(),
    )
    .unwrap();
    let result = app.build().unwrap();
    assert_eq!(result.written, 2);
    assert!(out.path().join("index.html").exists());
    assert!(!out.path().join("about.html").exists());
}

#[test]
fn titleless_first_document_uses_html_title_in_real_theme() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Homepage content only.\n").unwrap();

    let mut app = SphinxApp::new(
        src.path(),
        out.path(),
        doctrees.path(),
        "singlehtml",
        HashMap::new(),
    )
    .unwrap();
    app.build().unwrap();

    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(
        html.contains("<title>Project name not set  documentation</title>"),
        "{html}"
    );
    assert!(!html.contains("id=\"document-index\""), "{html}");
    assert!(html.contains("Homepage content only."), "{html}");
}
