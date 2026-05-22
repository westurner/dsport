//! `Extension` port of `sphinx.extension.Extension`.
//!
//! Upstream is a plain data holder: a constructor that takes the
//! extension's dotted name, the imported module, and a free-form
//! ``**kwargs`` metadata mapping (the dict returned by an
//! extension's ``setup()``). Three well-known keys — ``version``,
//! ``parallel_read_safe``, ``parallel_write_safe`` — are popped off
//! that mapping and exposed as attributes; whatever else the
//! extension returned stays in ``metadata``.
//!
//! Mirroring upstream, ``metadata`` keeps the *same* dict object
//! that callers pass in (popping in Python mutates it in place;
//! popping here mirrors that contract).

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

/// Default ``parallel_write_safe`` when not provided by ``setup()``.
const DEFAULT_PARALLEL_WRITE_SAFE: bool = true;
/// Default ``version`` string when not provided.
const UNKNOWN_VERSION: &str = "unknown version";

#[pyclass(name = "Extension", module = "sphinxdocrs")]
pub struct Extension {
    #[pyo3(get, set)]
    name: Py<PyAny>,
    #[pyo3(get, set)]
    module: Py<PyAny>,
    #[pyo3(get, set)]
    metadata: Py<PyDict>,
    #[pyo3(get, set)]
    version: Py<PyAny>,
    #[pyo3(get, set)]
    parallel_read_safe: Py<PyAny>,
    #[pyo3(get, set)]
    parallel_write_safe: Py<PyAny>,
}

#[pymethods]
impl Extension {
    #[new]
    #[pyo3(signature = (name, module, **kwargs))]
    fn new<'py>(
        py: Python<'py>,
        name: Py<PyAny>,
        module: Py<PyAny>,
        kwargs: Option<Bound<'py, PyDict>>,
    ) -> PyResult<Self> {
        // Upstream assigns ``self.metadata = kwargs`` *before* popping
        // the well-known keys, so the same dict object is shared.
        let kwargs = match kwargs {
            Some(d) => d,
            None => PyDict::new(py),
        };

        let version = match kwargs.get_item("version")? {
            Some(v) => {
                kwargs.del_item("version")?;
                v
            }
            None => UNKNOWN_VERSION.into_pyobject(py)?.into_any(),
        };

        let parallel_read_safe = match kwargs.get_item("parallel_read_safe")? {
            Some(v) => {
                kwargs.del_item("parallel_read_safe")?;
                v
            }
            None => py.None().into_bound(py),
        };

        let parallel_write_safe = match kwargs.get_item("parallel_write_safe")? {
            Some(v) => {
                kwargs.del_item("parallel_write_safe")?;
                v
            }
            None => DEFAULT_PARALLEL_WRITE_SAFE
                .into_pyobject(py)?
                .to_owned()
                .into_any(),
        };

        Ok(Extension {
            name,
            module,
            metadata: kwargs.unbind(),
            version: version.unbind(),
            parallel_read_safe: parallel_read_safe.unbind(),
            parallel_write_safe: parallel_write_safe.unbind(),
        })
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let name: String = self
            .name
            .bind(py)
            .str()
            .map(|s| s.to_string())
            .unwrap_or_else(|_| "<extension>".into());
        Ok(format!("<Extension {name}>"))
    }
}

/// Free-standing helper so test code can construct an ``Extension``
/// from Rust easily.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Extension>()
}

#[pyfunction(name = "verify_needs_extensions")]
#[pyo3(signature = (app, config))]
pub fn py_verify_needs_extensions<'py>(
    py: Python<'py>,
    app: Bound<'py, PyAny>,
    config: Bound<'py, PyAny>,
) -> PyResult<()> {
    // ``config.needs_extensions`` may be ``None`` — return early.
    let needs = config.getattr("needs_extensions")?;
    if needs.is_none() {
        return Ok(());
    }
    let extensions = app.getattr("extensions")?;

    // Iterate via .items() so we work with both dict and dict-like.
    let items = needs.call_method0("items")?;
    let iter = items.try_iter()?;
    for entry in iter {
        let pair: Bound<'_, PyTuple> = entry?.cast_into()?;
        let extname: String = pair.get_item(0)?.extract()?;
        let reqversion: String = pair.get_item(1)?.extract()?;

        let ext = extensions.call_method1("get", (extname.as_str(),))?;
        if ext.is_none() {
            // Match upstream: warn via the ``sphinx.util.logging`` logger.
            let _ = py.import("sphinx.util.logging").and_then(|logging| {
                let logger = logging.call_method1("getLogger", ("sphinxdocrs.extension",))?;
                logger.call_method1(
                    "warning",
                    (
                        "The %s extension is required by needs_extensions settings, \
                         but it is not loaded.",
                        extname.as_str(),
                    ),
                )?;
                Ok::<_, PyErr>(())
            });
            continue;
        }

        let ext_version: String = ext.getattr("version")?.extract()?;
        let mut fulfilled = true;
        if ext_version == UNKNOWN_VERSION {
            fulfilled = false;
        } else {
            // Delegate version comparison to ``packaging.version`` for
            // upstream parity; fall back to string compare on
            // ``InvalidVersion`` exactly like upstream.
            let packaging = py.import("packaging.version")?;
            let cmp = packaging.getattr("Version").and_then(|v_ctor| {
                let a = v_ctor.call1((reqversion.as_str(),))?;
                let b = v_ctor.call1((ext_version.as_str(),))?;
                let gt = a.gt(&b)?;
                Ok::<bool, PyErr>(gt)
            });
            match cmp {
                Ok(gt) => {
                    if gt {
                        fulfilled = false;
                    }
                }
                Err(_) => {
                    if reqversion.as_str() > ext_version.as_str() {
                        fulfilled = false;
                    }
                }
            }
        }

        if !fulfilled {
            let msg = format!(
                "This project needs the extension {extname} at least in version \
                 {reqversion} and therefore cannot be built with the loaded \
                 version ({ext_version})."
            );
            return Err(crate::errors::VersionRequirementError::new_err(msg));
        }
    }
    Ok(())
}
