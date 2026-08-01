//! Python directive plugin bridge.
//!
//! Allows Python callers to register a callable directive handler. When the
//! rST parser encounters that directive, it invokes the callable with the
//! directive's argument string and indented body and re-parses the returned
//! string as a block of rST. Native handlers use the same registry boundary
//! and run before built-in directives.
//!
//! This is the Phase 3 interop surface: Rust prefers its own directive
//! implementations, falling back to a Python plugin (if registered)
//! before degrading to the comment-swallow default.

#![allow(clippy::type_complexity)]

use std::collections::{BTreeMap, HashMap};
use std::sync::{Mutex, OnceLock};

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString, PyTuple};

use crate::doctree::{Doctree, NodeAttributeValue, NodeId, NodeKind};

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

/// Invoke a registered Python directive. Plain callables use the compact
/// `(arguments, body) -> replacement RST` contract. Callable classes receive
/// the standard docutils Directive constructor arguments and may return a
/// list of docutils nodes; those nodes are lowered into native blocks.
pub fn invoke_python_directive(
    name: &str,
    args: &str,
    content: &[String],
    source_line: usize,
) -> Option<Vec<crate::parser::Block>> {
    Python::try_attach(|py| -> Option<Vec<crate::parser::Block>> {
        let callable = {
            let guard = registry().lock().ok()?;
            guard.get(name)?.clone_ref(py)
        };
        let body = content.join("\n");
        if let Ok(result) = callable.bind(py).call1((args, body.as_str())) {
            if let Ok(rst) = result.extract::<String>() {
                return Some(parse_plugin_rst(&rst));
            }
        }

        let options = PyDict::new(py);
        let mut body_lines = Vec::new();
        let option_spec = callable.bind(py).getattr("option_spec").ok();
        for line in content {
            if let Some((key, value)) = line
                .trim()
                .strip_prefix(':')
                .and_then(|s| s.split_once(':'))
                && !key.trim().is_empty()
            {
                let key = key.trim();
                let value = value.trim();
                let converted = option_spec
                    .as_ref()
                    .and_then(|spec| spec.get_item(key).ok())
                    .and_then(|converter| converter.call1((value,)).ok())
                    .unwrap_or_else(|| value.into_pyobject(py).unwrap().into_any());
                options.set_item(key, converted).ok()?;
            } else {
                body_lines.push(line.clone());
            }
        }
        let arguments: Vec<String> = if args.trim().is_empty() {
            Vec::new()
        } else {
            args.split_whitespace().map(str::to_owned).collect()
        };
        let content_list = PyList::new(py, &body_lines).ok()?;
        let state_machine = py
            .eval(
                c"type('DocutilsRsStateMachine', (), {'reporter': None, 'get_source_and_line': lambda self, lineno=None: (self.source, self.line), 'nested_parse': lambda self, content, offset, node: (lambda document: (__import__('docutils.parsers.rst', fromlist=['Parser']).Parser().parse('\\n'.join(content), document), node.extend(document.children))[1])(__import__('docutils.utils', fromlist=['new_document']).new_document(self.source))})()",
                None,
                None,
            )
            .ok()?;
        state_machine.setattr("source", format!("<{name}>")).ok()?;
        state_machine.setattr("line", source_line).ok()?;
        let instance = callable
            .bind(py)
            .call1((
                name,
                arguments,
                options,
                content_list,
                0,
                0,
                format!(".. {name}:: {args}"),
                py.None(),
                state_machine,
            ))
            .ok()?;
        let result = instance.call_method0("run").ok()?;
        let nodes = result.cast::<PyList>().ok()?;
        let mut blocks = Vec::new();
        for node in nodes.iter() {
            if let Some(block) = python_node_to_block(&node) {
                blocks.push(block);
            }
        }
        Some(blocks)
    })?
}

fn parse_plugin_rst(rst: &str) -> Vec<crate::parser::Block> {
    let lines: Vec<&str> = rst.lines().collect();
    crate::parser::parse_plugin_blocks(&lines)
}

