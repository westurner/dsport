//! Minimal `conf.py` reader and math-renderer configuration.
//!
//! This is intentionally narrow: it covers the surface needed to wire
//! sphinx's math options (`extensions`, `mathjax_path`,
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

/// Read `conf.py` at `path` and return a raw `ConfigVal` map suitable for
/// passing to [`SphinxConfig::new`].
///
/// Executes `conf.py` via PyO3 (the same mechanism `Config::from_conf_py`
/// uses) and converts a broad set of common top-level variables — including
/// `intersphinx_mapping` — into [`ConfigVal`].  Unknown variables are
/// silently ignored; callers should fall back to `SphinxConfig::new_defaults`
/// on error.
///
/// # Errors
///
/// Returns a `PyErr` if the file cannot be read or if conf.py raises during
/// execution.
pub fn raw_config_from_conf_py(path: &Path) -> PyResult<HashMap<String, ConfigVal>> {
    use pyo3::types::{PyList, PyTuple};
    let source = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::new_err(format!("cannot read {}: {e}", path.display())))?;

    Python::attach(|py| {
        let globals = PyDict::new(py);
        // Provide a stub __file__ so conf.py code that inspects it works.
        globals.set_item("__file__", path.to_string_lossy().as_ref())?;
        py.run(
            &std::ffi::CString::new(source.as_str()).unwrap(),
            Some(&globals),
            None,
        )
        .map_err(|e| ConfigError::new_err(format!("conf.py failed: {e}")))?;

        let mut raw: HashMap<String, ConfigVal> = HashMap::new();

        // Helper: convert a single Python value to ConfigVal (shallow).
        let py_to_val = |v: &pyo3::Bound<'_, pyo3::PyAny>| -> Option<ConfigVal> {
            if v.is_none() {
                Some(ConfigVal::Null)
            } else if let Ok(b) = v.extract::<bool>() {
                Some(ConfigVal::Bool(b))
            } else if let Ok(i) = v.extract::<i64>() {
                Some(ConfigVal::Int(i))
            } else if let Ok(f) = v.extract::<f64>() {
                Some(ConfigVal::Float(f))
            } else if let Ok(s) = v.extract::<String>() {
                Some(ConfigVal::Str(s))
            } else {
                None
            }
        };

        // ── extensions ──────────────────────────────────────────────────────
        if let Ok(Some(v)) = globals.get_item("extensions") {
            if let Ok(list) = v.cast::<PyList>() {
                let exts: Vec<ConfigVal> = list
                    .iter()
                    .filter_map(|x| x.extract::<String>().ok().map(ConfigVal::Str))
                    .collect();
                raw.insert("extensions".into(), ConfigVal::List(exts));
            }
        }

        // ── scalar string / bool options ─────────────────────────────────────
        for key in &[
            "project",
            "author",
            "copyright",
            "version",
            "release",
            "language",
            "master_doc",
            "root_doc",
            "source_encoding",
            "html_theme",
            "html_title",
            "html_short_title",
        ] {
            if let Ok(Some(v)) = globals.get_item(*key) {
                if let Some(val) = py_to_val(&v) {
                    raw.insert((*key).into(), val);
                }
            }
        }

        // ── list-of-strings options ─────────────────────────────────────────
        for key in &[
            "html_static_path",
            "html_extra_path",
            "templates_path",
            "html_theme_path",
            "html_css_files",
        ] {
            if let Ok(Some(v)) = globals.get_item(*key) {
                if let Ok(list) = v.cast::<PyList>() {
                    let items: Vec<ConfigVal> = list
                        .iter()
                        .filter_map(|x| x.extract::<String>().ok().map(ConfigVal::Str))
                        .collect();
                    raw.insert((*key).into(), ConfigVal::List(items));
                }
            }
        }

        // ── project-level writer options ───────────────────────────────────
        // These are nested tuple/list values rather than list-of-strings;
        // preserve their complete shape for the LaTeX and manpage builders.
        for key in &["latex_documents", "man_pages"] {
            if let Ok(Some(v)) = globals.get_item(*key) {
                if let Some(val) = py_to_configval(&v) {
                    raw.insert((*key).into(), val);
                }
            }
        }

        // ── intersphinx_mapping ──────────────────────────────────────────────
        // Python shape: {'name': ('base_url', inv_url_or_None), …}
        if let Ok(Some(v)) = globals.get_item("intersphinx_mapping") {
            if let Ok(d) = v.cast::<PyDict>() {
                let mut mapping: Vec<(String, ConfigVal)> = Vec::new();
                for (k, val) in d.iter() {
                    let Ok(name) = k.extract::<String>() else {
                        continue;
                    };
                    // Value is a tuple (base_url, inv_url_or_None)
                    if let Ok(tup) = val.cast::<PyTuple>() {
                        let base_url = tup
                            .get_item(0)
                            .ok()
                            .and_then(|u| u.extract::<String>().ok());
                        let inv_url = tup
                            .get_item(1)
                            .ok()
                            .and_then(|u| u.extract::<String>().ok());
                        if let Some(url) = base_url {
                            let entry = ConfigVal::List(vec![
                                ConfigVal::Str(url),
                                inv_url.map(ConfigVal::Str).unwrap_or(ConfigVal::Null),
                            ]);
                            mapping.push((name, entry));
                        }
                    }
                }
                mapping.sort_by(|a, b| a.0.cmp(&b.0));
                raw.insert("intersphinx_mapping".into(), ConfigVal::Map(mapping));
            }
        }

        // ── source_suffix ────────────────────────────────────────────────────
        if let Ok(Some(v)) = globals.get_item("source_suffix") {
            if let Ok(d) = v.cast::<PyDict>() {
                let pairs: Vec<(String, ConfigVal)> = d
                    .iter()
                    .filter_map(|(k, val)| {
                        let ks = k.extract::<String>().ok()?;
                        let vs = val.extract::<String>().ok()?;
                        Some((ks, ConfigVal::Str(vs)))
                    })
                    .collect();
                raw.insert("source_suffix".into(), ConfigVal::Map(pairs));
            } else if let Ok(s) = v.extract::<String>() {
                raw.insert("source_suffix".into(), ConfigVal::Str(s));
            } else if let Ok(list) = v.cast::<PyList>() {
                let exts: Vec<ConfigVal> = list
                    .iter()
                    .filter_map(|x| x.extract::<String>().ok().map(ConfigVal::Str))
                    .collect();
                raw.insert("source_suffix".into(), ConfigVal::List(exts));
            }
        }

        // ── needs_extensions ─────────────────────────────────────────────────
        // Python shape: {'ext.name': 'required_version_str', …}
        if let Ok(Some(v)) = globals.get_item("needs_extensions") {
            if let Ok(d) = v.cast::<PyDict>() {
                let pairs: Vec<(String, ConfigVal)> = d
                    .iter()
                    .filter_map(|(k, val)| {
                        let ks = k.extract::<String>().ok()?;
                        let vs = val.extract::<String>().ok()?;
                        Some((ks, ConfigVal::Str(vs)))
                    })
                    .collect();
                raw.insert("needs_extensions".into(), ConfigVal::Map(pairs));
            }
        }

        // ── generic fallback: everything else ────────────────────────────────
        //
        // Every option handled above gets bespoke, precise conversion (e.g.
        // `intersphinx_mapping`'s tuple-valued dict). But real `conf.py`
        // files also set plenty of options this module has no dedicated
        // per-key handling for — most commonly an extension's own settings
        // (e.g. `sphinx.ext.autosummary`'s `autosummary_generate`). Without
        // this pass those names were silently dropped, so when the owning
        // extension's `setup()` later calls
        // `add_config_value(name, default, ...)`,
        // [`crate::app_facade::PyAppFacade::add_config_value`]'s
        // "first registration wins" semantics would see no existing entry
        // and seed the extension's *default* instead of the real
        // `conf.py` value — silently ignoring a user's override (e.g.
        // `autosummary_generate = False` in `conf.py` would still behave
        // as if it were `True`, the extension's built-in default). Catches
        // any remaining top-level name whose value is a plain data shape
        // (`None`/`bool`/`int`/`float`/`str`/`list`/`tuple`/`dict`,
        // recursively); anything else (an imported module, a function,
        // `conf.py`'s own helper classes, …) is skipped, matching how
        // upstream's `Config.read` only ever treats module-level *data*
        // attributes as config values.
        for (k, v) in globals.iter() {
            let Ok(key) = k.extract::<String>() else {
                continue;
            };
            if raw.contains_key(&key) || key.starts_with("__") {
                continue;
            }
            if v.cast::<pyo3::types::PyModule>().is_ok() || v.hasattr("__call__").unwrap_or(false) {
                continue;
            }
            if let Some(val) = py_to_configval(&v) {
                raw.insert(key, val);
            }
        }

        Ok(raw)
    })
}

/// Re-executes `conf.py` and returns its module-level `setup` callable, if
/// it defines one.
///
/// Mirrors upstream `sphinx.config.Config.setup` / the `if self.config.setup:
/// self.config.setup(self)` branch near the top of `Sphinx.__init__`
/// (`sphinx/application.py`) — `conf.py` itself is treated exactly like an
/// extension module: if it defines a top-level `def setup(app): ...`
/// function, Sphinx calls it with the running `app`, letting a project's
/// own `conf.py` register event listeners (`app.connect(...)`), add config
/// values, etc. without needing to package a separate extension module.
///
/// This is a second, separate execution of `conf.py` (rather than reusing
/// [`raw_config_from_conf_py`]'s `globals`) because that function's
/// `Python::attach` closure — and the `globals` dict living inside it — do
/// not outlive the call; re-running the (idempotent, side-effect-light)
/// module body is simpler than threading a GIL-bound `Bound<'py, PyDict>`
/// back out through a `PyResult`-returning API. `Py<PyAny>` (unlike
/// `Bound`) owns a GIL-independent reference, so it can be safely returned
/// and later invoked from a fresh `Python::attach` block by the caller.
///
/// # Errors
///
/// Returns a `PyErr` if the file cannot be read or if conf.py raises during
/// execution.
pub fn conf_py_setup(path: &Path) -> PyResult<Option<Py<PyAny>>> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::new_err(format!("cannot read {}: {e}", path.display())))?;

    Python::attach(|py| {
        let globals = PyDict::new(py);
        globals.set_item("__file__", path.to_string_lossy().as_ref())?;
        py.run(
            &std::ffi::CString::new(source.as_str()).unwrap(),
            Some(&globals),
            None,
        )
        .map_err(|e| ConfigError::new_err(format!("conf.py failed: {e}")))?;

        match globals.get_item("setup")? {
            Some(v) if v.hasattr("__call__").unwrap_or(false) => Ok(Some(v.unbind())),
            _ => Ok(None),
        }
    })
}

