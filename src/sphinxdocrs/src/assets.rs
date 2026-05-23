//! Asset helpers: download + cache JS/CSS files referenced from
//! `conf.py`, and generate sub-resource integrity (SRI) hashes for
//! them.
//!
//! Sphinx's current API for registering scripts and stylesheets is
//! [`Sphinx.add_js_file(filename, **kwargs)`][add_js] and
//! [`Sphinx.add_css_file(filename, **kwargs)`][add_css] (introduced in
//! Sphinx 1.8). The older `add_javascript` / `add_stylesheet` aliases
//! were deprecated in 1.8 and removed in 4.0. Both helpers accept an
//! `integrity=` keyword that is emitted directly as the
//! `<script integrity="…">` / `<link integrity="…">` Sub-Resource
//! Integrity attribute defined by the W3C SRI 2 spec
//! (`sha256` / `sha384` / `sha512`, base64-encoded, with an explicit
//! algorithm prefix).
//!
//! [add_js]:  https://www.sphinx-doc.org/en/master/extdev/appapi.html#sphinx.application.Sphinx.add_js_file
//! [add_css]: https://www.sphinx-doc.org/en/master/extdev/appapi.html#sphinx.application.Sphinx.add_css_file
//!
//! Implementation note: the fetch + hash primitives are implemented
//! through Python's `urllib.request` / `hashlib` / `base64` stdlib
//! modules via PyO3 so the crate does not gain a new Rust HTTP or
//! crypto dependency.

use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use crate::errors::ConfigError;

/// Hash algorithms permitted by the SRI 2 spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SriAlgo {
    Sha256,
    Sha384,
    Sha512,
}

impl SriAlgo {
    /// Lowercase algorithm name as it appears in an `integrity="…"`
    /// attribute (`sha256` / `sha384` / `sha512`).
    pub fn name(self) -> &'static str {
        match self {
            SriAlgo::Sha256 => "sha256",
            SriAlgo::Sha384 => "sha384",
            SriAlgo::Sha512 => "sha512",
        }
    }

    /// Parse an algorithm name from an `integrity="…"` prefix.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "sha256" => Some(SriAlgo::Sha256),
            "sha384" => Some(SriAlgo::Sha384),
            "sha512" => Some(SriAlgo::Sha512),
            _ => None,
        }
    }
}

/// Default SRI algorithm. `sha384` is what sphinx-doc and most CDNs
/// publish; it is also the W3C-recommended floor for new deployments.
pub const DEFAULT_SRI_ALGO: SriAlgo = SriAlgo::Sha384;

/// Stable cache subdirectory for a remote `url`:
/// `<cache_dir>/<sha256(url)[..16]>/<basename-of-url>`.
///
/// The 16-hex-char URL digest avoids collisions between two upstreams
/// that publish the same `basename` (e.g. `tex-mml-chtml.js`) while
/// keeping paths short and content-addressable.
pub fn cache_path_for(cache_dir: &Path, url: &str) -> PyResult<PathBuf> {
    let digest = Python::attach(|py| -> PyResult<String> {
        let hashlib = py.import("hashlib")?;
        let h = hashlib.call_method1("sha256", (PyBytes::new(py, url.as_bytes()),))?;
        h.call_method0("hexdigest")?.extract::<String>()
    })?;
    let basename = url.rsplit('/').next().unwrap_or("asset");
    let basename = basename.split('?').next().unwrap_or(basename);
    let basename = basename.split('#').next().unwrap_or(basename);
    let basename = if basename.is_empty() {
        "asset"
    } else {
        basename
    };
    Ok(cache_dir.join(&digest[..16]).join(basename))
}

