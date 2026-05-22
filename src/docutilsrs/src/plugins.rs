//! Python directive plugin bridge.
//!
//! Allows Python callers to register a callable for an unknown directive
//! name. When the rST parser encounters that directive, it invokes the
//! callable with the directive's argument string and indented body and
//! re-parses the returned string as a block of rST.
//!
//! This is the Phase 3 interop surface: Rust prefers its own directive
//! implementations, falling back to a Python plugin (if registered)
//! before degrading to the comment-swallow default.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use pyo3::prelude::*;
use pyo3::types::PyString;

fn registry() -> &'static Mutex<HashMap<String, Py<PyAny>>> {
    static R: OnceLock<Mutex<HashMap<String, Py<PyAny>>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

/// True if a Python plugin is registered for `name`.
pub fn has_plugin(name: &str) -> bool {
    registry()
        .lock()
        .map(|m| m.contains_key(name))
        .unwrap_or(false)
}

/// Invoke the registered plugin for `name`, passing `args` and `body`.
/// Returns the replacement rST string on success, or `None` if no plugin
/// is registered or the callable raised.
pub fn invoke_plugin(name: &str, args: &str, body: &str) -> Option<String> {
    Python::try_attach(|py| -> Option<String> {
        let cb = {
            let guard = registry().lock().ok()?;
            guard.get(name)?.clone_ref(py)
        };
        let result = cb.bind(py).call1((args, body)).ok()?;
        result.extract::<String>().ok()
    })?
}

#[pyfunction(name = "register_directive")]
pub(crate) fn py_register_directive(name: &str, callable: Py<PyAny>) -> PyResult<()> {
    let mut guard = registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("plugin registry poisoned"))?;
    guard.insert(name.to_string(), callable);
    Ok(())
}

#[pyfunction(name = "unregister_directive")]
pub(crate) fn py_unregister_directive(name: &str) -> PyResult<bool> {
    let mut guard = registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("plugin registry poisoned"))?;
    Ok(guard.remove(name).is_some())
}

#[pyfunction(name = "registered_directives")]
pub(crate) fn py_registered_directives(py: Python<'_>) -> PyResult<Vec<Py<PyString>>> {
    let guard = registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("plugin registry poisoned"))?;
    Ok(guard
        .keys()
        .map(|k| PyString::new(py, k).unbind())
        .collect())
}

#[pyfunction(name = "clear_directives")]
pub(crate) fn py_clear_directives() -> PyResult<()> {
    let mut guard = registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("plugin registry poisoned"))?;
    guard.clear();
    Ok(())
}
