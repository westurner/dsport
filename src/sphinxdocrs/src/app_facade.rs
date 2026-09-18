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
//! **`add_builder` / `add_domain` / `add_html_theme` / `add_post_transform`
//! are real**, wired to a [`crate::registry::SharedRegistry`] shared with
//! [`crate::application::SphinxApp::registry`] (same `Rc<RefCell<_>>`
//! pattern as `events`/`config`/`assets`): a name/class-name pair lands in
//! the same registry `SphinxApp` itself consults (e.g.
//! `SphinxComponentRegistry::get_builder`/`has_domain`), so a custom
//! extension is now *discoverable* by name post-`setup()` without an
//! `AttributeError`. **Accepted deviation:** this only records
//! bookkeeping metadata (name → class-name string) — `SphinxApp::build`'s
//! builder dispatch is still a hardcoded match over *native* builder names
//! with no `dyn Builder`/`dyn Domain` hook to actually construct and run
//! an arbitrary Python-provided class, so selecting a custom builder via
//! `-b <name>` or resolving a custom domain's roles/directives still
//! cannot dispatch to real Python execution. **`add_directive`/`add_role`
//! now record the same kind of registry bookkeeping** (name → class name),
//! so a typical `setup()` call no longer raises `AttributeError` — but,
//! like `add_domain`/`add_builder`, a document that actually *uses* the
//! registered directive/role still isn't executed through this
//! registration (see `SphinxComponentRegistry::directives`/`roles`'s doc
//! comments for the narrower bridge that does support real parse-time
//! directive execution). **`add_node` is real** (ADR 0006,
//! `docs/adr/0006-extension-nodes.md`): it registers `(visit, depart)`
//! callable pairs per builder format directly into
//! `docutilsrs::plugins`'s new node-visitor registry, which every native
//! writer consults for a `NodeKind::Extension` node — so a registered
//! extension node actually renders (with a graceful children-only
//! fallback for any format it didn't register). **Accepted deviation:**
//! this only covers *rendering* an already-existing `Extension` node — a
//! custom directive's `run()` *constructing* one is a separate, larger
//! gap (the doctree/Node Python bridge). **`add_autodocumenter` remains a
//! no-op stub** — explicitly out of scope for now. **`add_config_value`
//! /`app.config`([`PyConfigFacade`]) and `add_css_file`/`add_js_file`
//! ([`SharedAssets`]) are real too** — a typical `setup()` reads/writes
//! `app.config.*` and registers page assets before any of the
//! still-stubbed registries would even matter, so these slices were
//! implemented ahead of them.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList, PyTuple};

use docutilsrs::python::PyDoctree;

use crate::app_events::{EventArg, EventError, SharedEvents};
use crate::config::ConfigVal;
use crate::registry::{CssFile, JsFile, SharedRegistry};

/// Convert a `PyErr` raised by a connected listener callback into an
/// [`EventError`], with a message mirroring the PyO3-facing
/// [`crate::events::EventManager::emit`]'s wrapping text so both event
/// buses report a failing extension listener the same way.
fn py_err_to_event_error(
    py: Python<'_>,
    event: &str,
    callback: &Py<PyAny>,
    err: PyErr,
) -> EventError {
    let handler_repr = callback
        .bind(py)
        .repr()
        .map(|s| s.to_string())
        .unwrap_or_else(|_| "<handler>".to_string());
    EventError(format!(
        "Handler {handler_repr} for event '{event}' threw an exception: {err}"
    ))
}

fn event_arg_to_py(py: Python<'_>, arg: &EventArg) -> PyResult<Py<PyAny>> {
    Ok(match arg {
        EventArg::None => py.None(),
        EventArg::Str(s) => s.into_pyobject(py)?.into_any().unbind(),
        EventArg::StrList(items) => PyList::new(py, items)?.into_any().unbind(),
        EventArg::Doctree(tree) => Py::new(py, PyDoctree::new(tree.clone()))?.into_any(),
        EventArg::HtmlPageContext { .. } => {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "html-page-context must be expanded by the event bridge",
            ));
        }
    })
}

fn json_value_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<Py<PyAny>> {
    Ok(match value {
        serde_json::Value::Null => py.None(),
        serde_json::Value::Bool(value) => PyBool::new(py, *value).to_owned().into_any().unbind(),
        serde_json::Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                value.into_pyobject(py)?.into_any().unbind()
            } else {
                value
                    .as_f64()
                    .unwrap_or_default()
                    .into_pyobject(py)?
                    .into_any()
                    .unbind()
            }
        }
        serde_json::Value::String(value) => value.into_pyobject(py)?.into_any().unbind(),
        serde_json::Value::Array(values) => {
            let values = values
                .iter()
                .map(|value| json_value_to_py(py, value))
                .collect::<PyResult<Vec<_>>>()?;
            PyList::new(py, values)?.into_any().unbind()
        }
        serde_json::Value::Object(values) => {
            let dict = PyDict::new(py);
            for (key, value) in values {
                dict.set_item(key, json_value_to_py(py, value)?)?;
            }
            dict.into_any().unbind()
        }
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

/// Immutable snapshot of `conf.py`'s raw values (never mutated after
/// construction — unlike [`SharedConfig`], nothing needs `RefCell` here).
/// See [`crate::config::SphinxConfig::raw_config`]'s doc comment for why
/// [`PyAppFacade::add_config_value`] needs this alongside [`SharedConfig`].
pub type SharedRawConfig = Rc<HashMap<String, ConfigVal>>;

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

// ── `app.env` (live `BuildEnvironment` facade) ────────────────────────────────

fn path_to_pyobject(py: Python<'_>, path: &std::path::Path) -> PyResult<Py<PyAny>> {
    let pathlib = py.import("pathlib")?;
    let path_cls = pathlib.getattr("Path")?;
    Ok(path_cls
        .call1((path.to_string_lossy().to_string(),))?
        .unbind())
}

/// Convert a `RefCell` borrow conflict into a clear Python exception
/// instead of letting the panic (which PyO3 would otherwise convert into
/// an opaque `PanicException`) surface. The only realistic way to hit this
/// is a listener connected to a write-phase event (e.g.
/// `html-page-context`) calling an `app.env` *mutator* while the active
/// builder's `build_all` holds its own (immutable) borrow of the same
/// [`crate::environment::SharedEnv`] for the whole write phase — see
/// [`PyEnvFacade`]'s doc comment.
fn borrow_env_mut_or_err(
    env: &crate::environment::SharedEnv,
) -> PyResult<std::cell::RefMut<'_, crate::environment::BuildEnvironment>> {
    env.try_borrow_mut().map_err(|_| {
        pyo3::exceptions::PyRuntimeError::new_err(
            "app.env cannot be mutated while it is being read elsewhere \
             (e.g. from within the active builder's write phase)",
        )
    })
}

/// Shared store backing [`PyEnvFacade`]'s arbitrary-attribute fallback
/// (`env.some_custom_name = ...` / reading it back later) — see
/// [`PyEnvFacade`]'s doc comment for why this can't just be a PyO3
/// per-instance `__dict__` (`#[pyclass(dict)]`).
pub type SharedEnvExtra = Rc<RefCell<HashMap<String, Py<PyAny>>>>;

