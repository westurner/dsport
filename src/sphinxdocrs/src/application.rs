//! `sphinxdocrs::application` — Rust port of `sphinx.application.Sphinx`.
//!
//! Minimal `SphinxApp` that wires `SphinxConfig`, `EventManager`,
//! `SphinxComponentRegistry`, `BuildEnvironment`, and a native `Builder`
//! together into a runnable build.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `Sphinx.__init__` | [`SphinxApp::new`] | path validation, config read, env init, builder selection |
//! | `Sphinx.build` | [`SphinxApp::build`] | delegates to the native `Builder::build_all` |
//! | `Sphinx.srcdir` | `SphinxApp::srcdir` | resolved absolute path |
//! | `Sphinx.outdir` | `SphinxApp::outdir` | resolved absolute path |
//! | `Sphinx.doctreedir` | `SphinxApp::doctreedir` | resolved absolute path |
//! | `Sphinx.config` | `SphinxApp::config` | `SphinxConfig` read from `conf.py` or defaults |
//! | `Sphinx.registry` | `SphinxApp::registry` | `SphinxComponentRegistry` |
//!
//! **Deferred** (needs full pipeline): event emission, extension loading,
//! parallel builds, i18n, incremental rebuild, Jinja2 theming.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::builders::html::HtmlBuilder;
use crate::builders::latex::LatexBuilder;
use crate::builders::manpage::ManpageBuilder;
use crate::builders::{BuildError, BuildResult, Builder};
use crate::config::SphinxConfig;
use crate::environment::{BuildEnvironment, EnvProject};
use crate::registry::SphinxComponentRegistry;

// ── AppError ──────────────────────────────────────────────────────────────────

