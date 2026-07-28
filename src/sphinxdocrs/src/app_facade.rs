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
//! **Accepted deviation:** `add_config_value` / `add_directive` /
//! `add_role` / `add_domain` / `add_html_theme` are no-op stubs that only
//! prevent `AttributeError` from simple `setup()` functions that call them;
//! wiring them through to [`crate::registry::SphinxComponentRegistry`] /
//! `docutilsrs`'s directive-and-role registries is deferred to H5a/H5b
//! (directive/role execution) since those registries don't exist as
//! Python-callable surfaces yet.

use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use crate::app_events::{EventArg, SharedEvents};

fn event_arg_to_py(py: Python<'_>, arg: &EventArg) -> PyResult<Py<PyAny>> {
    Ok(match arg {
        EventArg::None => py.None(),
        EventArg::Str(s) => s.into_pyobject(py)?.into_any().unbind(),
        EventArg::StrList(items) => PyList::new(py, items)?.into_any().unbind(),
    })
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
}

impl PyAppFacade {
    pub fn new(events: SharedEvents) -> Self {
        Self { events }
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
        let mut mgr = self.events.borrow_mut();
        let id = mgr.connect(event.to_string(), priority, move |args: &[EventArg]| {
            let events_for_call = events_for_call.clone();
            let _ = Python::attach(|py| -> PyResult<()> {
                let facade = Py::new(py, PyAppFacade::new(events_for_call))?;
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

    /// No-op stub — see module docs. Accepts and ignores any positional /
    /// keyword arguments so `setup()` functions that call
    /// `add_config_value(name, default, rebuild, types=...)` don't crash.
    #[pyo3(signature = (*_args, **_kwargs))]
    fn add_config_value(
        &self,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
    ) {
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

    /// Mirrors `Sphinx.require_sphinx(version)` — accepted as a no-op
    /// since this port has no upstream Sphinx version to compare against.
    fn require_sphinx(&self, _version_info: &Bound<'_, PyAny>) {}
}
