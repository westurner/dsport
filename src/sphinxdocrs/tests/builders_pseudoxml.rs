//! Integration tests for
//! `sphinxdocrs::builders::pseudoxml::PseudoxmlBuilder` (H7a).
//!
//! Complements the inline `#[cfg(test)]` unit tests in
//! `src/builders/pseudoxml.rs` with a black-box, multi-document project
//! build and a byte-for-byte cross-check against `docutilsrs::pseudo_xml`
//! called directly (confirming the builder is a genuinely thin wrapper,
//! with no divergence introduced by the file-writing layer).

use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::pseudoxml::PseudoxmlBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn builder_identity() {
    let b = PseudoxmlBuilder::new();
    assert_eq!(b.name(), "pseudoxml");
    assert_eq!(b.format(), "pseudoxml");
    assert_eq!(b.out_suffix(), ".pseudoxml");
}

#[test]
fn build_doc_output_matches_docutilsrs_pseudo_xml_directly() {
    let tmp = TempDir::new().unwrap();
    let source = "Title\n=====\n\nA *emphasized* and **strong** paragraph.\n";
    PseudoxmlBuilder::new()
        .build_doc("index", source, tmp.path())
        .unwrap();
    let written = std::fs::read_to_string(tmp.path().join("index.pseudoxml")).unwrap();

    let tree = docutilsrs::parse_rst_with_source(source, "index");
    let expected = docutilsrs::pseudo_xml(&tree);
    assert_eq!(written, expected);
}

#[test]
fn build_all_over_a_multi_document_project_with_subdirectory() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::create_dir(src.path().join("guide")).unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    std::fs::write(src.path().join("guide/intro.rst"), "Intro\n=====\n\nBody.\n").unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();

    let result = PseudoxmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 2);
    assert!(out.path().join("index.pseudoxml").exists());
    assert!(out.path().join("guide/intro.pseudoxml").exists());
}

#[test]
fn get_target_uri_matches_upstream_pseudoxmlbuilder() {
    let b = PseudoxmlBuilder::new();
    assert_eq!(b.get_target_uri("index"), "index.pseudoxml");
    assert_eq!(b.get_target_uri("guide/intro"), "guide/intro.pseudoxml");
}
