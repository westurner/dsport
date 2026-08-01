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

/// The `docutilsrs_plugins` registry (and the `sys.path` entries the ADR
/// 0005 tests insert to reach it) are process-global, so two tests that
/// register/`clear_registry()` it must not run concurrently — the test
/// harness runs tests in parallel by default, and without this guard one
/// test's `clear_registry()` can race ahead of another's still-in-flight
/// `register()` → `load_extension()` sequence, wiping its registration out
/// from under it (this is what made
/// `load_extension_falls_back_when_version_guard_rejects` flaky). Mirrors
/// the `REGISTRY_LOCK` pattern in `tests/locale.rs`. Poisoning is ignored:
/// a panicking test leaves the registry dirty, and the next test clears it
/// via `clear_registry()` anyway.
static PLUGIN_REGISTRY_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Acquire [`PLUGIN_REGISTRY_LOCK`], returning a guard that must be held
/// for the duration of any test touching the `docutilsrs_plugins` registry.
fn locked_plugin_registry() -> std::sync::MutexGuard<'static, ()> {
    PLUGIN_REGISTRY_LOCK
        .lock()
        .unwrap_or_else(|e| e.into_inner())
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
    def on_config_inited(app, config):
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

/// **H4c** round trip: `app.config` reads/writes, `add_config_value`
/// establishing a default only when absent, and `add_css_file`/
/// `add_js_file` registrations actually reaching the rendered HTML output
/// (via the real `alabaster` theme -- the default `html_theme` -- since
/// the embedded placeholder theme fallback does not consult registered
/// assets at all).
#[test]
fn load_extension_config_and_assets_round_trip() {
    let pydir = TempDir::new().unwrap();
    std::fs::write(
        pydir.path().join("h4c_assets_ext.py"),
        r#"
snapshot = {}

def setup(app):
    # `add_config_value` establishes a default only if not already set.
    app.add_config_value("h4c_flag", True, "html")
    snapshot["flag_before"] = app.config.h4c_flag

    # `app.config.extensions` must be the *real*, mutable extensions list.
    assert "h4c_assets_ext" in app.config.extensions
    app.config.extensions.append("h4c_dummy_marker")
    snapshot["extensions_after"] = list(app.config.extensions)

    # Arbitrary attribute set/read (Config doesn't override __setattr__
    # upstream either).
    app.config.h4c_custom = "hello"
    snapshot["custom_after"] = app.config.h4c_custom

    # A name nobody ever set/registered must not raise AttributeError.
    snapshot["unset_is_none"] = app.config.h4c_never_set is None

    app.add_css_file("h4c_extra.css")
    app.add_js_file("h4c_extra.js", **{"data-x": "1"})

    return {"version": "0.1", "parallel_read_safe": True}
"#,
    )
    .unwrap();

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
        "extensions = ['h4c_assets_ext']\n",
    )
    .unwrap();

    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    app.build().unwrap();

    pyo3::Python::attach(|py| {
        let module = py.import("h4c_assets_ext").unwrap();
        let snapshot = module.getattr("snapshot").unwrap();
        let flag_before: bool = snapshot.get_item("flag_before").unwrap().extract().unwrap();
        assert!(
            flag_before,
            "add_config_value default not visible via app.config"
        );

        let extensions_after: Vec<String> = snapshot
            .get_item("extensions_after")
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            extensions_after.contains(&"h4c_dummy_marker".to_string()),
            "app.config.extensions.append(...) did not mutate the real list: {extensions_after:?}"
        );

        let custom_after: String = snapshot
            .get_item("custom_after")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(custom_after, "hello");

        let unset_is_none: bool = snapshot
            .get_item("unset_is_none")
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            unset_is_none,
            "reading an unset config name should yield None"
        );
    });

    // `SphinxApp::py_config` seeded `extensions` from `conf.py` at
    // construction time -- the same live list objects the Python side
    // mutated, so the Rust-side accumulation must see it too via the
    // shared `assets` registry (checked indirectly through the rendered
    // page below), and the config store itself must retain the write.
    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(
        html.contains("h4c_extra.css"),
        "add_css_file's registered stylesheet is missing from rendered HTML:\n{html}"
    );
    assert!(
        html.contains("h4c_extra.js"),
        "add_js_file's registered script is missing from rendered HTML:\n{html}"
    );
}

