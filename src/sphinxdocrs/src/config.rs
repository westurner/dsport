//! Minimal `conf.py` reader and math-renderer configuration.
//!
//! This is intentionally narrow: it covers the surface needed to wire
//! sphinx's math options (`extensions`, `mathjax_path`,
//! `mathjax_options`, `mathjax3_config`, `imgmath_image_format`,
//! `imgmath_latex`, `imgmath_dvipng`, `imgmath_dvisvgm`) and to pick a
//! default math backend. Anything else on `conf.py` is ignored.
//!
//! The reader executes the user's `conf.py` with PyO3 (sphinx itself
//! does the same via `exec()` in `sphinx.config.Config`), then reads
//! attributes off the module's globals. Missing attributes fall back
//! to sphinx's documented defaults.

use std::collections::HashMap;
use std::path::Path;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::errors::ConfigError;

/// Math backend selected by a sphinx project's `extensions` list.
///
/// Mirrors the docutilsrs / myst-md-rs `MathBackend`, but kept as a
/// separate type so sphinxdocrs does not have to depend on
/// `mathrenderrs` directly. The string form is the upstream sphinx
/// extension name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathRenderer {
    /// `sphinx.ext.mathjax` (sphinx's default).
    MathJax,
    /// `sphinx.ext.imgmath`.
    ImgMath,
    /// Rust-native RaTeX renderer (dsport extension; selected when
    /// the user writes `math_renderer = "ratex"` or lists
    /// `dsport.ext.ratex` in `extensions`).
    Ratex,
}

impl MathRenderer {
    /// Canonical name as it would appear in a sphinx `conf.py`
    /// (`math_renderer` value or the extension's import path).
    pub fn name(self) -> &'static str {
        match self {
            MathRenderer::MathJax => "mathjax",
            MathRenderer::ImgMath => "imgmath",
            MathRenderer::Ratex => "ratex",
        }
    }
}

/// Subset of sphinx's `Config` covering math-related options.
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// `extensions = [...]` from `conf.py`.
    pub extensions: Vec<String>,
    /// Explicit `math_renderer` setting (overrides extension-based
    /// detection when present). Sphinx itself reads this from
    /// `extensions`, but we expose it explicitly so projects can pick
    /// the RaTeX backend without editing `extensions`.
    pub math_renderer: Option<MathRenderer>,
    /// `mathjax_path` — URL to the MathJax bundle. Sphinx's documented
    /// default is the jsDelivr MathJax 3 CDN.
    pub mathjax_path: String,
    /// `mathjax_options` — extra `<script>` tag attributes.
    pub mathjax_options: HashMap<String, String>,
    /// `mathjax3_config` — passed as `window.MathJax = {...}` JSON.
    pub mathjax3_config: Option<String>,
    /// `imgmath_image_format` — `"png"` or `"svg"`. Sphinx default: `"png"`.
    pub imgmath_image_format: String,
    /// `imgmath_latex` — path to the `latex` executable.
    pub imgmath_latex: String,
    /// `imgmath_dvipng` — path to the `dvipng` executable.
    pub imgmath_dvipng: String,
    /// `imgmath_dvisvgm` — path to the `dvisvgm` executable.
    pub imgmath_dvisvgm: String,
}

/// Default `mathjax_path`. Mirrors sphinx 7.x default.
pub const DEFAULT_MATHJAX_PATH: &str =
    "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js";

impl Config {
    /// Sphinx-compatible defaults for an empty `conf.py`.
    pub fn defaults() -> Self {
        Self {
            extensions: Vec::new(),
            math_renderer: None,
            mathjax_path: DEFAULT_MATHJAX_PATH.to_string(),
            mathjax_options: HashMap::new(),
            mathjax3_config: None,
            imgmath_image_format: "png".to_string(),
            imgmath_latex: "latex".to_string(),
            imgmath_dvipng: "dvipng".to_string(),
            imgmath_dvisvgm: "dvisvgm".to_string(),
        }
    }

    /// Resolve the effective math renderer.
    ///
    /// Precedence (matches sphinx's documented behavior):
    /// 1. Explicit `math_renderer` setting.
    /// 2. First math extension found in `extensions` (`sphinx.ext.imgmath`
    ///    or `sphinx.ext.mathjax`; `dsport.ext.ratex` for RaTeX).
    /// 3. Fallback to MathJax (sphinx's built-in default).
    pub fn effective_math_renderer(&self) -> MathRenderer {
        if let Some(r) = self.math_renderer {
            return r;
        }
        for ext in &self.extensions {
            match ext.as_str() {
                "sphinx.ext.imgmath" => return MathRenderer::ImgMath,
                "sphinx.ext.mathjax" => return MathRenderer::MathJax,
                "dsport.ext.ratex" => return MathRenderer::Ratex,
                _ => {}
            }
        }
        MathRenderer::MathJax
    }

