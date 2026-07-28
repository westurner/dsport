//! Integration tests for the `rst` domain (Tier **H3c**) end-to-end
//! through `BuildEnvironment::read_all` + `resolve_references`.
//!
//! Mirrors the resolvable subset of `sphinx/tests/test_domains/test_domain_rst.py`.

use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::domains::XrefResolution;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};
use tempfile::TempDir;

fn make_disk_env(files: &[(&str, &str)]) -> (TempDir, TempDir, BuildEnvironment) {
    let src = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    for (docname, source) in files {
        let path = src.path().join(format!("{docname}.rst"));
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, source).unwrap();
    }
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    (src, doctrees, env)
}

#[test]
fn read_all_registers_rst_directive_and_role_objects() {
    let (_src, _dt, mut env) = make_disk_env(&[(
        "usage/directives",
        ".. rst:directive:: toctree\n\n   Body.\n\n.. rst:role:: ref\n\n   Body.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    assert!(
        env.rst_domain
            .objects
            .contains_key(&("directive".to_string(), "toctree".to_string()))
    );
    assert!(
        env.rst_domain
            .objects
            .contains_key(&("role".to_string(), "ref".to_string()))
    );
}

#[test]
fn resolve_rst_dir_xref_across_documents() {
    let (_src, _dt, mut env) = make_disk_env(&[
        (
            "usage/directives",
            ".. rst:directive:: toctree\n\n   Build a table of contents.\n",
        ),
        ("index", "Use :rst:dir:`toctree` to build a tree.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        XrefResolution::Resolved { target, .. } => {
            assert_eq!(target.docname, "usage/directives");
        }
        other => panic!("expected Resolved, got {other:?}"),
    }
}

#[test]
fn resolve_rst_role_xref() {
    let (_src, _dt, mut env) = make_disk_env(&[
        (
            "usage/roles",
            ".. rst:role:: ref\n\n   Cross-reference role.\n",
        ),
        ("index", "See :rst:role:`ref` for details.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    match &resolved[0] {
        XrefResolution::Resolved { target, .. } => {
            assert_eq!(target.docname, "usage/roles");
        }
        other => panic!("expected Resolved, got {other:?}"),
    }
}

#[test]
fn resolve_unknown_rst_dir_warns() {
    let (_src, _dt, mut env) = make_disk_env(&[("index", "Use :rst:dir:`nope` here.\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    match &resolved[0] {
        XrefResolution::Unresolved { warning, .. } => {
            assert_eq!(warning, "undefined rst dir: 'nope'");
        }
        other => panic!("expected Unresolved, got {other:?}"),
    }
}