/// Recursively convert an arbitrary Python value into a [`ConfigVal`].
///
/// Used by [`raw_config_from_conf_py`]'s generic fallback pass to capture
/// `conf.py` options this module has no bespoke per-key handling for.
/// Returns `None` for anything that isn't (recursively) one of
/// `NoneType`/`bool`/`int`/`float`/`str`/`list`/`tuple`/`dict` — e.g.
/// modules, functions, classes, or a container holding one of those —
/// since those aren't legitimate config values upstream would ever store
/// on `Config` either.
fn py_to_configval(v: &pyo3::Bound<'_, pyo3::PyAny>) -> Option<ConfigVal> {
    if v.is_none() {
        Some(ConfigVal::Null)
    } else if let (Ok(title), Ok(url)) = (
        v.getattr("title")
            .and_then(|value| value.extract::<String>()),
        v.getattr("url").and_then(|value| value.extract::<String>()),
    ) {
        // Sphinx extensions commonly put named tuples such as
        // pallets_sphinx_themes.ProjectLink in html_context. Preserve their
        // attribute shape so Jinja templates can use item.title/item.url.
        Some(ConfigVal::Map(vec![
            ("title".to_string(), ConfigVal::Str(title)),
            ("url".to_string(), ConfigVal::Str(url)),
        ]))
    } else if let Ok(b) = v.extract::<bool>() {
        Some(ConfigVal::Bool(b))
    } else if let Ok(i) = v.extract::<i64>() {
        Some(ConfigVal::Int(i))
    } else if let Ok(f) = v.extract::<f64>() {
        Some(ConfigVal::Float(f))
    } else if let Ok(s) = v.extract::<String>() {
        Some(ConfigVal::Str(s))
    } else if let Ok(list) = v.cast::<PyList>() {
        let mut items = Vec::with_capacity(list.len());
        for item in list.iter() {
            items.push(py_to_configval(&item)?);
        }
        Some(ConfigVal::List(items))
    } else if let Ok(tuple) = v.cast::<pyo3::types::PyTuple>() {
        let mut items = Vec::with_capacity(tuple.len());
        for item in tuple.iter() {
            items.push(py_to_configval(&item)?);
        }
        Some(ConfigVal::List(items))
    } else if let Ok(dict) = v.cast::<PyDict>() {
        let mut entries = Vec::with_capacity(dict.len());
        for (k, val) in dict.iter() {
            let key = k.extract::<String>().ok()?;
            entries.push((key, py_to_configval(&val)?));
        }
        Some(ConfigVal::Map(entries))
    } else {
        None
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

// ═════════════════════════════════════════════════════════════════════════════
// SphinxConfig — full port of sphinx.config.Config
// ═════════════════════════════════════════════════════════════════════════════

/// The "rebuild" scope for a config value — mirrors `_ConfigRebuild` in
/// `sphinx.config`.
///
/// An empty string means "no rebuild required when this value changes".
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RebuildKind {
    /// No rebuild required.
    None,
    /// Re-read the whole environment.
    Env,
    /// Rebuild epub output.
    Epub,
    /// Rebuild gettext output.
    Gettext,
    /// Rebuild html output.
    Html,
}

impl RebuildKind {
    /// Canonical string form used by upstream Sphinx.
    pub fn as_str(&self) -> &'static str {
        match self {
            RebuildKind::None => "",
            RebuildKind::Env => "env",
            RebuildKind::Epub => "epub",
            RebuildKind::Gettext => "gettext",
            RebuildKind::Html => "html",
        }
    }
}

impl std::str::FromStr for RebuildKind {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "env" => RebuildKind::Env,
            "epub" => RebuildKind::Epub,
            "gettext" => RebuildKind::Gettext,
            "html" => RebuildKind::Html,
            _ => RebuildKind::None,
        })
    }
}

/// A typed configuration value.
///
/// Mirrors `ConfigValue = NamedTuple(name, value, rebuild)` in
/// `sphinx.config`.
#[derive(Debug, Clone)]
pub struct ConfigValue {
    pub name: String,
    pub value: ConfigVal,
    pub rebuild: RebuildKind,
}

/// The runtime value of a sphinx config option.
///
/// We use a richer enum than a bare `serde_json::Value` to carry
/// Rust-native booleans and integers precisely.
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigVal {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<ConfigVal>),
    Map(Vec<(String, ConfigVal)>),
}

