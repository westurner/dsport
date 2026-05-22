//! Port of ``sphinx.util.matching``.
//!
//! Provides the glob-to-regex translator that Sphinx uses for
//! ``include_patterns`` / ``exclude_patterns``, plus the
//! [`Matcher`] type and [`get_matching_files`] walker. This is the
//! primary dependency of [`crate::project::Project::discover`].
//!
//! The translator is a direct port of upstream's
//! ``_translate_pattern``: single ``*`` does **not** cross slashes,
//! ``**`` does, ``?`` does not, and ``[…]`` honours the ``!``
//! negation prefix with the same edge-cases (unterminated class →
//! literal ``[``, etc).

use std::path::{Path, PathBuf};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use regex::Regex;

/// Translate a shell-style glob to an anchored regex string.
///
/// Direct port of ``sphinx.util.matching._translate_pattern``.
pub fn translate_pattern(pat: &str) -> String {
    let bytes: Vec<char> = pat.chars().collect();
    let n = bytes.len();
    let mut i = 0;
    let mut res = String::new();
    while i < n {
        let c = bytes[i];
        i += 1;
        match c {
            '*' => {
                if i < n && bytes[i] == '*' {
                    i += 1;
                    res.push_str(".*");
                } else {
                    res.push_str("[^/]*");
                }
            }
            '?' => res.push_str("[^/]"),
            '[' => {
                let mut j = i;
                if j < n && bytes[j] == '!' {
                    j += 1;
                }
                if j < n && bytes[j] == ']' {
                    j += 1;
                }
                while j < n && bytes[j] != ']' {
                    j += 1;
                }
                if j >= n {
                    res.push_str("\\[");
                } else {
                    let raw: String = bytes[i..j].iter().collect();
                    let stuff = raw.replace('\\', "\\\\");
                    i = j + 1;
                    let first = stuff.chars().next().unwrap_or('\0');
                    let body = if first == '!' {
                        format!("^/{}", &stuff[1..])
                    } else if first == '^' {
                        format!("\\{stuff}")
                    } else {
                        stuff
                    };
                    res.push('[');
                    res.push_str(&body);
                    res.push(']');
                }
            }
            other => {
                // Match Python's ``re.escape``: escape every non-alphanumeric.
                if other.is_ascii_alphanumeric() || other == '_' {
                    res.push(other);
                } else {
                    res.push('\\');
                    res.push(other);
                }
            }
        }
    }
    res.push('$');
    res
}

/// Compile a list of glob patterns into anchored regexes.
pub fn compile_matchers(patterns: &[String]) -> Result<Vec<Regex>, regex::Error> {
    patterns
        .iter()
        .map(|p| Regex::new(&format!("^{}", translate_pattern(p))))
        .collect()
}

/// Mirror of ``sphinx.util.matching.Matcher``. Expands ``**/`` prefix
/// patterns so e.g. ``**/index.rst`` also matches bare ``index.rst``.
pub struct Matcher {
    patterns: Vec<Regex>,
}

impl Matcher {
    pub fn new(exclude_patterns: &[String]) -> Result<Self, regex::Error> {
        let mut all: Vec<String> = exclude_patterns.to_vec();
        for p in exclude_patterns {
            if let Some(stripped) = p.strip_prefix("**/") {
                all.push(stripped.to_string());
            }
        }
        Ok(Matcher {
            patterns: compile_matchers(&all)?,
        })
    }

    /// Returns true iff ``s`` (after slash canonicalisation) matches
    /// any pattern.
    pub fn is_match(&self, s: &str) -> bool {
        let canon = canon_path(s);
        self.patterns.iter().any(|p| p.is_match(&canon))
    }
}

fn canon_path(s: &str) -> String {
    s.replace('\\', "/")
}

/// Walk ``dirname`` recursively, yielding source-relative POSIX paths
/// that match at least one include pattern and no exclude pattern.
/// File order within each directory is sorted; directory order is
/// sorted; both match upstream's ``sorted(files)`` / ``sorted(dirs)``.
pub fn get_matching_files(
    dirname: &Path,
    include_patterns: &[String],
    exclude_patterns: &[String],
) -> Result<Vec<String>, regex::Error> {
    let include_matchers = compile_matchers(include_patterns)?;
    let exclude_matchers = compile_matchers(exclude_patterns)?;

    let root = match dirname.canonicalize() {
        Ok(p) => p,
        Err(_) => dirname.to_path_buf(),
    };

    let mut out: Vec<String> = Vec::new();
    walk_dir(&root, &root, &include_matchers, &exclude_matchers, &mut out);
    Ok(out)
}

fn walk_dir(root: &Path, dir: &Path, include: &[Regex], exclude: &[Regex], out: &mut Vec<String>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(it) => it,
        Err(_) => return,
    };

    let mut files: Vec<String> = Vec::new();
    let mut subdirs: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let file_type = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().into_owned();
        if file_type.is_dir() {
            subdirs.push(entry.path());
        } else if file_type.is_file() || file_type.is_symlink() {
            files.push(name);
        }
    }
    files.sort();
    subdirs.sort();

    let relative_root = relative_posix(root, dir);

    // Filter files.
    for entry in &files {
        let candidate = if relative_root.is_empty() {
            entry.clone()
        } else {
            format!("{relative_root}/{entry}")
        };
        let mut keep = false;
        for m in include {
            if m.is_match(&candidate) {
                keep = true;
                break;
            }
        }
        for m in exclude {
            if m.is_match(&candidate) {
                keep = false;
                break;
            }
        }
        if keep {
            out.push(candidate);
        }
    }

    // Recurse into directories, applying exclude on the directory
    // name itself (upstream's `dirs[:] = filtered_dirs`).
    for sub in subdirs {
        let dirname = sub.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let candidate = if relative_root.is_empty() {
            dirname.to_string()
        } else {
            format!("{relative_root}/{dirname}")
        };
        let mut skip = false;
        for m in exclude {
            if m.is_match(&candidate) {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }
        walk_dir(root, &sub, include, exclude, out);
    }
}

