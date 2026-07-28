//! `PyAppFacade` — the `app`-shaped object passed to a Python extension's
//! `setup(app)` function by [`crate::application::SphinxApp::load_extension`].
//!
//! This is the **H4c** slice: enough of the upstream `Sphinx` surface for a
//! typical extension's `setup()` to register a `config-inited` (or other
//! core event) listener without crashing. It is intentionally a thin
//! bridge — `connect()` wraps the Python callable as a native
//! [`crate::app_events::AppEventManager`] listener (constructing a fresh
//! `PyAppFacade` to pass as the callback's `app` argument, mirroring
//! upstream's `emit(name, self, *args)`).
//!
//! **Accepted deviation:** `add_directive` / `add_role` / `add_domain` /
//! `add_html_theme` / `add_builder` / `add_node` / `add_post_transform` /
//! `add_autodocumenter` are no-op stubs that only prevent `AttributeError`
//! from simple `setup()` functions that call them; wiring them through to
//! [`crate::registry::SphinxComponentRegistry`] /
//! `docutilsrs`'s directive-and-role registries is deferred to H5a/H5b
//! (directive/role execution) since those registries don't exist as
//! Python-callable surfaces yet. **`add_config_value`/`app.config`
//! ([`PyConfigFacade`]) and `add_css_file`/`add_js_file` ([`SharedAssets`])
//! are real** — a typical `setup()` reads/writes `app.config.*` and
//! registers page assets before any of the still-stubbed registries would
//! even matter, so these slices are implemented ahead of them.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

use crate::app_events::{EventArg, SharedEvents};
use crate::config::ConfigVal;
use crate::registry::{CssFile, JsFile};

fn event_arg_to_py(py: Python<'_>, arg: &EventArg) -> PyResult<Py<PyAny>> {
    Ok(match arg {
        EventArg::None => py.None(),
        EventArg::Str(s) => s.into_pyobject(py)?.into_any().unbind(),
        EventArg::StrList(items) => PyList::new(py, items)?.into_any().unbind(),
    })
}

/// Convert a native [`ConfigVal`] into a Python object, recursively.
///
/// Used only to *seed* [`PyConfigFacade`]'s store from the already-resolved
/// [`crate::config::SphinxConfig`] once per [`crate::application::SphinxApp`]
/// — every subsequent read/write happens directly against the Python
/// objects already stored there (see `SharedConfig`'s docs).
fn configval_to_py(py: Python<'_>, value: &ConfigVal) -> PyResult<Py<PyAny>> {
    Ok(match value {
        ConfigVal::Null => py.None(),
        ConfigVal::Bool(b) => b.into_pyobject(py)?.to_owned().into_any().unbind(),
        ConfigVal::Int(i) => i.into_pyobject(py)?.into_any().unbind(),
        ConfigVal::Float(f) => f.into_pyobject(py)?.into_any().unbind(),
        ConfigVal::Str(s) => s.into_pyobject(py)?.into_any().unbind(),
        ConfigVal::List(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(configval_to_py(py, item)?);
            }
            PyList::new(py, out)?.into_any().unbind()
        }
        ConfigVal::Map(entries) => {
            let dict = PyDict::new(py);
            for (k, v) in entries {
                dict.set_item(k, configval_to_py(py, v)?)?;
            }
            dict.into_any().unbind()
        }
    })
}

/// Shared, mutable `app.config` store: `name -> Python object`.
///
/// One instance lives on [`crate::application::SphinxApp`] for the whole
/// build (`SphinxApp::py_config`) and is cloned (cheap `Rc` clone, same
/// backing map) into every [`PyAppFacade`] constructed for that app —
/// exactly the same sharing pattern [`SharedEvents`] already uses for
/// `connect`/`emit`. Seeded once from [`crate::config::SphinxConfig`]'s
/// already-resolved options (`conf.py` values merged with `-D` overrides
/// and built-in defaults) via [`configval_to_py`]; every read after that
/// point (`app.config.x`) and every write (`app.config.x = y` or
/// `add_config_value`) goes directly through this map, so mutations from
/// one extension's `setup()` are visible to every later listener
/// invocation and to every other extension loaded afterward — matching
/// upstream's single persistent `Config` object.
pub type SharedConfig = Rc<RefCell<HashMap<String, Py<PyAny>>>>;

