//! Integration tests for `sphinxdocrs::builders::gettext::GettextBuilder`
//! (H7c).
//!
//! Complements the inline `#[cfg(test)]` unit tests in
//! `src/builders/gettext.rs` with a full `SphinxApp::build()` dispatch
//! and a multi-document project producing a combined `.pot` catalog.

use std::collections::HashMap;

use tempfile::TempDir;

use sphinxdocrs::application::SphinxApp;
use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::gettext::GettextBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn builder_identity() {
    let b = GettextBuilder::new();
    assert_eq!(b.name(), "gettext");
    assert_eq!(b.format(), "gettext");
    assert_eq!(b.out_suffix(), ".pot");
}

#[test]
fn build_all_over_a_multi_document_project_writes_one_combined_pot() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nA shared greeting.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("about.rst"),
        "About\n=====\n\nA shared greeting.\n\nSomething unique to About.\n",
    )
    .unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, src.path(), out.path());
    let result = GettextBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 2);

    let pot_path = out.path().join("sphinx.pot");
    assert!(pot_path.exists());
    let pot = std::fs::read_to_string(&pot_path).unwrap();

    // Standard .pot header.
    assert!(pot.starts_with("# SOME DESCRIPTIVE TITLE."));
    assert!(pot.contains("msgid \"\"\nmsgstr \"\""));

    // A message shared by both documents appears once, with both
    // documents' location comments attached.
    let shared_idx = pot.find("msgid \"A shared greeting.\"").unwrap();
    let preceding = &pot[..shared_idx];
    let last_locations_block = preceding.rsplit("\n\n").next().unwrap();
    assert!(last_locations_block.contains("#: index"));
    assert!(last_locations_block.contains("#: about"));

    // A message unique to one document also appears.
    assert!(pot.contains("msgid \"Something unique to About.\""));
}

#[test]
fn sphinx_app_build_dispatches_to_gettext() {
    let src = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHello there.\n",
    )
    .unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();

    let mut app = SphinxApp::new(
        src.path(),
        out.path(),
        doctrees.path(),
        "gettext",
        HashMap::new(),
    )
    .unwrap();
    let result = app.build().unwrap();
    assert_eq!(result.written, 1);
    let pot = std::fs::read_to_string(out.path().join("sphinx.pot")).unwrap();
    assert!(pot.contains("msgid \"Hello there.\""));
}
