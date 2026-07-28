//! Integration tests for `sphinxdocrs::builders::changes::ChangesBuilder`
//! (H7d, `changes` builder only — see the plan doc for `epub`/`texinfo`,
//! which remain out of scope for this pass).
//!
//! Complements the inline `#[cfg(test)]` unit tests in
//! `src/builders/changes.rs` with a full `SphinxApp::build()` dispatch
//! and a multi-document project exercising version grouping/ordering.

use std::collections::HashMap;

use tempfile::TempDir;

use sphinxdocrs::application::SphinxApp;
use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::changes::ChangesBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn builder_identity() {
    let b = ChangesBuilder::new();
    assert_eq!(b.name(), "changes");
    assert_eq!(b.get_target_uri("index"), "");
}

#[test]
fn build_all_over_a_multi_document_project_groups_by_version() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\n.. versionadded:: 1.0\n   The initial release.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("api.rst"),
        "API\n===\n\n.. versionchanged:: 1.1\n   Renamed a parameter.\n\n.. deprecated:: 2.0\n   Use the new API instead.\n",
    )
    .unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, src.path(), out.path());
    let result = ChangesBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 2);

    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(html.contains("The initial release."));
    assert!(html.contains("Renamed a parameter."));
    assert!(html.contains("Use the new API instead."));
    // Newest version first.
    let pos_2_0 = html.find("Changes in version 2.0").unwrap();
    let pos_1_1 = html.find("Changes in version 1.1").unwrap();
    let pos_1_0 = html.find("Changes in version 1.0").unwrap();
    assert!(pos_2_0 < pos_1_1);
    assert!(pos_1_1 < pos_1_0);
}

#[test]
fn sphinx_app_build_dispatches_to_changes() {
    let src = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\n.. versionadded:: 1.0\n   Hello.\n",
    )
    .unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();

    let mut app = SphinxApp::new(
        src.path(),
        out.path(),
        doctrees.path(),
        "changes",
        HashMap::new(),
    )
    .unwrap();
    let result = app.build().unwrap();
    assert_eq!(result.written, 1);
    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(html.contains("Changes in version 1.0"));
}