/// `app.add_directive`/`app.add_role`: class registration remains visible in
/// the component registry, while a callable directive enters the docutilsrs
/// replacement-RST bridge and executes during parsing.
#[test]
fn load_extension_add_directive_and_add_role_recorded() {
    let pydir = TempDir::new().unwrap();
    std::fs::write(
        pydir.path().join("h5_directive_role_ext.py"),
        r#"
from docutils import nodes
from docutils.parsers.rst import Directive, directives


class MyNode(nodes.Element):
    pass


class MyDirective(Directive):
    has_content = True
    option_spec = {"count": directives.nonnegative_int}

    def run(self):
        return [
            nodes.paragraph(text="Class directive: " + self.arguments[0] + " x" + str(self.options["count"])),
            MyNode(),
        ]


def my_role(name, rawtext, text, lineno, inliner, options=None, content=None):
    return [
        nodes.strong(rawtext, "Custom role: "),
        nodes.emphasis(rawtext, text),
    ], []


def my_function(arguments, body):
    return "Custom directive: " + arguments


def visit_mynode(attrs):
    return '<div class="directive-node">'


def depart_mynode(attrs):
    return "</div>"


def setup(app):
    app.add_node(MyNode, html=(visit_mynode, depart_mynode))
    app.add_directive("mydirective", MyDirective)
    app.add_directive("myfunction", my_function)
    app.add_role("myrole", my_role)
    return {"version": "0.1", "parallel_read_safe": True}
"#,
    )
    .unwrap();

    pyo3::Python::attach(|py| {
        let sys = py.import("sys").unwrap();
        let path = sys.getattr("path").unwrap();
        path.call_method1("insert", (0, pydir.path().to_str().unwrap()))
            .unwrap();
    });

    let src = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n\n.. mydirective:: from class\n   :count: 3\n\n.. myfunction:: from registry\n\n:myrole:`role output`\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("conf.py"),
        "extensions = ['h5_directive_role_ext']\n",
    )
    .unwrap();

    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    app.build().unwrap();

    let registry = app.registry.borrow();
    assert_eq!(registry.get_directive("mydirective"), Some("MyDirective"));
    assert_eq!(registry.get_directive("myfunction"), Some("my_function"));
    assert_eq!(registry.get_role("myrole"), Some("my_role"));

    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(
        html.contains("Class directive: from x3"),
        "class directive output missing from HTML:\n{html}"
    );
    assert!(html.contains("class=\"directive-node\""));
    assert!(html.contains("Custom directive: from registry"));
    assert!(html.contains("<strong>Custom role: </strong><em>role output</em>"));
}

