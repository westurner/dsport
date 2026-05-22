//! Minimal port of `sphinx.project.Project`.
//!
//! Phase 4 surface: ``path2doc``, ``doc2path``, and ``discover``.
//! ``discover`` walks the source tree using the ported
//! [`crate::util_matching`] glob engine and returns the set of
//! docnames.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use pyo3::types::PySet;

use crate::util_matching;

#[pyclass(name = "Project", module = "sphinxdocrs")]
pub struct Project {
    srcdir: PathBuf,
    source_suffix: Vec<String>,
    inner: std::sync::Mutex<DiscoverState>,
}

#[derive(Default)]
struct DiscoverState {
    docnames: HashSet<String>,
    docname_to_path: std::collections::HashMap<String, String>,
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
            inner: std::sync::Mutex::new(DiscoverState::default()),
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

    /// Currently discovered docnames as a Python ``set``.
    #[getter]
    fn docnames<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PySet>> {
        let inner = self
            .inner
            .lock()
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Project mutex poisoned"))?;
        let set = PySet::empty(py)?;
        for n in inner.docnames.iter() {
            set.add(n)?;
        }
        Ok(set)
    }

    /// Walk the source tree and populate ``docnames``. Mirrors
    /// ``sphinx.project.Project.discover``.
    #[pyo3(signature = (exclude_paths = None, include_paths = None))]
    fn discover<'py>(
        &self,
        py: Python<'py>,
        exclude_paths: Option<Vec<String>>,
        include_paths: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PySet>> {
        let include = include_paths.unwrap_or_else(|| vec!["**".to_string()]);
        let mut exclude = exclude_paths.unwrap_or_default();
        // Upstream's ``EXCLUDE_PATHS`` constant — always applied.
        for p in ["**/_sources", ".#*", "**/.#*", "*.lproj/**"] {
            exclude.push(p.to_string());
        }

        let files = util_matching::get_matching_files(&self.srcdir, &include, &exclude)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;

        let mut inner = self
            .inner
            .lock()
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Project mutex poisoned"))?;
        inner.docnames.clear();
        inner.docname_to_path.clear();

        for filename in files {
            if let Some(docname) = self.filename_to_docname(&filename) {
                // Match upstream: skip if already recorded (warning
                // emission elided — would need ``sphinx.util.logging``).
                if inner.docnames.contains(&docname) {
                    continue;
                }
                inner.docnames.insert(docname.clone());
                inner.docname_to_path.insert(docname, filename);
            }
        }

        let set = PySet::empty(py)?;
        for n in inner.docnames.iter() {
            set.add(n)?;
        }
        Ok(set)
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
        let posix = path_to_posix(&path);
        Ok(self.filename_to_docname(&posix))
    }

    /// Return the filename for ``docname``. If ``absolute`` is true,
    /// returns the absolute path (joined with ``srcdir``); otherwise a
    /// relative one. If the docname is unknown, falls back to
    /// ``docname + first_source_suffix`` (matching upstream behavior).
    #[pyo3(signature = (docname, absolute))]
    fn doc2path(&self, docname: &str, absolute: bool) -> PyResult<String> {
        let recorded = {
            let inner = self
                .inner
                .lock()
                .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Project mutex poisoned"))?;
            inner.docname_to_path.get(docname).cloned()
        };
        let rel = recorded.unwrap_or_else(|| {
            let first = self.source_suffix.first().map(|s| s.as_str()).unwrap_or("");
            format!("{docname}{first}")
        });
        if absolute {
            let joined = self.srcdir.join(&rel);
            Ok(joined.to_string_lossy().into_owned())
        } else {
            Ok(rel)
        }
    }
}

impl Project {
    /// Apply ``source_suffix`` matching to a forward-slashed relative
    /// path, returning the docname if the suffix matches. Mirrors
    /// upstream's ``for suffix in self.source_suffix: if path.name.endswith(suffix)``.
    fn filename_to_docname(&self, posix_path: &str) -> Option<String> {
        let name = posix_path.rsplit('/').next().unwrap_or(posix_path);
        for suffix in &self.source_suffix {
            if name.ends_with(suffix) {
                let stripped = posix_path.strip_suffix(suffix).unwrap_or(posix_path);
                return Some(stripped.to_string());
            }
        }
        None
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