/// The `app.env`-shaped object backing [`PyAppFacade::env`]: a facade over
/// the *live* [`crate::environment::SharedEnv`] — the very same
/// `Rc<RefCell<BuildEnvironment>>` [`crate::application::SphinxApp`] owns
/// for the whole build — rather than a point-in-time copy. A listener that
/// fires late (e.g. `html-page-context`, long after `builder-inited`)
/// therefore always sees the current state, and (via the handful of
/// mutator methods below) can write back to it too, matching upstream
/// where `app.env`/`env` is the one persistent `BuildEnvironment` object.
///
/// **Reentrancy note:** every accessor here does a short-lived
/// `try_borrow()`/`try_borrow_mut()` rather than holding the `RefCell`
/// borrow across a Python callback. The one case that can still fail is a
/// *mutator* called from a listener that fires while the active builder's
/// `build_all` holds its own immutable borrow of the whole environment for
/// the duration of the write phase (`Builder::build_all(&self.srcdir,
/// &self.outdir, &self.env.borrow())` in `application.rs`) — that surfaces
/// as a `PyRuntimeError`, not a process-aborting panic (see
/// [`borrow_env_mut_or_err`]).
///
/// **Accepted deviation:** `domaindata`/`temp_data`/`ref_context`/
/// `metadata` are the simplified `HashMap<String, String>`-shaped fields
/// this Rust port already stores them as (not arbitrary pickled Python
/// objects like upstream) — reads/writes go through `str()`/plain strings
/// accordingly. `env.config` is not exposed here; use `app.config` instead
/// (this port doesn't yet share one `Config`-shaped object between the
/// two, unlike upstream where `app.config is env.config`).
///
/// **Arbitrary attributes:** real Sphinx extensions commonly stash their
/// own state directly on `env` as a plain instance attribute (e.g.
/// `sphinx.ext.intersphinx`'s `if not hasattr(env, 'intersphinx_cache'):
/// env.intersphinx_cache = {}`), relying on `env` being one persistent
/// Python object for the whole build. Since a *fresh* `PyEnvFacade` is
/// constructed on every `app.env` access (matching how [`PyAppFacade`]
/// itself works), a plain PyO3 per-instance `__dict__` (`#[pyclass(dict)]`)
/// would lose such an attribute the moment a different `PyEnvFacade`
/// instance is asked for it. [`__getattr__`](Self::__getattr__)/
/// [`__setattr__`](Self::__setattr__) instead fall back to `extra`
/// ([`SharedEnvExtra`]) — shared the same `Rc<RefCell<_>>` way as `env`
/// itself — so this behaves like the one persistent object upstream has.
#[pyclass(
    unsendable,
    skip_from_py_object,
    name = "_EnvFacade",
    module = "sphinxdocrs"
)]
#[derive(Clone)]
pub struct PyEnvFacade {
    env: crate::environment::SharedEnv,
    extra: SharedEnvExtra,
}

impl PyEnvFacade {
    pub fn new(env: crate::environment::SharedEnv, extra: SharedEnvExtra) -> Self {
        Self { env, extra }
    }
}

#[pymethods]
impl PyEnvFacade {
    /// Mirrors `BuildEnvironment.found_docs` (a `set[str]`).
    #[getter]
    fn found_docs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PySet>> {
        pyo3::types::PySet::new(py, self.env.borrow().found_docs().iter())
    }

    /// Mirrors `BuildEnvironment.all_docs` (`dict[str, int]`, docname →
    /// read-time in microseconds). Returns a plain `dict` copy.
    #[getter]
    fn all_docs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (docname, ts) in &self.env.borrow().all_docs {
            dict.set_item(docname, ts)?;
        }
        Ok(dict)
    }

    /// Mirrors `BuildEnvironment.titles` (`dict[docname, str]` — the title
    /// text of each document's first section).
    #[getter]
    fn titles<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (docname, title) in &self.env.borrow().titles {
            dict.set_item(docname, title)?;
        }
        Ok(dict)
    }

    /// Mirrors `BuildEnvironment.longtitles` (`dict[docname, str]`).
    #[getter]
    fn longtitles<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (docname, title) in &self.env.borrow().longtitles {
            dict.set_item(docname, title)?;
        }
        Ok(dict)
    }

    /// Mirrors `Sphinx.srcdir`/`BuildEnvironment.srcdir` (a `pathlib.Path`).
    #[getter]
    fn srcdir(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        path_to_pyobject(py, &self.env.borrow().srcdir)
    }

    /// Mirrors `BuildEnvironment.doctreedir` (a `pathlib.Path`).
    #[getter]
    fn doctreedir(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        path_to_pyobject(py, &self.env.borrow().doctreedir)
    }

    /// Mirrors `BuildEnvironment.doc2path(docname, base=True)`, returning a
    /// real `pathlib.Path` (not a plain `str`) since real extension code
    /// (e.g. `sphinx.ext.autosummary`) calls `.is_file()` on the result.
    #[pyo3(signature = (docname, base=true))]
    fn doc2path(&self, py: Python<'_>, docname: &str, base: bool) -> PyResult<Py<PyAny>> {
        let env = self.env.borrow();
        let rel = env
            .project
            .docname_to_path
            .get(docname)
            .cloned()
            .unwrap_or_else(|| {
                let first_suffix = env
                    .config
                    .source_suffix()
                    .into_keys()
                    .next()
                    .unwrap_or_else(|| ".rst".to_string());
                format!("{docname}{first_suffix}")
            });
        let path = if base {
            env.srcdir.join(rel)
        } else {
            std::path::PathBuf::from(rel)
        };
        path_to_pyobject(py, &path)
    }

    /// Mirrors `BuildEnvironment.metadata[docname]` (`dict[str, str]`;
    /// empty if `docname` has no recorded metadata).
    fn metadata<'py>(&self, py: Python<'py>, docname: &str) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        if let Some(meta) = self.env.borrow().metadata.get(docname) {
            for (k, v) in meta {
                dict.set_item(k, v)?;
            }
        }
        Ok(dict)
    }

    /// Mirrors `BuildEnvironment.domaindata[name]` (`dict[str, str]`;
    /// empty if `name` has no recorded domain data). See this type's
    /// accepted-deviation note re: string-only values.
    fn get_domaindata<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        if let Some(data) = self.env.borrow().domaindata.get(name) {
            for (k, v) in data {
                dict.set_item(k, v)?;
            }
        }
        Ok(dict)
    }

    /// Mirrors reading `env.temp_data[key]`; returns `None` if unset. See
    /// this type's accepted-deviation note re: string-only values.
    fn get_temp_data(&self, key: &str) -> PyResult<Option<String>> {
        Ok(self.env.borrow().temp_data.get(key).cloned())
    }

    /// Mirrors `env.temp_data[key] = value` (`value` is converted via
    /// Python `str()` — see this type's accepted-deviation note).
    fn set_temp_data(&self, py: Python<'_>, key: String, value: Py<PyAny>) -> PyResult<()> {
        let value_str = value.bind(py).str()?.extract::<String>()?;
        borrow_env_mut_or_err(&self.env)?
            .temp_data
            .insert(key, value_str);
        Ok(())
    }

    /// Mirrors reading `env.ref_context[key]`; returns `None` if unset.
    fn get_ref_context(&self, key: &str) -> PyResult<Option<String>> {
        Ok(self.env.borrow().ref_context.get(key).cloned())
    }

    /// Mirrors `env.ref_context[key] = value` (`value` is converted via
    /// Python `str()` — see this type's accepted-deviation note).
    fn set_ref_context(&self, py: Python<'_>, key: String, value: Py<PyAny>) -> PyResult<()> {
        let value_str = value.bind(py).str()?.extract::<String>()?;
        borrow_env_mut_or_err(&self.env)?
            .ref_context
            .insert(key, value_str);
        Ok(())
    }

    /// Mirrors `BuildEnvironment.dependencies[docname]` (`set[str]` of
    /// dependency file paths relative to `srcdir`), returned as a sorted
    /// `list[str]`.
    fn dependencies(&self, docname: &str) -> PyResult<Vec<String>> {
        let mut deps: Vec<String> = self
            .env
            .borrow()
            .dependencies
            .get(docname)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default();
        deps.sort();
        Ok(deps)
    }

    /// Mirrors `env.note_dependency(filename)`. **Accepted deviation:**
    /// upstream defaults to the document currently being read
    /// (`self.docname`) when `docname` isn't given; this port doesn't
    /// track a global "current document" outside the read-phase loop, so
    /// `docname` is a required argument here.
    fn note_dependency(&self, docname: &str, filename: &str) -> PyResult<()> {
        borrow_env_mut_or_err(&self.env)?
            .dependencies
            .entry(docname.to_string())
            .or_default()
            .insert(filename.to_string());
        Ok(())
    }

    /// Fallback for any attribute not covered by an explicit getter/method
    /// above — mirrors a plain Python object's arbitrary instance
    /// attributes (see this type's "Arbitrary attributes" doc note).
    /// Raises a real `AttributeError` for a name that was never set via
    /// [`__setattr__`](Self::__setattr__), matching upstream so
    /// `hasattr(env, name)` behaves correctly (`sphinx.ext.intersphinx`
    /// relies on exactly this to lazily initialize `env.intersphinx_cache`
    /// only once).
    fn __getattr__(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        match self.extra.borrow().get(name) {
            Some(v) => Ok(v.clone_ref(py)),
            None => Err(pyo3::exceptions::PyAttributeError::new_err(format!(
                "'_EnvFacade' object has no attribute {name:?}"
            ))),
        }
    }

    /// See [`__getattr__`](Self::__getattr__).
    fn __setattr__(&self, name: String, value: Py<PyAny>) {
        self.extra.borrow_mut().insert(name, value);
    }
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