/// Seed a fresh [`SharedConfig`] from a resolved [`crate::config::SphinxConfig`].
pub fn seed_shared_config(py: Python<'_>, config: &crate::config::SphinxConfig) -> SharedConfig {
    let mut map = HashMap::new();
    for entry in config.iter() {
        if let Ok(value) = configval_to_py(py, &entry.value) {
            map.insert(entry.name, value);
        }
    }
    Rc::new(RefCell::new(map))
}

/// The `app.config`-shaped object backing [`PyAppFacade::config`].
///
/// Mirrors `sphinx.config.Config`'s attribute-style access
/// (`app.config.project`, `app.config.html_baseurl = ...`) over a plain
/// Python object store (see [`SharedConfig`]). **Accepted deviation:**
/// reading a name that was never seeded from `SphinxConfig` and never
/// registered via [`PyAppFacade::add_config_value`] returns `None` rather
/// than raising `AttributeError` like real Sphinx — replicating upstream's
/// exact raise would require tracking every core option *and* every
/// extension's registrations with full fidelity, whereas a well-behaved
/// extension's `if app.config.some_flag:` treats `None` the same as
/// "unset" anyway.
#[pyclass(
    unsendable,
    skip_from_py_object,
    name = "_ConfigFacade",
    module = "sphinxdocrs"
)]
#[derive(Clone)]
pub struct PyConfigFacade {
    store: SharedConfig,
}

impl PyConfigFacade {
    pub fn new(store: SharedConfig) -> Self {
        Self { store }
    }
}

#[pymethods]
impl PyConfigFacade {
    /// Mirrors `Config.__getattr__` — see the accepted-deviation note above
    /// re: `None` vs. `AttributeError` for a name that was never set.
    fn __getattr__(&self, py: Python<'_>, name: &str) -> Py<PyAny> {
        self.store
            .borrow()
            .get(name)
            .map(|v| v.clone_ref(py))
            .unwrap_or_else(|| py.None())
    }

    /// Mirrors plain instance-attribute assignment (`Config` does not
    /// override `__setattr__` upstream, so `app.config.x = y` is just a
    /// dict write there too).
    fn __setattr__(&self, name: String, value: Py<PyAny>) {
        self.store.borrow_mut().insert(name, value);
    }

    /// Mirrors `Config.__contains__(name)`.
    fn __contains__(&self, name: &str) -> bool {
        self.store.borrow().contains_key(name)
    }
}

// ── page-asset registration (`add_css_file` / `add_js_file`) ─────────────────

/// Accumulates `app.add_css_file`/`app.add_js_file` registrations made by
/// Python extension `setup(app)` functions (and any event listener they
/// connect), for [`crate::application::SphinxApp::build`] to fold into
/// [`crate::environment::BuildEnvironment`] before dispatching to a
/// builder. Shared (via `Rc<RefCell<_>>`) between every [`PyAppFacade`]
/// constructed for the lifetime of one `SphinxApp`, the same way
/// [`SharedEvents`]/[`SharedConfig`] are.
pub type SharedAssets = Rc<RefCell<AssetAccumulator>>;

/// See [`SharedAssets`].
#[derive(Debug, Clone, Default)]
pub struct AssetAccumulator {
    pub css_files: Vec<CssFile>,
    pub js_files: Vec<JsFile>,
}

/// Convert a `**kwargs` dict of extension-supplied `<link>`/`<script>`
/// attributes (e.g. `data-project="..."`, `integrity="..."`) into a
/// `key -> value` string map, matching how upstream stores
/// `attributes: dict[str, Any]` and later renders each entry as an HTML
/// attribute. Non-string values are converted via Python `str()`.
fn kwargs_to_attrs(kwargs: Option<&Bound<'_, PyDict>>) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    if let Some(kwargs) = kwargs {
        for (k, v) in kwargs.iter() {
            let Ok(key) = k.extract::<String>() else {
                continue;
            };
            let value = v
                .str()
                .ok()
                .and_then(|s| s.extract::<String>().ok())
                .unwrap_or_default();
            attrs.insert(key, value);
        }
    }
    attrs
}