fn python_node_attributes(node: &Bound<'_, PyAny>) -> BTreeMap<String, NodeAttributeValue> {
    node.getattr("attributes")
        .ok()
        .and_then(|value| value.cast::<PyDict>().ok().cloned())
        .map(|dict| {
            dict.iter()
                .filter_map(|(key, value)| {
                    Some((
                        key.extract::<String>().ok()?,
                        python_attribute_value(&value),
                    ))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn python_attribute_value(value: &Bound<'_, PyAny>) -> NodeAttributeValue {
    if value.is_none() {
        return NodeAttributeValue::None;
    }
    if let Ok(value) = value.extract::<bool>() {
        return NodeAttributeValue::Bool(value);
    }
    if let Ok(value) = value.extract::<i64>() {
        return NodeAttributeValue::Signed(value);
    }
    if let Ok(value) = value.extract::<u64>() {
        return NodeAttributeValue::Unsigned(value);
    }
    if let Ok(value) = value.extract::<f64>() {
        return if value.is_finite() {
            NodeAttributeValue::Float(value)
        } else {
            NodeAttributeValue::Opaque {
                type_name: "float".to_string(),
                repr: value.to_string(),
            }
        };
    }
    if let Ok(items) = value.cast::<PyList>() {
        return NodeAttributeValue::List(
            items
                .iter()
                .map(|item| python_attribute_value(&item))
                .collect(),
        );
    }
    if let Ok(dict) = value.cast::<PyDict>() {
        let mut values = BTreeMap::new();
        for (key, item) in dict.iter() {
            if let Ok(key) = key.extract::<String>() {
                values.insert(key, python_attribute_value(&item));
            } else {
                return NodeAttributeValue::Opaque {
                    type_name: value
                        .get_type()
                        .name()
                        .map(|name| name.to_string())
                        .unwrap_or_default(),
                    repr: value.str().map(|v| v.to_string()).unwrap_or_default(),
                };
            }
        }
        return NodeAttributeValue::Map(values);
    }
    NodeAttributeValue::Opaque {
        type_name: value
            .get_type()
            .name()
            .map(|name| name.to_string())
            .unwrap_or_default(),
        repr: value.str().map(|v| v.to_string()).unwrap_or_default(),
    }
}

fn attribute_text(attrs: &BTreeMap<String, NodeAttributeValue>, name: &str) -> String {
    match attrs.get(name) {
        Some(NodeAttributeValue::String(value)) => value.clone(),
        Some(value) => value.to_string(),
        None => String::new(),
    }
}

fn python_node_to_inline(node: &Bound<'_, PyAny>) -> Option<crate::parser::InlineBlock> {
    let tag = node.getattr("tagname").ok()?.extract::<String>().ok()?;
    if tag == "#text" {
        return Some(crate::parser::InlineBlock::Text(
            node.call_method0("astext").ok()?.extract::<String>().ok()?,
        ));
    }
    let children = node
        .getattr("children")
        .ok()
        .and_then(|value| value.cast::<PyList>().ok().cloned())
        .map(|items| {
            items
                .iter()
                .filter_map(|child| python_node_to_inline(&child))
                .collect()
        })
        .unwrap_or_default();
    match tag.as_str() {
        "emphasis" => Some(crate::parser::InlineBlock::Emphasis(children)),
        "strong" => Some(crate::parser::InlineBlock::Strong(children)),
        "literal" => Some(crate::parser::InlineBlock::Literal(children)),
        "title_reference" => Some(crate::parser::InlineBlock::TitleReference(children)),
        "inline" => Some(crate::parser::InlineBlock::Inline {
            classes: attribute_text(&python_node_attributes(node), "classes"),
            children,
        }),
        _ => Some(crate::parser::InlineBlock::Inline {
            classes: format!("role-{tag}"),
            children,
        }),
    }
}

fn python_node_to_block(node: &Bound<'_, PyAny>) -> Option<crate::parser::Block> {
    let tag = node.getattr("tagname").ok()?.extract::<String>().ok()?;
    let text = node
        .call_method0("astext")
        .ok()
        .and_then(|v| v.extract::<String>().ok())
        .unwrap_or_default();
    let children: Vec<crate::parser::Block> = node
        .getattr("children")
        .ok()
        .and_then(|v| v.cast::<PyList>().ok().cloned())
        .map(|items| {
            items
                .iter()
                .filter_map(|child| python_node_to_block(&child))
                .collect()
        })
        .unwrap_or_default();
    let attrs = python_node_attributes(node);
    match tag.as_str() {
        "paragraph" | "title" | "term" | "rubric" => {
            let inline_children: Vec<crate::parser::InlineBlock> = node
                .getattr("children")
                .ok()
                .and_then(|value| value.cast::<PyList>().ok().cloned())
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|child| python_node_to_inline(&child))
                        .collect()
                })
                .unwrap_or_default();
            if inline_children.is_empty() && !text.is_empty() {
                Some(crate::parser::Block::Paragraph { text, line: 0 })
            } else {
                Some(crate::parser::Block::RichParagraph {
                    children: inline_children,
                    line: 0,
                })
            }
        }
        "literal_block" => Some(crate::parser::Block::LiteralBlock {
            classes: attribute_text(&attrs, "classes"),
            text,
            tokens: None,
        }),
        "raw" => Some(crate::parser::Block::Raw {
            format: attribute_text(&attrs, "format"),
            text,
        }),
        "bullet_list" => Some(crate::parser::Block::BulletList {
            bullet: '*',
            items: children.into_iter().map(|child| vec![child]).collect(),
        }),
        _ => Some(crate::parser::Block::Extension {
            class_name: node
                .getattr("__class__")
                .ok()
                .and_then(|class| class.getattr("__name__").ok())
                .and_then(|name| name.extract::<String>().ok())
                .unwrap_or(tag),
            attrs,
            children,
        }),
    }
}

/// Register a Python callable as a parse-time directive handler.
///
/// The callable receives `(arguments, body)` and returns replacement RST.
/// This is the native parser's intentionally small extension contract; full
/// docutils `Directive` class construction remains outside this bridge.
pub fn register_python_directive(name: &str, callable: Py<PyAny>) {
    if let Ok(mut guard) = registry().lock() {
        guard.insert(name.to_string(), callable);
    }
}

/// Register a Python callable as a parse-time interpreted-text role handler.
///
/// The preferred bridge contract is `(text) -> replacement_text`. For
/// compatibility with the common docutils role shape, invocation also tries
/// `(name, rawtext, text, lineno, inliner, options, content)` before the
/// compact form. A returned string becomes an inline node.
pub fn register_python_role(name: &str, callable: Py<PyAny>) {
    if let Ok(mut guard) = role_registry().lock() {
        guard.insert(name.to_string(), callable);
    }
}

pub struct PythonRoleNode {
    pub kind: NodeKind,
    pub text: String,
}

pub struct PythonRoleResult {
    pub nodes: Vec<PythonRoleNode>,
    pub messages: Vec<String>,
}

fn role_registry() -> &'static Mutex<HashMap<String, Py<PyAny>>> {
    static R: OnceLock<Mutex<HashMap<String, Py<PyAny>>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Invoke a registered Python role and return replacement inline text.
pub fn invoke_python_role(name: &str, content: &str) -> Option<PythonRoleResult> {
    Python::try_attach(|py| -> Option<PythonRoleResult> {
        let callable = {
            let guard = role_registry().lock().ok()?;
            guard.get(name)?.clone_ref(py)
        };
        let fallback = callable.bind(py).call1((content,));
        if let Ok(result) = fallback {
            if let Some(result) = role_result_nodes(&result) {
                return Some(result);
            }
        }
        let result = callable.bind(py).call(
            (
                name,
                content,
                content,
                0,
                py.None(),
                None::<Py<PyAny>>,
                Vec::<String>::new(),
            ),
            None,
        );
        role_result_nodes(&result.ok()?)
    })?
}

fn role_result_nodes(result: &Bound<'_, PyAny>) -> Option<PythonRoleResult> {
    if let Ok(text) = result.extract::<String>() {
        return Some(PythonRoleResult {
            nodes: vec![PythonRoleNode {
                kind: NodeKind::Inline {
                    classes: "role".to_string(),
                },
                text,
            }],
            messages: Vec::new(),
        });
    }
    let tuple = result.cast::<PyTuple>().ok()?;
    let nodes_item = tuple.get_item(0).ok()?;
    let nodes = nodes_item.cast::<PyList>().ok()?;
    let mut output = Vec::new();
    for node in nodes.iter() {
        let tag = node.getattr("tagname").ok()?.extract::<String>().ok()?;
        let text = node.call_method0("astext").ok()?.extract::<String>().ok()?;
        let kind = match tag.as_str() {
            "emphasis" => NodeKind::Emphasis,
            "strong" => NodeKind::Strong,
            "literal" => NodeKind::Literal,
            "title_reference" => NodeKind::TitleReference,
            "inline" => NodeKind::Inline {
                classes: "role".to_string(),
            },
            _ => NodeKind::Inline {
                classes: format!("role-{tag}"),
            },
        };
        output.push(PythonRoleNode { kind, text });
    }
    let messages_item = tuple.get_item(1).ok()?;
    let messages = messages_item
        .cast::<PyList>()
        .ok()?
        .iter()
        .filter_map(|message| message.call_method0("astext").ok()?.extract().ok())
        .collect();
    Some(PythonRoleResult {
        nodes: output,
        messages,
    })
}

#[pyfunction(name = "register_directive")]
pub(crate) fn py_register_directive(name: &str, callable: Py<PyAny>) -> PyResult<()> {
    let mut guard = registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("plugin registry poisoned"))?;
    guard.insert(name.to_string(), callable);
    Ok(())
}

#[pyfunction(name = "register_role")]
pub(crate) fn py_register_role(name: &str, callable: Py<PyAny>) -> PyResult<()> {
    let mut guard = role_registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("role registry poisoned"))?;
    guard.insert(name.to_string(), callable);
    Ok(())
}

