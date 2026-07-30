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

use crate::app_events::{AppEventManager, EventArg, EventError, SharedEvents};
use crate::app_facade::{PyAppFacade, SharedAssets, SharedConfig, seed_shared_config};
use crate::builders::changes::ChangesBuilder;
use crate::builders::dirhtml::DirhtmlBuilder;
use crate::builders::gettext::GettextBuilder;
use crate::builders::html::HtmlBuilder;
use crate::builders::json::JsonBuilder;
use crate::builders::latex::LatexBuilder;
use crate::builders::linkcheck::LinkcheckBuilder;
use crate::builders::manpage::ManpageBuilder;
use crate::builders::pseudoxml::PseudoxmlBuilder;
use crate::builders::singlehtml::SinglehtmlBuilder;
use crate::builders::text::TextBuilder;
use crate::builders::xml::XmlBuilder;
use crate::builders::{BuildError, BuildResult, Builder};
use crate::config::SphinxConfig;
use crate::environment::{BuildEnvironment, CONFIG_CHANGED, CONFIG_NEW, CONFIG_OK, EnvProject};
use crate::extension::Extension;
use crate::registry::{SharedRegistry, SphinxComponentRegistry};

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

impl From<EventError> for AppError {
    fn from(e: EventError) -> Self {
        AppError::Extension(e.0)
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
    ("text", "sphinxdocrs::builders::text::TextBuilder"),
    ("xml", "sphinxdocrs::builders::xml::XmlBuilder"),
    (
        "pseudoxml",
        "sphinxdocrs::builders::pseudoxml::PseudoxmlBuilder",
    ),
    ("dirhtml", "sphinxdocrs::builders::dirhtml::DirhtmlBuilder"),
    (
        "singlehtml",
        "sphinxdocrs::builders::singlehtml::SinglehtmlBuilder",
    ),
    ("gettext", "sphinxdocrs::builders::gettext::GettextBuilder"),
    ("changes", "sphinxdocrs::builders::changes::ChangesBuilder"),
];

/// Builder names that have a native Rust implementation.
pub const NATIVE_BUILDERS: &[&str] = &[
    "html",
    "json",
    "latex",
    "man",
    "linkcheck",
    "text",
    "xml",
    "pseudoxml",
    "dirhtml",
    "singlehtml",
    "gettext",
    "changes",
];

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
    /// Component registry. Shared (`Rc<RefCell<_>>`, same pattern as
    /// `events`/`py_config`/`assets`) with every [`PyAppFacade`] constructed
    /// by [`load_extension`](Self::load_extension), so `app.add_builder` /
    /// `app.add_domain` / `app.add_html_theme` / `app.add_post_transform`
    /// calls from a Python extension's `setup(app)` land in the same
    /// registry this field reads from.
    pub registry: SharedRegistry,
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
    /// Whether each loaded extension was satisfied by a Rust equivalent
    /// (`"rust"`) resolved via `docutilsrs_plugins`, or by the plain
    /// Python `setup()` (`"python"`). Diagnostic-only; not an upstream field.
    pub extension_sources: HashMap<String, &'static str>,
    /// The `app.config`-facing store backing every [`PyAppFacade::config`]
    /// (**H4c**). Seeded once from `self.config` in [`new`](Self::new)
    /// (before any extension's `setup(app)` runs), then shared — same
    /// `Rc<RefCell<_>>` sharing pattern as `events` — with every facade
    /// constructed for the lifetime of this app, so `app.config.x = y` in
    /// one extension's `setup()` is visible to every extension loaded
    /// afterward, matching upstream's single persistent `Config` object.
    pub py_config: SharedConfig,
    /// CSS/JS files registered via `app.add_css_file`/`app.add_js_file`
    /// (**H4c**) by any loaded extension. Synced into
    /// `BuildEnvironment::added_css_files`/`added_js_files` by
    /// [`build`](Self::build) just before dispatching to a builder.
    pub assets: SharedAssets,
    /// `-E`/`--fresh-env`: when `true`, [`read`](Self::read) never loads
    /// a previously-persisted environment (**H8a**/**H8b**), so every
    /// found document is reported `added` and gets a full fresh read.
    /// Set via [`set_incremental_options`](Self::set_incremental_options);
    /// defaults to `false` (use a saved environment when one exists,
    /// matching upstream's default).
    pub freshenv: bool,
    /// `-a`/`--write-all`. Recorded for CLI parity and diagnostics, but
    /// **does not change [`read`](Self::read)'s behavior**: every native
    /// builder in this crate already re-renders every document on every
    /// build (none has an "is this doc's output already up to date"
    /// write-skip to bypass), which is exactly what `-a` asks for — so
    /// the flag's effect is already the unconditional default here.
    /// Set via [`set_incremental_options`](Self::set_incremental_options).
    pub force_all: bool,
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
        let registry: SharedRegistry = std::rc::Rc::new(std::cell::RefCell::new(reg));

