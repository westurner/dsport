//! Integration tests for the `py` domain (Tier **H3b**) end-to-end
//! through `BuildEnvironment::read_all` + `resolve_references`.
//!
//! Mirrors the resolvable subset of `sphinx/tests/test_domains/test_domain_py.py`.

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
fn read_all_registers_module_function_and_class() {
    let (_src, _dt, mut env) = make_disk_env(&[(
        "api",
        ".. py:module:: pkg.widgets\n\n.. py:function:: greet(name, loud=False)\n\n   Say hello.\n\n.. py:class:: Widget(Base)\n\n   .. py:method:: render(self)\n\n      Render the widget.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    assert!(
        env.py_domain
            .objects
            .contains_key(&("module".to_string(), "pkg.widgets".to_string()))
    );
    let (docname, _anchor, sig) = env
        .py_domain
        .objects
        .get(&("function".to_string(), "pkg.widgets.greet".to_string()))
        .unwrap();
    assert_eq!(docname, "api");
    assert_eq!(sig, "name, loud=False");

    assert!(
        env.py_domain
            .objects
            .contains_key(&("class".to_string(), "pkg.widgets.Widget".to_string()))
    );
    assert!(env.py_domain.objects.contains_key(&(
        "method".to_string(),
        "pkg.widgets.Widget.render".to_string()
    )));
}

#[test]
fn resolve_py_func_xref_across_documents() {
    let (_src, _dt, mut env) = make_disk_env(&[
        (
            "api",
            ".. py:module:: pkg\n\n.. py:function:: greet(name)\n\n   Say hello.\n",
        ),
        ("index", "Call :py:func:`pkg.greet` to say hello.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        XrefResolution::Resolved { target, .. } => {
            assert_eq!(target.docname, "api");
        }
        other => panic!("expected Resolved, got {other:?}"),
    }
}

#[test]
fn resolve_py_meth_tilde_shortens_title() {
    let (_src, _dt, mut env) = make_disk_env(&[
        (
            "api",
            ".. py:module:: pkg\n\n.. py:class:: Widget\n\n   .. py:method:: render(self)\n",
        ),
        ("index", "See :py:meth:`~pkg.Widget.render` for details.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        XrefResolution::Resolved { target, .. } => {
            assert_eq!(target.docname, "api");
            assert_eq!(target.title, "render");
        }
        other => panic!("expected Resolved, got {other:?}"),
    }
}

#[test]
fn resolve_unknown_py_func_warns() {
    let (_src, _dt, mut env) = make_disk_env(&[("index", "See :py:func:`nope.missing` here.\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let resolved = env.resolve_references("index");
    assert_eq!(resolved.len(), 1);
    match &resolved[0] {
        XrefResolution::Unresolved { warning, .. } => {
            assert!(warning.contains("nope.missing"));
        }
        other => panic!("expected Unresolved, got {other:?}"),
    }
}

#[test]
fn rereading_document_clears_stale_py_objects() {
    let (src, _dt, mut env) = make_disk_env(&[("api", ".. py:function:: old_name()\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();
    assert!(
        env.py_domain
            .objects
            .contains_key(&("function".to_string(), "old_name".to_string()))
    );

    std::fs::write(src.path().join("api.rst"), ".. py:function:: new_name()\n").unwrap();
    env.read_all().unwrap();

    assert!(
        !env.py_domain
            .objects
            .contains_key(&("function".to_string(), "old_name".to_string()))
    );
    assert!(
        env.py_domain
            .objects
            .contains_key(&("function".to_string(), "new_name".to_string()))
    );
}
