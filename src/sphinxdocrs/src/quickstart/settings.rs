//! `QuickstartSettings` — the structured representation of everything
//! `sphinx-quickstart` knows after argument-parsing and/or interactive
//! prompts.
//!
//! Mirrors the `d: dict[str, Any]` dictionary that upstream passes
//! through `ask_user` → `generate`.

use std::path::PathBuf;

/// Canonical extension names available via `--ext-*` flags.
/// Order is fixed (matches Python's `EXTENSIONS` dict).
pub const EXTENSIONS: &[(&str, &str)] = &[
    ("autodoc", "automatically insert docstrings from modules"),
    (
        "doctest",
        "automatically test code snippets in doctest blocks",
    ),
    (
        "intersphinx",
        "link between Sphinx documentation of different projects",
    ),
    (
        "todo",
        "write \"todo\" entries that can be shown or hidden on build",
    ),
    ("coverage", "checks for documentation coverage"),
    ("imgmath", "include math, rendered as PNG or SVG images"),
    (
        "mathjax",
        "include math, rendered in the browser by MathJax",
    ),
    (
        "ifconfig",
        "conditional inclusion of content based on config values",
    ),
    (
        "viewcode",
        "include links to the source code of documented Python objects",
    ),
    (
        "githubpages",
        "create .nojekyll file to publish the document on GitHub pages",
    ),
];

/// All settings for a quickstart run.
///
/// Fields mirror the keys in upstream's `d` dict, with typed values.
/// Optional fields that upstream tests for presence (e.g. `'sep' not in d`)
/// are represented as `Option<T>` — `None` means "not yet set and the
/// user should be prompted".
#[derive(Debug, Clone)]
pub struct QuickstartSettings {
    /// Root path for the documentation project.
    pub path: PathBuf,
    /// Separate source/build dirs (`--sep`).  
    /// `false` = use `_build` inside source dir.
    pub sep: bool,
    /// Prefix char for `_templates` / `_static` (default `"_"`).
    pub dot: String,
    /// Project name.
    pub project: String,
    /// Author name(s).
    pub author: String,
    /// Short version string (default `""`).
    pub version: String,
    /// Release string (defaults to `version`).
    pub release: String,
    /// Language code (default `None` meaning English).
    pub language: Option<String>,
    /// Source file suffix (default `".rst"`).
    pub suffix: String,
    /// Master document name without suffix (default `"index"`).
    pub master: String,
    /// Enabled Sphinx extensions (fully-qualified names like
    /// `"sphinx.ext.autodoc"`).
    pub extensions: Vec<String>,
    /// Generate a Makefile.
    pub makefile: bool,
    /// Generate a `make.bat`.
    pub batchfile: bool,
    /// Quiet mode: suppress "Creating file …" messages.
    pub quiet: bool,
    /// Custom template directory (mirrors `--templatedir`).
    pub templatedir: Option<PathBuf>,
}

impl Default for QuickstartSettings {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            sep: false,
            dot: "_".into(),
            project: String::new(),
            author: String::new(),
            version: String::new(),
            release: String::new(),
            language: None,
            suffix: ".rst".into(),
            master: "index".into(),
            extensions: vec![],
            makefile: true,
            batchfile: true,
            quiet: false,
            templatedir: None,
        }
    }
}

impl QuickstartSettings {
    /// Convenience builder — override path.
    pub fn with_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.path = p.into();
        self
    }
}