        // Build environment.
        let project = EnvProject::new(&srcdir, &[(".rst", "restructuredtext")]);
        let env = BuildEnvironment::new(config.clone(), project, &srcdir, &doctreedir);

        // Seed the Python-facing `app.config` store from the already-resolved
        // `SphinxConfig` (conf.py + `-D` overrides + built-in defaults) before
        // any extension's `setup(app)` runs, so e.g. `app.config.extensions`
        // reflects `conf.py`'s real `extensions = [...]` list from the start.
        let py_config = Python::attach(|py| seed_shared_config(py, &config));
        let assets = SharedAssets::default();

        let mut app = Self {
            srcdir,
            outdir,
            doctreedir,
            config,
            registry,
            env,
            buildername,
            warnings: Vec::new(),
            events: AppEventManager::shared(),
            extensions: HashMap::new(),
            extension_sources: HashMap::new(),
            py_config,
            assets,
            freshenv: false,
            force_all: false,
        };

        // Mirrors the tail of `Sphinx.__init__`: load every extension named
        // in `conf.py`'s `extensions = [...]` list, verify `needs_extensions`,
        // then emit `config-inited` / `builder-inited` — in that order, so an
        // extension's `setup(app)` can register a `config-inited` listener
        // and actually see it fire. (A caller invoking
        // [`load_extension`](Self::load_extension) manually *after*
        // `new()` returns will, like upstream, miss `config-inited` — it has
        // already been emitted by then.)
        for ext_name in app.config.extensions() {
            app.load_extension(&ext_name)?;
        }
        app.verify_needs_extensions()?;

        app.events.borrow_mut().emit("config-inited", &[])?;
        app.events.borrow_mut().emit("builder-inited", &[])?;