fn relative_posix(root: &Path, dir: &Path) -> String {
    if dir == root {
        return String::new();
    }
    match dir.strip_prefix(root) {
        Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
        Err(_) => String::new(),
    }
}

// ----- PyO3 surface ---------------------------------------------------------

/// ``compile_matchers(patterns) -> list[Callable[[str], bool]]``.
#[pyfunction(name = "compile_matchers")]
pub fn py_compile_matchers<'py>(
    py: Python<'py>,
    patterns: Vec<String>,
) -> PyResult<Bound<'py, PyList>> {
    let compiled = compile_matchers(&patterns).map_err(to_py_err)?;
    let list = PyList::empty(py);
    for r in compiled {
        list.append(CompiledMatcher { regex: r })?;
    }
    Ok(list)
}

/// Callable wrapper so Python can do ``pat('hello.py')`` returning
/// truthy/falsey (we return ``re.Match | None`` semantics with
/// ``Option<bool>``: callers in upstream just check truthiness).
#[pyclass(name = "_CompiledMatcher", module = "sphinxdocrs")]
struct CompiledMatcher {
    regex: Regex,
}

#[pymethods]
impl CompiledMatcher {
    fn __call__(&self, s: &str) -> Option<String> {
        // Return the matched string (truthy) or None — mirrors
        // upstream which returns ``re.Match | None``.
        self.regex.find(s).map(|m| m.as_str().to_string())
    }
}

/// ``Matcher`` PyClass mirroring ``sphinx.util.matching.Matcher``.
#[pyclass(name = "Matcher", module = "sphinxdocrs")]
pub struct PyMatcher {
    inner: Matcher,
}

#[pymethods]
impl PyMatcher {
    #[new]
    fn new(exclude_patterns: Vec<String>) -> PyResult<Self> {
        Ok(PyMatcher {
            inner: Matcher::new(&exclude_patterns).map_err(to_py_err)?,
        })
    }

    fn __call__(&self, s: &str) -> bool {
        self.inner.is_match(s)
    }

    #[pyo3(name = "match")]
    fn py_match(&self, s: &str) -> bool {
        self.inner.is_match(s)
    }
}

#[pyfunction(name = "get_matching_files")]
#[pyo3(signature = (dirname, include_patterns = None, exclude_patterns = None))]
pub fn py_get_matching_files<'py>(
    py: Python<'py>,
    dirname: Bound<'py, PyAny>,
    include_patterns: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
) -> PyResult<Bound<'py, PyList>> {
    let path = pathlike_to_pathbuf(&dirname)?;
    let include = include_patterns.unwrap_or_else(|| vec!["**".to_string()]);
    let exclude = exclude_patterns.unwrap_or_default();
    let files = get_matching_files(&path, &include, &exclude).map_err(to_py_err)?;
    let out = PyList::empty(py);
    for f in files {
        out.append(f)?;
    }
    Ok(out)
}

fn pathlike_to_pathbuf(obj: &Bound<'_, PyAny>) -> PyResult<PathBuf> {
    if let Ok(s) = obj.extract::<String>() {
        return Ok(PathBuf::from(s));
    }
    let s: String = obj.call_method0("__fspath__")?.extract()?;
    Ok(PathBuf::from(s))
}

fn to_py_err(e: regex::Error) -> PyErr {
    PyValueError::new_err(format!("invalid pattern: {e}"))
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_compile_matchers, m)?)?;
    m.add_function(wrap_pyfunction!(py_get_matching_files, m)?)?;
    m.add_class::<PyMatcher>()?;
    m.add_class::<CompiledMatcher>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_single_star() {
        let re = Regex::new(&format!("^{}", translate_pattern("*.py"))).unwrap();
        assert!(re.is_match("hello.py"));
        assert!(!re.is_match("sub/hello.py"));
    }

    #[test]
    fn translate_double_star() {
        let re = Regex::new(&format!("^{}", translate_pattern("**.py"))).unwrap();
        assert!(re.is_match("hello.py"));
        assert!(re.is_match("sub/hello.py"));
    }

    #[test]
    fn translate_question() {
        let re = Regex::new(&format!("^{}", translate_pattern("hello.?"))).unwrap();
        assert!(re.is_match("hello.c"));
        assert!(!re.is_match("hello.py"));
    }

    #[test]
    fn translate_class_negate() {
        let re = Regex::new(&format!("^{}", translate_pattern("hello[!12].py"))).unwrap();
        assert!(!re.is_match("hello1.py"));
        assert!(!re.is_match("hello/.py"));
        assert!(re.is_match("hello3.py"));
    }

    #[test]
    fn matcher_expands_doublestar_prefix() {
        let m = Matcher::new(&["**/world.py".to_string()]).unwrap();
        assert!(m.is_match("world.py"));
        assert!(m.is_match("subdir/world.py"));
    }
}
