//! Minimal port of `sphinx.project.Project`.
//!
//! Phase 4 surface: ``path2doc`` and ``doc2path`` only. The upstream
//! ``discover()`` method relies on ``sphinx.util.matching`` for include /
//! exclude globbing and walks the source tree; porting that is tracked
//! in [docs/sphinx-port-inventory.md](../../docs/sphinx-port-inventory.md)
//! under P2 (`util/*`).

use std::path::{Path, PathBuf};

use pyo3::prelude::*;

#[pyclass(name = "Project", module = "sphinxdocrs")]
pub struct Project {
    srcdir: PathBuf,
    source_suffix: Vec<String>,
}

#[pymethods]
impl Project {
    /// Construct a project. ``source_suffix`` may be any Python
    /// iterable of strings (matching upstream's signature, which also
    /// accepts the ``{".rst": "restructuredtext"}`` mapping shape — for
    /// a mapping we use its keys).
    #[new]
    fn new(srcdir: Py<PyAny>, source_suffix: Bound<'_, PyAny>) -> PyResult<Self> {
        let srcdir = pathlike_to_pathbuf(srcdir.bind(source_suffix.py()))?;
        // Accept dict (use keys), list, tuple, set, any iterable of str.
        let mut suffixes: Vec<String> = Vec::new();
        let iter = source_suffix.try_iter()?;
        for item in iter {
            let s: String = item?.extract()?;
            suffixes.push(s);
        }
        Ok(Project {
            srcdir,
            source_suffix: suffixes,
        })
    }

    #[getter]
    fn srcdir(&self) -> String {
        self.srcdir.to_string_lossy().into_owned()
    }

    #[getter]
    fn source_suffix(&self) -> Vec<String> {
        self.source_suffix.clone()
    }

    /// Return the docname for ``filename`` if it is a document, else
    /// ``None``. ``filename`` may be absolute or relative to ``srcdir``.
    fn path2doc(&self, filename: Py<PyAny>, py: Python<'_>) -> PyResult<Option<String>> {
        let mut path = pathlike_to_pathbuf(filename.bind(py))?;
        if path.is_absolute() {
            if let Ok(rel) = path.strip_prefix(&self.srcdir) {
                path = rel.to_path_buf();
            }
        }
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        for suffix in &self.source_suffix {
            if name.ends_with(suffix) {
                // Strip suffix from the *full* path string in a
                // platform-neutral way that mirrors upstream
                // ``path_stabilize`` + ``removesuffix``.
                let full = path_to_posix(&path);
                let stripped = full.strip_suffix(suffix).unwrap_or(&full);
                return Ok(Some(stripped.to_string()));
            }
        }
        Ok(None)
    }

    /// Return the filename for ``docname``. If ``absolute`` is true,
    /// returns the absolute path (joined with ``srcdir``); otherwise a
    /// relative one. If the docname is unknown, falls back to
    /// ``docname + first_source_suffix`` (matching upstream behavior).
    #[pyo3(signature = (docname, absolute))]
    fn doc2path(&self, docname: &str, absolute: bool) -> String {
        let first = self
            .source_suffix
            .first()
            .map(|s| s.as_str())
            .unwrap_or("");
        let rel = format!("{docname}{first}");
        if absolute {
            let joined = self.srcdir.join(&rel);
            joined.to_string_lossy().into_owned()
        } else {
            rel
        }
    }
}

fn pathlike_to_pathbuf(obj: &Bound<'_, PyAny>) -> PyResult<PathBuf> {
    // Accept str or os.PathLike.
    if let Ok(s) = obj.extract::<String>() {
        return Ok(PathBuf::from(s));
    }
    let s: String = obj.call_method0("__fspath__")?.extract()?;
    Ok(PathBuf::from(s))
}

fn path_to_posix(p: &Path) -> String {
    // Convert backslashes to forward slashes so docnames are stable on
    // every platform.
    p.to_string_lossy().replace('\\', "/")
}