        Ok(app)
    }

    /// Load a Python extension by dotted module name.
    ///
    /// Mirrors `Sphinx.setup_extension` / `sphinx.extension.load_extension`,
    /// **plus** the plugin-discovery resolver from
    /// [ADR 0005](../../../docs/adr/0005-plugin-discovery.md): before
    /// importing `name` as a Python module, this checks whether a Rust
    /// equivalent is registered for it (`docutilsrs_plugins.discover()`,
    /// entry-point group `docutilsrs.equivalents`, keyed by `name`). If one
    /// is found and passes its upstream-version guard
    /// (`docutilsrs_plugins.upstream_compatible`), its `factory()` result is
    /// called instead of the Python extension's `setup()` — the Python
    /// module is never imported and its `setup()` never runs. If one is
    /// found but its version guard rejects it, a warning is pushed to
    /// [`Self::warnings`] and this falls back to the plain Python path.
    /// Likewise when the resolver is unavailable or no equivalent is
    /// registered at all — this then falls back to importing `name`
    /// directly and calling its `setup(app)`, exactly as before.
    ///
    /// Either way, the invoked callable is passed a fresh [`PyAppFacade`]
    /// bound to `self.events`, and its returned metadata is recorded as an
    /// [`Extension`] in `self.extensions`. [`Self::extension_sources`] records
    /// which path was taken (`"rust"` or `"python"`).
    ///
    /// Called automatically from [`new`](Self::new) for every entry in
    /// `conf.py`'s `extensions` list; exposed publicly so callers can load
    /// additional extensions afterward (matching upstream's
    /// `Sphinx.setup_extension`).
    ///
    /// **Accepted deviation:** dependency-ordered recursive loading
    /// (`setup_extension` recursing into an extension's own
    /// `needs_extensions`/`setup_extension` calls) is not wired in yet —
    /// callers must load extensions in dependency order themselves. The
    /// Rust equivalent's `factory()` result stands in for the upstream
    /// `module` object passed to `Extension(name, module, **kwargs)`, since
    /// there is no real Python module backing a Rust equivalent.
    ///
    /// # Errors
    ///
    /// `AppError::Extension` if the module cannot be imported, has no
    /// `setup` callable, or `setup(app)` raises.
    pub fn load_extension(&mut self, name: &str) -> Result<(), AppError> {
        let mut version_guard_warning = None;
        let result = Python::attach(|py| -> PyResult<()> {
            let (module, setup, source): (Bound<'_, PyAny>, Bound<'_, PyAny>, &'static str) =
                match resolve_rust_equivalent(py, name)? {
                    EquivalentLookup::Found(setup_callable) => {
                        (setup_callable.clone(), setup_callable, "rust")
                    }
                    EquivalentLookup::Incompatible => {
                        version_guard_warning = Some(format!(
                            "extension {name:?}: a Rust equivalent is registered but its \
                             upstream version guard rejected it; falling back to the Python \
                             implementation"
                        ));
                        let module = py.import(name)?;
                        let setup = module.getattr("setup")?;
                        (module.into_any(), setup, "python")
                    }
                    EquivalentLookup::None => {
                        let module = py.import(name)?;
                        let setup = module.getattr("setup")?;
                        (module.into_any(), setup, "python")
                    }
                };

            let facade = Py::new(
                py,
                PyAppFacade::new(
                    self.events.clone(),
                    self.py_config.clone(),
                    self.assets.clone(),
                    self.registry.clone(),
                ),
            )?;
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
            self.extension_sources.insert(name.to_string(), source);
            Ok(())
        })
        .map_err(AppError::from);
        if let Some(warning) = version_guard_warning {
            self.warnings.push(warning);
        }
        result
    }

    /// Verify `config.needs_extensions` against the currently loaded
    /// extensions.
    ///
    /// Native port of `sphinx.extension.verify_needs_extensions` for the
    /// `SphinxApp`/`SphinxConfig` pair (the existing PyO3-facing
    /// `extension::py_verify_needs_extensions` expects duck-typed Python
    /// `app`/`config` objects and is used by the hybrid bridge instead).
    /// Called automatically from [`new`](Self::new) after every configured
    /// extension has been loaded.
    ///
    /// A required-but-unloaded extension is a non-fatal warning (pushed to
    /// [`Self::warnings`]), matching upstream's `logger.warning`. A loaded
    /// extension whose version is too old (compared via
    /// `packaging.version.Version`, falling back to a string compare if
    /// `packaging` is unavailable) is fatal, matching upstream's
    /// `VersionRequirementError`.
    ///
    /// # Errors
    ///
    /// `AppError::Extension` if a loaded extension's version does not
    /// satisfy its `needs_extensions` requirement.
    pub fn verify_needs_extensions(&mut self) -> Result<(), AppError> {
        let needs = self.config.needs_extensions();
        if needs.is_empty() {
            return Ok(());
        }
        let mut warnings = Vec::new();
        let result = Python::attach(|py| -> PyResult<()> {
            for (extname, reqversion) in &needs {
                let Some(ext) = self.extensions.get(extname) else {
                    warnings.push(format!(
                        "The {extname} extension is required by needs_extensions settings, \
                         but it is not loaded."
                    ));
                    continue;
                };
                let ext_version: String = ext.bind(py).getattr("version")?.extract()?;
                let mut fulfilled = true;
                if ext_version == "unknown version" {
                    fulfilled = false;
                } else {
                    let cmp = (|| -> PyResult<bool> {
                        let v_ctor = py.import("packaging.version")?.getattr("Version")?;
                        let a = v_ctor.call1((reqversion.as_str(),))?;
                        let b = v_ctor.call1((ext_version.as_str(),))?;
                        a.gt(&b)
                    })();
                    match cmp {
                        Ok(gt) => {
                            if gt {
                                fulfilled = false;
                            }
                        }
                        Err(_) => {
                            if reqversion.as_str() > ext_version.as_str() {
                                fulfilled = false;
                            }
                        }
                    }
                }
                if !fulfilled {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "This project needs the extension {extname} at least in version \
                         {reqversion} and therefore cannot be built with the loaded \
                         version ({ext_version})."
                    )));
                }
            }
            Ok(())
        })
        .map_err(AppError::from);
        self.warnings.extend(warnings);
        result
    }

    /// Run the read phase: discover source files and parse every document
    /// into a doctree, persisting each to the doctree store.
    ///
    /// Mirrors `Sphinx._init_builder` → `BuildEnvironment.find_files` +
    /// `Builder.read` (i.e. `env-get-outdated` / `env-purge-doc` /
    /// `read-source` machinery collapsed into [`BuildEnvironment::find_files`]
    /// + [`BuildEnvironment::read_docs`] for the **H2** two-phase pipeline).
    ///
    /// **H8a/H8b** (incremental rebuild): unless [`freshenv`](Self::freshenv)
    /// is set, this first tries to load a previously-persisted environment
    /// ([`BuildEnvironment::load_persisted`]) from a prior invocation
    /// against the same `doctreedir`. If one loads, its
    /// [`SphinxConfig::stable_hash`] is compared against the current
    /// config's — a mismatch is treated as `CONFIG_CHANGED` (forcing every
    /// previously-read document to be reported as `changed`, matching
    /// upstream's "config changed -> re-read everything" behavior); a
    /// match is `CONFIG_OK`. No persisted environment (or `freshenv`) is
    /// `CONFIG_NEW`. [`BuildEnvironment::get_outdated`] then decides which
    /// documents actually need re-reading — only those are passed to
    /// [`BuildEnvironment::read_docs`], and every `removed` document is
    /// purged via [`BuildEnvironment::remove_doc`]. The environment is
    /// persisted again at the end via
    /// [`BuildEnvironment::save_persisted`] so the *next* invocation
    /// against the same `doctreedir` can skip unchanged documents too.
    ///
    /// Called automatically by [`build`](Self::build); exposed separately so
    /// callers/tests can inspect the environment (`found_docs`, `all_docs`,
    /// stored doctrees, `check_consistency`) between the read and write
    /// phases.
    pub fn read(&mut self) -> Result<(), AppError> {
        self.env.find_files()?;

        let mut config_changed = false;
        if !self.freshenv {
            if let Some(persisted) = self.env.load_persisted() {
                let saved_hash = self.env.apply_persisted(persisted);
                if saved_hash != self.config.stable_hash() {
                    config_changed = true;
                    self.env.set_config_status(CONFIG_CHANGED, "config changed");
                } else {
                    self.env.set_config_status(CONFIG_OK, "");
                }
            } else {
                self.env.set_config_status(CONFIG_NEW, "new config");
            }
        } else {
            self.env
                .set_config_status(CONFIG_NEW, "fresh environment requested (-E)");
        }

        let (added, changed, removed) = self.env.get_outdated(config_changed);

        for docname in &removed {
            self.env.remove_doc(docname);
        }

        let mut to_read = added.clone();
        to_read.extend(changed.iter().cloned());
        to_read.sort();
        to_read.dedup();

        self.events.borrow_mut().emit(
            "env-get-outdated",
            &[
                EventArg::StrList(added),
                EventArg::StrList(changed),
                EventArg::StrList(removed),
            ],
        )?;
        self.events.borrow_mut().emit(
            "env-before-read-docs",
            &[EventArg::StrList(to_read.clone())],
        )?;

        self.env.read_docs(to_read, Some(&self.events))?;

        self.events.borrow_mut().emit("env-updated", &[])?;
        self.warnings.extend(self.env.check_consistency());
        self.events.borrow_mut().emit("env-check-consistency", &[])?;

        self.env.save_persisted().map_err(AppError::from)?;

        Ok(())
    }

    /// Set the **H8a** incremental-rebuild flags
    /// (`-E`/`--fresh-env` and `-a`/`--write-all`). Must be called before
    /// [`read`](Self::read) (and therefore before [`build`](Self::build))
    /// to have any effect. Defaults to `false`/`false` (use a saved
    /// environment and read only outdated documents, matching upstream's
    /// default incremental behavior) when never called.
    pub fn set_incremental_options(&mut self, freshenv: bool, force_all: bool) {
        self.freshenv = freshenv;
        self.force_all = force_all;
    }

    /// Run the build.
    ///
    /// Mirrors `Sphinx.build(force_all=True)` for a full rebuild: runs the
    /// read phase ([`read`](Self::read)) followed by the selected builder's
    /// write phase (`Builder::build_all`), and finally emits
    /// `build-finished`. `config-inited` / `builder-inited` are emitted
    /// earlier, from [`new`](Self::new), matching upstream's
    /// `Sphinx.__init__` timing (after extensions are loaded).
    ///
    /// Returns a [`BuildResult`] with counts of written / skipped documents.
    pub fn build(&mut self) -> Result<BuildResult, AppError> {
        self.read()?;
        // H6c: let the write phase (currently only `HtmlBuilder`) emit
        // `html-page-context` per page.
        self.env.set_events(self.events.clone());
        // H4c: fold any `app.add_css_file`/`app.add_js_file` registrations
        // made during extension loading into the environment so the
        // HTML-family builders' theme renderer can link them.
        {
            let acc = self.assets.borrow();
            self.env
                .set_added_assets(acc.css_files.clone(), acc.js_files.clone());
        }
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
            "text" => {
                let builder = TextBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "xml" => {
                let builder = XmlBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "pseudoxml" => {
                let builder = PseudoxmlBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "dirhtml" => {
                let builder = DirhtmlBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "singlehtml" => {
                let builder = SinglehtmlBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "gettext" => {
                let builder = GettextBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            "changes" => {
                let builder = ChangesBuilder::new();
                builder
                    .build_all(&self.srcdir, &self.outdir, &self.env)
                    .map_err(AppError::from)
            }
            other => Err(AppError::UnknownBuilder(other.into())),
        };

        // Mirrors upstream: `build-finished` always fires, even when the
        // build itself failed. A pre-existing build error always wins over
        // a `build-finished` listener failure (never mask a real build
        // failure with an unrelated listener bug); a listener failure after
        // an otherwise-successful build does propagate, matching upstream's
        // `emit('build-finished', err)` re-raising when no listener
        // suppresses it.
        let emit_result = self.events.borrow_mut().emit("build-finished", &[]);
        match result {
            Ok(value) => emit_result.map(|()| value).map_err(AppError::from),
            Err(e) => Err(e),
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

// ── plugin-discovery resolver (ADR 0005) ──────────────────────────────────────

/// Outcome of consulting the `docutilsrs_plugins` resolver for a given
/// extension name. Distinguishes "no equivalent at all" from "an
/// equivalent exists but its version guard rejected it" so
/// [`SphinxApp::load_extension`] can surface a warning only in the latter
/// case.
enum EquivalentLookup<'py> {
    /// No resolver, or no equivalent registered for this name.
    None,
    /// An equivalent is registered but `upstream_compatible()` returned
    /// `false`.
    Incompatible,
    /// A compatible equivalent was found; this is its `factory()` result.
    Found(Bound<'py, PyAny>),
}

/// Check whether a Rust equivalent is registered for `name` in the
/// `docutilsrs_plugins` resolver (entry-point group `docutilsrs.equivalents`,
/// as decided in [ADR 0005](../../../docs/adr/0005-plugin-discovery.md)),
/// and its upstream-version guard passes.
///
/// Never a hard error: any resolver-side problem (module not importable,
/// no registered equivalent, or a version-guard rejection) comes back as
/// [`EquivalentLookup::None`] or [`EquivalentLookup::Incompatible`], both
/// of which mean "fall back to the plain Python extension".
fn resolve_rust_equivalent<'py>(py: Python<'py>, name: &str) -> PyResult<EquivalentLookup<'py>> {
    let resolver = match py.import("docutilsrs_plugins") {
        Ok(m) => m,
        Err(_) => return Ok(EquivalentLookup::None),
    };

    let discovered = resolver.call_method0("discover")?;
    let eq = discovered.call_method1("get", (name,))?;
    if eq.is_none() {
        return Ok(EquivalentLookup::None);
    }

    let compatible: bool = resolver
        .call_method1("upstream_compatible", (&eq,))?
        .extract()?;
    if !compatible {
        return Ok(EquivalentLookup::Incompatible);
    }

    let factory = eq.getattr("factory")?;
    Ok(EquivalentLookup::Found(factory.call0()?))
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
    fn native_builder_text_xml_pseudoxml() {
        assert!(is_native_builder("text"));
        assert!(is_native_builder("xml"));
        assert!(is_native_builder("pseudoxml"));
    }

    #[test]
    fn native_builder_dirhtml_singlehtml() {
        assert!(is_native_builder("dirhtml"));
        assert!(is_native_builder("singlehtml"));
    }

    #[test]
    fn native_builder_gettext() {
        assert!(is_native_builder("gettext"));
    }

    #[test]
    fn native_builder_changes() {
        assert!(is_native_builder("changes"));
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

    /// H6c: when the real theme resolves (Python + the configured theme are
    /// available), `build()` renders through the theme's own real templates
    /// rather than the embedded placeholder — this asserts markers that only
    /// appear in real Sphinx theme output (the default `alabaster` theme's
    /// sidebar search box, and its own footer credit line), and that the
    /// document body is not HTML-escaped (a regression this test would have
    /// caught: `body`/`toc` must be passed through as safe strings, not
    /// plain JSON, or the auto-escaper double-encodes them).
    ///
    /// Falls back to skipping the theme-specific assertions (but still
    /// checks the page rendered *something* usable) when the real theme
    /// could not be resolved in this environment, so the test remains
    /// meaningful without requiring Python/Sphinx to be installed.
    #[test]
    fn build_html_uses_real_theme_when_available() {
        let src = make_src();
        let out = TempDir::new().unwrap();
        let dt = TempDir::new().unwrap();
        let mut app =
            SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
        app.build().unwrap();
        let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(
            html.contains("<h1>Welcome</h1>"),
            "body must not be HTML-escaped:\n{html}"
        );
        if html.contains("Quick search") {
            // The real `alabaster` (extending `basic`) theme resolved.
            assert!(html.contains("sphinxsidebar"), "got:\n{html}");
            assert!(html.contains("Powered by"), "got:\n{html}");
        }
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

    #[test]
    fn native_builders_includes_h7a_builders() {
        assert!(NATIVE_BUILDERS.contains(&"text"));
        assert!(NATIVE_BUILDERS.contains(&"xml"));
        assert!(NATIVE_BUILDERS.contains(&"pseudoxml"));
    }

    #[test]
    fn native_builders_includes_h7b_builders() {
        assert!(NATIVE_BUILDERS.contains(&"dirhtml"));
        assert!(NATIVE_BUILDERS.contains(&"singlehtml"));
    }

    #[test]
    fn native_builders_includes_h7c_gettext() {
        assert!(NATIVE_BUILDERS.contains(&"gettext"));
    }

    #[test]
    fn native_builders_includes_h7d_changes() {
        assert!(NATIVE_BUILDERS.contains(&"changes"));
    }

    #[test]
    fn build_dispatches_to_text_xml_pseudoxml_builders() {
        for builder in [
            "text",
            "xml",
            "pseudoxml",
            "dirhtml",
            "singlehtml",
            "gettext",
            "changes",
        ] {
            let src = make_src();
            let out = TempDir::new().unwrap();
            let doctrees = TempDir::new().unwrap();
            let mut app = SphinxApp::new(
                src.path(),
                out.path(),
                doctrees.path(),
                builder,
                HashMap::new(),
            )
            .unwrap();
            let result = app.build().unwrap();
            assert_eq!(result.written, 1, "builder {builder} should write 1 doc");
        }
    }
}
