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

use pyo3::prelude::*;

use crate::app_events::{AppEventManager, EventArg, SharedEvents};
use crate::app_facade::PyAppFacade;
use crate::builders::html::HtmlBuilder;
use crate::builders::json::JsonBuilder;
use crate::builders::latex::LatexBuilder;
use crate::builders::linkcheck::LinkcheckBuilder;
use crate::builders::manpage::ManpageBuilder;
use crate::builders::{BuildError, BuildResult, Builder};
use crate::config::SphinxConfig;
use crate::environment::{BuildEnvironment, EnvProject};
use crate::extension::Extension;
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
    /// An error raised while loading or running a Python extension's
    /// `setup(app)` (mirrors `sphinx.errors.ExtensionError`).
    Extension(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidPath(s) => write!(f, "ApplicationError: {s}"),
            AppError::UnknownBuilder(s) => write!(f, "unknown builder: {s}"),
            AppError::Build(e) => write!(f, "build error: {e}"),
            AppError::Io(e) => write!(f, "I/O error: {e}"),
            AppError::Extension(s) => write!(f, "ExtensionError: {s}"),
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

impl From<PyErr> for AppError {
    fn from(e: PyErr) -> Self {
        AppError::Extension(e.to_string())
    }
}

// ── NATIVE_BUILDERS ───────────────────────────────────────────────────────────

/// Builder names that have a native Rust implementation, paired with the
/// crate path of the implementing type (used for registry bookkeeping).
///
/// When `sphinx-build -b <name>` is invoked, if the name appears here the
/// native builder is used; otherwise the Python fallback runs.
pub const NATIVE_BUILDER_CLASSES: &[(&str, &str)] = &[
    ("html", "sphinxdocrs::builders::html::HtmlBuilder"),
    ("json", "sphinxdocrs::builders::json::JsonBuilder"),
    ("latex", "sphinxdocrs::builders::latex::LatexBuilder"),
    ("man", "sphinxdocrs::builders::manpage::ManpageBuilder"),
    (
        "linkcheck",
        "sphinxdocrs::builders::linkcheck::LinkcheckBuilder",
    ),
];

/// Builder names that have a native Rust implementation.
pub const NATIVE_BUILDERS: &[&str] = &["html", "json", "latex", "man", "linkcheck"];

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
    /// Native event bus (**H4a**). Shared (via `Rc<RefCell<_>>`) with any
    /// [`PyAppFacade`] constructed by [`load_extension`](Self::load_extension)
    /// so Python-registered listeners fire alongside native ones.
    pub events: SharedEvents,
    /// Extensions loaded via [`load_extension`](Self::load_extension),
    /// keyed by dotted module name. Mirrors `Sphinx.extensions`.
    pub extensions: HashMap<String, Py<Extension>>,
}