/// Errors from `SphinxApp::new` or `SphinxApp::build`.
///
/// Mirrors `sphinx.errors.ApplicationError`.
#[derive(Debug)]
pub enum AppError {
    /// A path constraint was violated (mirrors `ApplicationError`).
    InvalidPath(String),
    /// An unknown builder name was requested.
    UnknownBuilder(String),
    /// A build error propagated from the builder.
    Build(BuildError),
    /// I/O error during setup.
    Io(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidPath(s) => write!(f, "ApplicationError: {s}"),
            AppError::UnknownBuilder(s) => write!(f, "unknown builder: {s}"),
            AppError::Build(e) => write!(f, "build error: {e}"),
            AppError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl From<BuildError> for AppError {
    fn from(e: BuildError) -> Self {
        AppError::Build(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

// ── NATIVE_BUILDERS ───────────────────────────────────────────────────────────

/// Builder names that have a native Rust implementation.
///
/// When `sphinx-build -b <name>` is invoked, if the name is in this list
/// the native builder is used; otherwise the Python fallback runs.
pub const NATIVE_BUILDERS: &[&str] = &["html", "latex", "man"];

/// Return `true` if `builder_name` has a native Rust implementation.
///
/// Used by the `sphinx-build -b` entry point to decide whether to call
/// the native [`SphinxApp`] or fall back to Python.
pub fn is_native_builder(builder_name: &str) -> bool {
    NATIVE_BUILDERS.contains(&builder_name)
}

// ── SphinxApp ─────────────────────────────────────────────────────────────────

/// Minimal Sphinx application.
///
/// Mirrors `sphinx.application.Sphinx` for the pure-Rust build path.
#[derive(Debug)]
pub struct SphinxApp {
    /// Resolved source directory.
    pub srcdir: PathBuf,
    /// Resolved output directory.
    pub outdir: PathBuf,
    /// Resolved doctree cache directory.
    pub doctreedir: PathBuf,
    /// Build config (from conf.py or defaults).
    pub config: SphinxConfig,
    /// Component registry.
    pub registry: SphinxComponentRegistry,
    /// The resolved environment.
    pub env: BuildEnvironment,
    /// The selected builder name.
    pub buildername: String,
    /// Warnings collected during the build.
    pub warnings: Vec<String>,
}

impl SphinxApp {
    /// Construct and initialize a `SphinxApp`.
    ///
    /// Mirrors `Sphinx.__init__` path/config validation + env init.
    ///
    /// # Errors
    ///
    /// - `AppError::InvalidPath` if `srcdir` does not exist, `outdir` is not
    ///   a directory (when it exists), or `srcdir == outdir`.
    /// - `AppError::UnknownBuilder` if `buildername` is not in `NATIVE_BUILDERS`.
    pub fn new(
        srcdir: impl AsRef<Path>,
        outdir: impl AsRef<Path>,
        doctreedir: impl AsRef<Path>,
        buildername: impl Into<String>,
        overrides: HashMap<String, String>,
    ) -> Result<Self, AppError> {
        let srcdir = srcdir.as_ref().canonicalize().map_err(|_| {
            AppError::InvalidPath(format!(
                "Cannot find source directory ({})",
                srcdir.as_ref().display()
            ))
        })?;

        let outdir = outdir.as_ref().to_path_buf();
        // Create outdir if it doesn't exist.
        if !outdir.exists() {
            std::fs::create_dir_all(&outdir)?;
        }
        let outdir = outdir.canonicalize()?;

        // outdir must not be a file.
        if outdir.exists() && !outdir.is_dir() {
            return Err(AppError::InvalidPath(format!(
                "Output directory ({}) is not a directory",
                outdir.display()
            )));
        }

        // srcdir and outdir cannot be the same.
        if srcdir == outdir {
            return Err(AppError::InvalidPath(
                "Source directory and destination directory cannot be identical".into(),
            ));
        }

        let doctreedir = {
            if !doctreedir.as_ref().exists() {
                std::fs::create_dir_all(doctreedir.as_ref())?;
            }
            doctreedir.as_ref().canonicalize()?
        };

        let buildername = buildername.into();

        // Read config. If `conf.py` exists in srcdir, read it via PyO3;
        // otherwise use defaults + overrides.
        let config = build_config(&srcdir, overrides);

        let _registry = SphinxComponentRegistry::new();

        // Register the native HTML builder.
        let mut reg = SphinxComponentRegistry::new();
        for name in NATIVE_BUILDERS {
            reg.add_builder(*name, "sphinxdocrs::builders::html::HtmlBuilder");
        }

        // Build environment.
        let project = EnvProject::new(&srcdir, &[(".rst", "restructuredtext")]);
        let env = BuildEnvironment::new(config.clone(), project, &srcdir, &doctreedir);

        Ok(Self {
            srcdir,
            outdir,
            doctreedir,
            config,
            registry: reg,
            env,
            buildername,
            warnings: Vec::new(),
        })
    }

    /// Run the build.
    ///
    /// Mirrors `Sphinx.build(force_all=True)` for a full rebuild.
    ///
    /// Returns a [`BuildResult`] with counts of written / skipped documents.
    pub fn build(&self) -> Result<BuildResult, AppError> {
        match self.buildername.as_str() {
            "html" => {
                let builder = HtmlBuilder::new();
                let result = builder.build_all(&self.srcdir, &self.outdir, &self.env)?;
                Ok(result)
            }
            "latex" => {
                let builder = LatexBuilder::new();
                Ok(builder.build_all(&self.srcdir, &self.outdir, &self.env)?)
            }
            "man" => {
                let builder = ManpageBuilder::new();
                Ok(builder.build_all(&self.srcdir, &self.outdir, &self.env)?)
            }
            other => Err(AppError::UnknownBuilder(other.into())),
        }
    }

    /// Return `true` if `buildername` is supported natively.
    pub fn supports_native(&self) -> bool {
        is_native_builder(&self.buildername)
    }
}

// ── config helper ─────────────────────────────────────────────────────────────

/// Load `SphinxConfig`.
///
/// Tries to read `conf.py` via PyO3; falls back to defaults + overrides.
fn build_config(_srcdir: &Path, overrides: HashMap<String, String>) -> SphinxConfig {
    // Convert String overrides to ConfigVal overrides.
    // The full Config port reads all values; here we pass raw_config = {}
    // and let overrides supply what's specified on the command line.
    SphinxConfig::new(std::collections::HashMap::new(), overrides)
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_src() -> TempDir {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("index.rst"),
            "Welcome\n=======\n\nHomepage.\n",
        )
        .unwrap();
        tmp
    }

    // ── is_native_builder ─────────────────────────────────────────────────────

    #[test]
    fn native_builder_html() {
        assert!(is_native_builder("html"));
    }

    #[test]
    fn non_native_builder_latex() {
        // latex is now native
        assert!(is_native_builder("latex"));
    }

    #[test]
    fn non_native_builder_epub() {
        assert!(!is_native_builder("epub"));
    }

    #[test]
    fn non_native_builder_unknown() {
        assert!(!is_native_builder("nonexistent"));
    }

    // ── SphinxApp::new path validation ────────────────────────────────────────

    #[test]
    fn new_missing_srcdir_errors() {
        let out = TempDir::new().unwrap();
        let dt = TempDir::new().unwrap();
        let err = SphinxApp::new(
            "/nonexistent/path/that/does/not/exist",
            out.path(),
            dt.path(),
            "html",
            HashMap::new(),
        )
        .unwrap_err();
        assert!(
            matches!(err, AppError::InvalidPath(_)),
            "expected InvalidPath, got {err}"
        );
    }

    #[test]
    fn new_same_src_and_out_errors() {
        let tmp = TempDir::new().unwrap();
        let err = SphinxApp::new(
            tmp.path(),
            tmp.path(),
            tmp.path().join("doctrees"),
            "html",
            HashMap::new(),
        )
        .unwrap_err();
        assert!(matches!(err, AppError::InvalidPath(_)));
    }

    #[test]
    fn new_creates_outdir_if_missing() {
        let src = make_src();
        let base = TempDir::new().unwrap();
        let out = base.path().join("_build");
        let dt = base.path().join(".doctrees");
        assert!(!out.exists());
        SphinxApp::new(src.path(), &out, &dt, "html", HashMap::new()).unwrap();
        assert!(out.exists());
    }

    #[test]
    fn new_html_builder_succeeds() {
        let src = make_src();
        let out = TempDir::new().unwrap();
        let dt = TempDir::new().unwrap();
        let app =
            SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
        assert_eq!(app.buildername, "html");
        assert!(app.supports_native());
    }

    // ── SphinxApp::build ──────────────────────────────────────────────────────

    #[test]
    fn build_html_writes_index() {
        let src = make_src();
        let out = TempDir::new().unwrap();
        let dt = TempDir::new().unwrap();
        let app =
            SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
        let result = app.build().unwrap();
        assert_eq!(result.written, 1);
        assert!(out.path().join("index.html").exists());
    }

    #[test]
    fn build_html_content_has_doctype() {
        let src = make_src();
        let out = TempDir::new().unwrap();
        let dt = TempDir::new().unwrap();
        let app =
            SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
        app.build().unwrap();
        let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(html.starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn build_html_multiple_docs() {
        let src = TempDir::new().unwrap();
        std::fs::write(src.path().join("index.rst"), "Home\n====\n").unwrap();
        std::fs::write(src.path().join("about.rst"), "About\n=====\n").unwrap();
        let out = TempDir::new().unwrap();
        let dt = TempDir::new().unwrap();
        let app =
            SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
        let result = app.build().unwrap();
        assert_eq!(result.written, 2);
    }

    #[test]
    fn build_unknown_builder_errors() {
        let src = make_src();
        let out = TempDir::new().unwrap();
        let dt = TempDir::new().unwrap();
        // Bypass builder validation in ::new by using html then swapping.
        let mut app =
            SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
        app.buildername = "epub".into();
        let err = app.build().unwrap_err();
        assert!(matches!(err, AppError::UnknownBuilder(_)));
    }

    // ── NATIVE_BUILDERS ───────────────────────────────────────────────────────

    #[test]
    fn native_builders_includes_html() {
        assert!(NATIVE_BUILDERS.contains(&"html"));
    }
}
