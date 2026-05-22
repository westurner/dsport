//! `EventManager` port of `sphinx.events.EventManager`.
//!
//! Mirrors the upstream API:
//!
//! * [`EventManager::add`] registers a custom event name
//! * [`EventManager::connect`] returns a numeric listener id (insertion
//!   order, like upstream's `next_listener_id`)
//! * [`EventManager::disconnect`] removes a listener by id
//! * [`EventManager::emit`] fires handlers in ascending priority order,
//!   wraps unexpected exceptions in [`crate::errors::ExtensionError`],
//!   passes through `allowed_exceptions` and `SphinxError`, and
//!   bypasses wrapping entirely when the bound `app.pdb` is truthy
//! * [`EventManager::emit_firstresult`] returns the first non-`None`
//!   result
//!
//! The same set of `core_events` as upstream is pre-registered so
//! `connect`/`emit` with a stock event name does not require an
//! `add()` call first.

use std::sync::Mutex;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple, PyType};

use crate::errors::{ExtensionError, SphinxError};

const CORE_EVENTS: &[&str] = &[
    "config-inited",
    "builder-inited",
    "env-get-outdated",
    "env-before-read-docs",
    "env-purge-doc",
    "source-read",
    "include-read",
    "doctree-read",
    "env-merge-info",
    "env-updated",
    "env-get-updated",
    "env-check-consistency",
    "write-started",
    "doctree-resolved",
    "missing-reference",
    "warn-missing-reference",
    "build-finished",
];

struct Listener {
    id: usize,
    handler: Py<PyAny>,
    priority: i64,
}

struct Inner {
    events: Vec<String>, // known event names (order: insertion)
    listeners: Vec<(String, Listener)>,
    next_id: usize,
}

#[pyclass(name = "EventManager", module = "sphinxdocrs")]
pub struct EventManager {
    app: Py<PyAny>,
    inner: Mutex<Inner>,
}

#[pymethods]
impl EventManager {
    #[new]
    fn new(app: Py<PyAny>) -> Self {
        let events = CORE_EVENTS.iter().map(|s| (*s).to_string()).collect();
        EventManager {
            app,
            inner: Mutex::new(Inner {
                events,
                listeners: Vec::new(),
                next_id: 0,
            }),
        }
    }

    /// Register a custom event name. Raises `ExtensionError` if the
    /// name is already known.
    fn add(&self, py: Python<'_>, name: &str) -> PyResult<()> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| PyException::new_err("EventManager mutex poisoned"))?;
        if inner.events.iter().any(|n| n == name) {
            return Err(ExtensionError::new_err(format!(
                "Event '{name}' already present"
            )));
        }
        let _ = py; // future: hook logging here
        inner.events.push(name.to_string());
        Ok(())
    }

