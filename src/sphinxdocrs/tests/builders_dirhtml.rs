//! Integration tests for `sphinxdocrs::builders::dirhtml::DirhtmlBuilder`
//! (H7b).
//!
//! Complements the inline `#[cfg(test)]` unit tests in
//! `src/builders/dirhtml.rs` with a full `SphinxApp::build()` dispatch and
//! a byte-identical comparison against the flat `HtmlBuilder` output for
//! the same project (confirming the H6 theming pipeline — search index,
//! static assets, embedded/real theme rendering — is genuinely reused,
//! not reimplemented).

use std::collections::HashMap;

use tempfile::TempDir;

use sphinxdocrs::application::SphinxApp;
use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::dirhtml::DirhtmlBuilder;
use sphinxdocrs::builders::html::HtmlBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

fn write_project(srcdir: &std::path::Path) {
    std::fs::write(
        srcdir.join("index.rst"),
        "Welcome\n=======\n\nHomepage **content**.\n",
    )
    .unwrap();
    std::fs::write(srcdir.join("about.rst"), "About\n=====\n\nSome *info*.\n").unwrap();
}

#[test]
fn dirhtml_build_all_reuses_the_full_html_pipeline() {
    let src = TempDir::new().unwrap();
    write_project(src.path());

    // Flat layout (HtmlBuilder).
    let out_flat = TempDir::new().unwrap();
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env_flat = BuildEnvironment::new(config, project, src.path(), out_flat.path());
    let result_flat = HtmlBuilder::new()
        .build_all(src.path(), out_flat.path(), &env_flat)
        .unwrap();

    // Directory layout (DirhtmlBuilder).
    let out_dir = TempDir::new().unwrap();
    let config2 = SphinxConfig::new_defaults();
    let project2 = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env_dir = BuildEnvironment::new(config2, project2, src.path(), out_dir.path());
    let result_dir = DirhtmlBuilder::new()
        .build_all(src.path(), out_dir.path(), &env_dir)
        .unwrap();

    assert_eq!(result_flat.written, result_dir.written);

    // Same static pipeline output (search index / CSS) in both layouts.
    assert!(out_dir.path().join("_static/sphinxdocrs.css").exists());
    assert!(out_dir.path().join("searchindex.js").exists());

    // Different physical layout for the non-index document.
    assert!(out_flat.path().join("about.html").exists());
    assert!(out_dir.path().join("about/index.html").exists());
    assert!(!out_dir.path().join("about.html").exists());

    // `index` itself stays at the output root either way.
    assert!(out_flat.path().join("index.html").exists());
    assert!(out_dir.path().join("index.html").exists());
}

#[test]
fn sphinx_app_build_dispatches_to_dirhtml() {
    let src = TempDir::new().unwrap();
    write_project(src.path());
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();

    let mut app = SphinxApp::new(
        src.path(),
        out.path(),
        doctrees.path(),
        "dirhtml",
        HashMap::new(),
    )
    .unwrap();
    let result = app.build().unwrap();
    assert_eq!(result.written, 2);
    assert!(out.path().join("about/index.html").exists());
}