impl ConfigVal {
    /// Return a string representation, matching Python `str(value)`.
    pub fn display(&self) -> String {
        match self {
            ConfigVal::Null => "None".into(),
            ConfigVal::Bool(b) => if *b { "True" } else { "False" }.into(),
            ConfigVal::Int(i) => i.to_string(),
            ConfigVal::Float(f) => f.to_string(),
            ConfigVal::Str(s) => s.clone(),
            ConfigVal::List(v) => {
                let items: Vec<_> = v.iter().map(|x| x.display()).collect();
                format!("[{}]", items.join(", "))
            }
            ConfigVal::Map(m) => {
                let items: Vec<_> = m
                    .iter()
                    .map(|(k, v)| format!("{k:?}: {}", v.display()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
        }
    }

    /// Coerce a string override value to the same type as a given default.
    ///
    /// Mirrors `Config.convert_overrides`.
    pub fn coerce_override(default: &ConfigVal, value: &str) -> Result<ConfigVal, String> {
        match default {
            ConfigVal::Bool(_) => match value {
                "0" => Ok(ConfigVal::Bool(false)),
                "1" => Ok(ConfigVal::Bool(true)),
                _ => Err(format!("must be '0' or '1', got {value:?}")),
            },
            ConfigVal::Int(_) => value
                .parse::<i64>()
                .map(ConfigVal::Int)
                .map_err(|_| format!("invalid number {value:?}")),
            ConfigVal::List(_) => Ok(ConfigVal::List(
                value
                    .split(',')
                    .map(|s| ConfigVal::Str(s.trim().to_string()))
                    .collect(),
            )),
            _ => Ok(ConfigVal::Str(value.to_string())),
        }
    }

    /// Return the string if this is a `Str`, otherwise `None`.
    pub fn as_str(&self) -> Option<&str> {
        if let ConfigVal::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Return the bool if this is a `Bool`, otherwise `None`.
    pub fn as_bool(&self) -> Option<bool> {
        if let ConfigVal::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// Return the integer if this is an `Int`, otherwise `None`.
    pub fn as_int(&self) -> Option<i64> {
        if let ConfigVal::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    /// Return the list items if this is a `List`, otherwise `None`.
    pub fn as_list(&self) -> Option<&[ConfigVal]> {
        if let ConfigVal::List(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Return the key/value pairs if this is a `Map`, otherwise `None`.
    pub fn as_map(&self) -> Option<&[(String, ConfigVal)]> {
        if let ConfigVal::Map(m) = self {
            Some(m)
        } else {
            None
        }
    }
}

/// A registered configuration option descriptor.
///
/// Mirrors `_Opt` in `sphinx.config`.
#[derive(Debug, Clone)]
pub struct ConfigOpt {
    /// Default value (static; callable defaults handled in `SphinxConfig`).
    pub default: ConfigVal,
    /// When the config value changes, what needs rebuilding.
    pub rebuild: RebuildKind,
    /// Human-readable description.
    pub description: String,
}

/// Full port of `sphinx.config.Config`.
///
/// Stores the raw values read from `conf.py` (as `raw_config`),
/// command-line overrides (`overrides`), and the registered option
/// descriptors (`options`). Values are resolved lazily via
/// [`SphinxConfig::get`].
///
/// Unlike the Python version which uses `__getattr__` magic, the Rust
/// port provides an explicit [`get`] method and typed helpers
/// (`project()`, `language()`, etc.).
///
/// # Example
///
/// ```rust
/// use sphinxdocrs::config::SphinxConfig;
/// let cfg = SphinxConfig::new_defaults();
/// assert_eq!(cfg.project(), "Project name not set");
/// assert_eq!(cfg.language(), "en");
/// assert!(cfg.extensions().is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct SphinxConfig {
    /// Values read from `conf.py` (string keys → `ConfigVal`).
    raw_config: HashMap<String, ConfigVal>,
    /// Command-line overrides (always strings from `-D key=value`).
    overrides: HashMap<String, String>,
    /// Registered options (built-in + extension-added).
    options: HashMap<String, ConfigOpt>,
    /// The `extensions` list, extracted from `raw_config` at construction.
    pub extensions: Vec<String>,
    /// HTML themes registered by loaded extensions. This is runtime state,
    /// not a conf.py option, and is populated by `SphinxApp` after setup.
    registered_themes: Vec<(String, std::path::PathBuf)>,
}

impl SphinxConfig {
    /// Construct with only the built-in default options and no `conf.py`.
    pub fn new_defaults() -> Self {
        let mut cfg = Self {
            raw_config: HashMap::new(),
            overrides: HashMap::new(),
            options: HashMap::new(),
            extensions: Vec::new(),
            registered_themes: Vec::new(),
        };
        cfg.register_builtin_options();
        cfg
    }

    /// Construct from a parsed `conf.py` namespace and command-line overrides.
    ///
    /// Mirrors `Config.__init__`.
    pub fn new(raw_config: HashMap<String, ConfigVal>, overrides: HashMap<String, String>) -> Self {
        let mut raw_config = raw_config;
        let current_year = {
            use crate::cli::io::{Clock, SystemClock};
            SystemClock.year().to_string()
        };
        for key in ["copyright", "project_copyright"] {
            if let Some(ConfigVal::Str(value)) = raw_config.get_mut(key) {
                *value = value.replace("%Y", &current_year);
            }
        }
        let extensions = match raw_config.get("extensions") {
            Some(ConfigVal::List(v)) => v
                .iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect(),
            _ => Vec::new(),
        };

        let mut cfg = Self {
            raw_config,
            overrides,
            options: HashMap::new(),
            extensions,
            registered_themes: Vec::new(),
        };
        cfg.register_builtin_options();
        cfg
    }

    /// Install HTML themes registered through `app.add_html_theme()`.
    pub(crate) fn set_registered_themes(&mut self, themes: Vec<(String, std::path::PathBuf)>) {
        self.registered_themes = themes;
    }

    /// Return HTML themes registered by extensions, in registration order.
    pub(crate) fn registered_themes(&self) -> &[(String, std::path::PathBuf)] {
        &self.registered_themes
    }

    // ── option registry ───────────────────────────────────────────────────────

    /// Register all built-in sphinx config options.
    ///
    /// Mirrors `Config.config_values` class attribute.
    fn register_builtin_options(&mut self) {
        use ConfigVal::*;
        use RebuildKind::*;
        let mut add = |name: &str, default: ConfigVal, rebuild: RebuildKind, desc: &str| {
            self.options.insert(
                name.to_string(),
                ConfigOpt {
                    default,
                    rebuild,
                    description: desc.to_string(),
                },
            );
        };

        // General options
        add(
            "project",
            Str("Project name not set".into()),
            Env,
            "Project name",
        );
        add(
            "author",
            Str("Author name not set".into()),
            Env,
            "Author name",
        );
        add(
            "project_copyright",
            Str(String::new()),
            Html,
            "Copyright string",
        );
        add(
            "source_suffix",
            Map(vec![(".rst".into(), Str("restructuredtext".into()))]),
            Env,
            "Source file suffix to parser mapping",
        );
        add(
            "source_backend",
            Str("static".into()),
            Env,
            "Source documentation backend: auto, static, lsp, or hybrid",
        );
        add(
            "source_lsp_servers",
            Map(Vec::new()),
            Env,
            "Language-server argv by source language",
        );
        add(
            "source_lsp_timeout",
            Int(30_000),
            Env,
            "Language-server request timeout in milliseconds",
        );
        add(
            "source_lsp_allow_fallback",
            Bool(true),
            Env,
            "Allow hybrid source analysis to fall back to static results",
        );
        add(
            "source_lsp_workspace_root",
            Null,
            Env,
            "Workspace root passed to configured language servers",
        );
        add(
            "source_lsp_read_only_roots",
            List(Vec::new()),
            Env,
            "Additional absolute read-only roots exposed to protected language servers",
        );
        add(
            "source_lsp_sandbox_environment",
            Map(Vec::new()),
            Env,
            "Explicit environment allowlist for protected language servers",
        );
        add(
            "source_lsp_sandbox",
            Str("off".into()),
            Env,
            "LSP process mode: off, trusted-local, or protected-lsp",
        );
        add(
            "source_build_sandbox",
            Str("off".into()),
            Env,
            "Build process mode: off or protected-build",
        );
        add("version", Str(String::new()), Env, "Version string");
        add("release", Str(String::new()), Env, "Release string");
        add("today", Str(String::new()), Env, "Date override");
        add("today_fmt", Null, Env, "strftime format");
        add("language", Str("en".into()), Env, "Language");
        add("epub_title", Str(String::new()), Env, "EPUB title");
        add("epub_author", Str(String::new()), Env, "EPUB author");
        add("epub_language", Str(String::new()), Env, "EPUB language");
        add(
            "epub_uid",
            Str(String::new()),
            Env,
            "EPUB unique identifier",
        );
        add(
            "epub_description",
            Str(String::new()),
            Env,
            "EPUB description",
        );
        add("epub_publisher", Str(String::new()), Env, "EPUB publisher");
        add("epub_copyright", Str(String::new()), Env, "EPUB rights");
        add(
            "epub_basename",
            Str(String::new()),
            Env,
            "EPUB output basename",
        );
        add(
            "locale_dirs",
            List(vec![Str("locales".into())]),
            Env,
            "Locale directories",
        );
        add(
            "figure_language_filename",
            Str("{root}.{language}{ext}".into()),
            Env,
            "Figure filename template",
        );
        add(
            "gettext_allow_fuzzy_translations",
            Bool(false),
            Gettext,
            "Allow fuzzy gettext",
        );
        add(
            "gettext_compact",
            Bool(true),
            Gettext,
            "Compact gettext catalog domains",
        );
        add(
            "gettext_location",
            Bool(true),
            Gettext,
            "Include gettext source locations",
        );
        add(
            "gettext_uuid",
            Bool(false),
            Gettext,
            "Include gettext message UUIDs",
        );
        add("master_doc", Str("index".into()), Env, "Master document");
        add(
            "root_doc",
            Str("index".into()),
            Env,
            "Root document (alias of master_doc)",
        );
        add(
            "source_encoding",
            Str("utf-8-sig".into()),
            Env,
            "Source encoding",
        );
        add("exclude_patterns", List(vec![]), Env, "Exclude patterns");
        add(
            "include_patterns",
            List(vec![Str("**".into())]),
            Env,
            "Include patterns",
        );
        add("default_role", Null, Env, "Default role");
        add(
            "add_function_parentheses",
            Bool(true),
            Env,
            "Add () to function refs",
        );
        add(
            "add_module_names",
            Bool(true),
            Env,
            "Add module names to signatures",
        );
        add("toc_object_entries", Bool(true), Env, "TOC object entries");
        add(
            "toc_object_entries_show_parents",
            Str("domain".into()),
            Env,
            "TOC parent visibility",
        );
        add(
            "trim_footnote_reference_space",
            Bool(false),
            Env,
            "Trim footnote space",
        );
        add(
            "show_authors",
            Bool(false),
            Env,
            "Show :sectionauthor:/:moduleauthor:",
        );
        add("pygments_style", Null, Html, "Pygments style");
        add(
            "html_add_external_link_class",
            Bool(true),
            Html,
            "Add reference and external classes to content links",
        );
        add(
            "highlight_language",
            Str("default".into()),
            Env,
            "Default highlight language",
        );
        add("highlight_options", Map(vec![]), Env, "Highlight options");
        add("templates_path", List(vec![]), Html, "Templates path");
        add("template_bridge", Null, Html, "Template bridge class");
        add("keep_warnings", Bool(false), Env, "Keep warnings in output");
        add(
            "suppress_warnings",
            List(vec![]),
            Env,
            "Suppressed warning types",
        );
        add(
            "show_warning_types",
            Bool(true),
            Env,
            "Show warning type codes",
        );
        add(
            "modindex_common_prefix",
            List(vec![]),
            Html,
            "Module index common prefix",
        );
        add("rst_epilog", Null, Env, "RST epilog");
        add("rst_prolog", Null, Env, "RST prolog");
        add("trim_doctest_flags", Bool(true), Env, "Trim doctest flags");
        add("primary_domain", Str("py".into()), Env, "Primary domain");
        add("needs_sphinx", Null, None, "Minimum sphinx version");
        add(
            "needs_extensions",
            Map(vec![]),
            None,
            "Required extension versions",
        );
        add(
            "latex_documents",
            List(vec![]),
            Env,
            "LaTeX output documents",
        );
        add("man_pages", List(vec![]), Env, "Man page output documents");
        add("manpages_url", Null, Env, "Manpages URL template");
        add("nitpicky", Bool(false), None, "Nitpicky mode");
        add("nitpick_ignore", List(vec![]), None, "Nitpick ignore list");
        add(
            "nitpick_ignore_regex",
            List(vec![]),
            None,
            "Nitpick ignore regex list",
        );
        add("numfig", Bool(false), Env, "Numbered figures");
        add("numfig_secnum_depth", Int(1), Env, "numfig section depth");
        add("numfig_format", Map(vec![]), Env, "numfig format strings");
        add(
            "maximum_signature_line_length",
            Null,
            Env,
            "Max signature line length",
        );
        add(
            "math_number_all",
            Bool(false),
            Env,
            "Number all math equations",
        );
        add(
            "math_eqref_format",
            Null,
            Env,
            "Math equation reference format",
        );
        add("math_numfig", Bool(true), Env, "Number math per figure");
        add(
            "math_numsep",
            Str(".".into()),
            Env,
            "Math numbering separator",
        );
        add("tls_verify", Bool(true), Env, "Verify TLS certs");
        add("tls_cacerts", Null, Env, "CA certs path");
        add("user_agent", Null, Env, "HTTP user agent");
        add("smartquotes", Bool(true), Env, "Enable smartquotes");
        add(
            "smartquotes_action",
            Str("qDe".into()),
            Env,
            "Smartquotes action",
        );
        add(
            "option_emphasise_placeholders",
            Bool(false),
            Env,
            "Emphasise option placeholders",
        );
        // Extensions list
        add("extensions", List(vec![]), Env, "Extensions list");
        add(
            "docindex_enabled",
            Bool(true),
            Html,
            "Enable the native DocIndex extension",
        );
        add(
            "docindex_artifact_enabled",
            Bool(true),
            Html,
            "Write the local DocIndex JSON artifact",
        );
        add(
            "docindex_artifact_path",
            Str("_static/docindex.json".into()),
            Html,
            "Path for the local DocIndex JSON artifact",
        );
        add(
            "docindex_rdf_hdt_enabled",
            Bool(true),
            Html,
            "Write the DocIndex RDF/HDT artifact",
        );
        add(
            "docindex_webmcp_enabled",
            Bool(true),
            Html,
            "Deprecated compatibility option; WebMCP remains enabled",
        );
        // HTML static assets
        add(
            "html_theme",
            Str("alabaster".into()),
            Html,
            "Active HTML theme name",
        );
        add(
            "html_theme_path",
            List(vec![]),
            Html,
            "Extra directories (relative to confdir) to search for HTML themes",
        );
        add(
            "html_static_path",
            List(vec![]),
            Html,
            "Directories of static files copied to _static/",
        );
        add(
            "html_css_files",
            List(vec![]),
            Html,
            "Additional CSS files linked on every HTML page",
        );
        add(
            "html_extra_path",
            List(vec![]),
            Html,
            "Directories of files copied verbatim to the output root",
        );
        // H6c: full page context — title/copyright/sourcelink/context options.
        add(
            "html_title",
            Null,
            Html,
            "Page title prefix (default: derived from project/release)",
        );
        add(
            "html_short_title",
            Null,
            Html,
            "Short title for the navigation bar",
        );
        add(
            "html_context",
            Map(vec![]),
            Html,
            "Extra template context merged into every page",
        );
        add(
            "html_theme_options",
            Map(vec![]),
            Html,
            "Theme-specific options (exposed to templates as `theme_<key>`)",
        );
        add(
            "html_show_copyright",
            Bool(true),
            Html,
            "Show the copyright line in the footer",
        );
        add(
            "html_show_sphinx",
            Bool(true),
            Html,
            "Show the \"Created using Sphinx\" credit line",
        );
        add(
            "html_show_search_summary",
            Bool(true),
            Html,
            "Show a summary next to each search result",
        );
        add(
            "html_copy_source",
            Bool(true),
            Html,
            "Copy source .rst files to _sources/",
        );
        add(
            "html_show_sourcelink",
            Bool(true),
            Html,
            "Show a \"View page source\" link",
        );
        add(
            "html_sourcelink_suffix",
            Str(".txt".into()),
            Html,
            "Suffix appended to copied source filenames",
        );
        add(
            "html_use_opensearch",
            Str(String::new()),
            Html,
            "Base URL for an OpenSearch description",
        );
        add(
            "html_baseurl",
            Str(String::new()),
            Html,
            "Base URL the docs will be hosted at",
        );
        add("html_logo", Null, Html, "Path (or URL) to a logo image");
        add("html_favicon", Null, Html, "Path (or URL) to a favicon");
        add(
            "html_last_updated_fmt",
            Null,
            Html,
            "strftime format for the \"last updated\" string",
        );
        add(
            "html_sidebars",
            Map(vec![]),
            Html,
            "Pattern -> sidebar template list mapping",
        );
        add(
            "html_domain_indices",
            Bool(true),
            Html,
            "Generate domain-specific indices",
        );
        add(
            "html_use_index",
            Bool(true),
            Html,
            "Generate the general index",
        );
        // Intersphinx
        add(
            "intersphinx_mapping",
            Map(vec![]),
            Env,
            "External project cross-references",
        );
        add(
            "intersphinx_cache_limit",
            Int(5),
            Env,
            "Number of days to cache intersphinx inventories",
        );
        // Linkcheck
        add(
            "linkcheck_ignore",
            List(vec![]),
            Env,
            "Regex patterns of URIs to skip (reported as ignored)",
        );
        add(
            "linkcheck_allowed_redirects",
            Map(vec![]),
            Env,
            "Regex map of URI pattern -> allowed redirect-target pattern",
        );
        add(
            "linkcheck_anchors",
            Bool(true),
            Env,
            "Check that #fragment anchors exist in the target page",
        );
        add(
            "linkcheck_anchors_ignore",
            List(vec![Str("^!".into())]),
            Env,
            "Regex patterns of anchors to skip checking",
        );
        add(
            "linkcheck_timeout",
            Int(30),
            Env,
            "Seconds to wait for a response before treating a link as broken",
        );
        add(
            "linkcheck_retries",
            Int(1),
            Env,
            "Number of times to retry a rate-limited or failed request",
        );
        add(
            "linkcheck_rate_limit_timeout",
            Float(300.0),
            Env,
            "Seconds to keep retrying a rate-limited host before giving up",
        );
    }

    /// Register an extension-provided config option.
    ///
    /// Mirrors `Config.add()`.
    pub fn add(
        &mut self,
        name: &str,
        default: ConfigVal,
        rebuild: RebuildKind,
        description: &str,
    ) -> Result<(), String> {
        if self.options.contains_key(name) {
            return Err(format!("Config value {name:?} already present"));
        }
        self.options.insert(
            name.to_string(),
            ConfigOpt {
                default,
                rebuild,
                description: description.to_string(),
            },
        );
        Ok(())
    }

    /// Return `true` if `name` is a known config option.
    ///
    /// Mirrors `name in cfg` (Python `__contains__`).
    pub fn contains(&self, name: &str) -> bool {
        self.options.contains_key(name)
    }

    /// The raw `conf.py` values (string keys → [`ConfigVal`]), unfiltered
    /// by option registration — i.e. every plain-data top-level name
    /// `conf.py` set, whether or not any `add_config_value` has (yet)
    /// registered a matching option.
    ///
    /// Mirrors upstream `Config._raw_config`. Used by
    /// [`crate::app_facade::PyAppFacade::add_config_value`] to replicate
    /// `Config.__getattr__`'s exact precedence (`_raw_config` always wins
    /// over a newly-registered option's `default`, but — matching upstream
    /// — the value only becomes visible *after* something registers the
    /// name): this port's `SphinxApp.config` isn't a shared/mutable
    /// `Rc<RefCell<_>>` the way `app.config`'s own `SharedConfig` is, so
    /// `add_config_value` can't call back into `SphinxConfig::get` — it
    /// consults this raw snapshot directly instead.
    pub fn raw_config(&self) -> &HashMap<String, ConfigVal> {
        &self.raw_config
    }

    // ── value resolution ─────────────────────────────────────────────────────

    /// Resolve the value of a config option.
    ///
    /// Resolution order:
    /// 1. Command-line override (coerced from string).
    /// 2. `conf.py` raw value.
    /// 3. Built-in default.
    ///
    /// Returns `None` if `name` is not a registered option.
    pub fn get(&self, name: &str) -> Option<ConfigVal> {
        let opt = self.options.get(name)?;
        // 1. override
        if let Some(raw) = self.overrides.get(name) {
            if let Ok(v) = ConfigVal::coerce_override(&opt.default, raw) {
                return Some(v);
            } // else fall through to raw_config
        }
        // 2. raw_config
        if let Some(v) = self.raw_config.get(name) {
            return Some(v.clone());
        }
        // 3. default
        // Alias resolution: root_doc ↔ master_doc, copyright ↔ project_copyright.
        let default = match name {
            "root_doc" => self
                .raw_config
                .get("master_doc")
                .cloned()
                .unwrap_or_else(|| opt.default.clone()),
            "master_doc" => self
                .raw_config
                .get("root_doc")
                .cloned()
                .unwrap_or_else(|| opt.default.clone()),
            "copyright" => self
                .raw_config
                .get("project_copyright")
                .cloned()
                .unwrap_or_else(|| opt.default.clone()),
            "project_copyright" => self
                .raw_config
                .get("copyright")
                .cloned()
                .unwrap_or_else(|| opt.default.clone()),
            _ => opt.default.clone(),
        };
        Some(default)
    }

    /// Encoding used to decode source documents, matching Sphinx's
    /// `source_encoding` setting and its `utf-8-sig` default.
    pub fn source_encoding(&self) -> String {
        self.get("source_encoding")
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "utf-8-sig".to_string())
    }

    /// Set a config value (mirrors `cfg[name] = value`).
    pub fn set(&mut self, name: impl Into<String>, value: ConfigVal) {
        let name = name.into();
        // Keep aliases in sync.
        match name.as_str() {
            "master_doc" => {
                self.raw_config.insert("root_doc".into(), value.clone());
            }
            "root_doc" => {
                self.raw_config.insert("master_doc".into(), value.clone());
            }
            "copyright" => {
                self.raw_config
                    .insert("project_copyright".into(), value.clone());
            }
            "project_copyright" => {
                self.raw_config.insert("copyright".into(), value.clone());
            }
            _ => {}
        }
        self.raw_config.insert(name, value);
    }

    /// Iterate over all `(ConfigValue)` entries.
    ///
    /// Mirrors `__iter__` in Python.
    pub fn iter(&self) -> impl Iterator<Item = ConfigValue> + '_ {
        self.options.iter().map(move |(name, opt)| ConfigValue {
            name: name.clone(),
            value: self.get(name).unwrap_or(ConfigVal::Null),
            rebuild: opt.rebuild.clone(),
        })
    }

    /// Iterate over entries matching a specific rebuild kind.
    ///
    /// Mirrors `Config.filter(rebuild)`.
    pub fn filter(&self, rebuild: &RebuildKind) -> impl Iterator<Item = ConfigValue> + '_ {
        let rebuild = rebuild.clone();
        self.iter().filter(move |cv| cv.rebuild == rebuild)
    }

    /// A stable hash over every config value whose `rebuild` kind is not
    /// [`RebuildKind::None`] (i.e. every value that upstream's own
    /// `BuildEnvironment.config_status` diffing would care about),
    /// persisted alongside the environment (**H8b**) so a later build can
    /// tell whether `conf.py`/`-D` overrides changed since the env was
    /// last saved.
    ///
    /// **Accepted deviation:** upstream compares the *previous* `Config`
    /// object value-by-value (distinguishing "an `Env`-rebuild value
    /// changed" from "only an `Html`-rebuild value changed", to choose
    /// between [`CONFIG_CHANGED`](crate::environment::CONFIG_CHANGED) and
    /// a builder-specific rebuild). This is a single combined hash: any
    /// change to any rebuild-relevant value is reported as
    /// `CONFIG_CHANGED`, forcing a full re-read rather than upstream's
    /// finer-grained partial rebuild. Simpler and safe (never under-detects
    /// a change), at the cost of over-invalidating in the rarer case where
    /// only an `Html`/`Epub`/`Gettext`-only value changed.
    pub fn stable_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut entries: Vec<(String, String)> = self
            .iter()
            .filter(|cv| cv.rebuild != RebuildKind::None)
            .map(|cv| (cv.name, cv.value.display()))
            .collect();
        entries.sort();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        entries.hash(&mut hasher);
        hasher.finish()
    }

    // ── typed accessors ───────────────────────────────────────────────────────

    /// `project` — project name.
    pub fn project(&self) -> String {
        self.get("project")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "Project name not set".into())
    }

    /// `author` — author name.
    pub fn author(&self) -> String {
        self.get("author")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "Author name not set".into())
    }

    /// `language` — document language (default `"en"`).
    pub fn language(&self) -> String {
        self.get("language")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "en".into())
    }

