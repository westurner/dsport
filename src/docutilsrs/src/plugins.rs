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

#![allow(clippy::type_complexity)]

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

pub type NativeDirectiveHandler =
    Box<dyn Fn(&str, &[&str]) -> Option<Vec<crate::parser::Block>> + Send + Sync>;

pub type NativeRoleHandler = Box<dyn Fn(&str, &str) -> Option<NodeKind> + Send + Sync>;

fn native_directives() -> &'static Mutex<HashMap<String, NativeDirectiveHandler>> {
    static R: OnceLock<Mutex<HashMap<String, NativeDirectiveHandler>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

fn native_roles() -> &'static Mutex<HashMap<String, NativeRoleHandler>> {
    static R: OnceLock<Mutex<HashMap<String, NativeRoleHandler>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Register a native Rust directive handler for `name`.
pub fn register_native_directive<F>(name: &str, handler: F)
where
    F: Fn(&str, &[&str]) -> Option<Vec<crate::parser::Block>> + Send + Sync + 'static,
{
    if let Ok(mut map) = native_directives().lock() {
        map.insert(name.to_string(), Box::new(handler));
    }
}

/// Invoke a registered native directive handler for `name`.
pub fn invoke_native_directive(
    name: &str,
    args: &str,
    content: &[&str],
) -> Option<Vec<crate::parser::Block>> {
    if let Ok(map) = native_directives().lock() {
        if let Some(handler) = map.get(name) {
            return handler(args, content);
        }
    }
    None
}

/// Register a native Rust role handler for `name`.
pub fn register_native_role<F>(name: &str, handler: F)
where
    F: Fn(&str, &str) -> Option<NodeKind> + Send + Sync + 'static,
{
    if let Ok(mut map) = native_roles().lock() {
        map.insert(name.to_string(), Box::new(handler));
    }
}

/// Invoke a registered native role handler for `name`.
pub fn invoke_native_role(name: &str, role_name: &str, content: &str) -> Option<NodeKind> {
    if let Ok(map) = native_roles().lock() {
        if let Some(handler) = map.get(name) {
            return handler(role_name, content);
        }
    }
    None
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

// ── extension-node visit/depart registry (ADR 0006) ────────────────────────
//
// Backs `NodeKind::Extension`: a real Sphinx extension's
// `app.add_node(cls, **format_pairs)` registers a `(visit, depart)`
// callable pair per builder *format* (`"html"`, `"latex"`, `"man"`,
// `"text"`, `"odt"`, `"xml"`, ...) for its own node class. Keyed by
// `(class_name, format)` rather than just `class_name` because upstream's
// model is exactly that granular (a format with no registered pair falls
// back to rendering children only, matching upstream's
// `unknown_visit`/base-class dispatch fallback).
//
// Unlike the directive/transform registries above, this one is populated
// via a plain Rust function ([`register_node_visitor`]), not a
// `#[pyfunction]` — the only caller is
// `sphinxdocrs::app_facade::PyAppFacade::add_node`, a normal Rust call
// site (`docutilsrs` is an ordinary path dependency of `sphinxdocrs`, so
// no Python round-trip is needed to reach this registry from there).

fn node_visitor_registry()
-> &'static Mutex<HashMap<(String, String), (Py<PyAny>, Option<Py<PyAny>>)>> {
    static R: OnceLock<Mutex<HashMap<(String, String), (Py<PyAny>, Option<Py<PyAny>>)>>> =
        OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Register a `(visit, depart)` callable pair for `class_name` under
/// builder `format`. Overwrites any existing registration for the same
/// `(class_name, format)` pair (mirrors `override=True` semantics — the
/// caller is expected to have already checked `override` if that matters).
pub fn register_node_visitor(
    class_name: String,
    format: String,
    visit: Py<PyAny>,
    depart: Option<Py<PyAny>>,
) {
    if let Ok(mut guard) = node_visitor_registry().lock() {
        guard.insert((class_name, format), (visit, depart));
    }
}

/// True if a visit callable is registered for `(class_name, format)`.
pub fn has_node_visitor(class_name: &str, format: &str) -> bool {
    node_visitor_registry()
        .lock()
        .map(|g| g.contains_key(&(class_name.to_string(), format.to_string())))
        .unwrap_or(false)
}

/// Invoke the registered `visit` callable for `(class_name, format)`,
/// passing `attrs` as a plain `dict`. Returns the text it produced (the
/// node's "opening" markup, emitted before its children are rendered), or
/// `None` if nothing is registered or the callable raised / didn't return
/// a `str` — a plugin bug must never abort the surrounding build, the same
/// fail-soft contract as [`invoke_plugin`].
pub fn invoke_node_visit(
    class_name: &str,
    format: &str,
    attrs: &HashMap<String, String>,
) -> Option<String> {
    invoke_node_hook(class_name, format, attrs, true)
}

/// Same as [`invoke_node_visit`] but calls the `depart` callable (the
/// node's "closing" markup, emitted after its children have been
/// rendered). Returns `None` (renders no closing text) if no `depart` was
/// registered for this pair, even if a `visit` was.
pub fn invoke_node_depart(
    class_name: &str,
    format: &str,
    attrs: &HashMap<String, String>,
) -> Option<String> {
    invoke_node_hook(class_name, format, attrs, false)
}

fn invoke_node_hook(
    class_name: &str,
    format: &str,
    attrs: &HashMap<String, String>,
    is_visit: bool,
) -> Option<String> {
    Python::try_attach(|py| -> Option<String> {
        let cb = {
            let guard = node_visitor_registry().lock().ok()?;
            let (visit, depart) = guard.get(&(class_name.to_string(), format.to_string()))?;
            if is_visit {
                visit.clone_ref(py)
            } else {
                depart.as_ref()?.clone_ref(py)
            }
        };
        let dict = pyo3::types::PyDict::new(py);
        for (k, v) in attrs {
            dict.set_item(k, v).ok()?;
        }
        let result = cb.bind(py).call1((dict,)).ok()?;
        result.extract::<String>().ok()
    })?
}
