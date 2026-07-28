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

    // `SphinxApp::new` auto-loads every entry in `conf.py`'s `extensions`
    // list (mirroring `Sphinx.__init__`) and fires `config-inited`
    // immediately afterward -- so the extension directory must be on
    // `sys.path` *before* `new()` runs, and the extension must be declared
    // via `conf.py` rather than loaded manually afterward (a manual
    // `load_extension` call made after `new()` returns would, like
    // upstream, miss `config-inited` entirely).
    pyo3::Python::attach(|py| {
        let sys = py.import("sys").unwrap();
        let path = sys.getattr("path").unwrap();
        path.call_method1("insert", (0, pydir.path().to_str().unwrap()))
            .unwrap();
    });

    let src = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("conf.py"),
        "extensions = ['h4b_trivial_ext']\n",
    )
    .unwrap();

    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    assert!(app.extensions.contains_key("h4b_trivial_ext"));
    assert_eq!(
        app.extension_sources.get("h4b_trivial_ext"),
        Some(&"python")
    );

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

/// **ADR 0005** round trip: when a Rust equivalent is registered for the
/// extension name being loaded, `load_extension` must call the
/// equivalent's `factory()` result instead of ever importing the Python
/// extension module or calling its `setup()`.
#[test]
fn load_extension_prefers_rust_equivalent_when_registered() {
    let pydir = TempDir::new().unwrap();
    // The Rust-equivalent implementation: `factory()` returns `rust_setup`
    // without calling it, matching `docutilsrs_plugins.Equivalent.factory`'s
    // `Callable[[], Any]` contract.
    std::fs::write(
        pydir.path().join("h4_rust_equiv_impl.py"),
        r#"
called_via_rust_equivalent = []

def rust_setup(app):
    called_via_rust_equivalent.append(True)
    return {"version": "rust-0.1"}

def factory():
    return rust_setup
"#,
    )
    .unwrap();
    // The "real" Python extension -- must NOT be invoked once a compatible
    // Rust equivalent is registered for its name.
    std::fs::write(
        pydir.path().join("h4_ext_with_rust_equivalent.py"),
        r#"
called_via_python = []

def setup(app):
    called_via_python.append(True)
    return {}
"#,
    )
    .unwrap();

    let src = make_src();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    // `docutilsrs_plugins` lives alongside the docutilsrs crate, not on the
    // default sys.path -- mirrors the `HYBRID_DIR` convention used by the
    // Python-side tests for the same module.
    let resolver_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docutilsrs")
        .join("python");

    pyo3::Python::attach(|py| {
        let sys = py.import("sys").unwrap();
        let path = sys.getattr("path").unwrap();
        path.call_method1("insert", (0, pydir.path().to_str().unwrap()))
            .unwrap();
        path.call_method1("insert", (0, resolver_dir.to_str().unwrap()))
            .unwrap();

        let plugins = py.import("docutilsrs_plugins").unwrap();
        let impl_module = py.import("h4_rust_equiv_impl").unwrap();
        let factory = impl_module.getattr("factory").unwrap();
        plugins
            .call_method1("register", ("h4_ext_with_rust_equivalent", factory))
            .unwrap();
    });

    app.load_extension("h4_ext_with_rust_equivalent").unwrap();
    assert_eq!(
        app.extension_sources.get("h4_ext_with_rust_equivalent"),
        Some(&"rust")
    );

    pyo3::Python::attach(|py| {
        let rust_impl = py.import("h4_rust_equiv_impl").unwrap();
        let called_rust: Vec<bool> = rust_impl
            .getattr("called_via_rust_equivalent")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            called_rust,
            vec![true],
            "the registered Rust equivalent was never invoked"
        );

        let py_ext = py.import("h4_ext_with_rust_equivalent").unwrap();
        let called_py: Vec<bool> = py_ext
            .getattr("called_via_python")
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            called_py.is_empty(),
            "the Python setup() should have been skipped entirely"
        );

        // Don't leak this registration into other tests in the same process.
        py.import("docutilsrs_plugins")
            .unwrap()
            .call_method0("clear_registry")
            .unwrap();
    });
}