/// Convert variadic Python `*args` (as passed to `events.emit(name, *args)`/
/// `events.emit_firstresult(name, *args)`) into native [`EventArg`]s.
///
/// **Accepted deviation:** intentionally lossy/stringifying — every
/// non-`None` argument becomes `EventArg::Str(arg.str())`. `EventArg` has
/// no richer payload shape for an arbitrary Python object anyway (see its
/// doc comment), and this is sufficient for the module names/docnames real
/// extensions pass to their own custom events (e.g. `viewcode-find-source`,
/// `viewcode-follow-imported`).
fn py_args_to_event_args(args: &Bound<'_, PyTuple>) -> PyResult<Vec<EventArg>> {
    let mut out = Vec::with_capacity(args.len());
    for arg in args.iter() {
        if arg.is_none() {
            out.push(EventArg::None);
        } else {
            out.push(EventArg::Str(arg.str()?.to_string()));
        }
    }
    Ok(out)
}

/// Minimal `app`-shaped facade over a [`SharedEvents`] handle.
///
/// Mirrors `sphinx.events.EventManager`, exposed as `app.events` for the
/// (surprisingly common — see the grep in this module's changelog note)
/// pattern real extensions use: `events = app.events;
/// events.emit_firstresult('some-custom-event', ...)` (e.g.
/// `sphinx.ext.viewcode`/`linkcode`'s `doctree_read`), rather than only
/// ever going through the top-level `app.connect`/`app.emit` aliases.
///
/// **Accepted deviation:** the native [`crate::app_events::AppEventManager`]
/// `Handler` type is `FnMut(&[EventArg]) -> Result<(), EventError>` — it has
/// no slot for a listener to return a value. So [`Self::emit`]/
/// [`Self::emit_firstresult`] still fully dispatch to every registered
/// listener (side effects happen), but always report "no result"
/// (`[]`/`None`) — which matches upstream's own behavior in the common
/// case where nothing happens to be connected to a given custom event name.
///
/// **Deliberately does NOT have its own `connect`/`disconnect`:** doing so
/// would need duplicating all of [`PyAppFacade::connect`]'s captured
/// `Shared*` fields just to reconstruct the callback's `app` argument.
/// Use the existing top-level `app.connect`/`app.disconnect` instead —
/// upstream's own `Sphinx.connect` is itself just `self.events.connect`.
#[pyclass(
    unsendable,
    skip_from_py_object,
    name = "_EventsFacade",
    module = "sphinxdocrs"
)]
#[derive(Clone)]
pub struct PyEventsFacade {
    events: SharedEvents,
}

impl PyEventsFacade {
    pub fn new(events: SharedEvents) -> Self {
        Self { events }
    }
}

#[pymethods]
impl PyEventsFacade {
    /// Mirrors `EventManager.emit(name, *args, allowed_exceptions=())`.
    /// See this struct's doc comment for the "always returns `[]`"
    /// accepted deviation.
    #[pyo3(signature = (event, *args))]
    fn emit(&self, event: &str, args: &Bound<'_, PyTuple>) -> PyResult<Vec<Py<PyAny>>> {
        let event_args = py_args_to_event_args(args)?;
        self.events
            .borrow_mut()
            .emit(event, &event_args)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.0))?;
        Ok(Vec::new())
    }

    /// Mirrors `EventManager.emit_firstresult(name, *args,
    /// allowed_exceptions=())`. See this struct's doc comment for the
    /// "always returns `None`" accepted deviation.
    #[pyo3(signature = (event, *args))]
    fn emit_firstresult(
        &self,
        py: Python<'_>,
        event: &str,
        args: &Bound<'_, PyTuple>,
    ) -> PyResult<Py<PyAny>> {
        self.emit(event, args)?;
        Ok(py.None())
    }

    /// Mirrors `Sphinx.add_event(name)` (`EventManager.add`).
    fn add_event(&self, name: &str) -> PyResult<()> {
        self.events.borrow_mut().add(name.to_string());
        Ok(())
    }
}

/// Minimal `app.builder`-shaped facade exposing only `.name`/`.format`
/// (mirrors `sphinx.builders.Builder.name`/`.format`, the two class
/// attributes real extension/`conf.py` code actually reads off
/// `app.builder`). Not a live view of anything mutable — just the fixed
/// buildername string the owning `SphinxApp` was constructed with.
#[pyclass(
    unsendable,
    skip_from_py_object,
    name = "_BuilderFacade",
    module = "sphinxdocrs"
)]
#[derive(Clone)]
pub struct PyBuilderFacade {
    name: String,
}

impl PyBuilderFacade {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[pymethods]
impl PyBuilderFacade {
    /// Mirrors `Builder.name` (e.g. `"html"`, `"dirhtml"`, `"latex"`).
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    /// Mirrors `Builder.format`: the *output format* class attribute,
    /// which several HTML-family builders share (`html`/`dirhtml`/
    /// `singlehtml` all report `format == "html"`, matching upstream's
    /// `StandaloneHTMLBuilder.format = 'html'` being inherited by its
    /// subclasses) — everything else defaults to its own `name`.
    #[getter]
    fn format(&self) -> &str {
        match self.name.as_str() {
            "html" | "dirhtml" | "singlehtml" => "html",
            other => other,
        }
    }
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
    registry: SharedRegistry,
    env: crate::environment::SharedEnv,
    raw_config: SharedRawConfig,
    env_extra: SharedEnvExtra,
    /// `Sphinx.outdir` / `Sphinx.builder.name`. `None` when this facade is
    /// constructed before the active builder is known (there is currently
    /// no such call site, but keeps the type honest rather than requiring
    /// a placeholder path/name at every call site — see
    /// [`Self::outdir`]/[`Self::builder`]).
    outdir: Option<std::rc::Rc<std::path::PathBuf>>,
    buildername: Option<std::rc::Rc<String>>,
}

impl PyAppFacade {
    pub fn new(
        events: SharedEvents,
        config: SharedConfig,
        assets: SharedAssets,
        registry: SharedRegistry,
        env: crate::environment::SharedEnv,
        raw_config: SharedRawConfig,
        env_extra: SharedEnvExtra,
    ) -> Self {
        Self {
            events,
            config,
            assets,
            registry,
            env,
            raw_config,
            env_extra,
            outdir: None,
            buildername: None,
        }
    }