    /// Read a `conf.py` file by executing it with PyO3.
    ///
    /// Errors are surfaced as [`ConfigError`] to match sphinx's own
    /// behavior in `sphinx.config.Config`.
    pub fn from_conf_py(path: &Path) -> PyResult<Self> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::new_err(format!("cannot read {}: {e}", path.display())))?;
        Python::attach(|py| Self::from_source(py, &source))
    }

    /// Read a `conf.py` from an in-memory source string.
    pub fn from_source(py: Python<'_>, source: &str) -> PyResult<Self> {
        let globals = PyDict::new(py);
        py.run(
            &std::ffi::CString::new(source).unwrap(),
            Some(&globals),
            None,
        )
        .map_err(|e| ConfigError::new_err(format!("conf.py failed: {e}")))?;

        let mut cfg = Self::defaults();

        if let Ok(Some(v)) = globals.get_item("extensions") {
            if let Ok(list) = v.cast::<PyList>() {
                cfg.extensions = list
                    .iter()
                    .filter_map(|x| x.extract::<String>().ok())
                    .collect();
            }
        }
        if let Ok(Some(v)) = globals.get_item("math_renderer") {
            if let Ok(s) = v.extract::<String>() {
                cfg.math_renderer = match s.as_str() {
                    "mathjax" | "sphinx.ext.mathjax" => Some(MathRenderer::MathJax),
                    "imgmath" | "sphinx.ext.imgmath" => Some(MathRenderer::ImgMath),
                    "ratex" | "dsport.ext.ratex" => Some(MathRenderer::Ratex),
                    other => {
                        return Err(ConfigError::new_err(format!(
                            "unknown math_renderer: {other:?}"
                        )));
                    }
                };
            }
        }
        if let Ok(Some(v)) = globals.get_item("mathjax_path") {
            if let Ok(s) = v.extract::<String>() {
                cfg.mathjax_path = s;
            }
        }
        if let Ok(Some(v)) = globals.get_item("mathjax_options") {
            if let Ok(d) = v.cast::<PyDict>() {
                for (k, val) in d.iter() {
                    if let (Ok(ks), Ok(vs)) = (k.extract::<String>(), val.extract::<String>()) {
                        cfg.mathjax_options.insert(ks, vs);
                    }
                }
            }
        }
        if let Ok(Some(v)) = globals.get_item("mathjax3_config") {
            // Stored as JSON-ish repr; sphinx serializes it server-side.
            cfg.mathjax3_config = Some(v.str()?.to_string());
        }
        if let Ok(Some(v)) = globals.get_item("imgmath_image_format") {
            if let Ok(s) = v.extract::<String>() {
                cfg.imgmath_image_format = s;
            }
        }
        if let Ok(Some(v)) = globals.get_item("imgmath_latex") {
            if let Ok(s) = v.extract::<String>() {
                cfg.imgmath_latex = s;
            }
        }
        if let Ok(Some(v)) = globals.get_item("imgmath_dvipng") {
            if let Ok(s) = v.extract::<String>() {
                cfg.imgmath_dvipng = s;
            }
        }
        if let Ok(Some(v)) = globals.get_item("imgmath_dvisvgm") {
            if let Ok(s) = v.extract::<String>() {
                cfg.imgmath_dvisvgm = s;
            }
        }

        Ok(cfg)
    }
}

#[pyfunction(name = "read_conf_py")]
pub fn py_read_conf_py(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    let cfg = Config::from_conf_py(Path::new(path))?;
    let effective = cfg.effective_math_renderer().name();
    let d = PyDict::new(py);
    d.set_item("extensions", cfg.extensions)?;
    d.set_item(
        "math_renderer",
        cfg.math_renderer.map(|r| r.name().to_string()),
    )?;
    d.set_item("effective_math_renderer", effective)?;
    d.set_item("mathjax_path", cfg.mathjax_path)?;
    d.set_item("mathjax_options", cfg.mathjax_options)?;
    d.set_item("mathjax3_config", cfg.mathjax3_config)?;
    d.set_item("imgmath_image_format", cfg.imgmath_image_format)?;
    d.set_item("imgmath_latex", cfg.imgmath_latex)?;
    d.set_item("imgmath_dvipng", cfg.imgmath_dvipng)?;
    d.set_item("imgmath_dvisvgm", cfg.imgmath_dvisvgm)?;
    Ok(d.into())
}