/// **ADR 0005** version guard: when a Rust equivalent is registered but its
/// `upstream_requires` specifier rejects the installed version,
/// `load_extension` must fall back to the plain Python extension (never
/// call the equivalent's callable) and record a warning explaining why.
#[test]
fn load_extension_falls_back_when_version_guard_rejects() {
    let pydir = TempDir::new().unwrap();
    // Doubles as (a) the "upstream package" the version guard checks (via
    // its `__version__`) and (b) the real Sphinx extension to fall back to.
    std::fs::write(
        pydir.path().join("h4_version_guard_ext.py"),
        r#"
__version__ = "0.1.0"
called_via_python_fallback = []

def setup(app):
    called_via_python_fallback.append(True)
    return {}

def factory():
    return setup
"#,
    )
    .unwrap();

    let src = make_src();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    let resolver_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docutilsrs")
        .join("python");

    pyo3::Python::attach(|py| {
        let sys = py.import("sys").unwrap();
        let path = sys.getattr("path").unwrap();
        path.call_method1("insert", (0, pydir.path().to_str().unwrap()))
            .unwrap();
        path.call_method1("insert", (0, resolver_dir.to_str().unwrap()))
            .unwrap();

        let plugins = py.import("docutilsrs_plugins").unwrap();
        let ext_module = py.import("h4_version_guard_ext").unwrap();
        let factory = ext_module.getattr("factory").unwrap();
        let kwargs = pyo3::types::PyDict::new(py);
        // Impossible to satisfy against the module's own `__version__ = "0.1.0"`.
        kwargs.set_item("upstream_requires", ">=99.0").unwrap();
        plugins
            .call_method("register", ("h4_version_guard_ext", factory), Some(&kwargs))
            .unwrap();
    });

    app.load_extension("h4_version_guard_ext").unwrap();

    assert_eq!(
        app.extension_sources.get("h4_version_guard_ext"),
        Some(&"python"),
        "a version-guard-rejected equivalent must fall back to the Python path"
    );
    assert!(
        app.warnings
            .iter()
            .any(|w| w.contains("h4_version_guard_ext") && w.contains("version guard")),
        "expected a version-guard warning; got: {:?}",
        app.warnings
    );

    pyo3::Python::attach(|py| {
        let m = py.import("h4_version_guard_ext").unwrap();
        let called: Vec<bool> = m
            .getattr("called_via_python_fallback")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(
            called,
            vec![true],
            "the Python fallback setup() was never invoked"
        );

        py.import("docutilsrs_plugins")
            .unwrap()
            .call_method0("clear_registry")
            .unwrap();
    });
}

/// `needs_extensions`: a required-but-unloaded extension is a non-fatal
/// warning, not a construction failure (mirrors upstream's `logger.warning`).
#[test]
fn needs_extensions_warns_when_required_extension_not_loaded() {
    let src = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("conf.py"),
        "needs_extensions = {'h4_missing_ext': '1.0'}\n",
    )
    .unwrap();

    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let app = SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    assert!(
        app.warnings.iter().any(|w| w.contains("h4_missing_ext")
            && w.contains("needs_extensions")
            && w.contains("not loaded")),
        "expected a needs_extensions warning; got: {:?}",
        app.warnings
    );
}

/// `needs_extensions`: a loaded extension whose version doesn't satisfy the
/// requirement is fatal (mirrors upstream's `VersionRequirementError`).
#[test]
fn needs_extensions_errors_on_version_mismatch() {
    let pydir = TempDir::new().unwrap();
    std::fs::write(
        pydir.path().join("h4_needs_ext.py"),
        r#"
def setup(app):
    return {"version": "0.1"}
"#,
    )
    .unwrap();

    pyo3::Python::attach(|py| {
        let sys = py.import("sys").unwrap();
        sys.getattr("path")
            .unwrap()
            .call_method1("insert", (0, pydir.path().to_str().unwrap()))
            .unwrap();
    });

    let src = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("conf.py"),
        "extensions = ['h4_needs_ext']\nneeds_extensions = {'h4_needs_ext': '1.0'}\n",
    )
    .unwrap();

    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let err =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap_err();
    assert!(matches!(
        err,
        sphinxdocrs::application::AppError::Extension(_)
    ));
    if let sphinxdocrs::application::AppError::Extension(msg) = err {
        assert!(
            msg.contains("h4_needs_ext") && msg.contains("1.0"),
            "unexpected error message: {msg}"
        );
    }
}
