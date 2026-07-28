//! Integration tests for the `js` domain (Tier **H3e**) end-to-end
//! through `BuildEnvironment::read_all` + `resolve_references`.
//!
//! Mirrors the resolvable subset of `sphinx/tests/test_domains/test_domain_js.py`.

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
        ".. js:module:: widgets\n\n.. js:function:: render(el, opts)\n\n   Render an element.\n\n.. js:class:: Widget()\n\n   .. js:method:: destroy()\n\n      Tear it down.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    assert!(
        env.js_domain
            .objects
            .contains_key(&("module".to_string(), "widgets".to_string()))
    );
    let (docname, _anchor, sig) = env
        .js_domain
        .objects
        .get(&("function".to_string(), "widgets.render".to_string()))
        .unwrap();
    assert_eq!(docname, "api");
    assert_eq!(sig, "el, opts");
    assert!(
        env.js_domain
            .objects
            .contains_key(&("method".to_string(), "widgets.Widget.destroy".to_string()))
    );
}

#[test]
fn resolve_js_func_xref_across_documents() {
    let (_src, _dt, mut env) = make_disk_env(&[
        (
            "api",
            ".. js:module:: widgets\n\n.. js:function:: render(el)\n",
        ),
        ("index", "Call :js:func:`widgets.render` to draw it.\n"),
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
fn resolve_unknown_js_func_warns() {
    let (_src, _dt, mut env) = make_disk_env(&[("index", "See :js:func:`nope.missing` here.\n")]);
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
