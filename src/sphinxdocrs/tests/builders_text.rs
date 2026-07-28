//! Integration tests for `sphinxdocrs::builders::text::TextBuilder` (H7a).
//!
//! Complements the inline `#[cfg(test)]` unit tests in
//! `src/builders/text.rs` with a black-box, multi-document project build
//! exercised through the public `Builder` trait and the H2 two-phase
//! (`find_files` + `read_all`) pipeline.

use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::text::TextBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn builder_identity() {
    let b = TextBuilder::new();
    assert_eq!(b.name(), "text");
    assert_eq!(b.format(), "text");
    assert_eq!(b.out_suffix(), ".txt");
}

#[test]
fn build_all_over_a_multi_document_project_with_subdirectory() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::create_dir(src.path().join("guide")).unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage **content**.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("guide/intro.rst"),
        "Intro\n=====\n\n- one\n- two\n",
    )
    .unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();

    let result = TextBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 2);

    let index_txt = std::fs::read_to_string(out.path().join("index.txt")).unwrap();
    assert!(index_txt.starts_with("Welcome\n=======\n"));
    assert!(index_txt.contains("Homepage **content**."));

    let intro_txt = std::fs::read_to_string(out.path().join("guide/intro.txt")).unwrap();
    assert!(intro_txt.contains("- one"));
    assert!(intro_txt.contains("- two"));
}

#[test]
fn get_target_uri_matches_upstream_textbuilder_always_empty() {
    // Mirrors `TextBuilder.get_target_uri`, which always returns `""`
    // (text output has no meaningful cross-document links).
    let b = TextBuilder::new();
    for docname in ["index", "guide/intro", "a/b/c"] {
        assert_eq!(b.get_target_uri(docname), "");
    }
}
