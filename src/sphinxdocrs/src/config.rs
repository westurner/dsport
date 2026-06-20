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
}

impl SphinxConfig {
    /// Construct with only the built-in default options and no `conf.py`.
    pub fn new_defaults() -> Self {
        let mut cfg = Self {
            raw_config: HashMap::new(),
            overrides: HashMap::new(),
            options: HashMap::new(),
            extensions: Vec::new(),
        };
        cfg.register_builtin_options();
        cfg
    }

    /// Construct from a parsed `conf.py` namespace and command-line overrides.
    ///
    /// Mirrors `Config.__init__`.
    pub fn new(raw_config: HashMap<String, ConfigVal>, overrides: HashMap<String, String>) -> Self {
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
        };
        cfg.register_builtin_options();
        cfg
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
        add("version", Str(String::new()), Env, "Version string");
        add("release", Str(String::new()), Env, "Release string");
        add("today", Str(String::new()), Env, "Date override");
        add("today_fmt", Null, Env, "strftime format");
        add("language", Str("en".into()), Env, "Language");
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

    /// `highlight_language` — default code-block language.
    pub fn highlight_language(&self) -> String {
        self.get("highlight_language")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "default".into())
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
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod sphinx_config_tests {
    use super::*;

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
    fn defaults_root_doc() {
        let cfg = SphinxConfig::new_defaults();
        assert_eq!(cfg.root_doc(), "index");
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
    fn config_val_display_bool() {
        assert_eq!(ConfigVal::Bool(true).display(), "True");
        assert_eq!(ConfigVal::Bool(false).display(), "False");
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