#[pyfunction(name = "unregister_role")]
pub(crate) fn py_unregister_role(name: &str) -> PyResult<bool> {
    let mut guard = role_registry()
        .lock()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("role registry poisoned"))?;
    Ok(guard.remove(name).is_some())
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
    attrs: &BTreeMap<String, NodeAttributeValue>,
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
    attrs: &BTreeMap<String, NodeAttributeValue>,
) -> Option<String> {
    invoke_node_hook(class_name, format, attrs, false)
}

fn invoke_node_hook(
    class_name: &str,
    format: &str,
    attrs: &BTreeMap<String, NodeAttributeValue>,
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
            dict.set_item(k, node_attribute_to_py(py, v)).ok()?;
        }
        let result = cb.bind(py).call1((dict,)).ok()?;
        result.extract::<String>().ok()
    })?
}

pub(crate) fn node_attribute_to_py<'py>(
    py: Python<'py>,
    value: &NodeAttributeValue,
) -> Bound<'py, PyAny> {
    match value {
        NodeAttributeValue::String(value) => value.into_pyobject(py).unwrap().into_any(),
        NodeAttributeValue::Bool(value) => {
            pyo3::types::PyBool::new(py, *value).to_owned().into_any()
        }
        NodeAttributeValue::Signed(value) => value.into_pyobject(py).unwrap().into_any(),
        NodeAttributeValue::Unsigned(value) => value.into_pyobject(py).unwrap().into_any(),
        NodeAttributeValue::Float(value) => value.into_pyobject(py).unwrap().into_any(),
        NodeAttributeValue::None => py.None().into_bound(py),
        NodeAttributeValue::Opaque { repr, .. } => repr.into_pyobject(py).unwrap().into_any(),
        NodeAttributeValue::List(values) => {
            let list = PyList::empty(py);
            for item in values {
                list.append(node_attribute_to_py(py, item)).unwrap();
            }
            list.into_any()
        }
        NodeAttributeValue::Map(values) => {
            let dict = PyDict::new(py);
            for (key, item) in values {
                dict.set_item(key, node_attribute_to_py(py, item)).unwrap();
            }
            dict.into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Block, InlineBlock};

    #[test]
    fn python_paragraph_conversion_preserves_inline_children() {
        Python::attach(|py| {
            let globals = PyDict::new(py);
            py.run(
                c"from docutils import nodes
p = nodes.paragraph(text='before ')
p += nodes.emphasis(text='inside')
",
                None,
                Some(&globals),
            )
            .unwrap();
            let paragraph = globals.get_item("p").unwrap().unwrap();

            let block = python_node_to_block(&paragraph).expect("paragraph conversion");
            match block {
                Block::RichParagraph { children, .. } => {
                    assert!(
                        matches!(children[0], InlineBlock::Text(ref text) if text == "before ")
                    );
                    assert!(matches!(children[1], InlineBlock::Emphasis(ref nested)
                        if matches!(nested.as_slice(), [InlineBlock::Text(text)] if text == "inside")));
                }
                _ => panic!("expected rich paragraph"),
            }
        });
    }
}
