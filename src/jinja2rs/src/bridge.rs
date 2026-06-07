//! `jinja2rs::bridge` — PyO3 Python bindings.
//!
//! Exposes a `jinja2rs` Python module that provides a partial drop-in for the
//! `jinja2` package surface used by Sphinx.  This allows incremental migration:
//! Python code can `import jinja2rs as jinja2` and get Rust-backed rendering
//! while the Rust (`sphinxdocrs`) path bypasses Python entirely.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};

/// Python-facing `Environment` class.
#[pyclass(name = "Environment")]
pub struct PyEnvironment {
    inner: crate::environment::Environment,
}

#[pymethods]
impl PyEnvironment {
    #[new]
    fn new() -> Self {
        Self {
            inner: crate::environment::Environment::new(),
        }
    }

    /// Render a template string with the given keyword-argument context.
    fn render_str(&self, source: &str, context: &Bound<'_, PyDict>) -> PyResult<String> {
        let ctx = pydict_to_json(context)?;
        self.inner
            .render_str(source, &ctx)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Add a template from a string source.
    fn add_template(&mut self, name: &str, source: &str) -> PyResult<()> {
        // Note: minijinja requires 'static lifetimes for string templates;
        // we leak the strings here.  For production use, the loader-based
        // API (FileSystemLoader) is preferred.
        let name_s: &'static str = Box::leak(name.to_owned().into_boxed_str());
        let source_s: &'static str = Box::leak(source.to_owned().into_boxed_str());
        self.inner
            .add_template(name_s, source_s)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
}

/// Python-facing `SandboxedEnvironment` class (mirrors `jinja2.sandbox.SandboxedEnvironment`).
#[pyclass(name = "SandboxedEnvironment")]
pub struct PySandboxedEnvironment {
    inner: crate::sandbox::SandboxedEnvironment,
}

#[pymethods]
impl PySandboxedEnvironment {
    #[new]
    fn new() -> Self {
        Self {
            inner: crate::sandbox::SandboxedEnvironment::new(),
        }
    }

    fn render_str(&self, source: &str, context: &Bound<'_, PyDict>) -> PyResult<String> {
        let ctx = pydict_to_json(context)?;
        self.inner
            .render_str(source, &ctx)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
}

/// Convert a Python `dict` to a `serde_json::Value` for use as a template context.
///
/// Only handles the types actually used in Sphinx template contexts: strings,
/// integers, floats, booleans, None, lists, and nested dicts.
fn pydict_to_json(d: &Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    let mut map = serde_json::Map::new();
    for (k, v) in d.iter() {
        let key: String = k.extract::<String>()?;
        let val = pyobj_to_json(&v)?;
        map.insert(key, val);
    }
    Ok(serde_json::Value::Object(map))
}

fn pyobj_to_json(obj: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        return Ok(serde_json::Value::Null);
    }
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(serde_json::Value::Bool(b));
    }
    if let Ok(n) = obj.extract::<i64>() {
        return Ok(serde_json::Value::Number(n.into()));
    }
    if let Ok(f) = obj.extract::<f64>() {
        let num = serde_json::Number::from_f64(f)
            .unwrap_or_else(|| serde_json::Number::from(0i64));
        return Ok(serde_json::Value::Number(num));
    }
    if let Ok(s) = obj.extract::<String>() {
        return Ok(serde_json::Value::String(s));
    }
    if let Ok(list) = obj.downcast::<PyList>() {
        let arr: PyResult<Vec<serde_json::Value>> = list.iter().map(|x| pyobj_to_json(&x)).collect();
        return Ok(serde_json::Value::Array(arr?));
    }
    if let Ok(d) = obj.downcast::<PyDict>() {
        return pydict_to_json(d);
    }
    // Fallback: str(obj)
    Ok(serde_json::Value::String(obj.str()?.to_string()))
}

/// Register the bridge classes and functions into the `jinja2rs` Python module.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEnvironment>()?;
    m.add_class::<PySandboxedEnvironment>()?;
    Ok(())
}