/// Minimal `app`-shaped facade over a [`SharedEvents`] handle.
///
/// Constructed by `SphinxApp::load_extension` for the duration of a
/// `setup(app)` call, and again — fresh — for every listener invocation
/// (so a connected callback's `app` argument is always a valid facade,
/// never a dangling reference into a `SphinxApp` that has since moved).
///
/// Marked `unsendable`: it is only ever used from the thread holding the
/// GIL for the duration of a single `setup(app)` call or listener
/// invocation, so it need not satisfy `Send + Sync` — which in turn lets
/// [`crate::app_events::AppEventManager`] stay a plain, `Rc`-friendly
/// native type instead of forcing every native listener closure to be
/// `Send`.
#[pyclass(
    unsendable,
    skip_from_py_object,
    name = "_AppFacade",
    module = "sphinxdocrs"
)]
#[derive(Clone)]
pub struct PyAppFacade {
    events: SharedEvents,
    config: SharedConfig,
    assets: SharedAssets,
}

impl PyAppFacade {
    pub fn new(events: SharedEvents, config: SharedConfig, assets: SharedAssets) -> Self {
        Self {
            events,
            config,
            assets,
        }
    }
}

#[pymethods]
impl PyAppFacade {
    /// Mirrors `Sphinx.connect(event, callback, priority=500)`: wraps
    /// `callback` as a native listener that, when the native pipeline
    /// emits `event`, re-enters Python and calls
    /// `callback(app_facade, *event_args)`.
    #[pyo3(signature = (event, callback, priority=500))]
    fn connect(&self, event: &str, callback: Py<PyAny>, priority: i64) -> PyResult<usize> {
        let events_for_call = self.events.clone();
        let config_for_call = self.config.clone();
        let assets_for_call = self.assets.clone();
        let mut mgr = self.events.borrow_mut();
        let id = mgr.connect(event.to_string(), priority, move |args: &[EventArg]| {
            let events_for_call = events_for_call.clone();
            let config_for_call = config_for_call.clone();
            let assets_for_call = assets_for_call.clone();
            let _ = Python::attach(|py| -> PyResult<()> {
                let facade = Py::new(
                    py,
                    PyAppFacade::new(events_for_call, config_for_call, assets_for_call),
                )?;
                let mut call_args: Vec<Py<PyAny>> = Vec::with_capacity(args.len() + 1);
                call_args.push(facade.into_any());
                for a in args {
                    call_args.push(event_arg_to_py(py, a)?);
                }
                let tuple = PyTuple::new(py, &call_args)?;
                callback.bind(py).call1(tuple)?;
                Ok(())
            });
        });
        Ok(id)
    }

    /// Mirrors `Sphinx.disconnect(listener_id)`.
    fn disconnect(&self, listener_id: usize) -> PyResult<()> {
        self.events.borrow_mut().disconnect(listener_id);
        Ok(())
    }

    /// Mirrors `Sphinx.add_event(name)`.
    fn add_event(&self, name: &str) -> PyResult<()> {
        self.events.borrow_mut().add(name.to_string());
        Ok(())
    }

    /// Mirrors `Sphinx.config` — a live [`PyConfigFacade`] over this app's
    /// [`SharedConfig`]. A fresh facade wrapper is returned on every access
    /// (cheap: it just clones the `Rc`), but all wrappers share the same
    /// underlying store, so a value written through one `app.config.x = y`
    /// call is visible to every later `app.config.x` read, from this
    /// `setup()` call or any later listener invocation.
    #[getter]
    fn config(&self, py: Python<'_>) -> PyResult<Py<PyConfigFacade>> {
        Py::new(py, PyConfigFacade::new(self.config.clone()))
    }

