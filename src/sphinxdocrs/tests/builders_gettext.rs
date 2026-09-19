//! Integration tests for `sphinxdocrs::builders::gettext::GettextBuilder`
//! (H7c).
//!
//! Complements the inline `#[cfg(test)]` unit tests in
//! `src/builders/gettext.rs` with a full `SphinxApp::build()` dispatch
//! and a multi-document project producing per-document `.pot` catalogs.

use std::collections::HashMap;

use tempfile::TempDir;

use sphinxdocrs::application::SphinxApp;
use sphinxdocrs::builders::gettext::GettextBuilder;
use sphinxdocrs::builders::Builder;
use sphinxdocrs::config::{ConfigVal, SphinxConfig};
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn builder_identity() {
    let b = GettextBuilder::new();
    assert_eq!(b.name(), "gettext");
    assert_eq!(b.format(), "gettext");
    assert_eq!(b.out_suffix(), ".pot");
}

#[test]
fn extracts_list_definition_field_and_image_messages() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Title\n=====\n\n* List item.\n\nTerm\n  Definition text.\n\n:Field: Field value.\n\n.. image:: image.png\n   :alt: Image description.\n",
    )
    .unwrap();
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, src.path(), out.path());

    GettextBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    let pot = std::fs::read_to_string(out.path().join("index.pot")).unwrap();
    for message in ["List item.", "Field", "Field value.", "Image description."] {
        assert!(
            pot.contains(&format!("msgid \"{message}\"")),
            "missing {message}"
        );
    }
    assert!(pot.contains("msgid \"Term\\n  Definition text.\""));
}

#[test]
fn build_all_over_a_multi_document_project_writes_per_document_pots() {
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

    let index_pot = out.path().join("index.pot");
    let about_pot = out.path().join("about.pot");
    assert!(index_pot.exists());
    assert!(about_pot.exists());
    let index = std::fs::read_to_string(index_pot).unwrap();
    let about = std::fs::read_to_string(about_pot).unwrap();

    // Standard .pot header.
    assert!(index.starts_with("# SOME DESCRIPTIVE TITLE."));
    assert!(index.contains("msgid \"\"\nmsgstr \"\""));

    // A message shared by both documents appears in each document catalog.
    assert!(index.contains("msgid \"A shared greeting.\""));
    assert!(about.contains("msgid \"A shared greeting.\""));
    assert!(index.contains("#: index"));
    assert!(about.contains("#: about"));

    // A message unique to one document also appears.
    assert!(about.contains("msgid \"Something unique to About.\""));
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
    let pot = std::fs::read_to_string(out.path().join("index.pot")).unwrap();
    assert!(pot.contains("msgid \"Hello there.\""));
}

#[test]
fn compact_false_writes_nested_document_catalog_paths() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::create_dir_all(src.path().join("guide")).unwrap();
    std::fs::write(
        src.path().join("guide").join("intro.rst"),
        "Intro\n=====\n\nNested message.\n",
    )
    .unwrap();

    let mut config = SphinxConfig::new_defaults();
    config.set("gettext_compact", ConfigVal::Bool(false));
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, src.path(), out.path());
    GettextBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let pot = out.path().join("guide").join("intro.pot");
    assert!(pot.exists());
    assert!(std::fs::read_to_string(pot)
        .unwrap()
        .contains("msgid \"Nested message.\""));
}