impl std::fmt::Debug for SphinxApp {
    // Manual impl: `events`/`extensions` hold PyO3 handles and closures
    // that aren't `Debug`, so this can't be `#[derive(Debug)]`d.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SphinxApp")
            .field("srcdir", &self.srcdir)
            .field("outdir", &self.outdir)
            .field("doctreedir", &self.doctreedir)
            .field("buildername", &self.buildername)
            .field("warnings", &self.warnings)
            .finish_non_exhaustive()
    }
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

        // Register every native builder under its implementing type.
        let mut reg = SphinxComponentRegistry::new();
        for (name, class) in NATIVE_BUILDER_CLASSES {
            reg.add_builder(*name, *class);
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
            events: AppEventManager::shared(),
            extensions: HashMap::new(),
        })
    }

    /// Load a Python extension by dotted module name.
    ///
    /// Mirrors `Sphinx.setup_extension` / `sphinx.extension.load_extension`:
    /// imports `name` (via PyO3), calls its `setup` function with a fresh
    /// [`PyAppFacade`] bound to `self.events`, and records the returned
    /// metadata as an [`Extension`] in `self.extensions`.
    ///
    /// **Accepted deviation:** `needs_extensions` verification
    /// (`crate::extension::py_verify_needs_extensions`) and dependency-order
    /// loading (`setup_extension` recursing into an extension's own
    /// `needs_extensions`/`setup_extension` calls) are not wired in yet —
    /// callers must load extensions in dependency order themselves.
    ///
    /// # Errors
    ///
    /// `AppError::Extension` if the module cannot be imported, has no
    /// `setup` callable, or `setup(app)` raises.
    pub fn load_extension(&mut self, name: &str) -> Result<(), AppError> {
        Python::attach(|py| -> PyResult<()> {
            let module = py.import(name)?;
            let setup = module.getattr("setup")?;

            let facade = Py::new(py, PyAppFacade::new(self.events.clone()))?;
            let metadata = setup.call1((facade,))?;

            let kwargs = if metadata.is_none() {
                None
            } else {
                metadata.cast::<pyo3::types::PyDict>().ok().cloned()
            };

            let ext_type = py.get_type::<Extension>();
            let ext_obj = ext_type.call((name, module.clone().unbind()), kwargs.as_ref())?;
            let ext: Py<Extension> = ext_obj.extract()?;
            self.extensions.insert(name.to_string(), ext);
            Ok(())
        })
        .map_err(AppError::from)
    }

    /// Run the read phase: discover source files and parse every document
    /// into a doctree, persisting each to the doctree store.
    ///
    /// Mirrors `Sphinx._init_builder` → `BuildEnvironment.find_files` +
    /// `Builder.read` (i.e. `env-get-outdated` / `env-purge-doc` /
    /// `read-source` machinery collapsed into [`BuildEnvironment::find_files`]
    /// + [`BuildEnvironment::read_all`] for the **H2** two-phase pipeline).
    ///
    /// Called automatically by [`build`](Self::build); exposed separately so
    /// callers/tests can inspect the environment (`found_docs`, `all_docs`,
    /// stored doctrees, `check_consistency`) between the read and write
    /// phases.
    pub fn read(&mut self) -> Result<(), AppError> {
        self.env.find_files()?;

        let mut docnames: Vec<String> = self.env.found_docs().iter().cloned().collect();
        docnames.sort();
        self.events
            .borrow_mut()
            .emit("env-get-outdated", &[EventArg::StrList(vec![])]);
        self.events
            .borrow_mut()
            .emit("env-before-read-docs", &[EventArg::StrList(docnames)]);

        self.env.read_all_with_events(&self.events)?;

        self.events.borrow_mut().emit("env-updated", &[]);
        self.warnings.extend(self.env.check_consistency());
        self.events.borrow_mut().emit("env-check-consistency", &[]);
        Ok(())
    }

    /// Run the build.
    ///
    /// Mirrors `Sphinx.build(force_all=True)` for a full rebuild: emits
    /// `config-inited` / `builder-inited` (mirroring `Sphinx.__init__`'s
    /// tail — deferred out of [`new`](Self::new) so
    /// [`load_extension`](Self::load_extension) can register listeners for
    /// them first), then runs the read phase ([`read`](Self::read))
    /// followed by the selected builder's write phase
    /// (`Builder::build_all`), and finally emits `build-finished`.
    ///
    /// Returns a [`BuildResult`] with counts of written / skipped documents.
    pub fn build(&mut self) -> Result<BuildResult, AppError> {
        self.events.borrow_mut().emit("config-inited", &[]);
        self.events.borrow_mut().emit("builder-inited", &[]);

        self.read()?;
        let result = match self.buildername.as_str() {
            "html" => {
                let builder = HtmlBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "latex" => {
                let builder = LatexBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "json" => {
                let builder = JsonBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "man" => {
                let builder = ManpageBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "linkcheck" => {
                let builder = LinkcheckBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            other => Err(AppError::UnknownBuilder(other.into())),
        };

        self.events.borrow_mut().emit("build-finished", &[]);
        result
    }

    /// Return `true` if `buildername` is supported natively.
    pub fn supports_native(&self) -> bool {
        is_native_builder(&self.buildername)
    }
}

// ── config helper ─────────────────────────────────────────────────────────────

/// Load `SphinxConfig`.
///
/// Reads `srcdir/conf.py` via the existing PyO3 infrastructure when present
/// (using [`crate::config::raw_config_from_conf_py`]); falls back to
/// defaults + overrides when conf.py is absent or PyO3 is unavailable.
fn build_config(srcdir: &Path, overrides: HashMap<String, String>) -> SphinxConfig {
    let conf_py = srcdir.join("conf.py");
    if conf_py.exists() {
        if let Ok(raw) = crate::config::raw_config_from_conf_py(&conf_py) {
            return SphinxConfig::new(raw, overrides);
        }
    }
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
        let mut app =
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
        let mut app =
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
        let mut app =
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
