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
    let env = BuildEnvironment::new(config, project, src.path(), out.path());
    let result = SinglehtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 2);

    // Exactly one page is written, at the output root.
    assert!(out.path().join("index.html").exists());
    assert!(!out.path().join("guide").exists());

    let combined = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(combined.contains("id=\"index\""));
    assert!(combined.contains("id=\"guide/intro\""));
    assert!(combined.contains("Homepage."));
    assert!(combined.contains("Nested doc."));
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