/// Fetch `url` into a local cache (creating directories as needed) and
/// return the on-disk path. If the cached file already exists no
/// network request is made.
pub fn fetch_and_cache(url: &str, cache_dir: &Path) -> PyResult<PathBuf> {
    let path = cache_path_for(cache_dir, url)?;
    if path.exists() {
        return Ok(path);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ConfigError::new_err(format!("mkdir {}: {e}", parent.display())))?;
    }
    let bytes: Vec<u8> = Python::attach(|py| -> PyResult<Vec<u8>> {
        let urllib = py.import("urllib.request")?;
        let resp = urllib.call_method1("urlopen", (url,))?;
        let data = resp.call_method0("read")?;
        let b: &[u8] = data.extract()?;
        Ok(b.to_vec())
    })?;
    std::fs::write(&path, &bytes)
        .map_err(|e| ConfigError::new_err(format!("write {}: {e}", path.display())))?;
    Ok(path)
}

/// Compute an SRI attribute value (e.g. `"sha384-…"`) for `bytes`.
pub fn sri_hash(bytes: &[u8], algo: SriAlgo) -> PyResult<String> {
    Python::attach(|py| -> PyResult<String> {
        let hashlib = py.import("hashlib")?;
        let h = hashlib.call_method1(algo.name(), (PyBytes::new(py, bytes),))?;
        let digest = h.call_method0("digest")?;
        let b64 = py.import("base64")?;
        let encoded: String = b64
            .call_method1("b64encode", (digest,))?
            .call_method0("decode")?
            .extract()?;
        Ok(format!("{}-{}", algo.name(), encoded))
    })
}

/// Compute an SRI attribute value for a file on disk.
pub fn sri_hash_file(path: &Path, algo: SriAlgo) -> PyResult<String> {
    let bytes = std::fs::read(path)
        .map_err(|e| ConfigError::new_err(format!("read {}: {e}", path.display())))?;
    sri_hash(&bytes, algo)
}

// ---------------- Python surface ----------------

#[pyfunction(name = "cache_path_for")]
pub fn py_cache_path_for(cache_dir: &str, url: &str) -> PyResult<String> {
    Ok(cache_path_for(Path::new(cache_dir), url)?
        .to_string_lossy()
        .into_owned())
}

#[pyfunction(name = "fetch_and_cache")]
pub fn py_fetch_and_cache(url: &str, cache_dir: &str) -> PyResult<String> {
    Ok(fetch_and_cache(url, Path::new(cache_dir))?
        .to_string_lossy()
        .into_owned())
}

fn parse_algo(name: Option<&str>) -> PyResult<SriAlgo> {
    match name {
        None => Ok(DEFAULT_SRI_ALGO),
        Some(n) => SriAlgo::from_name(n)
            .ok_or_else(|| ConfigError::new_err(format!("unknown SRI algo: {n:?}"))),
    }
}

#[pyfunction(name = "sri_hash", signature = (data, algo = None))]
pub fn py_sri_hash(data: &[u8], algo: Option<&str>) -> PyResult<String> {
    sri_hash(data, parse_algo(algo)?)
}

#[pyfunction(name = "sri_hash_file", signature = (path, algo = None))]
pub fn py_sri_hash_file(path: &str, algo: Option<&str>) -> PyResult<String> {
    sri_hash_file(Path::new(path), parse_algo(algo)?)
}

/// Fetch the asset and return `{path, integrity, url}` as a Python
/// dict so the result can be fed straight into `add_js_file` /
/// `add_css_file` keyword arguments.
#[pyfunction(name = "fetch_with_integrity", signature = (url, cache_dir, algo = None))]
pub fn py_fetch_with_integrity(
    py: Python<'_>,
    url: &str,
    cache_dir: &str,
    algo: Option<&str>,
) -> PyResult<Py<PyDict>> {
    let algo = parse_algo(algo)?;
    let path = fetch_and_cache(url, Path::new(cache_dir))?;
    let integrity = sri_hash_file(&path, algo)?;
    let d = PyDict::new(py);
    d.set_item("url", url)?;
    d.set_item("path", path.to_string_lossy().into_owned())?;
    d.set_item("integrity", integrity)?;
    d.set_item("algo", algo.name())?;
    Ok(d.into())
}