    /// Mirrors `Sphinx.add_config_value(name, default, rebuild, types=())`:
    /// registers `name` in `app.config`'s store with `default` **only if
    /// it isn't already present** (a prior `conf.py` value, command-line
    /// override, or an earlier `add_config_value` call for the same name
    /// always wins) — matching upstream's "first registration establishes
    /// the default, `conf.py`/overrides always take precedence" behavior,
    /// minus the `ExtensionError` upstream raises for a genuine duplicate
    /// registration (kept permissive since re-running `setup()` idempotently
    /// is more useful for this bridge than replicating that failure mode).
    #[pyo3(signature = (name, default, rebuild=None, types=None))]
    fn add_config_value(
        &self,
        name: String,
        default: Py<PyAny>,
        rebuild: Option<Py<PyAny>>,
        types: Option<Py<PyAny>>,
    ) {
        let _ = (rebuild, types);
        self.config.borrow_mut().entry(name).or_insert(default);
    }

    /// No-op stub — see module docs.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_directive(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// No-op stub — see module docs.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_role(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// No-op stub — see module docs.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_domain(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// No-op stub — see module docs.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_html_theme(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// No-op stub — see module docs.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_builder(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// No-op stub — see module docs.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_node(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// No-op stub — see module docs.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_post_transform(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// Mirrors `Sphinx.add_css_file(filename, priority=500, **kwargs)`:
    /// registers a stylesheet to be linked on every page. Recorded in
    /// `self.assets` (shared with `SphinxApp`); `SphinxApp::build` copies
    /// the accumulated entries into `BuildEnvironment::added_css_files`
    /// just before dispatching to a builder, and the HTML-family builders'
    /// theme renderer merges them into the `css_files` template list
    /// alongside whatever it discovers under `_static/` (see
    /// `crate::theme_render`'s module doc for the exact merge point).
    /// **Accepted deviation:** only the *reference* (filename + attributes)
    /// is tracked — actually copying a non-`html_static_path` asset file
    /// into `_static/` (e.g. one bundled inside an extension's own package)
    /// is not implemented, so a registered file only renders without a
    /// broken link when it already lands in `_static/` through some other
    /// path (`html_static_path`, the active theme's own static files, ...).
    #[pyo3(signature = (filename, priority=500, **kwargs))]
    fn add_css_file(&self, filename: String, priority: i64, kwargs: Option<Bound<'_, PyDict>>) {
        let mut attributes = kwargs_to_attrs(kwargs.as_ref());
        attributes.insert("priority".to_string(), priority.to_string());
        self.assets.borrow_mut().css_files.push(CssFile {
            filename,
            attributes,
        });
    }

    /// Mirrors `Sphinx.add_js_file(filename, priority=500,
    /// loading_method=None, **kwargs)`. See [`Self::add_css_file`] for the
    /// storage/merge path and its accepted deviation.
    ///
    /// `filename=None` (an inline `<script>` body passed via
    /// `kwargs['body']`) is accepted, matching upstream's signature, but
    /// not stored — this port has no per-page inline-script slot to place
    /// it in yet.
    #[pyo3(signature = (filename=None, priority=500, loading_method=None, **kwargs))]
    fn add_js_file(
        &self,
        filename: Option<String>,
        priority: i64,
        loading_method: Option<String>,
        kwargs: Option<Bound<'_, PyDict>>,
    ) {
        let Some(filename) = filename else {
            return;
        };
        let mut attributes = kwargs_to_attrs(kwargs.as_ref());
        attributes.insert("priority".to_string(), priority.to_string());
        if let Some(loading_method) = loading_method {
            attributes.insert("loading_method".to_string(), loading_method);
        }
        self.assets.borrow_mut().js_files.push(JsFile {
            filename: Some(filename),
            attributes,
        });
    }

    /// No-op stub — see module docs.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_autodocumenter(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// No-op stub — see module docs. Mirrors `Sphinx.setup_extension`, but
    /// (per the **accepted deviation** noted on
    /// [`SphinxApp::load_extension`](crate::application::SphinxApp::load_extension))
    /// does not recursively load the named extension here: callers are
    /// expected to list dependencies in `conf.py`'s `extensions` in
    /// load order already, so this only prevents `AttributeError` for
    /// `setup()` functions that re-assert a dependency that's already
    /// been loaded.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn setup_extension(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
    }

    /// Mirrors `Sphinx.require_sphinx(version)` — accepted as a no-op
    /// since this port has no upstream Sphinx version to compare against.
    fn require_sphinx(&self, _version_info: &Bound<'_, PyAny>) {}
}
