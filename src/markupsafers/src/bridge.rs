//! PyO3 bridge — makes `markupsafers` a drop-in Python `MarkupSafe` replacement.
//!
//! When the `extension-module` feature is enabled, this module exposes:
//!
//! - `markupsafers.Markup` — Python class wrapping `markupsafers::Markup`
//! - `markupsafers.escape(s)` — Python function
//! - `markupsafers.escape_silent(s)` — Python function
//! - `markupsafers.soft_str(s)` — Python function
//!
//! The Python `Markup` class:
//! - Inherits from `str` (via `pyo3::types::PyString` delegation).
//! - Implements `__html__()` so that Jinja2 / MarkupSafe treat it as pre-escaped.
//! - All string operations return `Markup` where appropriate.

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyString};

use crate::escape::{escape_silent as rs_escape_silent, escape_to};
use crate::markup::Markup;

// ── Python Markup class ────────────────────────────────────────────────────────

/// Python-visible `Markup` class.
///
/// Wraps a safe HTML string.  Constructing `Markup(s)` *escapes* `s`
/// (matching the Python MarkupSafe constructor).  Use
/// `Markup.__new_safe__(s)` to wrap without escaping.
#[pyclass(name = "Markup", module = "markupsafers")]
#[derive(Clone)]
pub struct PyMarkup {
    inner: Markup,
}

#[pymethods]
impl PyMarkup {
    /// `Markup(s)` — escape `s` and wrap.
    ///
    /// Matches `markupsafe.Markup(s)` Python constructor.
    #[new]
    fn new(s: &str) -> Self {
        PyMarkup {
            inner: Markup::escape(s),
        }
    }

    /// `Markup.__new_safe__(s)` — wrap without escaping (trusted content).
    #[staticmethod]
    fn __new_safe__(s: &str) -> PyMarkup {
        PyMarkup {
            inner: Markup::from_safe(s),
        }
    }

    /// `Markup.__html__()` — return the inner string unchanged.
    ///
    /// This is the Python `__html__()` protocol used by Jinja2/MarkupSafe
    /// to detect pre-escaped values.
    fn __html__(&self) -> &str {
        self.inner.as_str()
    }

    /// Return the inner safe string (as plain Python `str`).
    fn __str__(&self) -> &str {
        self.inner.as_str()
    }

    /// `repr(markup)`.
    fn __repr__(&self) -> String {
        format!("Markup({:?})", self.inner.as_str())
    }

    /// `len(markup)`.
    fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// `markup + other` — escapes `other` if it is a plain str.
    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyMarkup> {
        if let Ok(py_markup) = other.extract::<PyMarkup>() {
            // Other is already Markup — no escaping.
            Ok(PyMarkup {
                inner: self.inner.clone() + &py_markup.inner,
            })
        } else {
            // Plain string — escape before appending.
            let s: &str = other.extract()?;
            Ok(PyMarkup {
                inner: self.inner.clone() + s,
            })
        }
    }

    /// `markup + str` (right-side add).
    fn __radd__(&self, other: &str) -> PyMarkup {
        let mut escaped = String::with_capacity(other.len() + self.inner.len());
        escape_to(other, &mut escaped);
        escaped.push_str(self.inner.as_str());
        PyMarkup {
            inner: Markup::from_safe(escaped),
        }
    }

    /// `Markup.escape(s)` — class method; escape and return Markup.
    #[classmethod]
    fn escape(_cls: &Bound<'_, pyo3::types::PyType>, s: &str) -> PyMarkup {
        PyMarkup {
            inner: Markup::escape(s),
        }
    }

    /// `markup.unescape()` — reverse HTML entities to characters.
    fn unescape(&self) -> String {
        self.inner.unescape()
    }

    /// Return the raw inner string (same as `__html__`).
    fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// `markup.format(*args)` — substitute positional args with escaping.
    fn format(&self, args: Vec<String>) -> PyMarkup {
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        PyMarkup {
            inner: self.inner.format_args(&arg_refs),
        }
    }

    /// `bool(markup)` — false only when empty.
    fn __bool__(&self) -> bool {
        !self.inner.is_empty()
    }

    /// `markup == other`.
    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        if let Ok(m) = other.extract::<PyMarkup>() {
            self.inner == m.inner
        } else if let Ok(s) = other.extract::<&str>() {
            self.inner.as_str() == s
        } else {
            false
        }
    }

    /// `hash(markup)`.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        self.inner.hash(&mut h);
        h.finish()
    }
}

// ── Module-level functions ────────────────────────────────────────────────────

/// `markupsafers.escape(s)` — escape `s` and return `Markup`.
///
/// If `s` already has `__html__()` (i.e. is a `Markup`), return it unchanged.
#[pyfunction(name = "escape")]
fn py_escape(s: &Bound<'_, PyAny>) -> PyResult<PyMarkup> {
    // If the object has __html__(), it is already safe.
    if let Ok(html_method) = s.getattr("__html__") {
        let safe_str: String = html_method.call0()?.extract()?;
        return Ok(PyMarkup {
            inner: Markup::from_safe(safe_str),
        });
    }
    let text: &str = s.extract()?;
    Ok(PyMarkup {
        inner: Markup::escape(text),
    })
}

/// `markupsafers.escape_silent(s)` — escape `s`; return empty `Markup` on `None`.
#[pyfunction(name = "escape_silent")]
fn py_escape_silent(s: Option<&str>) -> PyMarkup {
    PyMarkup {
        inner: rs_escape_silent(s),
    }
}

/// `markupsafers.soft_str(s)` — return the str representation without escaping.
///
/// When `s` is a `Markup`, returns the inner string unchanged.
/// When `s` is a plain `str`, returns it as-is (no escaping).
#[pyfunction(name = "soft_str")]
fn py_soft_str(s: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(markup) = s.extract::<PyMarkup>() {
        Ok(markup.inner.into_string())
    } else {
        let text: String = s.extract()?;
        Ok(text)
    }
}

// ── Module registration ────────────────────────────────────────────────────────

/// Register the `markupsafers` Python extension module.
///
/// Called automatically by `maturin` via `#[pymodule]`.
#[pymodule]
pub fn markupsafers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMarkup>()?;
    m.add_function(wrap_pyfunction!(py_escape, m)?)?;
    m.add_function(wrap_pyfunction!(py_escape_silent, m)?)?;
    m.add_function(wrap_pyfunction!(py_soft_str, m)?)?;
    Ok(())
}
