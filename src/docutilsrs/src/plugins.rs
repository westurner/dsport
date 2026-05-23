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

use crate::doctree::{Doctree, NodeId, NodeKind};

fn registry() -> &'static Mutex<HashMap<String, Py<PyAny>>> {
    static R: OnceLock<Mutex<HashMap<String, Py<PyAny>>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

fn transform_registry() -> &'static Mutex<Vec<(String, Py<PyAny>)>> {
    static R: OnceLock<Mutex<Vec<(String, Py<PyAny>)>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(Vec::new()))
}

/// True if any Python transform plugin is currently registered.
pub fn has_transforms() -> bool {
    transform_registry()
        .lock()
        .map(|v| !v.is_empty())
        .unwrap_or(false)
}

/// Invoke every registered Python transform against `tree`. Each
/// transform is a Python callable that receives a read-only
/// [`crate::python::PyDoctree`] handle and returns a list of
/// `(node_id, new_text)` tuples; each tuple replaces the text of the
/// referenced `Text` node in-place.
///
/// Transforms that raise or return a non-list value are silently
/// skipped (the parser must not fail because of plugin bugs).
pub fn apply_transforms(tree: &mut Doctree) {
    if !has_transforms() {
        return;
    }
    let edits: Vec<(NodeId, String)> = Python::try_attach(|py| {
        let mut edits: Vec<(NodeId, String)> = Vec::new();
        // Clone callables out under the lock so the Python call below
        // does not hold the registry mutex.
        let callables: Vec<(String, Py<PyAny>)> = match transform_registry().lock() {
            Ok(g) => g
                .iter()
                .map(|(n, c)| (n.clone(), c.clone_ref(py)))
                .collect(),
            Err(_) => return edits,
        };
        // Build a single shared PyDoctree view by moving the current
        // tree out, handing it to Python, then taking it back. This is
        // safe because the GIL prevents concurrent access.
        let snapshot = std::mem::replace(tree, Doctree::new_document("<transient>"));
        let py_tree = Py::new(py, crate::python::PyDoctree::new(snapshot)).ok();
        let Some(py_tree) = py_tree else {
            return edits;
        };
        for (_name, cb) in callables {
            let Ok(result) = cb.bind(py).call1((py_tree.clone_ref(py),)) else {
                continue;
            };
            // Expected return: iterable of (int, str).
            let Ok(iter) = result.try_iter() else {
                continue;
            };
            for item in iter.flatten() {
                if let Ok((id, text)) = item.extract::<(usize, String)>() {
                    edits.push((id, text));
                }
            }
        }
        // Reclaim the doctree.
        let reclaimed = py_tree.borrow_mut(py).take_inner();
        *tree = reclaimed;
        edits
    })
    .unwrap_or_default();
    for (id, text) in edits {
        if id < tree.nodes_len() {
            if let NodeKind::Text(s) = &mut tree.node_mut(id).kind {
                *s = text;
            }
        }
    }
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

#[pyfunction(name = "register_transform")]
pub(crate) fn py_register_transform(name: &str, callable: Py<PyAny>) -> PyResult<()> {
    let mut guard = transform_registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("transform registry poisoned"))?;
    // Replace any existing entry with the same name to preserve uniqueness
    // by name while keeping insertion order otherwise.
    if let Some(slot) = guard.iter_mut().find(|(n, _)| n == name) {
        slot.1 = callable;
    } else {
        guard.push((name.to_string(), callable));
    }
    Ok(())
}

#[pyfunction(name = "unregister_transform")]
pub(crate) fn py_unregister_transform(name: &str) -> PyResult<bool> {
    let mut guard = transform_registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("transform registry poisoned"))?;
    let before = guard.len();
    guard.retain(|(n, _)| n != name);
    Ok(guard.len() != before)
}

#[pyfunction(name = "registered_transforms")]
pub(crate) fn py_registered_transforms(py: Python<'_>) -> PyResult<Vec<Py<PyString>>> {
    let guard = transform_registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("transform registry poisoned"))?;
    Ok(guard
        .iter()
        .map(|(k, _)| PyString::new(py, k).unbind())
        .collect())
}

#[pyfunction(name = "clear_transforms")]
pub(crate) fn py_clear_transforms() -> PyResult<()> {
    let mut guard = transform_registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("transform registry poisoned"))?;
    guard.clear();
    Ok(())
}