    /// `version` — short version string.
    pub fn version(&self) -> String {
        self.get("version")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default()
    }

    /// `release` — full release string.
    pub fn release(&self) -> String {
        self.get("release")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default()
    }

    /// `master_doc` / `root_doc` — root document name.
    pub fn root_doc(&self) -> String {
        self.get("root_doc")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "index".into())
    }

    /// `extensions` — list of extension module names.
    pub fn extensions(&self) -> Vec<String> {
        self.get("extensions")
            .and_then(|v| {
                if let ConfigVal::List(items) = v {
                    Some(
                        items
                            .iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    /// `html_css_files` — project stylesheets linked on every HTML page.
    pub fn html_css_files(&self) -> Vec<String> {
        self.get("html_css_files")
            .and_then(|v| {
                if let ConfigVal::List(items) = v {
                    Some(
                        items
                            .iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    /// `needs_extensions` — extension name → minimum required version
    /// string. Mirrors `Config.needs_extensions`; verified by
    /// `SphinxApp::verify_needs_extensions`.
    pub fn needs_extensions(&self) -> HashMap<String, String> {
        self.get("needs_extensions")
            .and_then(|v| {
                if let ConfigVal::Map(items) = v {
                    Some(
                        items
                            .into_iter()
                            .filter_map(|(k, val)| val.as_str().map(|s| (k, s.to_string())))
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    /// `exclude_patterns` — list of glob patterns to exclude.
    pub fn exclude_patterns(&self) -> Vec<String> {
        self.get("exclude_patterns")
            .and_then(|v| {
                if let ConfigVal::List(items) = v {
                    Some(
                        items
                            .iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    /// `include_patterns` — list of glob patterns to include (defaults to
    /// `["**"]`, i.e. everything).
    pub fn include_patterns(&self) -> Vec<String> {
        self.get("include_patterns")
            .and_then(|v| {
                if let ConfigVal::List(items) = v {
                    Some(
                        items
                            .iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_else(|| vec!["**".to_string()])
    }

    /// `highlight_language` — default code-block language.
    pub fn highlight_language(&self) -> String {
        self.get("highlight_language")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "default".into())
    }

    /// `html_add_external_link_class` — opt in to `class="external"` on
    /// non-internal content links.
    pub fn html_add_external_link_class(&self) -> bool {
        self.get("html_add_external_link_class")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// `numfig` — whether numbered figures are enabled.
    pub fn numfig(&self) -> bool {
        self.get("numfig")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// `nitpicky` — whether nitpicky mode is enabled.
    pub fn nitpicky(&self) -> bool {
        self.get("nitpicky")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// `smartquotes` — whether smart-quotes are enabled.
    pub fn smartquotes(&self) -> bool {
        self.get("smartquotes")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// `rst_prolog` — text prepended to every RST source file.
    pub fn rst_prolog(&self) -> Option<String> {
        self.get("rst_prolog")
            .and_then(|v| v.as_str().map(String::from))
    }

    /// `rst_epilog` — text appended to every RST source file.
    pub fn rst_epilog(&self) -> Option<String> {
        self.get("rst_epilog")
            .and_then(|v| v.as_str().map(String::from))
    }

    // ── source_suffix helpers ─────────────────────────────────────────────────

    /// Return the `source_suffix` map: extension → parser name.
    ///
    /// Defaults to `{".rst": "restructuredtext"}`.
    pub fn source_suffix(&self) -> HashMap<String, String> {
        match self.get("source_suffix") {
            Some(ConfigVal::Map(pairs)) => pairs
                .into_iter()
                .map(|(k, v)| (k, v.as_str().map(String::from).unwrap_or_default()))
                .collect(),
            Some(ConfigVal::Str(ext)) => {
                let mut m = HashMap::new();
                m.insert(ext, "restructuredtext".into());
                m
            }
            Some(ConfigVal::List(exts)) => exts
                .iter()
                .filter_map(|v| {
                    v.as_str()
                        .map(|s| (s.to_string(), "restructuredtext".to_string()))
                })
                .collect(),
            _ => {
                let mut m = HashMap::new();
                m.insert(".rst".into(), "restructuredtext".into());
                m
            }
        }
    }

    /// `intersphinx_mapping` — map of project name → `(base_url, inv_url)`.
    ///
    /// Returns an empty vec when the extension is not configured or the
    /// config was not loaded from a `conf.py` that contains the key.
    ///
    /// The optional `inv_url` is `None` when the Python value was `None`;
    /// callers should default to `"{base_url}/objects.inv"` in that case.
    pub fn intersphinx_mapping(&self) -> Vec<(String, String, Option<String>)> {
        let Some(ConfigVal::Map(entries)) = self.get("intersphinx_mapping") else {
            return Vec::new();
        };
        entries
            .into_iter()
            .filter_map(|(name, val)| {
                let ConfigVal::List(v) = val else { return None };
                let url = v.first()?.as_str()?.to_string();
                let inv = v.get(1).and_then(|x| x.as_str()).map(String::from);
                Some((name, url, inv))
            })
            .collect()
    }

    /// `linkcheck_ignore` — regex patterns of URIs to skip entirely (reported
    /// as `ignored`, never fetched).
    pub fn linkcheck_ignore(&self) -> Vec<String> {
        self.get("linkcheck_ignore")
            .and_then(|v| {
                v.as_list().map(|items| {
                    items
                        .iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                })
            })
            .unwrap_or_default()
    }

    /// `linkcheck_allowed_redirects` — map of URI-pattern regex to
    /// redirect-target-pattern regex. A redirect whose original URI and
    /// final URI both match a `(from, to)` pair is classified `working`
    /// rather than `redirected`.
    pub fn linkcheck_allowed_redirects(&self) -> Vec<(String, String)> {
        let Some(ConfigVal::Map(entries)) = self.get("linkcheck_allowed_redirects") else {
            return Vec::new();
        };
        entries
            .into_iter()
            .filter_map(|(from, to)| to.as_str().map(|to| (from, to.to_string())))
            .collect()
    }

    /// `linkcheck_anchors` — whether to verify that `#fragment` anchors exist
    /// in the fetched page. Defaults to `true`.
    pub fn linkcheck_anchors(&self) -> bool {
        matches!(
            self.get("linkcheck_anchors"),
            Some(ConfigVal::Bool(true)) | None
        )
    }

    /// `linkcheck_anchors_ignore` — regex patterns of anchor names to skip
    /// checking. Defaults to `["^!"]`, matching upstream (anchors starting
    /// with `!` are commonly generated dynamically by JS).
    pub fn linkcheck_anchors_ignore(&self) -> Vec<String> {
        match self.get("linkcheck_anchors_ignore") {
            Some(ConfigVal::List(items)) => items
                .iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect(),
            _ => vec!["^!".to_string()],
        }
    }

    /// `linkcheck_timeout` — seconds to wait for a response. Defaults to 30.
    pub fn linkcheck_timeout(&self) -> u32 {
        match self.get("linkcheck_timeout") {
            Some(ConfigVal::Int(i)) if i > 0 => i as u32,
            Some(ConfigVal::Float(f)) if f > 0.0 => f as u32,
            _ => 30,
        }
    }

    /// `linkcheck_retries` — number of retry attempts for a rate-limited or
    /// failed request. Defaults to 1.
    pub fn linkcheck_retries(&self) -> u32 {
        match self.get("linkcheck_retries") {
            Some(ConfigVal::Int(i)) if i >= 0 => i as u32,
            _ => 1,
        }
    }

    /// `linkcheck_rate_limit_timeout` — seconds to keep retrying a
    /// rate-limited host before giving up. Defaults to 300.
    pub fn linkcheck_rate_limit_timeout(&self) -> f64 {
        match self.get("linkcheck_rate_limit_timeout") {
            Some(ConfigVal::Float(f)) if f >= 0.0 => f,
            Some(ConfigVal::Int(i)) if i >= 0 => i as f64,
            _ => 300.0,
        }
    }

    /// `html_theme` — the active HTML theme name.  Sphinx's default is
    /// `"alabaster"`.
    pub fn html_theme(&self) -> String {
        self.get("html_theme")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "alabaster".into())
    }

    /// `html_static_path` — directories of static files copied into `_static/`.
    ///
    /// Defaults to an empty list (matching sphinx).  The `sphinx/doc` project
    /// sets `html_static_path = ['_static']`.
    pub fn html_static_path(&self) -> Vec<String> {
        self.get("html_static_path")
            .and_then(|v| {
                if let ConfigVal::List(items) = v {
                    Some(
                        items
                            .iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    /// `html_theme_path` — extra directories (relative to `confdir`) to
    /// search for HTML themes, in addition to Sphinx's builtin themes and
    /// any installed theme package's entry points.
    ///
    /// Defaults to an empty list (matching sphinx). The `sphinx/doc` project
    /// sets `html_theme_path = ['_themes']` to locate its local `sphinx13`
    /// theme.
    pub fn html_theme_path(&self) -> Vec<String> {
        self.get("html_theme_path")
            .and_then(|v| {
                if let ConfigVal::List(items) = v {
                    Some(
                        items
                            .iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    // ── H6c: full page context ────────────────────────────────────────────────

    /// `html_title` — mirrors `Config.html_title`'s computed default:
    /// `"{project} {release} documentation"`, preserving the separator when
    /// `release` is empty as upstream Sphinx does.
    pub fn html_title(&self) -> String {
        if let Some(ConfigVal::Str(s)) = self.get("html_title") {
            return s;
        }
        let project = self.project();
        let release = self.release();
        format!("{project} {release} documentation")
    }

    /// `html_short_title` — defaults to [`Self::html_title`].
    pub fn html_short_title(&self) -> String {
        match self.get("html_short_title") {
            Some(ConfigVal::Str(s)) if !s.is_empty() => s,
            _ => self.html_title(),
        }
    }

    /// `html_context` — extra template context merged into every page.
    pub fn html_context(&self) -> Vec<(String, ConfigVal)> {
        self.get("html_context")
            .and_then(|v| v.as_map().map(<[_]>::to_vec))
            .unwrap_or_default()
    }

    /// `html_theme_options` — theme-specific options, exposed to templates
    /// as `theme_<key>`.
    pub fn html_theme_options(&self) -> Vec<(String, ConfigVal)> {
        self.get("html_theme_options")
            .and_then(|v| v.as_map().map(<[_]>::to_vec))
            .unwrap_or_default()
    }

    /// `html_show_copyright`.
    pub fn html_show_copyright(&self) -> bool {
        self.get("html_show_copyright")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// `html_show_sphinx`.
    pub fn html_show_sphinx(&self) -> bool {
        self.get("html_show_sphinx")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// `html_show_search_summary`.
    pub fn html_show_search_summary(&self) -> bool {
        self.get("html_show_search_summary")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// `html_copy_source`.
    pub fn html_copy_source(&self) -> bool {
        self.get("html_copy_source")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// `html_show_sourcelink`.
    pub fn html_show_sourcelink(&self) -> bool {
        self.get("html_show_sourcelink")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// `html_sourcelink_suffix`.
    pub fn html_sourcelink_suffix(&self) -> String {
        self.get("html_sourcelink_suffix")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| ".txt".into())
    }

    /// `html_use_opensearch` — non-empty enables the OpenSearch `<link>`.
    pub fn html_use_opensearch(&self) -> String {
        self.get("html_use_opensearch")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default()
    }

    /// `html_baseurl`.
    pub fn html_baseurl(&self) -> String {
        self.get("html_baseurl")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default()
    }

    /// `html_logo` — path or URL to a logo image, if configured.
    pub fn html_logo(&self) -> Option<String> {
        self.get("html_logo")
            .and_then(|v| v.as_str().map(String::from))
    }

    /// `html_favicon` — path or URL to a favicon, if configured.
    pub fn html_favicon(&self) -> Option<String> {
        self.get("html_favicon")
            .and_then(|v| v.as_str().map(String::from))
    }

    /// `html_last_updated_fmt` — `Some("")` (the Sphinx default sentinel)
    /// means "show a default-formatted date"; `None` means don't show one
    /// at all; `Some(fmt)` is a custom strftime format.
    pub fn html_last_updated_fmt(&self) -> Option<String> {
        self.get("html_last_updated_fmt")
            .and_then(|v| v.as_str().map(String::from))
    }

    /// `html_sidebars` — pattern -> sidebar template list mapping. Only
    /// exact-docname and `"**"` wildcard patterns are honored (accepted
    /// deviation: no `fnmatch`-style glob matching).
    pub fn html_sidebars(&self) -> Vec<(String, Vec<String>)> {
        self.get("html_sidebars")
            .and_then(|v| v.as_map().map(<[_]>::to_vec))
            .unwrap_or_default()
            .into_iter()
            .map(|(pattern, val)| {
                let templates = val
                    .as_list()
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                (pattern, templates)
            })
            .collect()
    }

    /// `html_domain_indices`.
    pub fn html_domain_indices(&self) -> bool {
        match self.get("html_domain_indices") {
            Some(v) => v
                .as_bool()
                .or_else(|| v.as_list().map(|items| !items.is_empty()))
                .unwrap_or(true),
            None => true,
        }
    }

    /// `html_use_index` — whether to generate `genindex`.
    pub fn html_use_index(&self) -> bool {
        self.get("html_use_index")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod sphinx_config_tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn defaults_project() {
        let cfg = SphinxConfig::new_defaults();
        assert_eq!(cfg.project(), "Project name not set");
    }

    #[test]
    fn defaults_language() {
        let cfg = SphinxConfig::new_defaults();
        assert_eq!(cfg.language(), "en");
    }

    #[test]
    fn math_renderer_names_and_extension_precedence() {
        assert_eq!(MathRenderer::MathJax.name(), "mathjax");
        assert_eq!(MathRenderer::ImgMath.name(), "imgmath");
        assert_eq!(MathRenderer::Ratex.name(), "ratex");

        let mut cfg = Config::defaults();
        cfg.extensions = vec!["unknown.extension".into(), "sphinx.ext.imgmath".into()];
        assert_eq!(cfg.effective_math_renderer(), MathRenderer::ImgMath);
        cfg.extensions = vec!["sphinx.ext.mathjax".into(), "dsport.ext.ratex".into()];
        assert_eq!(cfg.effective_math_renderer(), MathRenderer::MathJax);
        cfg.extensions = vec!["unknown.extension".into()];
        assert_eq!(cfg.effective_math_renderer(), MathRenderer::MathJax);
        cfg.math_renderer = Some(MathRenderer::Ratex);
        assert_eq!(cfg.effective_math_renderer(), MathRenderer::Ratex);
    }

    #[test]
    fn math_config_reader_covers_option_shapes_and_errors() {
        Python::attach(|py| -> PyResult<()> {
            let cfg = Config::from_source(
                py,
                r#"
extensions = ['sphinx.ext.mathjax']
math_renderer = 'imgmath'
mathjax_path = 'https://example.test/mathjax.js'
mathjax_options = {'async': 'async'}
mathjax3_config = {'tex': {'inlineMath': [['$', '$']]}}
imgmath_image_format = 'svg'
imgmath_latex = 'latex-custom'
imgmath_dvipng = 'dvipng-custom'
imgmath_dvisvgm = 'dvisvgm-custom'
"#,
            )
            .unwrap();
            assert_eq!(cfg.extensions, vec!["sphinx.ext.mathjax"]);
            assert_eq!(cfg.math_renderer, Some(MathRenderer::ImgMath));
            assert_eq!(cfg.mathjax_path, "https://example.test/mathjax.js");
            assert_eq!(cfg.mathjax_options.get("async"), Some(&"async".into()));
            assert!(cfg.mathjax3_config.as_deref().is_some_and(|v| v.contains("tex")));
            assert_eq!(cfg.imgmath_image_format, "svg");
            assert_eq!(cfg.imgmath_latex, "latex-custom");
            assert_eq!(cfg.imgmath_dvipng, "dvipng-custom");
            assert_eq!(cfg.imgmath_dvisvgm, "dvisvgm-custom");

            let wrong_shapes = Config::from_source(
                py,
                "extensions = 'not-a-list'\nmath_renderer = 1\nmathjax_path = 1\nmathjax_options = []\nimgmath_image_format = 1\nimgmath_latex = 1\nimgmath_dvipng = 1\nimgmath_dvisvgm = 1\n",
            )
            .unwrap();
            assert!(wrong_shapes.extensions.is_empty());
            assert_eq!(wrong_shapes.mathjax_path, DEFAULT_MATHJAX_PATH);

            let error = Config::from_source(py, "math_renderer = 'unknown'\n").unwrap_err();
            assert!(error.to_string().contains("unknown math_renderer"));
            Ok(())
        })
        .unwrap();

        let error = Config::from_conf_py(Path::new("/no/such/conf.py")).unwrap_err();
        assert!(error.to_string().contains("cannot read"));
    }

    #[test]
    fn external_link_class_defaults_off_and_reads_conf_value() {
        let cfg = SphinxConfig::new_defaults();
        assert!(cfg.html_add_external_link_class());

        let mut raw = HashMap::new();
        raw.insert("html_add_external_link_class".into(), ConfigVal::Bool(true));
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert!(cfg.html_add_external_link_class());
    }

    #[test]
    fn defaults_root_doc() {
        let cfg = SphinxConfig::new_defaults();
        assert_eq!(cfg.root_doc(), "index");
    }

    #[test]
    fn html_title_preserves_empty_release_separator() {
        let mut raw = HashMap::new();
        raw.insert("project".into(), ConfigVal::Str("Docs".into()));
        raw.insert("release".into(), ConfigVal::Str(String::new()));
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert_eq!(cfg.html_title(), "Docs  documentation");
    }

    #[test]
    fn defaults_extensions_empty() {
        let cfg = SphinxConfig::new_defaults();
        assert!(cfg.extensions().is_empty());
    }

    #[test]
    fn raw_config_overrides_defaults() {
        let mut raw = HashMap::new();
        raw.insert("project".into(), ConfigVal::Str("My Docs".into()));
        raw.insert("language".into(), ConfigVal::Str("de".into()));
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert_eq!(cfg.project(), "My Docs");
        assert_eq!(cfg.language(), "de");
    }

    #[test]
    fn command_line_override_str() {
        let mut overrides = HashMap::new();
        overrides.insert("project".into(), "CLI Project".into());
        let cfg = SphinxConfig::new(HashMap::new(), overrides);
        assert_eq!(cfg.project(), "CLI Project");
    }

    #[test]
    fn command_line_override_bool() {
        let mut overrides = HashMap::new();
        overrides.insert("nitpicky".into(), "1".into());
        let cfg = SphinxConfig::new(HashMap::new(), overrides);
        assert!(cfg.nitpicky());
    }

    #[test]
    fn command_line_override_bool_zero() {
        let mut overrides = HashMap::new();
        overrides.insert("smartquotes".into(), "0".into());
        let cfg = SphinxConfig::new(HashMap::new(), overrides);
        assert!(!cfg.smartquotes());
    }

    #[test]
    fn command_line_override_list_csv() {
        let mut overrides = HashMap::new();
        overrides.insert("modindex_common_prefix".into(), "path1,path2".into());
        let cfg = SphinxConfig::new(HashMap::new(), overrides);
        let val = cfg.get("modindex_common_prefix").unwrap();
        if let ConfigVal::List(items) = val {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0].as_str().unwrap(), "path1");
        } else {
            panic!("expected List");
        }
    }

    #[test]
    fn set_updates_raw_config() {
        let mut cfg = SphinxConfig::new_defaults();
        cfg.set("project", ConfigVal::Str("Updated".into()));
        assert_eq!(cfg.project(), "Updated");
    }

    #[test]
    fn master_doc_root_doc_alias() {
        let mut raw = HashMap::new();
        raw.insert("master_doc".into(), ConfigVal::Str("contents".into()));
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert_eq!(cfg.root_doc(), "contents");
    }

    #[test]
    fn set_master_doc_syncs_root_doc() {
        let mut cfg = SphinxConfig::new_defaults();
        cfg.set("master_doc", ConfigVal::Str("contents".into()));
        assert_eq!(cfg.root_doc(), "contents");
    }

    #[test]
    fn contains_registered_option() {
        let cfg = SphinxConfig::new_defaults();
        assert!(cfg.contains("project"));
        assert!(cfg.contains("language"));
        assert!(!cfg.contains("nonexistent_key"));
    }

    #[test]
    fn add_extension_option() {
        let mut cfg = SphinxConfig::new_defaults();
        cfg.add(
            "myext_option",
            ConfigVal::Bool(false),
            RebuildKind::Env,
            "My option",
        )
        .unwrap();
        assert!(cfg.contains("myext_option"));
        assert_eq!(cfg.get("myext_option"), Some(ConfigVal::Bool(false)));
    }

    #[test]
    fn add_duplicate_option_errors() {
        let mut cfg = SphinxConfig::new_defaults();
        let err = cfg
            .add(
                "project",
                ConfigVal::Str(String::new()),
                RebuildKind::None,
                "",
            )
            .unwrap_err();
        assert!(err.contains("already present"), "err: {err}");
    }

    #[test]
    fn iter_yields_all_options() {
        let cfg = SphinxConfig::new_defaults();
        let names: Vec<_> = cfg.iter().map(|cv| cv.name.clone()).collect();
        assert!(names.contains(&"project".to_string()));
        assert!(names.contains(&"language".to_string()));
            assert!(names.contains(&"source_backend".to_string()));
            assert!(names.contains(&"source_lsp_servers".to_string()));
            assert!(names.contains(&"source_lsp_timeout".to_string()));
        assert!(names.contains(&"extensions".to_string()));
    }

    #[test]
    fn filter_by_rebuild_env() {
        let cfg = SphinxConfig::new_defaults();
        let env_names: Vec<_> = cfg.filter(&RebuildKind::Env).map(|cv| cv.name).collect();
        assert!(env_names.contains(&"project".to_string()));
        // "needs_sphinx" has rebuild=None, should not appear
        assert!(!env_names.contains(&"needs_sphinx".to_string()));
    }

    #[test]
    fn source_suffix_defaults_to_rst() {
        let cfg = SphinxConfig::new_defaults();
        let sfx = cfg.source_suffix();
        assert_eq!(
            sfx.get(".rst").map(String::as_str),
            Some("restructuredtext")
        );
    }

    #[test]
    fn config_val_coerce_list() {
        let default = ConfigVal::List(vec![]);
        let result = ConfigVal::coerce_override(&default, "a,b,c").unwrap();
        if let ConfigVal::List(items) = result {
            assert_eq!(items.len(), 3);
        } else {
            panic!("expected List");
        }
    }

    #[test]
    fn config_val_coerce_int() {
        let default = ConfigVal::Int(0);
        assert_eq!(
            ConfigVal::coerce_override(&default, "42"),
            Ok(ConfigVal::Int(42))
        );
        assert!(ConfigVal::coerce_override(&default, "abc").is_err());
    }

    #[test]
    fn config_val_coerce_bool_and_list_edges() {
        assert_eq!(
            ConfigVal::coerce_override(&ConfigVal::Bool(false), "0"),
            Ok(ConfigVal::Bool(false))
        );
        assert_eq!(
            ConfigVal::coerce_override(&ConfigVal::Bool(false), "1"),
            Ok(ConfigVal::Bool(true))
        );
        assert!(ConfigVal::coerce_override(&ConfigVal::Bool(false), "yes").is_err());
        assert_eq!(
            ConfigVal::coerce_override(&ConfigVal::List(vec![]), " a, b ,, c "),
            Ok(ConfigVal::List(vec![
                ConfigVal::Str("a".into()),
                ConfigVal::Str("b".into()),
                ConfigVal::Str(String::new()),
                ConfigVal::Str("c".into()),
            ]))
        );
    }

    #[test]
    fn config_val_accessors_and_display_cover_nested_values() {
        let map = ConfigVal::Map(vec![("key".into(), ConfigVal::Int(1))]);
        let list = ConfigVal::List(vec![ConfigVal::Str("x".into())]);
        assert_eq!(ConfigVal::Str("x".into()).as_str(), Some("x"));
        assert_eq!(ConfigVal::Bool(true).as_bool(), Some(true));
        assert_eq!(ConfigVal::Int(7).as_int(), Some(7));
        assert_eq!(list.as_list().unwrap().len(), 1);
        assert_eq!(map.as_map().unwrap().len(), 1);
        assert_eq!(ConfigVal::Float(1.5).display(), "1.5");
        assert_eq!(list.display(), "[x]");
        assert_eq!(map.display(), "{\"key\": 1}");
        assert_eq!(ConfigVal::Null.as_str(), None);
        assert_eq!(ConfigVal::Null.as_bool(), None);
        assert_eq!(ConfigVal::Null.as_int(), None);
        assert!(ConfigVal::Null.as_list().is_none());
        assert!(ConfigVal::Null.as_map().is_none());
    }

    #[test]
    fn python_config_value_conversion_covers_supported_shapes() {
        Python::attach(|py| -> PyResult<()> {
            let named = py.eval(
                &CString::new(
                    "type('Link', (), {'title': 'Docs', 'url': 'https://example.test'})()",
                )
                .unwrap(),
                None,
                None,
            )?;
            assert!(matches!(py_to_configval(&named), Some(ConfigVal::Map(_))));

            for (source, expected) in [
                ("None", ConfigVal::Null),
                ("True", ConfigVal::Bool(true)),
                ("7", ConfigVal::Int(7)),
                ("1.5", ConfigVal::Float(1.5)),
                ("'text'", ConfigVal::Str("text".into())),
            ] {
                let value = py.eval(&CString::new(source).unwrap(), None, None)?;
                assert_eq!(py_to_configval(&value), Some(expected));
            }

            let list = py.eval(&CString::new("[1, 'two']").unwrap(), None, None)?;
            assert!(matches!(py_to_configval(&list), Some(ConfigVal::List(_))));
            let tuple = py.eval(&CString::new("(1, 'two')").unwrap(), None, None)?;
            assert!(matches!(py_to_configval(&tuple), Some(ConfigVal::List(_))));
            let dict = py.eval(&CString::new("{'key': 1}").unwrap(), None, None)?;
            assert!(matches!(py_to_configval(&dict), Some(ConfigVal::Map(_))));

            let invalid = py.eval(&CString::new("[object()]").unwrap(), None, None)?;
            assert!(py_to_configval(&invalid).is_none());
            let invalid_key = py.eval(&CString::new("{1: 'value'}").unwrap(), None, None)?;
            assert!(py_to_configval(&invalid_key).is_none());
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn config_val_display_bool() {
        assert_eq!(ConfigVal::Bool(true).display(), "True");
        assert_eq!(ConfigVal::Bool(false).display(), "False");
    }

    #[test]
    fn source_suffix_accepts_string_list_and_map_values() {
        let mut raw = HashMap::new();
        raw.insert("source_suffix".into(), ConfigVal::Str(".md".into()));
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert_eq!(
            cfg.source_suffix().get(".md").map(String::as_str),
            Some("restructuredtext")
        );

        let mut raw = HashMap::new();
        raw.insert(
            "source_suffix".into(),
            ConfigVal::List(vec![ConfigVal::Str(".txt".into())]),
        );
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert_eq!(
            cfg.source_suffix().get(".txt").map(String::as_str),
            Some("restructuredtext")
        );

        let mut raw = HashMap::new();
        raw.insert(
            "source_suffix".into(),
            ConfigVal::Map(vec![(".md".into(), ConfigVal::Str("myst".into()))]),
        );
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert_eq!(
            cfg.source_suffix().get(".md").map(String::as_str),
            Some("myst")
        );
    }

    #[test]
    fn typed_accessors_read_configured_values_and_filter_shapes() {
        let mut raw = HashMap::new();
        raw.insert("author".into(), ConfigVal::Str("A. Author".into()));
        raw.insert("version".into(), ConfigVal::Str("1.2".into()));
        raw.insert("release".into(), ConfigVal::Str("1.2.3".into()));
        raw.insert(
            "extensions".into(),
            ConfigVal::List(vec![ConfigVal::Str("ext.demo".into()), ConfigVal::Int(1)]),
        );
        raw.insert(
            "needs_extensions".into(),
            ConfigVal::Map(vec![
                ("ext.demo".into(), ConfigVal::Str("1.0".into())),
                ("ignored".into(), ConfigVal::Int(1)),
            ]),
        );
        raw.insert(
            "exclude_patterns".into(),
            ConfigVal::List(vec![ConfigVal::Str("draft/**".into()), ConfigVal::Int(1)]),
        );
        raw.insert(
            "include_patterns".into(),
            ConfigVal::List(vec![ConfigVal::Str("docs/**".into()), ConfigVal::Int(1)]),
        );
        raw.insert("highlight_language".into(), ConfigVal::Str("rust".into()));
        raw.insert("html_add_external_link_class".into(), ConfigVal::Bool(false));
        raw.insert("numfig".into(), ConfigVal::Bool(true));
        raw.insert("nitpicky".into(), ConfigVal::Bool(true));
        raw.insert("smartquotes".into(), ConfigVal::Bool(false));
        raw.insert("rst_prolog".into(), ConfigVal::Str(".. prolog::".into()));
        raw.insert("rst_epilog".into(), ConfigVal::Str(".. epilog::".into()));
        raw.insert("html_theme".into(), ConfigVal::Str("basic".into()));
        raw.insert(
            "html_static_path".into(),
            ConfigVal::List(vec![ConfigVal::Str("_static".into()), ConfigVal::Int(1)]),
        );
        raw.insert(
            "html_theme_path".into(),
            ConfigVal::List(vec![ConfigVal::Str("_themes".into())]),
        );
        raw.insert(
            "html_css_files".into(),
            ConfigVal::List(vec![ConfigVal::Str("custom.css".into()), ConfigVal::Int(1)]),
        );
        raw.insert("html_title".into(), ConfigVal::Str("Docs".into()));
        raw.insert("html_short_title".into(), ConfigVal::Str("D".into()));
        raw.insert(
            "html_context".into(),
            ConfigVal::Map(vec![("owner".into(), ConfigVal::Str("team".into()))]),
        );
        raw.insert(
            "html_theme_options".into(),
            ConfigVal::Map(vec![("navigation_depth".into(), ConfigVal::Int(3))]),
        );
        for name in [
            "html_show_copyright",
            "html_show_sphinx",
            "html_show_search_summary",
            "html_copy_source",
            "html_show_sourcelink",
        ] {
            raw.insert(name.into(), ConfigVal::Bool(false));
        }
        raw.insert("html_sourcelink_suffix".into(), ConfigVal::Str(".rst".into()));
        raw.insert(
            "html_use_opensearch".into(),
            ConfigVal::Str("https://docs.example.test".into()),
        );
        raw.insert("html_baseurl".into(), ConfigVal::Str("/docs".into()));
        raw.insert("html_logo".into(), ConfigVal::Str("logo.svg".into()));
        raw.insert("html_favicon".into(), ConfigVal::Str("favicon.ico".into()));
        raw.insert(
            "html_last_updated_fmt".into(),
            ConfigVal::Str("%Y-%m-%d".into()),
        );
        raw.insert(
            "html_sidebars".into(),
            ConfigVal::Map(vec![
                (
                    "index".into(),
                    ConfigVal::List(vec![ConfigVal::Str("localtoc.html".into()), ConfigVal::Int(1)]),
                ),
                ("guide/*".into(), ConfigVal::Str("not-a-list".into())),
            ]),
        );
        raw.insert("html_domain_indices".into(), ConfigVal::Bool(false));
        raw.insert("html_use_index".into(), ConfigVal::Bool(false));
        let cfg = SphinxConfig::new(raw, HashMap::new());

        assert_eq!(cfg.author(), "A. Author");
        assert_eq!(cfg.version(), "1.2");
        assert_eq!(cfg.release(), "1.2.3");
        assert_eq!(cfg.extensions(), vec!["ext.demo"]);
        assert_eq!(cfg.needs_extensions().get("ext.demo"), Some(&"1.0".into()));
        assert_eq!(cfg.exclude_patterns(), vec!["draft/**"]);
        assert_eq!(cfg.include_patterns(), vec!["docs/**"]);
        assert_eq!(cfg.highlight_language(), "rust");
        assert!(!cfg.html_add_external_link_class());
        assert!(cfg.numfig());
        assert!(cfg.nitpicky());
        assert!(!cfg.smartquotes());
        assert_eq!(cfg.rst_prolog().as_deref(), Some(".. prolog::"));
        assert_eq!(cfg.rst_epilog().as_deref(), Some(".. epilog::"));
        assert_eq!(cfg.html_theme(), "basic");
        assert_eq!(cfg.html_static_path(), vec!["_static"]);
        assert_eq!(cfg.html_theme_path(), vec!["_themes"]);
        assert_eq!(cfg.html_css_files(), vec!["custom.css"]);
        assert_eq!(cfg.html_title(), "Docs");
        assert_eq!(cfg.html_short_title(), "D");
        assert_eq!(cfg.html_context().len(), 1);
        assert_eq!(cfg.html_theme_options().len(), 1);
        assert!(!cfg.html_show_copyright());
        assert!(!cfg.html_show_sphinx());
        assert!(!cfg.html_show_search_summary());
        assert!(!cfg.html_copy_source());
        assert!(!cfg.html_show_sourcelink());
        assert_eq!(cfg.html_sourcelink_suffix(), ".rst");
        assert_eq!(cfg.html_use_opensearch(), "https://docs.example.test");
        assert_eq!(cfg.html_baseurl(), "/docs");
        assert_eq!(cfg.html_logo().as_deref(), Some("logo.svg"));
        assert_eq!(cfg.html_favicon().as_deref(), Some("favicon.ico"));
        assert_eq!(cfg.html_last_updated_fmt().as_deref(), Some("%Y-%m-%d"));
        assert_eq!(cfg.html_sidebars()[0].1, vec!["localtoc.html"]);
        assert!(cfg.html_sidebars()[1].1.is_empty());
        assert!(!cfg.html_domain_indices());
        assert!(!cfg.html_use_index());
    }

    #[test]
    fn typed_accessors_use_defaults_for_wrong_shapes() {
        let mut raw = HashMap::new();
        for name in [
            "author",
            "version",
            "release",
            "highlight_language",
            "rst_prolog",
            "rst_epilog",
            "html_theme",
            "html_sourcelink_suffix",
            "html_use_opensearch",
            "html_baseurl",
            "html_logo",
            "html_favicon",
            "html_last_updated_fmt",
        ] {
            raw.insert(name.into(), ConfigVal::Int(1));
        }
        for name in [
            "extensions",
            "needs_extensions",
            "exclude_patterns",
            "include_patterns",
            "html_static_path",
            "html_theme_path",
            "html_context",
            "html_theme_options",
            "html_sidebars",
        ] {
            raw.insert(name.into(), ConfigVal::Bool(true));
        }
        for name in [
            "html_add_external_link_class",
            "numfig",
            "nitpicky",
            "smartquotes",
            "html_show_copyright",
            "html_show_sphinx",
            "html_show_search_summary",
            "html_copy_source",
            "html_show_sourcelink",
            "html_domain_indices",
            "html_use_index",
        ] {
            raw.insert(name.into(), ConfigVal::Str("wrong".into()));
        }
        let cfg = SphinxConfig::new(raw, HashMap::new());

        assert_eq!(cfg.author(), "Author name not set");
        assert_eq!(cfg.version(), "");
        assert_eq!(cfg.release(), "");
        assert_eq!(cfg.extensions(), Vec::<String>::new());
        assert_eq!(cfg.needs_extensions().len(), 0);
        assert_eq!(cfg.exclude_patterns(), Vec::<String>::new());
        assert_eq!(cfg.include_patterns(), vec!["**"]);
        assert_eq!(cfg.highlight_language(), "default");
        assert!(!cfg.html_add_external_link_class());
        assert!(!cfg.numfig());
        assert!(!cfg.nitpicky());
        assert!(cfg.smartquotes());
        assert_eq!(cfg.rst_prolog(), None);
        assert_eq!(cfg.rst_epilog(), None);
        assert_eq!(cfg.html_theme(), "alabaster");
        assert!(cfg.html_static_path().is_empty());
        assert!(cfg.html_theme_path().is_empty());
        assert_eq!(cfg.html_title(), "Project name not set  documentation");
        assert_eq!(cfg.html_short_title(), "Project name not set  documentation");
        assert!(cfg.html_context().is_empty());
        assert!(cfg.html_theme_options().is_empty());
        assert!(cfg.html_show_copyright());
        assert!(cfg.html_show_sphinx());
        assert!(cfg.html_show_search_summary());
        assert!(cfg.html_copy_source());
        assert!(cfg.html_show_sourcelink());
        assert_eq!(cfg.html_sourcelink_suffix(), ".txt");
        assert_eq!(cfg.html_use_opensearch(), "");
        assert_eq!(cfg.html_baseurl(), "");
        assert_eq!(cfg.html_logo(), None);
        assert_eq!(cfg.html_favicon(), None);
        assert_eq!(cfg.html_last_updated_fmt(), None);
        assert!(cfg.html_sidebars().is_empty());
        assert!(cfg.html_domain_indices());
        assert!(cfg.html_use_index());
    }

    #[test]
    fn linkcheck_and_intersphinx_accessors_cover_mapping_and_numeric_shapes() {
        let mut raw = HashMap::new();
        raw.insert(
            "intersphinx_mapping".into(),
            ConfigVal::Map(vec![
                (
                    "docs".into(),
                    ConfigVal::List(vec![
                        ConfigVal::Str("https://docs.example.test".into()),
                        ConfigVal::Str("objects.inv".into()),
                    ]),
                ),
                (
                    "default-inv".into(),
                    ConfigVal::List(vec![ConfigVal::Str("https://other.example.test".into())]),
                ),
                ("not-a-list".into(), ConfigVal::Str("skip".into())),
                ("empty-list".into(), ConfigVal::List(vec![])),
                (
                    "bad-url".into(),
                    ConfigVal::List(vec![ConfigVal::Int(1), ConfigVal::Str("skip".into())]),
                ),
            ]),
        );
        raw.insert(
            "linkcheck_ignore".into(),
            ConfigVal::List(vec![ConfigVal::Str("^https://skip".into()), ConfigVal::Int(1)]),
        );
        raw.insert(
            "linkcheck_allowed_redirects".into(),
            ConfigVal::Map(vec![
                ("^https://old".into(), ConfigVal::Str("^https://new".into())),
                ("ignored".into(), ConfigVal::Int(1)),
            ]),
        );
        raw.insert("linkcheck_anchors".into(), ConfigVal::Bool(false));
        raw.insert(
            "linkcheck_anchors_ignore".into(),
            ConfigVal::List(vec![ConfigVal::Str("^generated".into()), ConfigVal::Int(1)]),
        );
        raw.insert("linkcheck_timeout".into(), ConfigVal::Float(2.5));
        raw.insert("linkcheck_retries".into(), ConfigVal::Int(3));
        raw.insert("linkcheck_rate_limit_timeout".into(), ConfigVal::Int(12));
        let cfg = SphinxConfig::new(raw, HashMap::new());

        assert_eq!(
            cfg.intersphinx_mapping(),
            vec![
                (
                    "docs".into(),
                    "https://docs.example.test".into(),
                    Some("objects.inv".into()),
                ),
                (
                    "default-inv".into(),
                    "https://other.example.test".into(),
                    None,
                ),
            ]
        );
        assert_eq!(cfg.linkcheck_ignore(), vec!["^https://skip"]);
        assert_eq!(
            cfg.linkcheck_allowed_redirects(),
            vec![("^https://old".into(), "^https://new".into())]
        );
        assert!(!cfg.linkcheck_anchors());
        assert_eq!(cfg.linkcheck_anchors_ignore(), vec!["^generated"]);
        assert_eq!(cfg.linkcheck_timeout(), 2);
        assert_eq!(cfg.linkcheck_retries(), 3);
        assert_eq!(cfg.linkcheck_rate_limit_timeout(), 12.0);

        let mut raw = HashMap::new();
        raw.insert("linkcheck_timeout".into(), ConfigVal::Int(-1));
        raw.insert("linkcheck_retries".into(), ConfigVal::Int(-1));
        raw.insert("linkcheck_rate_limit_timeout".into(), ConfigVal::Float(-1.0));
        raw.insert("html_domain_indices".into(), ConfigVal::List(vec![]));
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert_eq!(cfg.linkcheck_timeout(), 30);
        assert_eq!(cfg.linkcheck_retries(), 1);
        assert_eq!(cfg.linkcheck_rate_limit_timeout(), 300.0);
        assert!(!cfg.html_domain_indices());

        let mut raw = HashMap::new();
        raw.insert(
            "html_domain_indices".into(),
            ConfigVal::List(vec![ConfigVal::Str("py".into())]),
        );
        let cfg = SphinxConfig::new(raw, HashMap::new());
        assert!(cfg.html_domain_indices());
    }

    #[test]
    fn config_resolution_covers_aliases_invalid_overrides_and_wrong_shapes() {
        let mut raw = HashMap::new();
        raw.insert("root_doc".into(), ConfigVal::Str("contents".into()));
        raw.insert("copyright".into(), ConfigVal::Str("2026, Docs".into()));
        raw.insert("linkcheck_timeout".into(), ConfigVal::Int(7));
        raw.insert("source_suffix".into(), ConfigVal::Bool(true));
        raw.insert("linkcheck_allowed_redirects".into(), ConfigVal::Bool(true));
        raw.insert("linkcheck_anchors_ignore".into(), ConfigVal::Bool(true));
        let mut overrides = HashMap::new();
        overrides.insert("linkcheck_timeout".into(), "not-a-number".into());
        let cfg = SphinxConfig::new(raw, overrides);

        assert_eq!(cfg.get("master_doc"), Some(ConfigVal::Str("contents".into())));
        assert_eq!(
            cfg.get("project_copyright"),
            Some(ConfigVal::Str("2026, Docs".into()))
        );
        assert_eq!(cfg.linkcheck_timeout(), 7);
        assert_eq!(cfg.source_suffix().get(".rst").map(String::as_str), Some("restructuredtext"));
        assert!(cfg.linkcheck_allowed_redirects().is_empty());
        assert_eq!(cfg.linkcheck_anchors_ignore(), vec!["^!"]);

        let mut cfg = SphinxConfig::new_defaults();
        cfg.set("copyright", ConfigVal::Str("2027, Docs".into()));
        assert_eq!(cfg.get("project_copyright"), Some(ConfigVal::Str("2027, Docs".into())));
        cfg.set("project_copyright", ConfigVal::Str("2028, Docs".into()));
        assert_eq!(
            cfg.raw_config().get("copyright"),
            Some(&ConfigVal::Str("2028, Docs".into()))
        );
    }

    #[test]
    fn config_val_display_null() {
        assert_eq!(ConfigVal::Null.display(), "None");
    }

    #[test]
    fn extensions_in_raw_config() {
        let mut raw = HashMap::new();
        raw.insert(
            "extensions".into(),
            ConfigVal::List(vec![
                ConfigVal::Str("sphinx.ext.autodoc".into()),
                ConfigVal::Str("sphinx.ext.mathjax".into()),
            ]),
        );
        let cfg = SphinxConfig::new(raw, HashMap::new());
        let exts = cfg.extensions();
        assert!(exts.contains(&"sphinx.ext.autodoc".to_string()));
        assert!(exts.contains(&"sphinx.ext.mathjax".to_string()));
    }
}

#[cfg(test)]
mod raw_config_from_conf_py_tests {
    use super::*;
    use tempfile::TempDir;

    fn write_conf(body: &str) -> (TempDir, std::path::PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("conf.py");
        std::fs::write(&path, body).unwrap();
        (dir, path)
    }

    #[test]
    fn reads_extensions_and_scalar_options() {
        let (_dir, path) = write_conf(
            r#"
extensions = ["sphinx.ext.autodoc", "sphinx.ext.mathjax"]
project = "My Project"
author = "Author Name"
copyright = "2026, Author Name"
version = "1.0"
release = "1.0.0"
language = "en"
master_doc = "index"
root_doc = "index"
source_encoding = "utf-8-sig"
html_theme = "alabaster"
html_title = "My Docs"
html_short_title = "Docs"
"#,
        );
        let raw = raw_config_from_conf_py(&path).unwrap();
        assert_eq!(
            raw["extensions"],
            ConfigVal::List(vec![
                ConfigVal::Str("sphinx.ext.autodoc".into()),
                ConfigVal::Str("sphinx.ext.mathjax".into()),
            ])
        );
        for (key, expected) in [
            ("project", "My Project"),
            ("author", "Author Name"),
            ("copyright", "2026, Author Name"),
            ("version", "1.0"),
            ("release", "1.0.0"),
            ("language", "en"),
            ("master_doc", "index"),
            ("root_doc", "index"),
            ("source_encoding", "utf-8-sig"),
            ("html_theme", "alabaster"),
            ("html_title", "My Docs"),
            ("html_short_title", "Docs"),
        ] {
            assert_eq!(raw[key].as_str(), Some(expected), "key {key}");
        }
    }

    #[test]
    fn reads_list_of_strings_options() {
        let (_dir, path) = write_conf(
            r#"
html_static_path = ["_static", "_more_static"]
html_extra_path = ["_extra"]
templates_path = ["_templates"]
html_theme_path = ["_themes"]
html_css_files = ["custom.css"]
"#,
        );
        let raw = raw_config_from_conf_py(&path).unwrap();
        assert_eq!(
            raw["html_static_path"],
            ConfigVal::List(vec![
                ConfigVal::Str("_static".into()),
                ConfigVal::Str("_more_static".into()),
            ])
        );
        assert_eq!(
            raw["html_extra_path"],
            ConfigVal::List(vec![ConfigVal::Str("_extra".into())])
        );
        assert_eq!(
            raw["templates_path"],
            ConfigVal::List(vec![ConfigVal::Str("_templates".into())])
        );
        assert_eq!(
            raw["html_theme_path"],
            ConfigVal::List(vec![ConfigVal::Str("_themes".into())])
        );
        assert_eq!(
            raw["html_css_files"],
            ConfigVal::List(vec![ConfigVal::Str("custom.css".into())])
        );
    }

    #[test]
    fn reads_latex_documents_and_man_pages_as_nested_lists() {
        let (_dir, path) = write_conf(
            r#"
latex_documents = [("index", "proj.tex", "Project", "Author", "manual")]
man_pages = [("index", "proj", "Project docs", ["Author"], 1)]
"#,
        );
        let raw = raw_config_from_conf_py(&path).unwrap();
        let ConfigVal::List(docs) = &raw["latex_documents"] else {
            panic!("expected List");
        };
        assert_eq!(docs.len(), 1);
        let ConfigVal::List(entry) = &docs[0] else {
            panic!("expected nested List for tuple");
        };
        assert_eq!(entry[0].as_str(), Some("index"));
        assert_eq!(entry[4].as_str(), Some("manual"));

        let ConfigVal::List(pages) = &raw["man_pages"] else {
            panic!("expected List");
        };
        let ConfigVal::List(page_entry) = &pages[0] else {
            panic!("expected nested List for tuple");
        };
        assert_eq!(page_entry[1].as_str(), Some("proj"));
    }

    #[test]
    fn reads_intersphinx_mapping_and_skips_invalid_entries() {
        let (_dir, path) = write_conf(
            r#"
intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
    "numpy": ("https://numpy.org/doc/stable", "objects.inv"),
    123: ("https://bad-key.example", None),
}
"#,
        );
        let raw = raw_config_from_conf_py(&path).unwrap();
        let ConfigVal::Map(mapping) = &raw["intersphinx_mapping"] else {
            panic!("expected Map");
        };
        // Non-string keys are skipped; entries are sorted by name.
        assert_eq!(mapping.len(), 2);
        assert_eq!(mapping[0].0, "numpy");
        assert_eq!(mapping[1].0, "python");
        let ConfigVal::List(python_entry) = &mapping[1].1 else {
            panic!("expected List [url, inv]");
        };
        assert_eq!(python_entry[0].as_str(), Some("https://docs.python.org/3"));
        assert_eq!(python_entry[1], ConfigVal::Null);
    }

    #[test]
    fn source_suffix_dict_form() {
        let (_dir, path) = write_conf(r#"source_suffix = {".rst": "restructuredtext", ".md": "markdown"}"#);
        let raw = raw_config_from_conf_py(&path).unwrap();
        let ConfigVal::Map(pairs) = &raw["source_suffix"] else {
            panic!("expected Map");
        };
        assert!(pairs
            .iter()
            .any(|(k, v)| k == ".rst" && v.as_str() == Some("restructuredtext")));
    }

    #[test]
    fn source_suffix_string_form() {
        let (_dir, path) = write_conf(r#"source_suffix = ".rst""#);
        let raw = raw_config_from_conf_py(&path).unwrap();
        assert_eq!(raw["source_suffix"].as_str(), Some(".rst"));
    }

    #[test]
    fn source_suffix_list_form() {
        let (_dir, path) = write_conf(r#"source_suffix = [".rst", ".md"]"#);
        let raw = raw_config_from_conf_py(&path).unwrap();
        assert_eq!(
            raw["source_suffix"],
            ConfigVal::List(vec![ConfigVal::Str(".rst".into()), ConfigVal::Str(".md".into())])
        );
    }

    #[test]
    fn reads_needs_extensions() {
        let (_dir, path) = write_conf(r#"needs_extensions = {"sphinx.ext.autodoc": "1.0"}"#);
        let raw = raw_config_from_conf_py(&path).unwrap();
        let ConfigVal::Map(pairs) = &raw["needs_extensions"] else {
            panic!("expected Map");
        };
        assert_eq!(pairs[0], ("sphinx.ext.autodoc".to_string(), ConfigVal::Str("1.0".into())));
    }

    #[test]
    fn generic_fallback_keeps_data_but_skips_modules_and_callables() {
        let (_dir, path) = write_conf(
            r#"
import os
autosummary_generate = False
nested = {"a": [1, 2, {"b": True}]}
def my_setup(app):
    pass
"#,
        );
        let raw = raw_config_from_conf_py(&path).unwrap();
        assert_eq!(raw["autosummary_generate"], ConfigVal::Bool(false));
        assert_eq!(
            raw["nested"],
            ConfigVal::Map(vec![(
                "a".into(),
                ConfigVal::List(vec![
                    ConfigVal::Int(1),
                    ConfigVal::Int(2),
                    ConfigVal::Map(vec![("b".into(), ConfigVal::Bool(true))]),
                ])
            )])
        );
        assert!(!raw.contains_key("os"));
        assert!(!raw.contains_key("my_setup"));
        assert!(!raw.contains_key("__file__"));
        assert!(!raw.contains_key("__builtins__"));
    }

    #[test]
    fn generic_fallback_preserves_named_tuple_title_url_shape() {
        let (_dir, path) = write_conf(
            r#"
class Link:
    def __init__(self, title, url):
        self.title = title
        self.url = url

html_context = {"links": [Link("Home", "https://example.org")]}
"#,
        );
        let raw = raw_config_from_conf_py(&path).unwrap();
        let ConfigVal::Map(ctx) = &raw["html_context"] else {
            panic!("expected Map");
        };
        let ConfigVal::List(links) = &ctx[0].1 else {
            panic!("expected List");
        };
        let ConfigVal::Map(link) = &links[0] else {
            panic!("expected Map for Link named-tuple-like object");
        };
        assert_eq!(link[0], ("title".to_string(), ConfigVal::Str("Home".into())));
        assert_eq!(
            link[1],
            ("url".to_string(), ConfigVal::Str("https://example.org".into()))
        );
    }

    #[test]
    fn errors_on_missing_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("does_not_exist.py");
        let err = raw_config_from_conf_py(&path).unwrap_err();
        Python::attach(|py| {
            assert!(err.value(py).to_string().contains("cannot read"));
        });
    }

    #[test]
    fn errors_on_python_exception() {
        let (_dir, path) = write_conf("raise ValueError('boom')");
        let err = raw_config_from_conf_py(&path).unwrap_err();
        Python::attach(|py| {
            assert!(err.value(py).to_string().contains("conf.py failed"));
        });
    }

    #[test]
    fn conf_py_setup_returns_callable_when_defined() {
        let (_dir, path) = write_conf(
            r#"
def setup(app):
    app.custom_setup_called = True
"#,
        );
        let setup = conf_py_setup(&path).unwrap();
        assert!(setup.is_some());
        Python::attach(|py| {
            assert!(setup.unwrap().bind(py).hasattr("__call__").unwrap());
        });
    }

    #[test]
    fn conf_py_setup_returns_none_when_absent() {
        let (_dir, path) = write_conf("project = 'No Setup Here'");
        assert!(conf_py_setup(&path).unwrap().is_none());
    }

    #[test]
    fn conf_py_setup_returns_none_when_setup_is_not_callable() {
        let (_dir, path) = write_conf("setup = 42");
        assert!(conf_py_setup(&path).unwrap().is_none());
    }

    #[test]
    fn rebuild_kind_from_str_covers_every_variant() {
        use std::str::FromStr;
        assert_eq!(RebuildKind::from_str("env").unwrap(), RebuildKind::Env);
        assert_eq!(RebuildKind::from_str("epub").unwrap(), RebuildKind::Epub);
        assert_eq!(RebuildKind::from_str("gettext").unwrap(), RebuildKind::Gettext);
        assert_eq!(RebuildKind::from_str("html").unwrap(), RebuildKind::Html);
        assert_eq!(RebuildKind::from_str("unknown").unwrap(), RebuildKind::None);
        assert_eq!(RebuildKind::from_str("").unwrap(), RebuildKind::None);
    }

    #[test]
    fn rebuild_kind_as_str_round_trips() {
        for kind in [
            RebuildKind::None,
            RebuildKind::Env,
            RebuildKind::Epub,
            RebuildKind::Gettext,
            RebuildKind::Html,
        ] {
            use std::str::FromStr;
            assert_eq!(RebuildKind::from_str(kind.as_str()).unwrap(), kind);
        }
    }

    #[test]
    fn raw_config_skips_wrong_shapes_and_unsupported_nested_values() {
        let (_dir, path) = write_conf(
            r#"
extensions = "not-a-list"
project = object()
html_static_path = "not-a-list"
latex_documents = [object()]
man_pages = [object()]
intersphinx_mapping = {"bad": "not-a-tuple", "short": ("https://example.test",)}
source_suffix = 42
needs_extensions = []
fallback_list = [object()]
fallback_dict = {1: "bad-key"}
"#,
        );
        let raw = raw_config_from_conf_py(&path).unwrap();
        assert_eq!(raw.get("extensions"), Some(&ConfigVal::Str("not-a-list".into())));
        assert!(!raw.contains_key("project"));
        assert_eq!(
            raw.get("html_static_path"),
            Some(&ConfigVal::Str("not-a-list".into()))
        );
        assert!(!raw.contains_key("latex_documents"));
        assert!(!raw.contains_key("man_pages"));
        assert!(matches!(raw.get("intersphinx_mapping"), Some(ConfigVal::Map(_))));
        assert!(matches!(raw.get("source_suffix"), Some(ConfigVal::Int(42))));
        assert_eq!(raw.get("needs_extensions"), Some(&ConfigVal::List(vec![])));
        assert!(!raw.contains_key("fallback_list"));
        assert!(!raw.contains_key("fallback_dict"));
    }

    #[test]
    fn read_conf_py_wrapper_returns_math_configuration_dict() {
        let (_dir, path) = write_conf(
            "extensions = ['sphinx.ext.imgmath']\nmathjax_path = 'custom.js'\nimgmath_image_format = 'svg'\n",
        );
        Python::attach(|py| -> PyResult<()> {
            let result = py_read_conf_py(py, path.to_str().unwrap())?;
            let dict = result.bind(py);
            assert_eq!(dict.get_item("effective_math_renderer")?.unwrap().extract::<String>()?, "imgmath");
            assert_eq!(dict.get_item("mathjax_path")?.unwrap().extract::<String>()?, "custom.js");
            assert_eq!(dict.get_item("imgmath_image_format")?.unwrap().extract::<String>()?, "svg");
            assert!(dict.get_item("mathjax_options")?.is_some());
            Ok(())
        })
        .unwrap();
    }
}
