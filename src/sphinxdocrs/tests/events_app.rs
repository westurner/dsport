//! **H4a/H4b/H4c** — event emission order, and a Python `load_extension`
//! round trip.
//!
//! Mirrors the port plan's H4 gate: "tests/events_app.rs asserting
//! emission order, plus a Python round-trip test that loads a trivial
//! extension and observes its config-inited callback firing."

use std::collections::HashMap;

use pyo3::types::PyAnyMethods;
use sphinxdocrs::application::SphinxApp;
use tempfile::TempDir;

fn make_src() -> TempDir {
    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    tmp
}

#[test]
fn core_event_emission_order() {
    let src = make_src();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    app.build().unwrap();

    let mgr = app.events.borrow();
    let log = mgr.emitted();

    // Fixed-position events.
    assert_eq!(log[0], "config-inited");
    assert_eq!(log[1], "builder-inited");
    assert_eq!(log.last().unwrap(), "build-finished");

    // Every core event fired at least once, in upstream relative order.
    let expected_order = [
        "config-inited",
        "builder-inited",
        "env-get-outdated",
        "env-before-read-docs",
        "source-read",
        "doctree-read",
        "env-updated",
        "env-check-consistency",
        "build-finished",
    ];
    let mut last_pos = 0usize;
    for name in expected_order {
        let pos = log
            .iter()
            .position(|n| n == name)
            .unwrap_or_else(|| panic!("event {name:?} was never emitted; full log: {log:?}"));
        assert!(
            pos >= last_pos,
            "event {name:?} fired out of upstream order; full log: {log:?}"
        );
        last_pos = pos;
    }
}

#[test]
fn source_read_and_doctree_read_fire_per_document() {
    let src = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Index\n=====\n\nBody.\n").unwrap();
    std::fs::write(src.path().join("other.rst"), "Other\n=====\n\nBody.\n").unwrap();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    app.build().unwrap();

    let mgr = app.events.borrow();
    let log = mgr.emitted();
    let source_read_count = log.iter().filter(|n| *n == "source-read").count();
    let doctree_read_count = log.iter().filter(|n| *n == "doctree-read").count();
    assert_eq!(source_read_count, 2);
    assert_eq!(doctree_read_count, 2);
}

#[test]
fn load_extension_config_inited_round_trip() {
    // A trivial Python extension: connects a listener for `config-inited`
    // that records that it fired.
    let pydir = TempDir::new().unwrap();
    std::fs::write(
        pydir.path().join("h4b_trivial_ext.py"),
        r#"
called = []

def setup(app):
    def on_config_inited(app):
        called.append(True)

    app.connect("config-inited", on_config_inited)
    return {"version": "0.1", "parallel_read_safe": True}
"#,
    )
    .unwrap();

    let src = make_src();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    pyo3::Python::attach(|py| {
        let sys = py.import("sys").unwrap();
        let path = sys.getattr("path").unwrap();
        path.call_method1("insert", (0, pydir.path().to_str().unwrap()))
            .unwrap();
    });

    app.load_extension("h4b_trivial_ext").unwrap();
    assert!(app.extensions.contains_key("h4b_trivial_ext"));

    app.build().unwrap();

    pyo3::Python::attach(|py| {
        let module = py.import("h4b_trivial_ext").unwrap();
        let called: Vec<bool> = module.getattr("called").unwrap().extract().unwrap();
        assert_eq!(called, vec![true], "config-inited listener never fired");
    });
}

#[test]
fn load_extension_unknown_module_errors() {
    let src = make_src();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    let err = app
        .load_extension("this_module_does_not_exist_h4b")
        .unwrap_err();
    assert!(matches!(
        err,
        sphinxdocrs::application::AppError::Extension(_)
    ));
}