/// **ADR 0006** — `app.add_node(cls, html=(visit, depart))`: the
/// `(class_name, "html")` pair must land in both the shared
/// `SphinxComponentRegistry::nodes` bookkeeping map (discoverability) and
/// `docutilsrs::plugins`'s real node-visitor registry (rendering). Since
/// this port has no directive that can construct an `Extension` node yet
/// (see the ADR's "Related/out of scope" section), the rendering half is
/// verified by directly building a `Doctree` containing one and rendering
/// it through `docutilsrs::html5` — the same registry the native HTML
/// writer consults, populated here via a real end-to-end `setup()` call.
#[test]
fn load_extension_add_node_records_and_renders() {
    let pydir = TempDir::new().unwrap();
    std::fs::write(
        pydir.path().join("h5_node_ext.py"),
        r#"
class TodoNode:
    pass


def visit_todo(attrs):
    return '<div class="todo">'


def depart_todo(attrs):
    return "</div>"


def setup(app):
    app.add_node(TodoNode, html=(visit_todo, depart_todo))
    return {"version": "0.1", "parallel_read_safe": True}
"#,
    )
    .unwrap();

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
    std::fs::write(src.path().join("conf.py"), "extensions = ['h5_node_ext']\n").unwrap();

    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    app.build().unwrap();

    {
        let registry = app.registry.borrow();
        assert_eq!(
            registry.get_node_formats("TodoNode"),
            Some(&["html".to_string()][..])
        );
    }

    let mut tree = docutilsrs::Doctree::new_document("<test>");
    let root = tree.root();
    let ext_id = tree.append(
        root,
        docutilsrs::NodeKind::Extension {
            class_name: "TodoNode".to_string(),
            attrs: std::collections::BTreeMap::new(),
        },
    );
    tree.append(
        ext_id,
        docutilsrs::NodeKind::Text("Remember this.".to_string()),
    );

    let html = docutilsrs::html5(
        &tree,
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert!(
        html.contains("<div class=\"todo\">Remember this.</div>"),
        "add_node's registered visit/depart pair was not used to render the Extension node:\n{html}"
    );
}

/// `app.events` (`_EventsFacade`): extensions commonly hold on to
/// `app.events` directly rather than always going through
/// `app.connect`/`app.emit` (e.g. `sphinx.ext.viewcode`'s `doctree_read`
/// does `events = app.events; events.emit_firstresult(...)`). Regression
/// test for the `AttributeError: '_AppFacade' object has no attribute
/// 'events'` crash: a `setup()` that reads `app.events`, connects a
/// listener, and calls both `emit`/`emit_firstresult` on it must not
/// raise, and the connected listener must actually run (side effects
/// asserted directly inside `setup()`, so any mismatch fails the build).
#[test]
fn load_extension_app_events_facade_emits_and_dispatches() {
    let pydir = TempDir::new().unwrap();
    std::fs::write(
        pydir.path().join("h5_events_ext.py"),
        r#"
seen = []


def my_listener(app, x):
    seen.append(x)


def setup(app):
    app.connect("my-custom-event", my_listener)
    events = app.events
    result = events.emit_firstresult("my-custom-event", "hello")
    assert result is None, f"expected None, got {result!r}"
    assert seen == ["hello"], f"listener did not run via emit_firstresult: {seen!r}"
    events.emit("my-custom-event", "world")
    assert seen == ["hello", "world"], f"listener did not run via emit: {seen!r}"
    return {"version": "0.1", "parallel_read_safe": True}
"#,
    )
    .unwrap();

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
        "extensions = ['h5_events_ext']\n",
    )
    .unwrap();

    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    app.build().unwrap();
}

/// `doctree-read`'s `doctree` argument (`_Doctree`/`docutilsrs.Doctree`):
/// regression test for the `AttributeError: 'str' object has no attribute
/// 'findall'` crash confirmed by a real `make otherdocs-sphinx-rs` run of
/// `sphinx.ext.viewcode`/`linkcode` — both connect `doctree_read(app,
/// doctree)` and immediately call `doctree.findall(addnodes.desc)`. This
/// mirrors that exact shape (a class the native port has no matching node
/// kind for, so it must find nothing rather than crash) plus a positive
/// match against a real `paragraph` node, asserted directly inside
/// `setup()` so any regression fails the build.
#[test]
fn load_extension_doctree_read_receives_real_doctree_with_findall() {
    let pydir = TempDir::new().unwrap();
    std::fs::write(
        pydir.path().join("h5_doctree_read_ext.py"),
        r#"
seen = {}


class desc:
    """Stand-in for `sphinx.addnodes.desc` — this port never produces a
    node tagged "desc" (domain objects are text-scanned, not modeled as
    real doctree nodes), so matching by class must find nothing."""


class FakeParagraph:
    tagname = "paragraph"


def doctree_read(app, doctree):
    assert hasattr(doctree, "findall"), f"doctree has no findall(): {doctree!r}"

    # Mirrors viewcode/linkcode's exact call shape: a class this port
    # doesn't model as a real node kind must yield an empty list, not
    # raise an AttributeError.
    descs = list(doctree.findall(desc))
    assert descs == [], f"expected no 'desc' matches, got {descs!r}"

    # A real node kind actually present in the parsed document must be
    # found via a class carrying the matching `tagname`.
    paragraphs = list(doctree.findall(FakeParagraph))
    assert len(paragraphs) == 1, f"expected 1 paragraph, got {paragraphs!r}"
    assert paragraphs[0].tag == "paragraph"

    seen["ran"] = True


def setup(app):
    app.connect("doctree-read", doctree_read)
    return {"version": "0.1", "parallel_read_safe": True}
"#,
    )
    .unwrap();

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
        "extensions = ['h5_doctree_read_ext']\n",
    )
    .unwrap();

    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();

    app.build().unwrap();
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
    let _registry = locked_plugin_registry();

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
    let _registry = locked_plugin_registry();

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
