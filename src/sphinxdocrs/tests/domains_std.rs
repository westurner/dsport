//! Integration tests for the `std` domain (Tier **H3a**) end-to-end
//! through `BuildEnvironment::read_all` + `resolve_references` (**H5b**/**H5c**).
//!
//! Mirrors the resolvable subset of `sphinx/tests/test_domains/test_domain_std.py`
//! adapted to this port's text-scan accepted deviation (see
//! `sphinxdocrs::domains`' module doc for why).

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

// ── labels / `:ref:` ───────────────────────────────────────────────────────────

#[test]
fn read_all_registers_explicit_target_with_section_as_label() {
    let (_src, _dt, mut env) = make_disk_env(&[(
        "index",
        ".. _my-label:\n\nMy Section\n===========\n\nSee :ref:`my-label`.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    assert_eq!(
        env.std_domain.labels.get("my-label"),
        Some(&(
            "index".to_string(),
            "my-label".to_string(),
            "My Section".to_string()
        ))
    );
}

#[test]
fn resolve_ref_to_named_label_uses_section_title() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("guide", ".. _setup-label:\n\nSetup\n=====\n\nBody.\n"),
        ("index", "See :ref:`setup-label` for details.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        XrefResolution::Resolved { target, .. } => {
            assert_eq!(target.docname, "guide");
            assert_eq!(target.anchor, "setup-label");
            assert_eq!(target.title, "Setup");
        }
        other => panic!("expected Resolved, got {other:?}"),
    }
}

#[test]
fn resolve_ref_with_explicit_title_overrides_display_title() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("guide", ".. _setup-label:\n\nSetup\n=====\n"),
        (
            "index",
            "See :ref:`Custom Title <setup-label>` for details.\n",
        ),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    match &resolved[0] {
        XrefResolution::Resolved { target, .. } => {
            assert_eq!(target.title, "Custom Title");
        }
        other => panic!("expected Resolved, got {other:?}"),
    }
}

#[test]
fn resolve_ref_undefined_label_warns() {
    let (_src, _dt, mut env) = make_disk_env(&[("index", "See :ref:`nowhere` here.\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        XrefResolution::Unresolved { warning, .. } => {
            assert_eq!(warning, "undefined label: 'nowhere'");
        }
        other => panic!("expected Unresolved, got {other:?}"),
    }
}

#[test]
fn target_without_following_section_has_no_named_label() {
    let (_src, _dt, mut env) = make_disk_env(&[(
        "index",
        ".. _standalone:\n\nJust a plain paragraph, not a heading.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    // No sectionname was recognized, so the plain `:ref:` lookup (which
    // requires `labels`, not just `anonlabels`) must fail — mirrors
    // upstream requiring an explicit title for non-section targets.
    assert!(!env.std_domain.labels.contains_key("standalone"));
    assert!(env.std_domain.anonlabels.contains_key("standalone"));
}

// ── documents / `:doc:` ────────────────────────────────────────────────────────

#[test]
fn resolve_doc_relative_target() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("guide/setup", "Setup\n=====\n"),
        ("guide/intro", "See :doc:`setup` for install steps.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("guide/intro");
    match &resolved[0] {
        XrefResolution::Resolved { target, .. } => {
            assert_eq!(target.docname, "guide/setup");
            assert_eq!(target.title, "Setup");
        }
        other => panic!("expected Resolved, got {other:?}"),
    }
}

#[test]
fn resolve_doc_unknown_document_warns() {
    let (_src, _dt, mut env) = make_disk_env(&[("index", "See :doc:`nowhere` for details.\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    match &resolved[0] {
        XrefResolution::Unresolved { warning, .. } => {
            assert_eq!(warning, "unknown document: 'nowhere'");
        }
        other => panic!("expected Unresolved, got {other:?}"),
    }
}

// ── glossary / `:term:` ────────────────────────────────────────────────────────

#[test]
fn read_all_registers_glossary_terms() {
    let (_src, _dt, mut env) = make_disk_env(&[(
        "glossary",
        ".. glossary::\n\n   Widget\n      A small reusable thing.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    assert!(env.std_domain.terms.contains_key("widget"));
}

#[test]
fn resolve_term_case_insensitive() {
    let (_src, _dt, mut env) = make_disk_env(&[
        (
            "glossary",
            ".. glossary::\n\n   Widget\n      A small reusable thing.\n",
        ),
        ("index", "See :term:`widget` in the glossary.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    match &resolved[0] {
        XrefResolution::Resolved { target, .. } => {
            assert_eq!(target.docname, "glossary");
        }
        other => panic!("expected Resolved, got {other:?}"),
    }
}

#[test]
fn resolve_term_not_in_glossary_warns() {
    let (_src, _dt, mut env) = make_disk_env(&[("index", "See :term:`unknown` here.\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    match &resolved[0] {
        XrefResolution::Unresolved { warning, .. } => {
            assert_eq!(warning, "term not in glossary: 'unknown'");
        }
        other => panic!("expected Unresolved, got {other:?}"),
    }
}

// ── re-reading a document clears stale data ────────────────────────────────────

#[test]
fn rereading_document_clears_stale_labels() {
    let (src, _dt, mut env) =
        make_disk_env(&[("index", ".. _old-label:\n\nOld Section\n===========\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();
    assert!(env.std_domain.labels.contains_key("old-label"));

    std::fs::write(
        src.path().join("index.rst"),
        ".. _new-label:\n\nNew Section\n===========\n",
    )
    .unwrap();
    env.read_all().unwrap();

    assert!(!env.std_domain.labels.contains_key("old-label"));
    assert!(env.std_domain.labels.contains_key("new-label"));
}

// ── virtual labels ─────────────────────────────────────────────────────────────

#[test]
fn genindex_modindex_search_are_seeded() {
    let (_src, _dt, env) = make_disk_env(&[]);
    assert!(env.std_domain.labels.contains_key("genindex"));
    assert!(env.std_domain.labels.contains_key("modindex"));
    assert!(env.std_domain.labels.contains_key("search"));
}