    /// Same as [`Self::new`], but also sets `outdir`/`builder.name` (mirrors
    /// `Sphinx.outdir`/`Sphinx.builder.name`) — used wherever the calling
    /// `SphinxApp` already knows both (i.e. everywhere except the very
    /// first `conf.py`-`setup(app)` call, which happens before a builder
    /// is selected... actually `SphinxApp::new` takes `buildername` up
    /// front, so this is always available in practice; kept as `Option`
    /// defensively rather than assuming every current and future call site
    /// has it to hand).
    pub fn with_builder(
        mut facade: Self,
        outdir: std::rc::Rc<std::path::PathBuf>,
        buildername: std::rc::Rc<String>,
    ) -> Self {
        facade.outdir = Some(outdir);
        facade.buildername = Some(buildername);
        facade
    }
}

#[pymethods]
impl PyAppFacade {
    /// Mirrors `Sphinx.connect(event, callback, priority=500)`: wraps
    /// `callback` as a native listener that, when the native pipeline
    /// emits `event`, re-enters Python and calls
    /// `callback(app_facade, *event_args)`.
    ///
    /// A raising `callback` no longer fails silently: its `PyErr` is
    /// converted into an [`EventError`] (message mirrors the PyO3-facing
    /// [`crate::events::EventManager::emit`]'s `"Handler {handler!r} for
    /// event '{name}' threw an exception"` wrapping) and propagated through
    /// [`crate::app_events::AppEventManager::emit`], aborting the rest of
    /// dispatch — see this module's doc comment.
    #[pyo3(signature = (event, callback, priority=500))]
    fn connect(&self, event: &str, callback: Py<PyAny>, priority: i64) -> PyResult<usize> {
        let events_for_call = self.events.clone();
        let config_for_call = self.config.clone();
        let assets_for_call = self.assets.clone();
        let registry_for_call = self.registry.clone();
        let env_for_call = self.env.clone();
        let raw_config_for_call = self.raw_config.clone();
        let env_extra_for_call = self.env_extra.clone();
        let outdir_for_call = self.outdir.clone();
        let buildername_for_call = self.buildername.clone();
        let event_name = event.to_string();
        let mut mgr = self.events.borrow_mut();
        let id = mgr.connect(event_name.clone(), priority, move |args: &[EventArg]| {
            let events_for_call = events_for_call.clone();
            let config_for_call = config_for_call.clone();
            let assets_for_call = assets_for_call.clone();
            let registry_for_call = registry_for_call.clone();
            let env_for_call = env_for_call.clone();
            let raw_config_for_call = raw_config_for_call.clone();
            let env_extra_for_call = env_extra_for_call.clone();
            let outdir_for_call = outdir_for_call.clone();
            let buildername_for_call = buildername_for_call.clone();
            let config_for_second_arg = config_for_call.clone();
            Python::attach(|py| -> Result<(), EventError> {
                let mut new_facade = PyAppFacade::new(
                    events_for_call,
                    config_for_call,
                    assets_for_call,
                    registry_for_call,
                    env_for_call,
                    raw_config_for_call,
                    env_extra_for_call,
                );
                new_facade.outdir = outdir_for_call;
                new_facade.buildername = buildername_for_call;
                let facade = Py::new(py, new_facade)
                    .map_err(|e| py_err_to_event_error(py, &event_name, &callback, e))?;
                let mut call_args: Vec<Py<PyAny>> = Vec::with_capacity(args.len() + 2);
                call_args.push(facade.into_any());
                let mut page_context: Option<(
                    Py<PyDict>,
                    Rc<RefCell<std::collections::BTreeMap<String, serde_json::Value>>>,
                )> = None;
                // Mirrors upstream `emit("config-inited", self, self.config)`:
                // this event's second positional argument is always the
                // live `Config` object, never one of the native pipeline's
                // `EventArg`s (which is why `application.rs` emits this
                // event with an empty `args` slice — the config argument is
                // synthesized here instead, since only this Python-facing
                // closure has a `Py<PyAny>`-capable `SharedConfig` handle).
                if event_name == "config-inited" {
                    let config_facade =
                        Py::new(py, PyConfigFacade::new(config_for_second_arg.clone()))
                            .map_err(|e| py_err_to_event_error(py, &event_name, &callback, e))?;
                    call_args.push(config_facade.into_any());
                }
                for a in args {
                    if let EventArg::HtmlPageContext {
                        pagename,
                        templatename,
                        context,
                        doctree,
                    } = a
                    {
                        call_args.push(
                            pagename
                                .into_pyobject(py)
                                .map_err(|e| {
                                    py_err_to_event_error(py, &event_name, &callback, e.into())
                                })?
                                .into_any()
                                .unbind(),
                        );
                        call_args.push(
                            templatename
                                .into_pyobject(py)
                                .map_err(|e| {
                                    py_err_to_event_error(py, &event_name, &callback, e.into())
                                })?
                                .into_any()
                                .unbind(),
                        );
                        let context_dict = PyDict::new(py);
                        for (key, value) in context.borrow().iter() {
                            context_dict
                                .set_item(
                                    key,
                                    json_value_to_py(py, value).map_err(|e| {
                                        py_err_to_event_error(py, &event_name, &callback, e)
                                    })?,
                                )
                                .map_err(|e| {
                                    py_err_to_event_error(py, &event_name, &callback, e)
                                })?;
                        }
                        let context_obj = context_dict.unbind();
                        call_args.push(context_obj.clone_ref(py).into_any());
                        call_args.push(match doctree {
                            Some(tree) => Py::new(py, PyDoctree::new(tree.clone()))
                                .map_err(|e| py_err_to_event_error(py, &event_name, &callback, e))?
                                .into_any(),
                            None => py.None(),
                        });
                        page_context = Some((context_obj, context.clone()));
                    } else {
                        let v = event_arg_to_py(py, a)
                            .map_err(|e| py_err_to_event_error(py, &event_name, &callback, e))?;
                        call_args.push(v);
                    }
                }
                let tuple = PyTuple::new(py, &call_args)
                    .map_err(|e| py_err_to_event_error(py, &event_name, &callback, e))?;
                callback
                    .bind(py)
                    .call1(tuple)
                    .map(|_| ())
                    .map_err(|e| py_err_to_event_error(py, &event_name, &callback, e))?;
                if let Some((context_obj, native_context)) = page_context {
                    let context_dict = context_obj.bind(py);
                    let mut updated = native_context.borrow_mut();
                    for (key, value) in context_dict.iter() {
                        let Ok(key) = key.extract::<String>() else {
                            continue;
                        };
                        if let Ok(value) = value.extract::<String>() {
                            updated.insert(key, serde_json::Value::String(value));
                        }
                    }
                }
                Ok(())
            })
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

    /// Mirrors `Sphinx.env` — a live [`PyEnvFacade`] over the same
    /// [`crate::environment::SharedEnv`] this app owns. Before the first
    /// `self.env.find_files()` call (i.e. during a `setup(app)` call
    /// itself, same as upstream — `env` doesn't exist yet at that point
    /// either) this returns a facade over a freshly-constructed empty
    /// environment rather than raising, since a well-behaved extension
    /// only reads `app.env` from a listener connected for later
    /// (`builder-inited` or after).
    #[getter]
    fn env(&self, py: Python<'_>) -> PyResult<Py<PyEnvFacade>> {
        Py::new(
            py,
            PyEnvFacade::new(self.env.clone(), self.env_extra.clone()),
        )
    }

    /// Mirrors `Sphinx.srcdir` (a `pathlib.Path`).
    #[getter]
    fn srcdir(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        path_to_pyobject(py, &self.env.borrow().srcdir)
    }

    /// Mirrors `Sphinx.doctreedir` (a `pathlib.Path`).
    #[getter]
    fn doctreedir(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        path_to_pyobject(py, &self.env.borrow().doctreedir)
    }

    /// Mirrors `Sphinx.outdir` (a `pathlib.Path`). `None`/unset when this
    /// facade predates builder selection (see
    /// [`PyAppFacade::with_builder`]'s doc comment) — falls back to
    /// `env.doctreedir`'s parent as a best-effort guess rather than
    /// raising, since a listener reading `app.outdir` too early is a
    /// caller bug this bridge shouldn't crash over.
    #[getter]
    fn outdir(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &self.outdir {
            Some(p) => path_to_pyobject(py, p),
            None => path_to_pyobject(py, &self.env.borrow().doctreedir),
        }
    }

    /// Mirrors `Sphinx.builder` — a minimal [`PyBuilderFacade`] exposing
    /// only `.name`/`.format` (the two attributes real extensions actually
    /// read; see e.g. `sphinx/doc/conf.py`'s `build_redirects`, which
    /// checks `app.builder.name != 'html'`).
    #[getter]
    fn builder(&self, py: Python<'_>) -> PyResult<Py<PyBuilderFacade>> {
        let name = self.buildername.as_deref().cloned().unwrap_or_default();
        Py::new(py, PyBuilderFacade::new(name))
    }

    /// Mirrors `Sphinx.events` — a live [`PyEventsFacade`] over this app's
    /// [`SharedEvents`] (same handle `connect`/`disconnect`/`add_event`
    /// already use). A fresh facade is returned on every access (cheap
    /// `Rc` clone), same pattern as [`Self::config`]/[`Self::env`].
    #[getter]
    fn events(&self, py: Python<'_>) -> PyResult<Py<PyEventsFacade>> {
        Py::new(py, PyEventsFacade::new(self.events.clone()))
    }

    /// Mirrors `Sphinx.add_config_value(name, default, rebuild, types=())`:
    /// registers `name` in `app.config`'s store with `default` **only if
    /// it isn't already present** (a prior `add_config_value` call for the
    /// same name always wins, matching upstream's rejection of a genuine
    /// duplicate registration — kept permissive here since re-running
    /// `setup()` idempotently is more useful for this bridge than
    /// replicating that failure mode) — but when a name is registered for
    /// the *first* time, `conf.py`'s raw value (if any) always wins over
    /// `default`, exactly like upstream `Config.__getattr__`: `_raw_config`
    /// is consulted before falling back to a newly-registered option's
    /// default. Without this, an extension whose `setup()` runs
    /// `add_config_value("foo", True, ...)` after `conf.py` already set
    /// `foo = False` would silently see `True` — since [`SharedConfig`] is
    /// only pre-seeded with the built-in/already-registered options (see
    /// [`seed_shared_config`]), not with every raw `conf.py` name.
    #[pyo3(signature = (name, default, rebuild=None, types=None))]
    fn add_config_value(
        &self,
        py: Python<'_>,
        name: String,
        default: Py<PyAny>,
        rebuild: Option<Py<PyAny>>,
        types: Option<Py<PyAny>>,
    ) -> PyResult<()> {
        let _ = (rebuild, types);
        if self.config.borrow().contains_key(&name) {
            return Ok(());
        }
        let value = match self.raw_config.get(&name) {
            Some(raw) => configval_to_py(py, raw)?,
            None => default,
        };
        self.config.borrow_mut().insert(name, value);
        Ok(())
    }

    /// Mirrors `Sphinx.add_directive(name, cls, override=False)`: records
    /// `name` → `cls.__name__` in the shared [`SharedRegistry`]
    /// (`**_kwargs` absorbs `override` and any newer upstream keyword
    /// without erroring). Callable handlers also enter docutilsrs' parse-time
    /// `(args, body) -> replacement RST` registry; full docutils
    /// `Directive` class construction remains an accepted deviation.
    #[pyo3(signature = (name, cls, *_args, **_kwargs))]
    fn add_directive(
        &self,
        name: String,
        cls: Bound<'_, PyAny>,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<()> {
        if cls.is_callable() {
            docutilsrs::plugins::register_python_directive(&name, cls.clone().unbind());
        }
        let class_name = cls
            .getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_else(|_| cls.repr().map(|s| s.to_string()).unwrap_or_default());
        self.registry.borrow_mut().add_directive(name, class_name);
        Ok(())
    }

    /// Mirrors `Sphinx.add_role(name, role, override=False)`: records
    /// `name` → `role.__name__` in the shared [`SharedRegistry`] and enters
    /// callable roles into docutilsrs' parse-time inline-role bridge.
    #[pyo3(signature = (name, role, *_args, **_kwargs))]
    fn add_role(
        &self,
        name: String,
        role: Bound<'_, PyAny>,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<()> {
        if role.is_callable() {
            docutilsrs::plugins::register_python_role(&name, role.clone().unbind());
        }
        let class_name = role
            .getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_else(|_| role.repr().map(|s| s.to_string()).unwrap_or_default());
        self.registry.borrow_mut().add_role(name, class_name);
        Ok(())
    }

    /// Mirrors `Sphinx.add_object_type(directivename, rolename, indextemplate='',
    /// parse_node=None, ref_nodeclass=None, objname='', doc_field_types=[],
    /// override=False)`: records `directivename` → `rolename` in the
    /// shared [`SharedRegistry`] (bookkeeping only — see
    /// [`crate::registry::SphinxComponentRegistry::object_types`]'s doc
    /// comment; there is no generic std-domain object-description renderer
    /// in this port for a registration to actually drive). Accepts and
    /// discards every other positional/keyword argument via `*_args`/
    /// `**_kwargs`, matching this module's `add_directive`/`add_role`
    /// pattern of tolerating upstream's full, wider signature without
    /// erroring.
    #[pyo3(signature = (directivename, rolename, *_args, **_kwargs))]
    fn add_object_type(
        &self,
        directivename: String,
        rolename: String,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<()> {
        self.registry
            .borrow_mut()
            .add_object_type(directivename, rolename);
        Ok(())
    }

    /// Mirrors `Sphinx.add_domain(domain, override=False)`: records
    /// `domain.name` in the shared [`SharedRegistry`] (`**_kwargs` absorbs
    /// `override` and any newer upstream keyword without erroring). See
    /// this module's doc comment for what registering unlocks (name-based
    /// bookkeeping/lookup) versus what it doesn't (the domain's own
    /// roles/directives aren't executed by this bridge).
    #[pyo3(signature = (domain, *_args, **_kwargs))]
    fn add_domain(
        &self,
        domain: Bound<'_, PyAny>,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<()> {
        let name: String = domain.getattr("name")?.extract()?;
        let class_name = domain
            .getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_else(|_| name.clone());
        self.registry.borrow_mut().add_domain(name, class_name);
        Ok(())
    }

    /// Mirrors `Sphinx.add_html_theme(name, theme_path)`: records the
    /// theme directory in the shared [`SharedRegistry`]. `SphinxApp`
    /// synchronizes this registry with the application and environment
    /// configs after extension setup and before theme resolution.
    fn add_html_theme(&self, name: String, theme_path: String) {
        self.registry
            .borrow_mut()
            .add_html_theme(name, std::path::PathBuf::from(theme_path));
    }

    /// Mirrors `Sphinx.add_builder(builder, override=False)`: records
    /// `builder.name` in the shared [`SharedRegistry`]. See this module's
    /// doc comment for the accepted deviation (bookkeeping only — `-b
    /// <name>` dispatch is still the hardcoded native match).
    #[pyo3(signature = (builder, *_args, **_kwargs))]
    fn add_builder(
        &self,
        builder: Bound<'_, PyAny>,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<()> {
        let name: String = builder.getattr("name")?.extract()?;
        let class_name = builder
            .getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_else(|_| name.clone());
        self.registry.borrow_mut().add_builder(name, class_name);
        Ok(())
    }

    /// Mirrors `Sphinx.add_node(node, override=False, **kwargs)` (see ADR
    /// 0006, `docs/adr/0006-extension-nodes.md`): extracts
    /// `node.__name__` as `class_name`, then for every `(format, pair)`
    /// entry in `**kwargs` (the `override` keyword, if present, is
    /// skipped — it isn't a format) registers `pair` as a `(visit,
    /// depart)` callable pair in `docutilsrs::plugins`'s per-`(class_name,
    /// format)` registry. Every native writer consults that registry
    /// directly at render time for a `NodeKind::Extension` node carrying
    /// this `class_name`, calling `visit`/`depart` (each `(attrs: dict)
    /// -> str`) around its children's own rendering, or falling back to
    /// rendering children only if nothing is registered for that writer's
    /// format — matching upstream's base-class/`unknown_visit` dispatch
    /// fallback. `pair` may be a 2-tuple `(visit, depart)` or a single
    /// `visit` callable (`depart` omitted, matching upstream's
    /// optional-depart shape); anything else for a given format is
    /// silently skipped (fail-soft, consistent with the rest of this
    /// bridge). Also records `class_name` → registered formats in the
    /// shared [`SharedRegistry`] (`SphinxComponentRegistry::nodes`) for
    /// discoverability. **Accepted deviation (still, even after this ADR
    /// lands):** this only wires up *rendering* an `Extension` node that
    /// already exists in a doctree — a custom directive's `run()`
    /// actually *constructing* one is a separate, larger gap (the
    /// doctree/Node Python bridge; see this module's doc comment and the
    /// ADR's "Related/out of scope" section).
    #[pyo3(signature = (cls, *_args, **kwargs))]
    fn add_node(
        &self,
        cls: Bound<'_, PyAny>,
        _args: &Bound<'_, PyTuple>,
        kwargs: Option<Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let class_name = cls
            .getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_else(|_| cls.repr().map(|s| s.to_string()).unwrap_or_default());
        let Some(kwargs) = kwargs else {
            return Ok(());
        };
        for (key, value) in kwargs.iter() {
            let Ok(format) = key.extract::<String>() else {
                continue;
            };
            if format == "override" {
                continue;
            }
            let (visit, depart) = if let Ok(tup) = value.cast::<PyTuple>() {
                match tup.len() {
                    1 => (tup.get_item(0)?.unbind(), None),
                    2 => (tup.get_item(0)?.unbind(), Some(tup.get_item(1)?.unbind())),
                    _ => continue,
                }
            } else if value.hasattr("__call__").unwrap_or(false) {
                (value.clone().unbind(), None)
            } else {
                continue;
            };
            docutilsrs::plugins::register_node_visitor(
                class_name.clone(),
                format.clone(),
                visit,
                depart,
            );
            self.registry
                .borrow_mut()
                .add_node(class_name.clone(), format);
        }
        Ok(())
    }

    /// Mirrors `Sphinx.add_post_transform(transform)`: records the
    /// transform class's name in the shared [`SharedRegistry`]'s
    /// post-transform list. **Accepted deviation:** bookkeeping only — no
    /// write-phase hook in this crate actually instantiates and runs a
    /// registered post-transform against a doctree yet (same blocker as
    /// [`Self::add_node`]).
    fn add_post_transform(&self, transform: Bound<'_, PyAny>) -> PyResult<()> {
        let class_name = transform
            .getattr("__name__")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_else(|_| transform.repr().map(|s| s.to_string()).unwrap_or_default());
        self.registry.borrow_mut().add_post_transform(class_name);
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_events::AppEventManager;
    use crate::config::SphinxConfig;
    use crate::environment::{BuildEnvironment, EnvProject};
    use crate::registry::SphinxComponentRegistry;
    use std::collections::HashSet;
    use std::ffi::CString;

    fn make_env() -> crate::environment::SharedEnv {
        let config = SphinxConfig::new_defaults();
        let project = EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]);
        Rc::new(RefCell::new(BuildEnvironment::new(
            config,
            project,
            "/tmp/src",
            "/tmp/doctrees",
        )))
    }

    fn make_facade() -> PyAppFacade {
        make_facade_with_raw(HashMap::new())
    }

    fn make_facade_with_raw(raw: HashMap<String, ConfigVal>) -> PyAppFacade {
        PyAppFacade::new(
            AppEventManager::shared(),
            Rc::new(RefCell::new(HashMap::new())),
            Rc::new(RefCell::new(AssetAccumulator::default())),
            Rc::new(RefCell::new(SphinxComponentRegistry::new())),
            make_env(),
            Rc::new(raw),
            Rc::new(RefCell::new(HashMap::new())),
        )
    }

    fn py_def<'py>(py: Python<'py>, name: &str, body: &str) -> Py<PyAny> {
        let globals = PyDict::new(py);
        let code = CString::new(format!("def {name}(*args):\n    {body}\n")).unwrap();
        py.run(&code, Some(&globals), None).unwrap();
        globals.get_item(name).unwrap().unwrap().unbind()
    }

    #[test]
    fn py_err_to_event_error_formats_handler_and_message() {
        Python::attach(|py| {
            let callback = py_def(py, "boom", "raise ValueError('x')");
            let err = callback.bind(py).call0().unwrap_err();
            let event_err = py_err_to_event_error(py, "myevent", &callback, err);
            assert!(event_err.0.contains("myevent"));
            assert!(event_err.0.contains("threw an exception"));
        });
    }

    #[test]
    fn event_arg_to_py_converts_every_variant() {
        Python::attach(|py| {
            assert!(event_arg_to_py(py, &EventArg::None).unwrap().is_none(py));
            assert_eq!(
                event_arg_to_py(py, &EventArg::Str("hi".into()))
                    .unwrap()
                    .extract::<String>(py)
                    .unwrap(),
                "hi"
            );
            let list = event_arg_to_py(py, &EventArg::StrList(vec!["a".into(), "b".into()]))
                .unwrap();
            assert_eq!(
                list.extract::<Vec<String>>(py).unwrap(),
                vec!["a".to_string(), "b".to_string()]
            );
            let tree = docutilsrs::doctree::Doctree::new_document("src.rst");
            let doctree_obj = event_arg_to_py(py, &EventArg::Doctree(tree)).unwrap();
            assert!(doctree_obj.bind(py).cast::<PyDoctree>().is_ok());

            let page_context = EventArg::HtmlPageContext {
                pagename: "index".into(),
                templatename: "page.html".into(),
                context: Rc::new(RefCell::new(Default::default())),
                doctree: None,
            };
            assert!(event_arg_to_py(py, &page_context).is_err());
        });
    }

    #[test]
    fn json_value_to_py_converts_every_variant() {
        Python::attach(|py| {
            assert!(json_value_to_py(py, &serde_json::Value::Null)
                .unwrap()
                .is_none(py));
            assert!(json_value_to_py(py, &serde_json::json!(true))
                .unwrap()
                .extract::<bool>(py)
                .unwrap());
            assert_eq!(
                json_value_to_py(py, &serde_json::json!(7))
                    .unwrap()
                    .extract::<i64>(py)
                    .unwrap(),
                7
            );
            assert!(
                (json_value_to_py(py, &serde_json::json!(1.5))
                    .unwrap()
                    .extract::<f64>(py)
                    .unwrap()
                    - 1.5)
                    .abs()
                    < f64::EPSILON
            );
            assert_eq!(
                json_value_to_py(py, &serde_json::json!("s"))
                    .unwrap()
                    .extract::<String>(py)
                    .unwrap(),
                "s"
            );
            let arr = json_value_to_py(py, &serde_json::json!([1, "two"])).unwrap();
            assert_eq!(arr.bind(py).len().unwrap(), 2);
            let obj = json_value_to_py(py, &serde_json::json!({"k": "v"})).unwrap();
            assert_eq!(
                obj.bind(py)
                    .get_item("k")
                    .unwrap()
                    .extract::<String>()
                    .unwrap(),
                "v"
            );
        });
    }

    #[test]
    fn configval_to_py_converts_every_variant() {
        Python::attach(|py| {
            assert!(configval_to_py(py, &ConfigVal::Null).unwrap().is_none(py));
            assert!(configval_to_py(py, &ConfigVal::Bool(true))
                .unwrap()
                .extract::<bool>(py)
                .unwrap());
            assert_eq!(
                configval_to_py(py, &ConfigVal::Int(3))
                    .unwrap()
                    .extract::<i64>(py)
                    .unwrap(),
                3
            );
            assert!(
                (configval_to_py(py, &ConfigVal::Float(2.5))
                    .unwrap()
                    .extract::<f64>(py)
                    .unwrap()
                    - 2.5)
                    .abs()
                    < f64::EPSILON
            );
            assert_eq!(
                configval_to_py(py, &ConfigVal::Str("x".into()))
                    .unwrap()
                    .extract::<String>(py)
                    .unwrap(),
                "x"
            );
            let list = configval_to_py(
                py,
                &ConfigVal::List(vec![ConfigVal::Int(1), ConfigVal::Str("a".into())]),
            )
            .unwrap();
            assert_eq!(list.bind(py).len().unwrap(), 2);
            let map = configval_to_py(
                py,
                &ConfigVal::Map(vec![("k".to_string(), ConfigVal::Bool(false))]),
            )
            .unwrap();
            assert!(!map
                .bind(py)
                .get_item("k")
                .unwrap()
                .extract::<bool>()
                .unwrap());
        });
    }

    #[test]
    fn seed_shared_config_populates_from_sphinx_config() {
        Python::attach(|py| {
            let config = SphinxConfig::new_defaults();
            let shared = seed_shared_config(py, &config);
            assert!(!shared.borrow().is_empty());
        });
    }

    #[test]
    fn config_facade_getattr_setattr_contains() {
        Python::attach(|py| {
            let store: SharedConfig = Rc::new(RefCell::new(HashMap::new()));
            let facade = PyConfigFacade::new(store.clone());
            assert!(facade.__getattr__(py, "missing").is_none(py));
            assert!(!facade.__contains__("missing"));
            facade.__setattr__("project".to_string(), "MyProj".into_pyobject(py).unwrap().into_any().unbind());
            assert!(facade.__contains__("project"));
            assert_eq!(
                facade.__getattr__(py, "project").extract::<String>(py).unwrap(),
                "MyProj"
            );
        });
    }

    #[test]
    fn kwargs_to_attrs_handles_missing_and_present_and_non_string_keys() {
        Python::attach(|py| {
            assert!(kwargs_to_attrs(None).is_empty());
            let dict = PyDict::new(py);
            dict.set_item("integrity", "sha256-xyz").unwrap();
            dict.set_item(1, "skip-me").unwrap();
            let attrs = kwargs_to_attrs(Some(&dict));
            assert_eq!(attrs.get("integrity").unwrap(), "sha256-xyz");
            assert_eq!(attrs.len(), 1);
        });
    }

    #[test]
    fn py_args_to_event_args_converts_none_and_values() {
        Python::attach(|py| {
            let tuple = PyTuple::new(py, [py.None(), 5i64.into_pyobject(py).unwrap().into_any().unbind()]).unwrap();
            let args = py_args_to_event_args(&tuple).unwrap();
            assert_eq!(args.len(), 2);
            assert!(matches!(args[0], EventArg::None));
            assert!(matches!(&args[1], EventArg::Str(s) if s == "5"));
        });
    }

    #[test]
    fn events_facade_emit_add_event_and_firstresult() {
        Python::attach(|py| {
            let events = AppEventManager::shared();
            let facade = PyEventsFacade::new(events.clone());
            facade.add_event("custom").unwrap();
            let seen = Rc::new(RefCell::new(false));
            let seen_clone = seen.clone();
            events.borrow_mut().connect("custom", 0, move |_| {
                *seen_clone.borrow_mut() = true;
                Ok(())
            });
            let args = PyTuple::empty(py);
            let result = facade.emit("custom", &args).unwrap();
            assert!(result.is_empty());
            assert!(*seen.borrow());
            let none = facade.emit_firstresult(py, "custom", &args).unwrap();
            assert!(none.is_none(py));
        });
    }

    #[test]
    fn builder_facade_reports_name_and_format() {
        for (name, expected_format) in [
            ("html", "html"),
            ("dirhtml", "html"),
            ("singlehtml", "html"),
            ("latex", "latex"),
        ] {
            let facade = PyBuilderFacade::new(name.to_string());
            assert_eq!(facade.name(), name);
            assert_eq!(facade.format(), expected_format);
        }
    }

    #[test]
    fn app_facade_basic_getters_without_builder() {
        Python::attach(|py| {
            let facade = make_facade();
            assert!(facade.config(py).is_ok());
            assert!(facade.env(py).is_ok());
            assert!(facade.srcdir(py).is_ok());
            assert!(facade.doctreedir(py).is_ok());
            // No `with_builder` call: falls back to doctreedir.
            assert!(facade.outdir(py).is_ok());
            assert_eq!(facade.builder(py).unwrap().borrow(py).name(), "");
            assert!(facade.events(py).is_ok());
        });
    }

    #[test]
    fn app_facade_with_builder_reports_outdir_and_buildername() {
        Python::attach(|py| {
            let facade = make_facade();
            let facade = PyAppFacade::with_builder(
                facade,
                std::rc::Rc::new(std::path::PathBuf::from("/tmp/out")),
                std::rc::Rc::new("html".to_string()),
            );
            assert!(facade.outdir(py).is_ok());
            assert_eq!(facade.builder(py).unwrap().borrow(py).name(), "html");
        });
    }

    #[test]
    fn add_config_value_prefers_existing_then_raw_then_default() {
        Python::attach(|py| {
            // Case 1: name already present -> no-op, existing value kept.
            let facade = make_facade();
            facade.config.borrow_mut().insert(
                "already".to_string(),
                "existing".into_pyobject(py).unwrap().into_any().unbind(),
            );
            facade
                .add_config_value(py, "already".to_string(), py.None(), None, None)
                .unwrap();
            assert_eq!(
                facade
                    .config
                    .borrow()
                    .get("already")
                    .unwrap()
                    .extract::<String>(py)
                    .unwrap(),
                "existing"
            );

            // Case 2: name absent, raw_config has a value -> raw wins.
            let mut raw = HashMap::new();
            raw.insert("from_conf".to_string(), ConfigVal::Bool(false));
            let facade = make_facade_with_raw(raw);
            facade
                .add_config_value(
                    py,
                    "from_conf".to_string(),
                    true.into_pyobject(py).unwrap().to_owned().into_any().unbind(),
                    None,
                    None,
                )
                .unwrap();
            assert!(!facade
                .config
                .borrow()
                .get("from_conf")
                .unwrap()
                .extract::<bool>(py)
                .unwrap());

            // Case 3: name absent, no raw value -> default is used.
            let facade = make_facade();
            facade
                .add_config_value(
                    py,
                    "brand_new".to_string(),
                    "default-val".into_pyobject(py).unwrap().into_any().unbind(),
                    None,
                    None,
                )
                .unwrap();
            assert_eq!(
                facade
                    .config
                    .borrow()
                    .get("brand_new")
                    .unwrap()
                    .extract::<String>(py)
                    .unwrap(),
                "default-val"
            );
        });
    }

    #[test]
    fn add_directive_and_add_role_register_callable_and_non_callable() {
        Python::attach(|py| {
            let facade = make_facade();
            let empty_args = PyTuple::empty(py);
            let callable = py_def(py, "my_directive", "return 'ok'");
            facade
                .add_directive(
                    "mydirective".to_string(),
                    callable.bind(py).clone(),
                    &empty_args,
                    None,
                )
                .unwrap();
            assert!(facade.registry.borrow().has_directive("mydirective"));

            // Non-callable class-like object (falls back to repr()).
            let non_callable = 42i64.into_pyobject(py).unwrap().into_any();
            facade
                .add_directive("otherdirective".to_string(), non_callable, &empty_args, None)
                .unwrap();
            assert!(facade.registry.borrow().has_directive("otherdirective"));

            let role_callable = py_def(py, "my_role", "return 'ok'");
            facade
                .add_role(
                    "myrole".to_string(),
                    role_callable.bind(py).clone(),
                    &empty_args,
                    None,
                )
                .unwrap();
            assert!(facade.registry.borrow().has_role("myrole"));
        });
    }

    #[test]
    fn add_object_type_domain_theme_and_builder_register_bookkeeping() {
        Python::attach(|py| {
            let facade = make_facade();
            let empty_args = PyTuple::empty(py);
            facade
                .add_object_type(
                    "confval".to_string(),
                    "confval".to_string(),
                    &empty_args,
                    None,
                )
                .unwrap();

            let globals = PyDict::new(py);
            let code = CString::new(
                "class MyDomain:\n    name = 'mydomain'\n",
            )
            .unwrap();
            py.run(&code, Some(&globals), None).unwrap();
            let domain_cls = globals.get_item("MyDomain").unwrap().unwrap();
            let domain_instance = domain_cls.call0().unwrap();
            facade
                .add_domain(domain_instance, &empty_args, None)
                .unwrap();
            assert!(facade.registry.borrow().has_domain("mydomain"));

            facade.add_html_theme("mytheme".to_string(), "/tmp/theme".to_string());

            let builder_globals = PyDict::new(py);
            let builder_code = CString::new(
                "class MyBuilder:\n    name = 'mybuilder'\n",
            )
            .unwrap();
            py.run(&builder_code, Some(&builder_globals), None)
                .unwrap();
            let builder_cls = builder_globals.get_item("MyBuilder").unwrap().unwrap();
            let builder_instance = builder_cls.call0().unwrap();
            facade
                .add_builder(builder_instance, &empty_args, None)
                .unwrap();
            assert!(facade.registry.borrow().has_builder("mybuilder"));
        });
    }

    #[test]
    fn add_node_registers_pair_single_and_skips_invalid_and_override() {
        Python::attach(|py| {
            let facade = make_facade();
            let empty_args = PyTuple::empty(py);
            let globals = PyDict::new(py);
            let code = CString::new(
                "class MyNode:\n    pass\ndef visit(attrs):\n    return ''\ndef depart(attrs):\n    return ''\n",
            )
            .unwrap();
            py.run(&code, Some(&globals), None).unwrap();
            let node_cls = globals.get_item("MyNode").unwrap().unwrap();
            let visit = globals.get_item("visit").unwrap().unwrap();
            let depart = globals.get_item("depart").unwrap().unwrap();

            let kwargs = PyDict::new(py);
            kwargs
                .set_item("html", PyTuple::new(py, [&visit, &depart]).unwrap())
                .unwrap();
            kwargs.set_item("text", &visit).unwrap();
            kwargs
                .set_item("bogus", PyTuple::new(py, [&visit, &depart, &visit]).unwrap())
                .unwrap();
            kwargs.set_item("override", true).unwrap();
            kwargs.set_item("nothing", py.None()).unwrap();

            facade
                .add_node(node_cls.clone(), &empty_args, Some(kwargs))
                .unwrap();
            let registry = facade.registry.borrow();
            let formats = registry.get_node_formats("MyNode").unwrap();
            assert!(formats.contains(&"html".to_string()));
            assert!(formats.contains(&"text".to_string()));
            assert!(!formats.contains(&"bogus".to_string()));
            assert!(!formats.contains(&"override".to_string()));
            assert!(!formats.contains(&"nothing".to_string()));
            drop(registry);

            // No kwargs at all -> early return, no panic.
            facade.add_node(node_cls, &empty_args, None).unwrap();
        });
    }

    #[test]
    fn add_post_transform_records_class_name() {
        Python::attach(|py| {
            let facade = make_facade();
            let globals = PyDict::new(py);
            let code = CString::new("class MyTransform:\n    pass\n").unwrap();
            py.run(&code, Some(&globals), None).unwrap();
            let cls = globals.get_item("MyTransform").unwrap().unwrap();
            facade.add_post_transform(cls).unwrap();
        });
    }

    #[test]
    fn add_css_and_js_file_variants() {
        Python::attach(|py| {
            let facade = make_facade();
            facade.add_css_file("style.css".to_string(), 500, None);
            assert_eq!(facade.assets.borrow().css_files.len(), 1);

            let kwargs = PyDict::new(py);
            kwargs.set_item("integrity", "abc").unwrap();
            facade.add_css_file("other.css".to_string(), 200, Some(kwargs));
            assert_eq!(facade.assets.borrow().css_files.len(), 2);

            // filename = None -> not recorded.
            facade.add_js_file(None, 500, None, None);
            assert!(facade.assets.borrow().js_files.is_empty());

            facade.add_js_file(
                Some("script.js".to_string()),
                500,
                Some("async".to_string()),
                None,
            );
            assert_eq!(facade.assets.borrow().js_files.len(), 1);
        });
    }

    #[test]
    fn autodocumenter_setup_extension_and_require_sphinx_are_noops() {
        Python::attach(|py| {
            let facade = make_facade();
            let empty_args = PyTuple::empty(py);
            facade.add_autodocumenter(&empty_args, None);
            facade.setup_extension(&empty_args, None);
            facade.require_sphinx(&py.None().into_bound(py));
        });
    }

    #[test]
    fn connect_and_disconnect_dispatch_and_remove_listener() {
        Python::attach(|py| {
            let facade = make_facade();
            let globals = PyDict::new(py);
            let code = CString::new(
                "APP_CALLS = []\ndef handler(app, *args):\n    APP_CALLS.append(1)\n",
            )
            .unwrap();
            py.run(&code, Some(&globals), None).unwrap();
            let handler = globals.get_item("handler").unwrap().unwrap().unbind();

            let id = facade.connect("doctree-resolved", handler, 500).unwrap();
            facade
                .events
                .borrow_mut()
                .emit("doctree-resolved", &[EventArg::Str("index".to_string())])
                .unwrap();
            assert_eq!(
                globals
                    .get_item("APP_CALLS")
                    .unwrap()
                    .unwrap()
                    .extract::<Vec<i64>>()
                    .unwrap()
                    .len(),
                1
            );

            facade.disconnect(id).unwrap();
            facade
                .events
                .borrow_mut()
                .emit("doctree-resolved", &[EventArg::Str("index".to_string())])
                .unwrap();
            assert_eq!(
                globals
                    .get_item("APP_CALLS")
                    .unwrap()
                    .unwrap()
                    .extract::<Vec<i64>>()
                    .unwrap()
                    .len(),
                1
            );
        });
    }

    #[test]
    fn connect_config_inited_passes_second_config_argument() {
        Python::attach(|py| {
            let facade = make_facade();
            let globals = PyDict::new(py);
            let code = CString::new(
                "SEEN = []\ndef handler(app, config):\n    SEEN.append(config.__class__.__name__)\n",
            )
            .unwrap();
            py.run(&code, Some(&globals), None).unwrap();
            let handler = globals.get_item("handler").unwrap().unwrap().unbind();
            facade.connect("config-inited", handler, 500).unwrap();
            facade
                .events
                .borrow_mut()
                .emit("config-inited", &[])
                .unwrap();
            let seen = globals
                .get_item("SEEN")
                .unwrap()
                .unwrap()
                .extract::<Vec<String>>()
                .unwrap();
            assert_eq!(seen, vec!["_ConfigFacade".to_string()]);
        });
    }

    #[test]
    fn connect_html_page_context_expands_and_writes_back_context() {
        Python::attach(|py| {
            let facade = make_facade();
            let globals = PyDict::new(py);
            let code = CString::new(
                "def handler(app, pagename, templatename, context, doctree):\n    context['injected'] = 'yes'\n",
            )
            .unwrap();
            py.run(&code, Some(&globals), None).unwrap();
            let handler = globals.get_item("handler").unwrap().unwrap().unbind();
            facade
                .connect("html-page-context", handler, 500)
                .unwrap();

            let context = Rc::new(RefCell::new(std::collections::BTreeMap::new()));
            facade
                .events
                .borrow_mut()
                .emit(
                    "html-page-context",
                    &[EventArg::HtmlPageContext {
                        pagename: "index".to_string(),
                        templatename: "page.html".to_string(),
                        context: context.clone(),
                        doctree: None,
                    }],
                )
                .unwrap();
            assert_eq!(
                context.borrow().get("injected"),
                Some(&serde_json::Value::String("yes".to_string()))
            );
        });
    }

    #[test]
    fn connect_wraps_callback_exception_as_event_error() {
        Python::attach(|py| {
            let facade = make_facade();
            let handler = py_def(py, "boom_handler", "raise ValueError('boom')");
            facade.connect("build-finished", handler, 500).unwrap();
            let result = facade.events.borrow_mut().emit("build-finished", &[]);
            assert!(result.is_err());
        });
    }

    #[test]
    fn env_facade_getters_and_setters_cover_all_fields() {
        Python::attach(|py| {
            let env = make_env();
            {
                let mut e = env.borrow_mut();
                e.project
                    .docname_to_path
                    .insert("index".to_string(), "index.rst".to_string());
                e.all_docs.insert("index".to_string(), 100);
                e.titles.insert("index".to_string(), "Title".to_string());
                e.longtitles
                    .insert("index".to_string(), "Long Title".to_string());
                e.metadata.insert(
                    "index".to_string(),
                    HashMap::from([("author".to_string(), "me".to_string())]),
                );
                e.domaindata.insert(
                    "std".to_string(),
                    HashMap::from([("k".to_string(), "v".to_string())]),
                );
                e.dependencies
                    .insert("index".to_string(), HashSet::from(["dep.txt".to_string()]));
            }
            let extra: SharedEnvExtra = Rc::new(RefCell::new(HashMap::new()));
            let facade = PyEnvFacade::new(env.clone(), extra);

            assert!(facade
                .found_docs(py)
                .unwrap()
                .contains(py.None())
                .is_ok());
            assert_eq!(facade.all_docs(py).unwrap().len(), 1);
            assert_eq!(facade.titles(py).unwrap().len(), 1);
            assert_eq!(facade.longtitles(py).unwrap().len(), 1);
            assert!(facade.srcdir(py).is_ok());
            assert!(facade.doctreedir(py).is_ok());

            // doc2path: present in docname_to_path.
            let path = facade.doc2path(py, "index", true).unwrap();
            assert!(path
                .bind(py)
                .str()
                .unwrap()
                .to_string()
                .ends_with("index.rst"));

            // doc2path: fallback path (docname not registered).
            let fallback = facade.doc2path(py, "missing", false).unwrap();
            assert!(fallback.bind(py).str().unwrap().to_string().ends_with(".rst"));

            assert_eq!(
                facade
                    .metadata(py, "index")
                    .unwrap()
                    .get_item("author")
                    .unwrap()
                    .unwrap()
                    .extract::<String>()
                    .unwrap(),
                "me"
            );
            assert!(facade.metadata(py, "missing").unwrap().is_empty());

            assert_eq!(
                facade
                    .get_domaindata(py, "std")
                    .unwrap()
                    .get_item("k")
                    .unwrap()
                    .unwrap()
                    .extract::<String>()
                    .unwrap(),
                "v"
            );
            assert!(facade.get_domaindata(py, "missing").unwrap().is_empty());

            assert!(facade.get_temp_data("missing").unwrap().is_none());
            facade
                .set_temp_data(py, "key".to_string(), "value".into_pyobject(py).unwrap().into_any().unbind())
                .unwrap();
            assert_eq!(facade.get_temp_data("key").unwrap().unwrap(), "value");

            assert!(facade.get_ref_context("missing").unwrap().is_none());
            facade
                .set_ref_context(py, "mod".to_string(), "mymod".into_pyobject(py).unwrap().into_any().unbind())
                .unwrap();
            assert_eq!(facade.get_ref_context("mod").unwrap().unwrap(), "mymod");

            assert_eq!(facade.dependencies("index").unwrap(), vec!["dep.txt".to_string()]);
            facade.note_dependency("index", "extra.txt").unwrap();
            assert_eq!(
                facade.dependencies("index").unwrap(),
                vec!["dep.txt".to_string(), "extra.txt".to_string()]
            );

            // __getattr__/__setattr__ fallback to `extra`.
            assert!(facade.__getattr__(py, "custom_attr").is_err());
            facade
                .__setattr__("custom_attr".to_string(), 1i64.into_pyobject(py).unwrap().into_any().unbind());
            assert_eq!(
                facade
                    .__getattr__(py, "custom_attr")
                    .unwrap()
                    .extract::<i64>(py)
                    .unwrap(),
                1
            );
        });
    }

    #[test]
    fn borrow_env_mut_or_err_surfaces_conflicting_borrow_as_python_error() {
        Python::attach(|py| {
            let env = make_env();
            let extra: SharedEnvExtra = Rc::new(RefCell::new(HashMap::new()));
            let facade = PyEnvFacade::new(env.clone(), extra);
            let _held = env.borrow(); // held for the whole call below
            let result = facade.set_temp_data(
                py,
                "key".to_string(),
                "value".into_pyobject(py).unwrap().into_any().unbind(),
            );
            assert!(result.is_err());
        });
    }
}