    /// Connect a callback to an event. Returns a listener id.
    fn connect(&self, name: &str, callback: Py<PyAny>, priority: i64) -> PyResult<usize> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| PyException::new_err("EventManager mutex poisoned"))?;
        if !inner.events.iter().any(|n| n == name) {
            return Err(ExtensionError::new_err(format!(
                "Unknown event name: {name}"
            )));
        }
        let id = inner.next_id;
        inner.next_id += 1;
        inner.listeners.push((
            name.to_string(),
            Listener {
                id,
                handler: callback,
                priority,
            },
        ));
        Ok(id)
    }

    /// Remove a listener previously registered with `connect`.
    fn disconnect(&self, listener_id: usize) -> PyResult<()> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| PyException::new_err("EventManager mutex poisoned"))?;
        inner.listeners.retain(|(_, l)| l.id != listener_id);
        Ok(())
    }

    /// Emit *name* with positional *args*, returning the list of
    /// non-`None` handler results (matching upstream behavior: all
    /// results, including `None`, are appended).
    #[pyo3(signature = (name, *args, allowed_exceptions = None))]
    fn emit<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        args: &Bound<'py, PyTuple>,
        allowed_exceptions: Option<Bound<'py, PyTuple>>,
    ) -> PyResult<Bound<'py, PyList>> {
        // Snapshot listeners under the lock, then release and call.
        let mut snapshot: Vec<(usize, Py<PyAny>, i64)> = {
            let inner = self
                .inner
                .lock()
                .map_err(|_| PyException::new_err("EventManager mutex poisoned"))?;
            if !inner.events.iter().any(|n| n == name) {
                return Err(ExtensionError::new_err(format!(
                    "Unknown event name: {name}"
                )));
            }
            inner
                .listeners
                .iter()
                .filter(|(n, _)| n == name)
                .map(|(_, l)| (l.id, l.handler.clone_ref(py), l.priority))
                .collect()
        };
        // Stable sort by priority (matches Python's stable `sorted`).
        snapshot.sort_by_key(|(_, _, p)| *p);

        let results = PyList::empty(py);

        // Build the full argument tuple (app, *args) once.
        let mut call_args: Vec<Py<PyAny>> = Vec::with_capacity(args.len() + 1);
        call_args.push(self.app.clone_ref(py));
        for a in args.iter() {
            call_args.push(a.unbind());
        }
        let call_tuple = PyTuple::new(py, &call_args)?;

        // Resolve `app.pdb` once (default False if attribute missing).
        let pdb = self
            .app
            .bind(py)
            .getattr("pdb")
            .ok()
            .and_then(|v| v.extract::<bool>().ok())
            .unwrap_or(false);

        for (_, handler, _) in snapshot {
            match handler.bind(py).call1(&call_tuple) {
                Ok(v) => {
                    results.append(v)?;
                }
                Err(err) => {
                    // Pass-through: pdb mode short-circuits everything.
                    if pdb {
                        return Err(err);
                    }
                    // Pass-through: allowed_exceptions and SphinxError.
                    if let Some(allowed) = allowed_exceptions.as_ref() {
                        if is_instance_of_any(py, &err, allowed)? {
                            return Err(err);
                        }
                    }
                    if err.is_instance_of::<SphinxError>(py) {
                        return Err(err);
                    }
                    let modname = handler
                        .bind(py)
                        .getattr("__module__")
                        .ok()
                        .and_then(|v| v.extract::<String>().ok())
                        .unwrap_or_default();
                    let handler_repr = handler
                        .bind(py)
                        .repr()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|_| "<handler>".into());
                    let msg =
                        format!("Handler {handler_repr} for event '{name}' threw an exception");
                    let _ = modname;
                    let new = ExtensionError::new_err(msg);
                    new.set_cause(py, Some(err));
                    return Err(new);
                }
            }
        }
        Ok(results)
    }

    /// Emit *name* and return the first non-`None` result, or `None`.
    #[pyo3(signature = (name, *args, allowed_exceptions = None))]
    fn emit_firstresult<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        args: &Bound<'py, PyTuple>,
        allowed_exceptions: Option<Bound<'py, PyTuple>>,
    ) -> PyResult<Option<Py<PyAny>>> {
        let results = self.emit(py, name, args, allowed_exceptions)?;
        for v in results.iter() {
            if !v.is_none() {
                return Ok(Some(v.unbind()));
            }
        }
        Ok(None)
    }

    /// Number of currently registered listeners (across all events).
    fn listener_count(&self) -> PyResult<usize> {
        Ok(self
            .inner
            .lock()
            .map_err(|_| PyException::new_err("EventManager mutex poisoned"))?
            .listeners
            .len())
    }

    /// Known event names, in registration order.
    fn known_events<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let inner = self
            .inner
            .lock()
            .map_err(|_| PyException::new_err("EventManager mutex poisoned"))?;
        let list = PyList::empty(py);
        for n in inner.events.iter() {
            list.append(n)?;
        }
        Ok(list)
    }
}

fn is_instance_of_any(py: Python<'_>, err: &PyErr, types: &Bound<'_, PyTuple>) -> PyResult<bool> {
    let inst = err.value(py);
    for ty in types.iter() {
        let ty: Bound<'_, PyType> = ty.cast_into()?;
        if inst.is_instance(&ty)? {
            return Ok(true);
        }
    }
    Ok(false)
}
