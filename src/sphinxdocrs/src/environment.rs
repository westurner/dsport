//! `sphinxdocrs::environment` — Rust port of `sphinx.environment`.
//!
//! `BuildEnvironment` skeleton: the global state accumulator that tracks
//! all parsed documents and their metadata. This is the structural port
//! of `sphinx.environment.BuildEnvironment.__init__`.
//!
//! ## What is ported
//!
//! | upstream attribute | Rust field | notes |
//! | --- | --- | --- |
//! | `all_docs` | [`BuildEnvironment::all_docs`] | docname → read-time μs |
//! | `dependencies` | [`BuildEnvironment::dependencies`] | docname → dep file set |
//! | `included` | [`BuildEnvironment::included`] | docname → included docnames |
//! | `reread_always` | [`BuildEnvironment::reread_always`] | force-reread docnames |
//! | `metadata` | [`BuildEnvironment::metadata`] | docname → metadata dict |
//! | `titles` | [`BuildEnvironment::titles`] | docname → title text |
//! | `longtitles` | [`BuildEnvironment::longtitles`] | docname → override title |
//! | `toc_num_entries` | [`BuildEnvironment::toc_num_entries`] | docname → entry count |
//! | `toc_secnumbers` | [`BuildEnvironment::toc_secnumbers`] | section numbering |
//! | `toctree_includes` | [`BuildEnvironment::toctree_includes`] | docname → includes |
//! | `files_to_rebuild` | [`BuildEnvironment::files_to_rebuild`] | rebuild dependents |
//! | `glob_toctrees` | [`BuildEnvironment::glob_toctrees`] | docnames with :glob: |
//! | `numbered_toctrees` | [`BuildEnvironment::numbered_toctrees`] | :numbered: docnames |
//! | `domaindata` | [`BuildEnvironment::domaindata`] | domain-specific data |
//! | `temp_data` | [`BuildEnvironment::temp_data`] | per-read scratch space |
//! | `ref_context` | [`BuildEnvironment::ref_context`] | cross-ref context |
//! | `config_status` | [`BuildEnvironment::config_status`] | config change state |
//! | `settings` | [`BuildEnvironment::settings`] | docutils settings |
//! | `srcdir` | [`BuildEnvironment::srcdir`] | source directory |
//! | `doctreedir` | [`BuildEnvironment::doctreedir`] | doctree output directory |
//!
//! **Deferred** (needs full Sphinx app wiring): `domains` (DomainsContainer),
//! full `setup()` hook, `get_doctree`, `resolve_references`, search index.

use std::collections::{HashMap, HashSet};

pub(crate) fn read_source_file(path: &Path, encoding: &str) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|err| err.to_string())?;
    docutilsrs::decode_source(&bytes, encoding).map_err(|err| err.to_string())
}
use std::path::{Path, PathBuf};

use docutilsrs::doctree::{Doctree, NodeId, NodeKind};
use serde::{Deserialize, Serialize};

use crate::builders::BuildError;
use crate::config::{ConfigVal, SphinxConfig};
use crate::domains::{
    IndexEntry, ObjectEntry, PendingXref, StdDomain, JsDomain, LeanDomain,  PyDomain, RstDomain, RustDomain,
    SourceObjectEntry, XrefResolution, scan,
};
use crate::source_docs::{
    AnalysisError, AnalysisSnapshot, SourceAnalysisRequest, SourceLanguage,
    SourceSnapshotProvider, source_input_hash,
};

// ── project shim ─────────────────────────────────────────────────────────────

/// Minimal project descriptor for the environment.
///
/// The full PyO3-backed `Project` is in `crate::project`; this struct
/// carries the Rust-native subset needed for `BuildEnvironment`.
#[derive(Debug, Clone, Default)]
pub struct EnvProject {
    pub srcdir: PathBuf,
    pub source_suffix: Vec<(String, String)>,
    /// Known docnames (populated by `discover` / `find_files`).
    pub docnames: HashSet<String>,
    /// docname → source-relative POSIX path (populated by `find_files`).
    ///
    /// Mirrors `Project.path2doc` / `Project.doc2path`'s internal table.
    pub docname_to_path: HashMap<String, String>,
}

impl EnvProject {
    pub fn new(srcdir: impl Into<PathBuf>, source_suffix: &[(&str, &str)]) -> Self {
        Self {
            srcdir: srcdir.into(),
            source_suffix: source_suffix
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            docnames: HashSet::new(),
            docname_to_path: HashMap::new(),
        }
    }
}

// ── config status constants ───────────────────────────────────────────────────

/// Config is not yet evaluated.
pub const CONFIG_UNSET: i32 = -1;
/// Config matches the previous build.
pub const CONFIG_OK: i32 = 1;
/// Config is new (first build).
pub const CONFIG_NEW: i32 = 2;
/// Config has changed.
pub const CONFIG_CHANGED: i32 = 3;
/// Extension set has changed.
pub const CONFIG_EXTENSIONS_CHANGED: i32 = 4;

// ── default docutils settings ─────────────────────────────────────────────────

/// Default docutils writer settings injected by Sphinx.
///
/// Mirrors `sphinx.environment.default_settings`.
pub fn default_settings() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("auto_id_prefix".into(), "id".into());
    m.insert("image_loading".into(), "link".into());
    m.insert("embed_stylesheet".into(), "false".into());
    m.insert("cloak_email_addresses".into(), "true".into());
    m.insert("pep_base_url".into(), "https://peps.python.org/".into());
    m.insert(
        "rfc_base_url".into(),
        "https://datatracker.ietf.org/doc/html/".into(),
    );
    m.insert("input_encoding".into(), "utf-8-sig".into());
    m.insert("doctitle_xform".into(), "false".into());
    m.insert("sectsubtitle_xform".into(), "false".into());
    m.insert("section_self_link".into(), "false".into());
    m.insert("halt_level".into(), "5".into());
    m.insert("file_insertion_enabled".into(), "true".into());
    m
}

// ── BuildEnvironment ──────────────────────────────────────────────────────────

/// Global build environment — the central accumulator for a Sphinx build.
///
/// Mirrors `sphinx.environment.BuildEnvironment`.
///
/// Construction uses [`BuildEnvironment::new`]. The caller supplies a
/// `SphinxConfig` and a `Project`; the remaining fields start empty
/// (matching the default-dict behaviour in Python).
#[derive(Debug, Clone)]
pub struct BuildEnvironment {
    // ── source paths ─────────────────────────────────────────────────────────
    /// Absolute path to the source directory.
    pub srcdir: PathBuf,
    /// Absolute path to the doctree output directory.
    pub doctreedir: PathBuf,

    // ── config ───────────────────────────────────────────────────────────────
    /// The project config (from `conf.py`).
    pub config: SphinxConfig,
    /// Change status vs. the previous build.
    pub config_status: i32,
    /// Human-readable explanation of the config status.
    pub config_status_extra: String,

    // ── project ──────────────────────────────────────────────────────────────
    /// The `Project` (docname↔path mapping).
    pub project: EnvProject,

    // ── docutils settings ─────────────────────────────────────────────────────
    /// Docutils writer settings.
    pub settings: HashMap<String, String>,

    // ── document inventory ────────────────────────────────────────────────────
    /// docname → time of reading (integer microseconds).
    pub all_docs: HashMap<String, i64>,

    /// docname → set of dependency file paths (relative to srcdir).
    pub dependencies: HashMap<String, HashSet<String>>,

    /// docname → set of docnames included from it.
    pub included: HashMap<String, HashSet<String>>,

    /// Docnames that must always be re-read.
    pub reread_always: HashSet<String>,

    // ── metadata ─────────────────────────────────────────────────────────────
    /// docname → metadata dict (arbitrary string key-value pairs).
    pub metadata: HashMap<String, HashMap<String, String>>,

    // ── TOC inventory ────────────────────────────────────────────────────────
    /// docname → title text.
    pub titles: HashMap<String, String>,

    /// docname → override title text (`:title:` directive).
    pub longtitles: HashMap<String, String>,

    /// docname → number of real TOC entries.
    pub toc_num_entries: HashMap<String, usize>,

    /// docname → section-number map (`sectionid → (n, ...)`).
    pub toc_secnumbers: HashMap<String, HashMap<String, Vec<u32>>>,

    /// docname → list of toctree include files.
    pub toctree_includes: HashMap<String, Vec<String>>,

    /// docname → set of files-containing-its-TOC to rebuild.
    pub files_to_rebuild: HashMap<String, HashSet<String>>,

    /// Docnames that contain `:glob:` toctrees.
    pub glob_toctrees: HashSet<String>,

    /// Docnames that contain `:numbered:` toctrees.
    pub numbered_toctrees: HashSet<String>,

    // ── domain data ───────────────────────────────────────────────────────────
    /// domainname → domain-specific data (free-form string map).
    pub domaindata: HashMap<String, HashMap<String, String>>,

    // ── scratch ────────────────────────────────────────────────────────────────
    /// Per-read temporary data cleared at the start of each document read.
    pub temp_data: HashMap<String, String>,

    /// Cross-reference context (e.g. current module, current class).
    pub ref_context: HashMap<String, String>,

    // ── domains (Tier H3) ────────────────────────────────────────────────────
    /// The `std` domain: labels, documents, glossary terms (**H3a**).
    pub std_domain: StdDomain,
    /// The `rst` domain: `rst:directive` / `rst:role` descriptions (**H3c**).
    pub rst_domain: RstDomain,
    /// The `py` domain: modules/functions/classes/methods/attributes/data (**H3b**).
    pub py_domain: PyDomain,
    /// The `js` domain: modules/functions/classes/methods/attributes/data (**H3e**).
    pub js_domain: JsDomain,
    /// The `rust` domain: declarations lowered from rustdoc JSON (**H14c**).
    pub rust_domain: RustDomain,
    /// The `lean` domain: declarations lowered from Arborium (**H14d**).
    pub lean_domain: LeanDomain,
    /// Source-analysis snapshots retained for incremental invalidation and
    /// backend diagnostics (**H14f**).
    pub source_snapshots: HashMap<String, AnalysisSnapshot>,
    /// docname → cross-references recovered from that document's source
    /// during the read phase (**H5b**). See `crate::domains`' module doc
    /// for why this is a text-scan result rather than real `pending_xref`
    /// doctree nodes.
    pub pending_xrefs: HashMap<String, Vec<PendingXref>>,
    /// docname → `.. index::` entries recovered from that document's
    /// source during the read phase (**H3d**/**H5e** input).
    pub indexentries: HashMap<String, Vec<IndexEntry>>,

    /// Optional handle to the app's native event bus (**H4a**), set by
    /// [`crate::application::SphinxApp`] before dispatching to a builder so
    /// the write phase can emit `html-page-context` (H6c) per page. `None`
    /// when there is no owning app (e.g. a builder driven directly in a
    /// test) — event emission is then simply skipped.
    pub events: EventsHandle,

    // ── extension-registered page assets (H4c) ───────────────────────────────
    /// CSS files registered by a Python extension's `app.add_css_file(...)`
    /// during `setup(app)` (or an event listener it connects). Synced from
    /// `SphinxApp::assets` by [`crate::application::SphinxApp::build`] just
    /// before dispatching to a builder; merged into the `css_files`
    /// template list by `crate::theme_render::build_global_context`
    /// alongside whatever is discovered under `outdir/_static/`.
    pub added_css_files: Vec<crate::registry::CssFile>,
    /// JS files registered via `app.add_js_file(...)`. See
    /// `added_css_files`.
    pub added_js_files: Vec<crate::registry::JsFile>,
}

/// Wraps `Option<crate::app_events::SharedEvents>` so [`BuildEnvironment`]
/// can keep deriving `Debug` — `AppEventManager` holds boxed `FnMut`
/// listener closures, which aren't `Debug`.
#[derive(Clone, Default)]
pub struct EventsHandle(pub Option<crate::app_events::SharedEvents>);

impl std::fmt::Debug for EventsHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EventsHandle")
            .field(&self.0.as_ref().map(|_| "SharedEvents"))
            .finish()
    }
}

/// Shared handle to a [`BuildEnvironment`].
///
/// [`crate::application::SphinxApp`] owns the single instance for the
/// lifetime of a build; the very same `Rc` is cloned into every
/// [`crate::app_facade::PyAppFacade`] (as `app.env`) constructed for that
/// app, so a Python extension listener reads/writes the *live* environment
/// — not a point-in-time copy — exactly like upstream, where `app.env` and
/// `env` (the argument passed to most events) are the one persistent
/// `BuildEnvironment` object for the whole build.
pub type SharedEnv = std::rc::Rc<std::cell::RefCell<BuildEnvironment>>;

impl BuildEnvironment {
    /// Construct a new `BuildEnvironment`.
    ///
    /// The `srcdir` and `doctreedir` are paths on disk.
    /// All document-tracking maps start empty.
    ///
    /// Mirrors `BuildEnvironment.__init__` minus the Sphinx app wiring.
    pub fn new(
        config: SphinxConfig,
        project: EnvProject,
        srcdir: impl Into<PathBuf>,
        doctreedir: impl Into<PathBuf>,
    ) -> Self {
        let mut settings = default_settings();
        settings.insert("input_encoding".to_string(), config.source_encoding());
        // upstream injects `self` into settings['env'] — we skip that here.

        Self {
            srcdir: srcdir.into(),
            doctreedir: doctreedir.into(),
            config,
            config_status: CONFIG_UNSET,
            config_status_extra: String::new(),
            project,
            settings,
            all_docs: HashMap::new(),
            dependencies: HashMap::new(),
            included: HashMap::new(),
            reread_always: HashSet::new(),
            metadata: HashMap::new(),
            titles: HashMap::new(),
            longtitles: HashMap::new(),
            toc_num_entries: HashMap::new(),
            toc_secnumbers: HashMap::new(),
            toctree_includes: HashMap::new(),
            files_to_rebuild: HashMap::new(),
            glob_toctrees: HashSet::new(),
            numbered_toctrees: HashSet::new(),
            domaindata: HashMap::new(),
            temp_data: HashMap::new(),
            ref_context: HashMap::new(),
            std_domain: StdDomain::new(),
            rst_domain: RstDomain::new(),
            py_domain: PyDomain::new(),
            js_domain: JsDomain::new(),
            rust_domain: RustDomain::new(),
            lean_domain: LeanDomain::new(),
            source_snapshots: HashMap::new(),
            pending_xrefs: HashMap::new(),
            indexentries: HashMap::new(),
            events: EventsHandle::default(),
            added_css_files: Vec::new(),
            added_js_files: Vec::new(),
        }
    }

    /// Install a handle to the app's native event bus (**H4a**).
    ///
    /// Called by [`crate::application::SphinxApp`] before dispatching to a
    /// builder so the write phase can emit `html-page-context` per page.
    pub fn set_events(&mut self, events: crate::app_events::SharedEvents) {
        self.events = EventsHandle(Some(events));
    }

    /// Install the extension-registered CSS/JS files accumulated during
    /// extension loading (**H4c**). Called by
    /// [`crate::application::SphinxApp::build`] just before dispatching to
    /// a builder, mirroring [`set_events`](Self::set_events)'s timing.
    pub fn set_added_assets(
        &mut self,
        css_files: Vec<crate::registry::CssFile>,
        js_files: Vec<crate::registry::JsFile>,
    ) {
        self.added_css_files = css_files;
        self.added_js_files = js_files;
    }

    /// Return the installed event bus handle, if any.
    pub fn events_handle(&self) -> Option<&crate::app_events::SharedEvents> {
        self.events.0.as_ref()
    }

    /// Register one analyzer snapshot against the document that requested
    /// it. The analyzer remains responsible for acquisition and parsing;
    /// the environment only owns the language-neutral records and cache
    /// identity needed by domains and incremental builds.
    pub fn note_source_snapshot(
        &mut self,
        docname: &str,
        snapshot: AnalysisSnapshot,
    ) -> Result<(), AnalysisError> {
        if snapshot.source_root.is_empty() {
            return Err(AnalysisError::InvalidRequest(
                "source analysis snapshot has an empty source root".to_string(),
            ));
        }
        match snapshot.language {
            SourceLanguage::Rust => self
                .rust_domain
                .note_snapshot(docname, &snapshot.declarations),
            SourceLanguage::Lean => self
                .lean_domain
                .note_snapshot(docname, &snapshot.declarations),
            SourceLanguage::Python => {
                return Err(AnalysisError::InvalidRequest(
                    "Python snapshots are not registered by the Rust/Lean source domains"
                        .to_string(),
                ));
            }
        }
        let metadata = self.domaindata.entry(snapshot.language.to_string()).or_default();
        metadata.insert("backend".to_string(), snapshot.backend.clone());
        metadata.insert(
            "backend_kind".to_string(),
            format!("{:?}", snapshot.backend_kind).to_lowercase(),
        );
        metadata.insert("backend_version".to_string(), snapshot.backend_version.clone());
        metadata.insert("source_root".to_string(), snapshot.source_root.clone());
        metadata.insert("source_hash".to_string(), snapshot.source_hash.clone());
        metadata.insert(
            "schema_version".to_string(),
            snapshot.schema_version.to_string(),
        );
        if !snapshot.request_identity.is_empty() {
            metadata.insert(
                "request_identity".to_string(),
                snapshot.request_identity.clone(),
            );
        }
        if !snapshot.provider_identity.is_empty() {
            metadata.insert(
                "provider_identity".to_string(),
                snapshot.provider_identity.clone(),
            );
        }
        if let Some(toolchain) = &snapshot.toolchain {
            metadata.insert("toolchain".to_string(), toolchain.clone());
        }
        self.source_snapshots.insert(docname.to_string(), snapshot);
        Ok(())
    }

    /// Run a source analyzer and register its result in one operation.
    pub fn analyze_source<A: SourceSnapshotProvider>(
        &mut self,
        docname: &str,
        analyzer: &A,
        request: &SourceAnalysisRequest,
    ) -> Result<(), AnalysisError> {
        let mut snapshot = analyzer.analyze(request)?;
        snapshot.backend_kind = analyzer.backend_kind();
        snapshot.request_identity = request.cache_identity();
        snapshot.provider_identity = analyzer.cache_identity(request);
        self.note_source_snapshot(docname, snapshot)
    }

    /// Return whether a cached snapshot was produced for the same source
    /// analysis configuration as `request`.
    pub fn source_snapshot_matches_request(
        &self,
        docname: &str,
        request: &SourceAnalysisRequest,
    ) -> bool {
        self.source_snapshots
            .get(docname)
            .is_some_and(|snapshot| {
                !snapshot.request_identity.is_empty()
                    && snapshot.request_identity == request.cache_identity()
                    && source_input_hash(request, snapshot.language)
                        .is_ok_and(|hash| hash == snapshot.source_hash)
            })
    }

    /// Return whether a cached snapshot matches both request inputs and the
    /// identity of the provider that produced it.
    pub fn source_snapshot_matches_provider<P: SourceSnapshotProvider>(
        &self,
        docname: &str,
        provider: &P,
        request: &SourceAnalysisRequest,
    ) -> bool {
        self.source_snapshots.get(docname).is_some_and(|snapshot| {
            let provider_identity = if snapshot.provider_identity.is_empty() {
                &snapshot.request_identity
            } else {
                &snapshot.provider_identity
            };
            provider.prepare_for_cache().is_ok()
                && !provider_identity.is_empty()
                && provider_identity == &provider.cache_identity(request)
                && provider
                    .source_hash(request, snapshot.language)
                    .is_ok_and(|hash| hash == snapshot.source_hash)
        })
    }

    /// Rich source objects for search builders and API consumers. The
    /// legacy [`domain_objects`](Self::domain_objects) projection remains
    /// unchanged for Python/JS compatibility.
    pub fn source_domain_objects(&self) -> Vec<SourceObjectEntry> {
        let mut objects = self.rust_domain.source_objects();
        objects.extend(self.lean_domain.source_objects());
        objects
    }

    // ── document tracking ─────────────────────────────────────────────────────

    /// Record that `docname` was read at time `read_time` (μs since epoch).
    ///
    /// Mirrors accumulation into `self.all_docs`.
    pub fn record_doc_read(&mut self, docname: impl Into<String>, read_time: i64) {
        self.all_docs.insert(docname.into(), read_time);
    }

    /// Return `true` if `docname` has been read in this build.
    pub fn is_doc_read(&self, docname: &str) -> bool {
        self.all_docs.contains_key(docname)
    }

    /// Set the title for `docname`.
    pub fn set_title(&mut self, docname: impl Into<String>, title: impl Into<String>) {
        self.titles.insert(docname.into(), title.into());
    }

    /// Get the title for `docname`.
    pub fn get_title(&self, docname: &str) -> Option<&str> {
        self.titles.get(docname).map(String::as_str)
    }

    /// Mark `docname` as depending on `dep_path`.
    pub fn note_dependency(&mut self, docname: impl Into<String>, dep_path: impl Into<String>) {
        self.dependencies
            .entry(docname.into())
            .or_default()
            .insert(dep_path.into());
    }

    /// Clear the per-read scratch (`temp_data` and `ref_context`).
    ///
    /// Called at the start of reading each document, mirroring
    /// `BuildEnvironment.prepare_settings`.
    pub fn clear_temp_data(&mut self) {
        self.temp_data.clear();
        self.ref_context.clear();
    }

    // ── config status ─────────────────────────────────────────────────────────

    /// Set config status and explanation.
    pub fn set_config_status(&mut self, status: i32, extra: impl Into<String>) {
        self.config_status = status;
        self.config_status_extra = extra.into();
    }

    /// Return a human-readable label for the current config status.
    pub fn config_status_label(&self) -> &'static str {
        match self.config_status {
            CONFIG_OK => "config OK",
            CONFIG_NEW => "new config",
            CONFIG_CHANGED => "config changed",
            CONFIG_EXTENSIONS_CHANGED => "extensions changed",
            _ => "config unset",
        }
    }

    // ── found_docs proxy ──────────────────────────────────────────────────────

    /// Delegates to `self.project.docnames`.
    ///
    /// Mirrors `env.found_docs` / `project.discovered`.
    pub fn found_docs(&self) -> &HashSet<String> {
        &self.project.docnames
    }

    // ── H2a: file discovery ───────────────────────────────────────────────────

    /// Walk `self.srcdir`, honouring `exclude_patterns` / `include_patterns`
    /// from config, and populate `self.project.docnames` (+ path map).
    ///
    /// Mirrors `sphinx.project.Project.discover`, folded into the env per
    /// **H2a** so `BuildEnvironment` is the single source of truth for
    /// which documents exist.
    pub fn find_files(&mut self) -> Result<(), BuildError> {
        let include = self.config.include_patterns();
        let mut exclude = self.config.exclude_patterns();
        for p in PROJECT_EXCLUDE_PATHS {
            exclude.push((*p).to_string());
        }

        let files = crate::util_matching::get_matching_files(&self.srcdir, &include, &exclude)
            .map_err(|e| BuildError::Other(format!("invalid exclude/include pattern: {e}")))?;

        // Longest-suffix-first so e.g. `.rst.txt` (if configured) matches
        // before the shorter `.txt`.
        let mut suffixes: Vec<String> = self.config.source_suffix().into_keys().collect();
        suffixes.sort_by_key(|b| std::cmp::Reverse(b.len()));

        self.project.docnames.clear();
        self.project.docname_to_path.clear();

        for filename in files {
            let name = filename.rsplit('/').next().unwrap_or(&filename);
            let matched = suffixes.iter().find(|sfx| name.ends_with(sfx.as_str()));
            if let Some(sfx) = matched {
                let docname = filename
                    .strip_suffix(sfx.as_str())
                    .unwrap_or(&filename)
                    .to_string();
                // Match upstream: first-registered wins on collision.
                if self.project.docnames.contains(&docname) {
                    continue;
                }
                self.project.docnames.insert(docname.clone());
                self.project.docname_to_path.insert(docname, filename);
            }
        }
        Ok(())
    }

    fn register_yaml_notebooks(&mut self) -> Result<(), BuildError> {
        let mut targets = Vec::new();
        let toc_path = self.srcdir.join("_toc.yml");
        if toc_path.exists() {
            let data = std::fs::read_to_string(&toc_path).map_err(|e| {
                BuildError::Other(format!("failed to read {}: {e}", toc_path.display()))
            })?;
            let entries = yaml_toc_entries(&data)?;
            targets.extend(entries.iter().map(|entry| entry.target.clone()));
            let root = yaml_toc_root_and_children(&data, &self.config.root_doc())?;
            if let Some((root, children)) = root {
                self.note_toctree(root, children);
            }
            for entry in entries {
                if let (Some(docname), Some(title)) = (yaml_docname(&entry.target), entry.title) {
                    self.longtitles.insert(docname, title);
                }
            }
        }

        let source_paths: Vec<PathBuf> = self
            .project
            .docname_to_path
            .values()
            .map(|path| self.srcdir.join(path))
            .collect();
        for path in source_paths {
            let source = std::fs::read_to_string(&path).map_err(|e| {
                BuildError::Other(format!("failed to read {}: {e}", path.display()))
            })?;
            let expanded = expand_yaml_toctree_directives(&source, &self.srcdir)?;
            targets.extend(scan_toctree_entries_preserving_suffixes(&expanded));
        }

        for target in targets {
            if !target.ends_with(".ipynb") || target.starts_with("/") || target.contains("://") {
                continue;
            }
            let path = self.srcdir.join(&target);
            let Ok(path) = path.canonicalize() else {
                continue;
            };
            let Ok(relative) = path.strip_prefix(&self.srcdir) else {
                continue;
            };
            let relative = relative.to_string_lossy().replace('\\', "/");
            let Some(docname) = relative.strip_suffix(".ipynb") else {
                continue;
            };
            if self.project.docnames.insert(docname.to_string()) {
                self.project
                    .docname_to_path
                    .insert(docname.to_string(), relative);
            }
        }
        Ok(())
    }

    /// Return the absolute source path for `docname`.
    ///
    /// Uses the path recorded by [`find_files`](Self::find_files) when
    /// available; otherwise falls back to `docname + <first source suffix>`,
    /// matching upstream `Project.doc2path`.
    pub fn doc2path(&self, docname: &str) -> PathBuf {
        if let Some(rel) = self.project.docname_to_path.get(docname) {
            return self.srcdir.join(rel);
        }
        let first_suffix = self
            .config
            .source_suffix()
            .into_keys()
            .next()
            .unwrap_or_else(|| ".rst".to_string());
        self.srcdir.join(format!("{docname}{first_suffix}"))
    }

    // ── H2b: doctree store ────────────────────────────────────────────────────

    /// Path to the persisted doctree for `docname` under `self.doctreedir`.
    pub fn doctree_path(&self, docname: &str) -> Result<PathBuf, BuildError> {
        sanitize_docname(docname)?;
        let rel: PathBuf = format!("{docname}.doctree").split('/').collect();
        Ok(self.doctreedir.join(rel))
    }

    /// Parse `docname`'s current source file into a fresh [`Doctree`]
    /// (does not touch the store). Mirrors the parsing half of
    /// `BuildEnvironment.read_doc`.
    pub fn parse_doc(&self, docname: &str) -> Result<Doctree, BuildError> {
        sanitize_docname(docname)?;
        let path = self.doc2path(docname);
        let source = read_source_file(&path, &self.config.source_encoding())
            .map_err(|e| BuildError::Other(format!("failed to read {}: {e}", path.display())))?;
        self.parse_source(docname, &source)
    }

    /// Return the parser identity configured for a source path.
    ///
    /// Suffixes are matched longest-first, matching `find_files()`. A path
    /// with no configured match keeps the historical RST default so direct
    /// callers that provide an unregistered path remain compatible.
    pub fn parser_for_path(&self, path: &Path) -> String {
        let mut suffixes: Vec<(String, String)> = self.config.source_suffix().into_iter().collect();
        suffixes.sort_by_key(|(suffix, _)| std::cmp::Reverse(suffix.len()));
        let path = path.to_string_lossy();
        suffixes
            .into_iter()
            .find(|(suffix, _)| path.ends_with(suffix))
            .map(|(_, parser)| parser)
            .unwrap_or_else(|| "restructuredtext".into())
    }

    /// Parse source using the parser selected by `source_suffix`.
    ///
    /// The native MyST path produces a `docutilsrs::Doctree` directly. It
    /// deliberately does not call the standalone HTML renderer, preserving
    /// the Sphinx read-phase contract for transforms, persistence, and
    /// writers.
    pub fn parse_source(&self, docname: &str, source: &str) -> Result<Doctree, BuildError> {
        let path = self.doc2path(docname);
        if path
            .extension()
            .is_some_and(|extension| extension == "ipynb")
        {
            let notebook: nbformat::v4::Notebook = serde_json::from_str(source).map_err(|e| {
                BuildError::Other(format!("invalid notebook {}: {e}", path.display()))
            })?;
            let markdown = nbconvertrs::notebook_to_markdown(&notebook);
            return Ok(myst_md_rs::parse_to_doctree(
                &markdown,
                path.to_string_lossy().into_owned(),
                &myst_md_rs::DoctreeOptions::default(),
            ));
        }
        let mut tree = match self.parser_for_path(&path).as_str() {
            "restructuredtext" => Ok(docutilsrs::parse_rst_with_options(
                source,
                docname,
                docutilsrs::TitlePromotion::Preserve,
            )),
            "myst" | "markdown" => Ok(myst_md_rs::parse_to_doctree(
                source,
                path.to_string_lossy().into_owned(),
                &myst_md_rs::DoctreeOptions::default(),
            )),
            parser => Err(BuildError::Other(format!(
                "unknown source parser {parser:?} configured for {docname:?}"
            ))),
        }?;
        apply_module_section_ids(&mut tree, source);
        if self.config.smartquotes() {
            apply_smartquotes(&mut tree);
        }
        Ok(tree)
    }

    /// Persist `tree` to `doctreedir/<docname>.doctree`.
    ///
    /// Mirrors the on-disk half of `BuildEnvironment.read_doc` (upstream
    /// pickles the whole env; here each doctree is stored independently so
    /// the write phase can fetch just the documents it needs).
    pub fn store_doctree(&self, docname: &str, tree: &Doctree) -> Result<(), BuildError> {
        let path = self.doctree_path(docname)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, tree.to_bytes())?;
        Ok(())
    }

    /// Return `true` if a doctree has been persisted for `docname`.
    pub fn has_stored_doctree(&self, docname: &str) -> bool {
        self.doctree_path(docname)
            .map(|p| p.is_file())
            .unwrap_or(false)
    }

    /// Read back a doctree previously persisted by
    /// [`store_doctree`](Self::store_doctree).
    ///
    /// Mirrors `BuildEnvironment.get_doctree`.
    pub fn get_doctree(&self, docname: &str) -> Result<Doctree, BuildError> {
        let path = self.doctree_path(docname)?;
        let bytes = std::fs::read(&path)
            .map_err(|e| BuildError::Other(format!("no stored doctree for {docname:?}: {e}")))?;
        Doctree::from_bytes(&bytes)
            .map_err(|e| BuildError::Other(format!("corrupt doctree for {docname:?}: {e}")))
    }

    /// Return the resolved doctree for `docname`, ready for the write phase.
    ///
    /// Mirrors `BuildEnvironment.get_and_resolve_doctree`. Post-read
    /// transforms (cross-reference resolution, toctree expansion) belong to
    /// the domain machinery in **H3**/**H5** and are not implemented yet —
    /// this currently returns the stored doctree unchanged, which is
    /// correct as long as no domain has registered a transform.
    pub fn get_and_resolve_doctree(&self, docname: &str) -> Result<Doctree, BuildError> {
        self.get_doctree(docname)
    }

    // ── H2c: read phase ───────────────────────────────────────────────────────

    /// Read every doc in [`found_docs`](Self::found_docs), recording titles
    /// and toctree entries, and persisting each parsed doctree to the store
    /// so the write phase can retrieve it without re-parsing. Returns the
    /// docnames processed, in sorted order.
    ///
    /// Mirrors the per-document loop inside `Sphinx.read()` /
    /// `BuildEnvironment.read_doc`. UID-based versioning
    /// (`apply_uid_transform`, incremental rebuild) and full toctree/include
    /// directive parsing (currently a text-level scan — see
    /// [`scan_toctree_entries`]) are accepted deviations for this phase of
    /// the port; they are picked up again in **H8** and **H3a**
    /// respectively.
    pub fn read_all(&mut self) -> Result<Vec<String>, BuildError> {
        self.register_yaml_notebooks()?;
        let mut docnames: Vec<String> = self.found_docs().iter().cloned().collect();
        docnames.sort();
        self.read_all_impl(docnames, None)
    }

    /// Same as [`read_all`](Self::read_all), but emits `source-read`
    /// (before each document is parsed) and `doctree-read` (after it is
    /// stored) on `events` — the **H4a** per-document event hooks used by
    /// [`crate::application::SphinxApp::read`].
    pub fn read_all_with_events(
        &mut self,
        events: &crate::app_events::SharedEvents,
    ) -> Result<Vec<String>, BuildError> {
        self.register_yaml_notebooks()?;
        let mut docnames: Vec<String> = self.found_docs().iter().cloned().collect();
        docnames.sort();
        self.read_all_impl(docnames, Some(events))
    }

    /// Read exactly `docnames` (parsed, domain-scanned, and persisted to
    /// the doctree store — same per-document work `read_all` does),
    /// rather than every [`found_docs`](Self::found_docs) entry.
    ///
    /// Used by [`crate::application::SphinxApp::read`]'s **H8a**
    /// incremental path to re-read only the documents `get_outdated`
    /// reported as added/changed, leaving already-up-to-date documents'
    /// persisted doctrees and domain data (restored by
    /// [`apply_persisted`](Self::apply_persisted)) untouched.
    pub fn read_docs(
        &mut self,
        docnames: Vec<String>,
        events: Option<&crate::app_events::SharedEvents>,
    ) -> Result<Vec<String>, BuildError> {
        self.read_all_impl(docnames, events)
    }

    fn read_all_impl(
        &mut self,
        docnames: Vec<String>,
        events: Option<&crate::app_events::SharedEvents>,
    ) -> Result<Vec<String>, BuildError> {
        use crate::app_events::EventArg;

        for docname in &docnames {
            let path = self.doc2path(docname);
            let source = read_source_file(&path, &self.config.source_encoding()).map_err(|e| {
                BuildError::Other(format!("failed to read {}: {e}", path.display()))
            })?;

            if let Some(events) = events {
                events
                    .borrow_mut()
                    .emit(
                        "source-read",
                        &[
                            EventArg::Str(docname.clone()),
                            EventArg::StrList(vec![source.clone()]),
                        ],
                    )
                    .map_err(|e| BuildError::Other(e.0))?;
            }

            self.read_one_with_source(docname, &source)?;

            if let Some(events) = events {
                // Read the just-stored doctree back so listeners get a
                // real, `.findall()`-capable object (see `EventArg::Doctree`'s
                // doc comment for the accepted "not read back after mutation"
                // deviation) instead of the docname string upstream's
                // `doctree-read(app, doctree)` never actually passes.
                let tree = self.get_doctree(docname)?;
                events
                    .borrow_mut()
                    .emit("doctree-read", &[EventArg::Doctree(tree)])
                    .map_err(|e| BuildError::Other(e.0))?;
            }
        }

        // Domain inventories are complete only after every selected document
        // has been read. Rewrite pending xrefs once at that boundary so all
        // builders consume the same persisted doctree instead of each builder
        // repeating the resolution walk during rendering.
        let mut stored_docnames: Vec<String> = self.all_docs.keys().cloned().collect();
        stored_docnames.sort();
        for docname in stored_docnames {
            let mut tree = self.get_doctree(&docname)?;
            self.resolve_xref_nodes(&mut tree, &docname);
            self.store_doctree(&docname, &tree)?;
        }

        Ok(docnames)
    }

    /// Parse, domain-scan, and persist a single document, reading its
    /// source from disk itself. Thin wrapper around
    /// [`read_one_with_source`](Self::read_one_with_source) — see that
    /// method for the actual body and for why callers that need to fire
    /// the upstream `source-read` event (with its mutable `source: list[str]`
    /// argument) should read the file *themselves* and call
    /// [`read_one_with_source`](Self::read_one_with_source) directly
    /// instead.
    pub fn read_one(&mut self, docname: &str) -> Result<(), BuildError> {
        let path = self.doc2path(docname);
        let source = read_source_file(&path, &self.config.source_encoding())
            .map_err(|e| BuildError::Other(format!("failed to read {}: {e}", path.display())))?;
        self.read_one_with_source(docname, &source)
    }

    /// Parse, domain-scan, and persist a single document from an
    /// already-read `source` string — the per-document body of
    /// [`read_all_impl`](Self::read_all_impl), factored out so
    /// [`crate::application::SphinxApp::read`] can call it with only a
    /// short-lived `RefCell` borrow (see `SharedEnv`'s doc comment): holding
    /// a `borrow_mut()` across the surrounding `source-read`/`doctree-read`
    /// event emissions would panic if a Python listener on either event
    /// touches `app.env` (the exact same `Rc<RefCell<_>>`) while that
    /// borrow is live.
    ///
    /// Taking `source` as a parameter (rather than reading the file here)
    /// lets a caller emit the upstream `source-read` event — whose second
    /// argument is the mutable `source: list[str]` a listener may rewrite
    /// in place — *before* parsing, and feed back whatever content that
    /// event left behind. **Accepted deviation:** this port does not
    /// currently read back a Python listener's in-place edit to that list
    /// (would need `emit`'s generic `EventArg` bus to support an
    /// after-the-fact readback of a mutable arg); `source` is always
    /// exactly the file's on-disk content.
    pub fn read_one_with_source(&mut self, docname: &str, source: &str) -> Result<(), BuildError> {
        let yaml_source = expand_yaml_toctree_directives(source, &self.srcdir)?;
        let included_source = expand_rst_includes(
            &yaml_source,
            &self.doc2path(docname)
                .parent()
                .unwrap_or(self.srcdir.as_path())
                .to_path_buf(),
            &self.config.source_encoding(),
        );
        let expanded_source = self.expand_autodoc(&included_source);
        let source = expanded_source.as_deref().unwrap_or(&included_source);
        let highlighted_source = self.apply_highlight_language(source);
        let parse_source = highlighted_source.as_deref().unwrap_or(source);
        let tree = self.parse_source(docname, parse_source)?;

        let title = Self::document_title_from_tree(&tree)
            .unwrap_or_else(|| docname.rsplit('/').next().unwrap_or(docname).to_string());
        self.set_title(docname.to_string(), title);

        // Blank out code-block/literal-block bodies before text-scanning
        // for directives: a documentation page that *illustrates*
        // `.. toctree::`/`.. include::` syntax inside a
        // `.. code-block:: rst` example (as Sphinx's own docs do) must not
        // have that example misread as a real directive.
        let scan_source = strip_opaque_literal_blocks(parse_source);

        // `.. toctree::` entries are written *relative to the directory
        // containing this document* (e.g. `tutorial/index.rst` listing
        // `getting-started` really means `tutorial/getting-started`), not
        // relative to the project root. Mirrors upstream
        // `sphinx.util.docname_join` / `TocTree.run()` qualifying each
        // entry against its own docname before recording it — without
        // this, `toctree_includes`/sidebar-nav lookups keyed by the real
        // docname (`env.titles`, `get_target_uri`, ...) would silently
        // miss every non-root-level toctree entry.
        let entries: Vec<String> = scan_toctree_entries(&scan_source)
            .into_iter()
            .map(|entry| docname_join(docname, &entry))
            .collect();
        for (entry, title) in scan_toctree_entries_with_titles(&scan_source) {
            if let Some(title) = title {
                self.longtitles.insert(docname_join(docname, &entry), title);
            }
        }
        if !entries.is_empty() {
            self.note_toctree(docname.to_string(), entries);
        }

        for include in scan_include_entries(&scan_source) {
            self.note_dependency(docname.to_string(), include);
        }

        self.note_domain_data(docname, parse_source);

        self.store_doctree(docname, &tree)?;
        self.record_doc_read(docname.to_string(), now_micros());

        Ok(())
    }

    fn document_title_from_tree(tree: &Doctree) -> Option<String> {
        let root = tree.root();
        if let NodeKind::Document { title, .. } = &tree.node(root).kind {
            if !title.is_empty() {
                return Some(title.clone());
            }
        }

        for &child in &tree.node(root).children {
            if !matches!(tree.node(child).kind, NodeKind::Section { .. }) {
                continue;
            }
            for &section_child in &tree.node(child).children {
                if !matches!(tree.node(section_child).kind, NodeKind::Title) {
                    continue;
                }
                let mut text = String::new();
                let mut pending = vec![section_child];
                while let Some(node_id) = pending.pop() {
                    if let NodeKind::Text(value) = &tree.node(node_id).kind {
                        text.push_str(value);
                    }
                    pending.extend(tree.node(node_id).children.iter().rev().copied());
                }
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }
        None
    }

    /// Expand the top-level `automodule` directive before docutils parsing.
    ///
    /// The autodoc renderer already owns the static/runtime import fallback;
    /// this small integration point makes its generated RST participate in
    /// the normal parser pipeline, including native code-block highlighting.
    fn expand_autodoc(&self, source: &str) -> Option<String> {
        let lines: Vec<&str> = source.lines().collect();
        let mut output: Vec<String> = Vec::with_capacity(lines.len());
        let mut changed = false;
        let mut index = 0;

        while index < lines.len() {
            let line = lines[index];
            let trimmed = line.trim_start();
            let indent = line.len() - trimmed.len();
            if indent == 0 && trimmed.starts_with(".. automodule::") {
                let module_name = trimmed[15..].trim();
                if !module_name.is_empty()
                    && let Some(path) = self.resolve_python_module(module_name)
                {
                    let mut pairs = Vec::new();
                    let mut next = index + 1;
                    while next < lines.len() {
                        let option = lines[next].trim();
                        if let Some(option) = option.strip_prefix(':')
                            && let Some((name, value)) = option.split_once(':')
                        {
                            pairs.push((name.trim().to_string(), value.trim().to_string()));
                            next += 1;
                            continue;
                        }
                        if option.is_empty() {
                            next += 1;
                        }
                        break;
                    }
                    let options = crate::autodoc::AutodocOptions::from_option_pairs(&pairs);
                    if let Ok(rendered) =
                        crate::autodoc::document_module_auto(&path, module_name, &options, &[])
                    {
                        output.extend(rendered.lines().map(str::to_owned));
                        output.push(String::new());
                        index = next;
                        changed = true;
                        continue;
                    }
                }
            }
            output.push(line.to_owned());
            index += 1;
        }

        changed.then(|| output.join("\n"))
    }

    fn resolve_python_module(&self, module_name: &str) -> Option<PathBuf> {
        let relative = module_name.replace('.', "/");
        let module = self.srcdir.join(format!("{relative}.py"));
        if module.is_file() {
            return Some(module);
        }
        let package = self.srcdir.join(relative).join("__init__.py");
        package.is_file().then_some(package)
    }

    /// Resolve Sphinx's current `highlight` directive for code directives
    /// without an explicit language. This keeps language state at the Sphinx
    /// environment boundary while leaving docutilsrs' parser stateless.
    fn apply_highlight_language(&self, source: &str) -> Option<String> {
        let mut language = self.config.highlight_language();
        let mut output = Vec::new();
        let mut changed = false;

        for line in source.lines() {
            let trimmed = line.trim_start();
            let indent = line.len() - trimmed.len();
            if indent > 0
                && (trimmed.starts_with(".. highlight::")
                    || ["code", "code-block", "sourcecode"]
                        .iter()
                        .any(|name| trimmed.starts_with(&format!(".. {name}::"))))
            {
                output.push(line.to_string());
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix(".. highlight::") {
                let next = rest.trim();
                if !next.is_empty() {
                    language = next.to_string();
                }
                output.push(String::new());
                changed = true;
                continue;
            }

            let directive = ["code", "code-block", "sourcecode"]
                .iter()
                .find(|name| trimmed.starts_with(&format!(".. {name}::")));
            if let Some(name) = directive {
                let prefix = format!(".. {name}::");
                let args = trimmed[prefix.len()..].trim();
                if args.is_empty() && !language.is_empty() && language != "none" {
                    let prefix_indent = &line[..line.len() - trimmed.len()];
                    output.push(format!("{prefix_indent}{prefix} {language}"));
                    changed = true;
                    continue;
                }
            }
            output.push(line.to_string());
        }

        if !changed {
            return None;
        }
        let mut rewritten = output.join("\n");
        if source.ends_with('\n') {
            rewritten.push('\n');
        }
        Some(rewritten)
    }

    /// Populate the `std`/`rst`/`py`/`js` domains, `indexentries`, and
    /// `pending_xrefs` for `docname` from its source text
    /// (**H3a**/**H3b**/**H3c**/**H3d**/**H3e**/**H5b**).
    ///
    /// Forgets any previously-noted data for `docname` first (mirrors
    /// `Domain.clear_doc`, called by upstream before a document is
    /// re-read), so re-reading a changed document doesn't accumulate
    /// stale labels/terms/objects.
    fn note_domain_data(&mut self, docname: &str, source: &str) {
        use crate::domains::Domain as _;

        self.std_domain.clear_doc(docname);
        self.rst_domain.clear_doc(docname);
        self.py_domain.clear_doc(docname);
        self.js_domain.clear_doc(docname);
        self.rust_domain.clear_doc(docname);
        self.lean_domain.clear_doc(docname);
        self.source_snapshots.remove(docname);

        for (name, sectionname) in scan::scan_labels(source) {
            let labelid = StdDomain::label_id(&name);
            self.std_domain
                .note_label(name, docname, labelid, sectionname.unwrap_or_default());
        }

        for term in scan::scan_glossary_terms(source) {
            let labelid = format!("term-{}", crate::domains::normalize_id(&term));
            self.std_domain.note_term(term, docname, labelid);
        }

        for (objtype, name) in scan::scan_rst_domain_objects(source) {
            // Mirrors real Sphinx's `make_id(env, document, objtype, name)`
            // anchor scheme (e.g. `directive-toctree`, `role-ref`), not a
            // `rst-`-prefixed anchor — this must match the `id="..."`
            // docutilsrs's parser renders on the actual `.. rst:directive::`/
            // `.. rst:role::` `<dt>` element (see
            // `docutilsrs::parser::parse_directive`'s `"rst:directive" |
            // "rst:role"` arm) for cross-references to resolve to the
            // right place.
            let id_prefix = objtype.replace(':', "-");
            let labelid = format!("{id_prefix}-{}", crate::domains::normalize_id(&name));
            self.rst_domain.note_object(objtype, name, docname, labelid);
        }

        self.py_domain.note_source(docname, source);
        self.js_domain.note_source(docname, source);

        let index_entries = scan::scan_index_entries(source);
        if index_entries.is_empty() {
            self.indexentries.remove(docname);
        } else {
            self.indexentries.insert(docname.to_string(), index_entries);
        }

        let xrefs = scan::scan_xref_roles(source);
        if xrefs.is_empty() {
            self.pending_xrefs.remove(docname);
        } else {
            self.pending_xrefs.insert(docname.to_string(), xrefs);
        }
    }

    // ── H3d: search-object population ─────────────────────────────────────────

    /// Aggregate every registered domain's `get_objects()`, each tagged
    /// with its owning domain name, for search-index / index-page
    /// population. Mirrors iterating `env.domains.sorted()` and calling
    /// `domain.get_objects()` upstream.
    pub fn domain_objects(&self) -> Vec<(String, ObjectEntry)> {
        use crate::domains::Domain as _;

        let mut out = Vec::new();
        for entry in self.std_domain.get_objects() {
            out.push(("std".to_string(), entry));
        }
        for entry in self.rst_domain.get_objects() {
            out.push(("rst".to_string(), entry));
        }
        for entry in self.py_domain.get_objects() {
            out.push(("py".to_string(), entry));
        }
        for entry in self.js_domain.get_objects() {
            out.push(("js".to_string(), entry));
        }
        for entry in self.rust_domain.get_objects() {
            out.push(("rust".to_string(), entry));
        }
        for entry in self.lean_domain.get_objects() {
            out.push(("lean".to_string(), entry));
        }
        out
    }

    // ── H5c: reference resolution ──────────────────────────────────────────────

    /// Resolve every cross-reference recovered from `docname`'s source
    /// (via [`note_domain_data`](Self::note_domain_data) during
    /// [`read_all`](Self::read_all)) against the owning domain, falling
    /// back to a dangling-reference warning.
    ///
    /// Mirrors `env.resolve_references(doctree, fromdocname, builder)`,
    /// minus the actual doctree node rewriting (there is no `pending_xref`
    /// node to rewrite yet — see the accepted-deviation note on
    /// `crate::domains`). Callers get the resolution/warning list instead.
    pub fn resolve_references(&self, docname: &str) -> Vec<XrefResolution> {
        use crate::domains::Domain as _;

        let Some(xrefs) = self.pending_xrefs.get(docname) else {
            return Vec::new();
        };

        xrefs
            .iter()
            .map(|xref| {
                let resolved = match xref.domain.as_str() {
                    "rst" => {
                        self.rst_domain
                            .resolve_xref(self, docname, &xref.reftype, &xref.target)
                    }
                    "std" => self.std_domain.resolve_xref_explicit(
                        self,
                        docname,
                        &xref.reftype,
                        &xref.target,
                        xref.explicit_title.is_some(),
                    ),
                    "py" => self
                        .py_domain
                        .resolve_xref(self, docname, &xref.reftype, &xref.target),
                    "js" => self
                        .js_domain
                        .resolve_xref(self, docname, &xref.reftype, &xref.target),
                    "rust" => self
                        .rust_domain
                        .resolve_xref(self, docname, &xref.reftype, &xref.target),
                    "lean" => self
                        .lean_domain
                        .resolve_xref(self, docname, &xref.reftype, &xref.target),
                    _ => None,
                };
                match resolved {
                    Some(mut target) => {
                        if let Some(title) = &xref.explicit_title {
                            target.title = title.clone();
                        } else if xref.shorten {
                            // The `~` truncation prefix (generic to every
                            // `XRefRole` upstream): display only the last
                            // dotted component of the target.
                            target.title = xref
                                .target
                                .rsplit('.')
                                .next()
                                .unwrap_or(&xref.target)
                                .to_string();
                        }
                        XrefResolution::Resolved {
                            xref: xref.clone(),
                            target,
                        }
                    }
                    None => XrefResolution::Unresolved {
                        warning: crate::domains::dangling_warning(xref),
                        xref: xref.clone(),
                    },
                }
            })
            .collect()
    }

    /// [`resolve_references`](Self::resolve_references) for every
    /// document that has recorded cross-references.
    pub fn resolve_all_references(&self) -> HashMap<String, Vec<XrefResolution>> {
        self.pending_xrefs
            .keys()
            .map(|docname| (docname.clone(), self.resolve_references(docname)))
            .collect()
    }

    /// Resolve every recognized standard-domain cross-reference role
    /// (`:ref:`/`:doc:`/`:term:`/`:numref:`/`:keyword:`), as well as the
    /// `rst` domain's `:rst:dir:`/`:rst:role:` roles, found anywhere in
    /// `tree` into a real internal hyperlink, rewriting the doctree node
    /// in place.
    ///
    /// Mirrors what upstream does to the `pending_xref` node inside
    /// `env.resolve_references` (`_resolve_ref_xref` et al. build a real
    /// `nodes.reference` and the transform swaps it in). This port has no
    /// `pending_xref` node (see the accepted-deviation note on
    /// `crate::domains`), but rather than leave resolution as a
    /// side-channel report ([`resolve_references`](Self::resolve_references),
    /// which has nothing to rewrite because it works from a source-text
    /// scan), this walks the already-parsed doctree directly: every
    /// generic `Inline { classes }` node docutilsrs's role-agnostic parser
    /// produces for an unrecognized role name is inspected, and — when
    /// `classes` names a std-domain reftype — turned into a
    /// `Reference { classes: "reference internal", .. }` node wrapping an
    /// `Inline { classes: "std std-<reftype>" | "doc" }` span, matching
    /// real Sphinx's `<a class="reference internal" href="..."><span
    /// class="...">title</span></a>` output.
    ///
    /// Walking the tree (rather than re-using the text-scanned
    /// `pending_xrefs` list) also means roles nested inside directive
    /// bodies (containers, admonitions, ...) resolve exactly like
    /// top-level ones, since there's no source-position bookkeeping to
    /// keep in sync with the parser's own recursion.
    ///
    /// Unresolved roles are left as the original generic `Inline` node
    /// (matching the pre-existing `<span class="ref">...</span>`
    /// fallback) rather than silently dropping content, mirroring
    /// upstream logging a dangling-reference warning but leaving the
    /// text in place.
    pub fn resolve_xref_nodes(&self, tree: &mut Doctree, docname: &str) {
        use crate::builders::Builder as _;
        use crate::builders::html::HtmlBuilder;
        use crate::domains::Domain as _;
        use crate::util_osutil::relative_uri;

        const KNOWN_STD_REFTYPES: &[&str] = &["ref", "doc", "term", "numref", "keyword"];
        const KNOWN_RST_REFTYPES: &[&str] = &["dir", "role"];

        // `sphinx.ext.extlinks`' `extlinks = {name: (url_template, caption_template), ...}`
        // registers one role per key that renders an external link, e.g.
        // `:dudir:`error`` -> `<a class="extlink-dudir reference external"
        // href="https://.../directives.html#error">error</a>`. The extension
        // itself isn't executed (no `add_role` call happens), so its role
        // names are recognized here directly from the raw `conf.py` dict
        // (`self.config` may not have `extlinks` "registered" as a typed
        // option, hence `raw_config()` rather than `get()`).
        let extlinks: Vec<(&str, &str, Option<&str>)> = self
            .config
            .raw_config()
            .get("extlinks")
            .and_then(ConfigVal::as_map)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|(name, val)| {
                        let parts = val.as_list()?;
                        let url = parts.first()?.as_str()?;
                        let caption = parts.get(1).and_then(ConfigVal::as_str);
                        Some((name.as_str(), url, caption))
                    })
                    .collect()
            })
            .unwrap_or_default();

        fn flatten_text(tree: &Doctree, id: NodeId) -> String {
            let mut out = String::new();
            for &child in &tree.node(id).children {
                if let NodeKind::Text(s) = &tree.node(child).kind {
                    out.push_str(s);
                }
            }
            out
        }

        let builder = HtmlBuilder::new();
        let base_uri = builder.get_target_uri(docname);

        // A single forward pass over the arena (`0..nodes_len()`) rather
        // than a recursive parent->child walk collected into a `Vec`
        // first: `Doctree`'s arena already stores nodes in (pre-)creation
        // order, since `append`/`push` always add a parent before any of
        // its children, so indices alone give the same traversal order
        // without the extra recursion or intermediate allocation. Nodes
        // this loop itself appends (the new `Reference`'s `Inline` span
        // and its `Text` child) land past the upfront `nodes_len()` and
        // are simply never visited, which is correct — they can't match
        // `KNOWN_STD_REFTYPES` anyway.
        for id in 0..tree.nodes_len() {
            // `sphinx.ext.extlinks` roles (e.g. `dudir`, `duref`) take
            // priority over the std/rst-domain dispatch below since their
            // names never collide with a domain-prefixed (`domain:reftype`)
            // or std reftype name.
            if let NodeKind::Inline { classes } = &tree.node(id).kind {
                if let Some(&(name, url_tmpl, caption_tmpl)) =
                    extlinks.iter().find(|(name, ..)| *name == classes.as_str())
                {
                    let content = flatten_text(tree, id);
                    let old_children: Vec<NodeId> = tree.node(id).children.clone();
                    if let Some(rest) = content.strip_prefix('!') {
                        // Bang prefix disables the link (same `XRefRole`
                        // "disabled" behavior as std/rst-domain roles).
                        for child in old_children {
                            tree.detach(child);
                        }
                        tree.append(id, NodeKind::Text(rest.to_string()));
                        continue;
                    }
                    let (part, explicit_title) = crate::domains::scan::split_phrase(&content);
                    let url = url_tmpl.replace("%s", &part);
                    let title = explicit_title.unwrap_or_else(|| match caption_tmpl {
                        Some(c) => c.replace("%s", &part),
                        None => part.clone(),
                    });
                    for child in old_children {
                        tree.detach(child);
                    }
                    tree.set_kind(
                        id,
                        NodeKind::Reference {
                            name: String::new(),
                            refuri: url,
                            anonymous: false,
                            classes: format!("extlink-{name} reference external"),
                        },
                    );
                    tree.append(id, NodeKind::Text(title));
                    continue;
                }
            }

            // `domain` is `"std"` for a bare std-domain reftype
            // (`Inline{classes: "ref"}`), `"rst"` for a domain-prefixed one,
            // or explicitly stored on a `NodeKind::PendingXref`.
            let (is_pending_xref, domain, reftype, raw_target, explicit_title) = match &tree
                .node(id)
                .kind
            {
                NodeKind::PendingXref {
                    reftype,
                    reftarget,
                    refdomain,
                    refexplicit,
                    ..
                } => {
                    let content = flatten_text(tree, id);
                    let exp_title = if *refexplicit { Some(content) } else { None };
                    let dom = if refdomain.is_empty() {
                        "std".to_string()
                    } else {
                        refdomain.clone()
                    };
                    (true, dom, reftype.clone(), reftarget.clone(), exp_title)
                }
                NodeKind::Inline { classes } if KNOWN_STD_REFTYPES.contains(&classes.as_str()) => {
                    let content = flatten_text(tree, id);
                    let (raw_target, explicit_title) = crate::domains::scan::split_phrase(&content);
                    (
                        false,
                        "std".to_string(),
                        classes.clone(),
                        raw_target,
                        explicit_title,
                    )
                }
                NodeKind::Inline { classes } => match classes.split_once(':') {
                    Some(("rst", rt)) if KNOWN_RST_REFTYPES.contains(&rt) => {
                        let content = flatten_text(tree, id);
                        let (raw_target, explicit_title) =
                            crate::domains::scan::split_phrase(&content);
                        (
                            false,
                            "rst".to_string(),
                            rt.to_string(),
                            raw_target,
                            explicit_title,
                        )
                    }
                    Some((dom, rt)) => {
                        let content = flatten_text(tree, id);
                        let (raw_target, explicit_title) =
                            crate::domains::scan::split_phrase(&content);
                        (
                            false,
                            dom.to_string(),
                            rt.to_string(),
                            raw_target,
                            explicit_title,
                        )
                    }
                    _ => continue,
                },
                _ => continue,
            };

            let content = flatten_text(tree, id);
            // A leading `!` disables cross-reference resolution entirely
            // (mirrors `sphinx.roles.XRefRole.__call__`'s `disabled` flag):
            // the role still renders, but as plain text with the bang
            // stripped and no `pending_xref`/link ever created.
            if let Some(rest) = content.strip_prefix('!') {
                let old_children: Vec<NodeId> = tree.node(id).children.clone();
                for child in old_children {
                    tree.detach(child);
                }
                tree.append(id, NodeKind::Text(rest.to_string()));
                continue;
            }
            let (target, shorten) = match raw_target.strip_prefix('~') {
                Some(rest) => (rest.to_string(), true),
                None => (raw_target.clone(), false),
            };

            let resolved = match domain.as_str() {
                "std" | "" => self.std_domain.resolve_xref_explicit(
                    self,
                    docname,
                    &reftype,
                    &target,
                    explicit_title.is_some(),
                ),
                "js" => self
                    .js_domain
                    .resolve_xref(self, docname, &reftype, &target),
                "lean" => self
                    .lean_domain
                    .resolve_xref(self, docname, &reftype, &target),
                "py" => self
                    .py_domain
                    .resolve_xref(self, docname, &reftype, &target),
                "rst" => self
                    .rst_domain
                    .resolve_xref(self, docname, &reftype, &target),
                "rust" => self
                    .rust_domain
                    .resolve_xref(self, docname, &reftype, &target),
                _ => None,
            };

            let unresolved_class = match domain.as_str() {
                "js" => format!("xref js js-{reftype}"),
                "lean" => format!("xref lean lean-{reftype}"),
                "py" => format!("xref py py-{reftype}"),
                "rst" => format!("xref rst rst-{reftype}"),
                "rust" => format!("xref rust rust-{reftype}"),
                _ if reftype == "doc" => {
                    if resolved.is_some() {
                        "doc".to_string()
                    } else {
                        "xref doc".to_string()
                    }
                }
                _ => format!("xref std std-{reftype}"),
            };

            let Some(resolved) = resolved else {
                if is_pending_xref {
                    let title = if let Some(t) = &explicit_title {
                        t.clone()
                    } else if shorten {
                        shorten_xref_target(&target, &domain)
                    } else {
                        target.clone()
                    };
                    let old_children: Vec<NodeId> = tree.node(id).children.clone();
                    for child in old_children {
                        tree.detach(child);
                    }
                    tree.set_kind(
                        id,
                        NodeKind::Inline {
                            classes: unresolved_class,
                        },
                    );
                    tree.append(id, NodeKind::Text(title));
                }
                continue;
            };

            let resolved_class = match domain.as_str() {
                "js" => format!("js js-{reftype}"),
                "lean" => format!("lean lean-{reftype}"),
                "py" => format!("py py-{reftype}"),
                "rst" => format!("rst rst-{reftype}"),
                "rust" => format!("rust rust-{reftype}"),
                _ if reftype == "doc" => "doc".to_string(),
                _ => format!("std std-{reftype}"),
            };

            let title = if let Some(t) = &explicit_title {
                t.clone()
            } else if shorten {
                shorten_xref_target(&target, &domain)
            } else {
                resolved.title.clone()
            };

            let target_uri = builder.get_target_uri(&resolved.docname);
            // Mirrors `sphinx.util.nodes.make_refnode`: a same-document
            // reference with a target id becomes a bare `#targetid`
            // fragment (docutils resolves `refid` this way at write
            // time), bypassing `relative_uri` entirely — which would
            // otherwise collapse a same-page `to` down to `''`,
            // silently dropping the fragment (see its own `b2 == t2`
            // special case, matched here on purpose since our `to` is
            // always fragment-stripped for that comparison too).
            //
            // For the cross-document case, `make_refnode` computes
            // `get_relative_uri(fromdocname, todocname)` on the *bare*
            // uri and only appends `'#' + targetid` to that *result*
            // afterwards — never feeding the anchor into `relative_uri`
            // itself. That ordering matters: `relative_uri` (both here
            // and upstream) strips any `#fragment` off `to` before
            // comparing paths, so embedding the anchor into `target_uri`
            // first (rather than after) would just have it silently
            // stripped back out again.
            let href = if resolved.docname == docname && !resolved.anchor.is_empty() {
                format!("#{}", resolved.anchor)
            } else {
                let rel = relative_uri(&base_uri, &target_uri);
                if resolved.anchor.is_empty() {
                    rel
                } else {
                    format!("{rel}#{}", resolved.anchor)
                }
            };

            let old_children: Vec<NodeId> = tree.node(id).children.clone();
            for child in old_children {
                tree.detach(child);
            }
            tree.set_kind(
                id,
                NodeKind::Reference {
                    name: String::new(),
                    refuri: href,
                    anonymous: false,
                    classes: "reference internal".to_string(),
                },
            );
            let span = tree.append(
                id,
                NodeKind::Inline {
                    classes: resolved_class,
                },
            );
            tree.append(span, NodeKind::Text(title));
        }
    }

    /// Expands every `docutilsrs::doctree::NodeKind::Toctree` placeholder
    /// node in `tree` into the real HTML5-visible subtree Sphinx renders
    /// inline in the body for a non-hidden `.. toctree::`: a
    /// `<div class="toctree-wrapper compound">` (mirrored here as a
    /// [`NodeKind::Container`]) containing an optional caption paragraph
    /// followed by a nested bullet list of [`NodeKind::Reference`]s,
    /// titled from `env.titles`/`env.longtitles` and linked with
    /// `docname`-relative hrefs (mirrors `sphinx.util.nodes.make_refnode`
    /// via the same `relative_uri`/`get_target_uri` pair
    /// `resolve_xref_nodes` uses).
    ///
    /// Must run after `resolve_xref_nodes` splices in `:ref:`/`:doc:`
    /// links (order doesn't actually matter between the two passes, but
    /// keeping this one second avoids the new nodes it appends being
    /// walked by the other pass's `0..nodes_len()` loop for no reason).
    ///
    /// **Accepted deviation** (see `crate::toctree`'s own module doc):
    /// nested entries are only resolved via `env.toctree_includes`
    /// (i.e. a listed document that itself contains a `.. toctree::`),
    /// never by pulling in a target document's own internal section
    /// structure the way upstream's `TocTree.resolve` does for
    /// `maxdepth` levels beyond 1 when no nested toctree exists.
    pub fn resolve_toctree_nodes(&self, tree: &mut Doctree, docname: &str) {
        use crate::builders::Builder as _;
        use crate::builders::html::HtmlBuilder;

        let builder = HtmlBuilder::new();
        let base_uri = builder.get_target_uri(docname);

        for id in 0..tree.nodes_len() {
            let (caption, maxdepth, hidden, entries) = match &tree.node(id).kind {
                NodeKind::Toctree {
                    caption,
                    maxdepth,
                    hidden,
                    entries,
                } => (caption.clone(), *maxdepth, *hidden, entries.clone()),
                _ => continue,
            };
            if hidden {
                tree.set_kind(id, NodeKind::Comment);
                continue;
            }
            tree.set_kind(
                id,
                NodeKind::Container {
                    classes: "toctree-wrapper compound".to_string(),
                },
            );
            // Entries are written relative to the directory containing
            // *this* document (e.g. `usage/restructuredtext/index.rst`
            // listing `basics` really means `usage/restructuredtext/basics`),
            // matching `scan_toctree_entries`'s own qualification above.
            let entries: Vec<String> = entries
                .into_iter()
                .map(|entry| docname_join(docname, &entry))
                .collect();
            let resolve_depth = if maxdepth <= 0 { 0 } else { maxdepth as usize };
            let toc_depth = if maxdepth <= 0 {
                usize::MAX
            } else {
                maxdepth as usize
            };
            let mut resolved =
                crate::toctree::resolve_from_entries(self, &entries, resolve_depth);
            for entry in &mut resolved {
                Self::enrich_toc_entry_with_sections(self, entry, toc_depth.saturating_sub(1));
            }
            if !resolved.is_empty() {
                if let Some(caption) = caption {
                    let p = tree.append(id, NodeKind::Caption);
                    let span = tree.append(
                        p,
                        NodeKind::Inline {
                            classes: "caption-text".to_string(),
                        },
                    );
                    tree.append(span, NodeKind::Text(caption));
                }
                Self::append_toc_entries(tree, id, &resolved, &builder, &base_uri);
            }
        }
    }

    /// Recursively append `entries` under `parent` as a
    /// `NodeKind::BulletList` of `NodeKind::ListItem`s, each holding a
    /// `NodeKind::Reference` (linked via `docname`-relative href) and,
    /// when the entry has children, a nested `BulletList` after it —
    /// mirrors the `<li><a href="...">Title</a><ul>...</ul></li>` shape
    /// `resolve_toctree_nodes` doc comment describes.
    fn append_toc_entries(
        tree: &mut Doctree,
        parent: NodeId,
        entries: &[crate::toctree::TocEntry],
        builder: &crate::builders::html::HtmlBuilder,
        base_uri: &str,
    ) {
        use crate::builders::Builder as _;
        use crate::util_osutil::relative_uri;

        let list = tree.append(parent, NodeKind::BulletList { bullet: '*' });
        for entry in entries {
            let item = tree.append(list, NodeKind::ListItem);
            let (docname, fragment) = entry.docname.split_once('#').unwrap_or((&entry.docname, ""));
            let mut target_uri = builder.get_target_uri(docname);
            if !fragment.is_empty() {
                target_uri.push('#');
                target_uri.push_str(fragment);
            }
            let fragment = target_uri
                .split_once('#')
                .map(|(_, fragment)| fragment.to_string());
            let mut href = relative_uri(base_uri, &target_uri);
            if let Some(fragment) = fragment {
                if href.is_empty() {
                    href = format!("#{fragment}");
                } else {
                    href.push('#');
                    href.push_str(&fragment);
                }
            }
            let refnode = tree.append(
                item,
                NodeKind::Reference {
                    name: String::new(),
                    refuri: href,
                    anonymous: false,
                    classes: "reference internal".to_string(),
                },
            );
            Self::append_toc_title(tree, refnode, &entry.title);
            if !entry.children.is_empty() {
                Self::append_toc_entries(tree, item, &entry.children, builder, base_uri);
            }
        }
    }

    fn append_toc_title(tree: &mut Doctree, parent: NodeId, title: &str) {
        let mut remaining = title;
        while let Some(start) = remaining.find('\x01') {
            if start > 0 {
                tree.append(parent, NodeKind::Text(remaining[..start].to_string()));
            }
            let literal_start = start + '\x01'.len_utf8();
            let Some(end) = remaining[literal_start..].find('\x02') else {
                tree.append(parent, NodeKind::Text(remaining[start..].to_string()));
                return;
            };
            let literal = tree.append(parent, NodeKind::Literal);
            tree.append(
                literal,
                NodeKind::Text(remaining[literal_start..literal_start + end].to_string()),
            );
            remaining = &remaining[literal_start + end + '\x02'.len_utf8()..];
        }
        if !remaining.is_empty() {
            tree.append(parent, NodeKind::Text(remaining.to_string()));
        }
    }

    fn enrich_toc_entry_with_sections(
        env: &BuildEnvironment,
        entry: &mut crate::toctree::TocEntry,
        remaining_depth: usize,
    ) {
        if remaining_depth == 0 {
            entry.children.clear();
            return;
        }
        if !entry.docname.contains('#') {
            let sections = local_section_entries(env, &entry.docname);
            let nested = std::mem::take(&mut entry.children);
            entry.children = if sections.is_empty() { nested } else { sections };
        }
        for child in &mut entry.children {
            Self::enrich_toc_entry_with_sections(env, child, remaining_depth.saturating_sub(1));
        }
    }

    /// Record that `docname` contains a toctree with `entries` (already
    /// bare docnames, in document order).
    ///
    /// Mirrors `BuildEnvironment.note_toctree`: updates `toctree_includes`
    /// / `toc_num_entries` and marks `docname` as a rebuild-dependent of
    /// every entry (consulted by incremental rebuild, **H8**, to know which
    /// parent page's toctree needs regenerating when a child doc changes).
    pub fn note_toctree(&mut self, docname: impl Into<String>, entries: Vec<String>) {
        let docname = docname.into();
        self.toc_num_entries.insert(docname.clone(), entries.len());
        for entry in &entries {
            self.files_to_rebuild
                .entry(entry.clone())
                .or_default()
                .insert(docname.clone());
        }
        self.toctree_includes.insert(docname, entries);
    }

    // ── H2f: consistency check ────────────────────────────────────────────────

    /// Return a warning for every found document that is not the root
    /// document and is not reachable from any toctree.
    ///
    /// Mirrors `BuildEnvironment.check_consistency`'s
    /// `"document isn't included in any toctree"` warning text.
    pub fn check_consistency(&self) -> Vec<String> {
        let root_doc = self.config.root_doc();
        let mut included: HashSet<&str> = HashSet::new();
        for entries in self.toctree_includes.values() {
            for e in entries {
                included.insert(e.as_str());
            }
        }

        let mut warnings: Vec<String> = self
            .found_docs()
            .iter()
            .filter(|d| d.as_str() != root_doc && !included.contains(d.as_str()))
            .map(|d| {
                format!(
                    "{}: WARNING: document isn't included in any toctree [toc.not_included]",
                    self.doc2path(d).display()
                )
            })
            .collect();
        warnings.sort();
        warnings
    }

    // ── H8a: incremental rebuild — outdated detection ─────────────────────────

    /// Determine which documents need (re-)reading.
    ///
    /// Mirrors `BuildEnvironment.get_outdated_files(config_changed)`,
    /// returning `(added, changed, removed)` docname lists (each sorted).
    ///
    /// - `added`: found but never read before (no `all_docs` entry).
    /// - `changed`: previously read, but `config_changed` is `true`, the
    ///   docname is in [`reread_always`](Self) (mirrors upstream's
    ///   `env.reread_always`, e.g. documents using `today`/`now`), or its
    ///   source file (or any recorded dependency) has a newer mtime than
    ///   its last-read time.
    /// - `removed`: previously read (has an `all_docs` entry) but no
    ///   longer in [`found_docs`](Self::found_docs).
    ///
    /// `config_changed` should be `true` when the caller has determined
    /// the resolved config differs from what was persisted with this env
    /// (compare [`SphinxConfig::stable_hash`](crate::config::SphinxConfig::stable_hash)
    /// against [`EnvPersisted::config_hash`]) — every previously-read
    /// document is then reported as `changed` regardless of mtime,
    /// matching upstream's "config changed -> re-read everything"
    /// fallback.
    ///
    /// **Accepted deviation:** mtime resolution is whatever the
    /// filesystem/OS clock gives (the same source upstream's `os.stat`
    /// uses), not content hashing.
    pub fn get_outdated(&self, config_changed: bool) -> (Vec<String>, Vec<String>, Vec<String>) {
        let found = self.found_docs();
        let mut removed: Vec<String> = self
            .all_docs
            .keys()
            .filter(|d| !found.contains(d.as_str()))
            .cloned()
            .collect();
        removed.sort();

        let mut added = Vec::new();
        let mut changed = Vec::new();

        for docname in found {
            let Some(&last_read) = self.all_docs.get(docname) else {
                added.push(docname.clone());
                continue;
            };
            if config_changed || self.reread_always.contains(docname) {
                changed.push(docname.clone());
                continue;
            }
            let src_path = self.doc2path(docname);
            let mut outdated = mtime_micros(&src_path)
                .map(|m| m > last_read)
                .unwrap_or(true);
            if !outdated {
                if let Some(deps) = self.dependencies.get(docname) {
                    for dep in deps {
                        let dep_path = self.srcdir.join(dep);
                        if mtime_micros(&dep_path)
                            .map(|m| m > last_read)
                            .unwrap_or(true)
                        {
                            outdated = true;
                            break;
                        }
                    }
                }
            }
            if outdated {
                changed.push(docname.clone());
            }
        }
        added.sort();
        changed.sort();
        (added, changed, removed)
    }

    /// Purge every trace of `docname` from this environment (mirrors
    /// `BuildEnvironment.clear_doc`), including its persisted doctree
    /// file. Called for every docname [`get_outdated`](Self::get_outdated)
    /// reports as `removed`.
    pub fn remove_doc(&mut self, docname: &str) {
        use crate::domains::Domain as _;

        self.all_docs.remove(docname);
        self.dependencies.remove(docname);
        self.included.remove(docname);
        self.reread_always.remove(docname);
        self.metadata.remove(docname);
        self.titles.remove(docname);
        self.longtitles.remove(docname);
        self.toc_num_entries.remove(docname);
        self.toc_secnumbers.remove(docname);
        self.toctree_includes.remove(docname);
        self.files_to_rebuild.remove(docname);
        for deps in self.files_to_rebuild.values_mut() {
            deps.remove(docname);
        }
        self.glob_toctrees.remove(docname);
        self.numbered_toctrees.remove(docname);
        self.pending_xrefs.remove(docname);
        self.indexentries.remove(docname);
        self.std_domain.clear_doc(docname);
        self.js_domain.clear_doc(docname);
        self.lean_domain.clear_doc(docname);
        self.py_domain.clear_doc(docname);
        self.rst_domain.clear_doc(docname);
        self.rust_domain.clear_doc(docname);
        self.source_snapshots.remove(docname);
        if let Ok(path) = self.doctree_path(docname) {
            let _ = std::fs::remove_file(path);
        }
    }

    // ── H8b: environment persistence ──────────────────────────────────────────

    /// Snapshot the persistable subset of this environment. See
    /// [`EnvPersisted`] for exactly what is (and isn't) included.
    pub fn to_persisted(&self) -> EnvPersisted {
        EnvPersisted {
            version: ENV_PERSISTED_VERSION,
            config_hash: self.config.stable_hash(),
            all_docs: self.all_docs.clone(),
            dependencies: self.dependencies.clone(),
            included: self.included.clone(),
            reread_always: self.reread_always.clone(),
            metadata: self.metadata.clone(),
            titles: self.titles.clone(),
            longtitles: self.longtitles.clone(),
            toc_num_entries: self.toc_num_entries.clone(),
            toc_secnumbers: self.toc_secnumbers.clone(),
            toctree_includes: self.toctree_includes.clone(),
            files_to_rebuild: self.files_to_rebuild.clone(),
            glob_toctrees: self.glob_toctrees.clone(),
            numbered_toctrees: self.numbered_toctrees.clone(),
            domaindata: self.domaindata.clone(),
            std_domain: self.std_domain.clone(),
            js_domain: self.js_domain.clone(),
            lean_domain: self.lean_domain.clone(),
            py_domain: self.py_domain.clone(),
            rst_domain: self.rst_domain.clone(),
            rust_domain: self.rust_domain.clone(),
            source_snapshots: self.source_snapshots.clone(),
            pending_xrefs: self.pending_xrefs.clone(),
            indexentries: self.indexentries.clone(),
        }
    }

    /// Apply a previously-saved snapshot onto this (freshly-constructed)
    /// environment, overwriting every field [`EnvPersisted`] carries.
    /// Returns the snapshot's `config_hash` so the caller can compare it
    /// against the current config's
    /// [`SphinxConfig::stable_hash`](crate::config::SphinxConfig::stable_hash)
    /// to decide whether a full re-read is needed anyway.
    pub fn apply_persisted(&mut self, p: EnvPersisted) -> u64 {
        let config_hash = p.config_hash;
        self.all_docs = p.all_docs;
        self.dependencies = p.dependencies;
        self.included = p.included;
        self.reread_always = p.reread_always;
        self.metadata = p.metadata;
        self.titles = p.titles;
        self.longtitles = p.longtitles;
        self.toc_num_entries = p.toc_num_entries;
        self.toc_secnumbers = p.toc_secnumbers;
        self.toctree_includes = p.toctree_includes;
        self.files_to_rebuild = p.files_to_rebuild;
        self.glob_toctrees = p.glob_toctrees;
        self.numbered_toctrees = p.numbered_toctrees;
        self.domaindata = p.domaindata;
        self.std_domain = p.std_domain;
        self.rst_domain = p.rst_domain;
        self.py_domain = p.py_domain;
        self.js_domain = p.js_domain;
        self.rust_domain = p.rust_domain;
        self.lean_domain = p.lean_domain;
        self.source_snapshots = p.source_snapshots;
        self.pending_xrefs = p.pending_xrefs;
        self.indexentries = p.indexentries;
        config_hash
    }

    /// Path to the persisted environment snapshot. Mirrors upstream's
    /// `doctreedir/environment.pickle`, using `.json` instead (see
    /// [`EnvPersisted`]'s doc comment for the format deviation).
    pub fn persisted_path(&self) -> PathBuf {
        self.doctreedir.join("environment.json")
    }

    /// Serialize [`to_persisted`](Self::to_persisted) to
    /// [`persisted_path`](Self::persisted_path).
    pub fn save_persisted(&self) -> Result<(), BuildError> {
        let path = self.persisted_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let bytes = serde_json::to_vec(&self.to_persisted())
            .map_err(|e| BuildError::Other(format!("failed to serialize environment: {e}")))?;
        std::fs::write(&path, bytes)?;
        Ok(())
    }

    /// Load a previously-saved environment snapshot, if one exists at
    /// [`persisted_path`](Self::persisted_path) and its
    /// [`EnvPersisted::version`] matches [`ENV_PERSISTED_VERSION`].
    ///
    /// Returns `None` (never an error) for a missing file, an
    /// unreadable/corrupt file, or a version mismatch — all three are
    /// treated identically to upstream's `EnvironmentError` fallback:
    /// "no usable saved environment, start fresh".
    pub fn load_persisted(&self) -> Option<EnvPersisted> {
        let bytes = std::fs::read(self.persisted_path()).ok()?;
        let p: EnvPersisted = serde_json::from_slice(&bytes).ok()?;
        if p.version != ENV_PERSISTED_VERSION {
            return None;
        }
        Some(p)
    }
}

fn shorten_xref_target(target: &str, domain: &str) -> String {
    let separator = match domain {
        "rust" => "::",
        _ => ".",
    };
    target.rsplit(separator).next().unwrap_or(target).to_string()
}

fn local_section_entries(
    env: &BuildEnvironment,
    docname: &str,
) -> Vec<crate::toctree::TocEntry> {
    fn text_content(tree: &Doctree, id: NodeId) -> String {
        match &tree.node(id).kind {
            NodeKind::Text(text) => text.clone(),
            NodeKind::Literal => format!(
                "\x01{}\x02",
                tree.node(id)
                    .children
                    .iter()
                    .map(|&child| text_content(tree, child))
                    .collect::<String>()
            ),
            _ => tree.node(id).children.iter().map(|&child| text_content(tree, child)).collect(),
        }
    }

    fn collect(
        tree: &Doctree,
        parent: NodeId,
        docname: &str,
    ) -> Vec<crate::toctree::TocEntry> {
        tree.node(parent)
            .children
            .iter()
            .filter_map(|&id| {
                let NodeKind::Section { ids, .. } = &tree.node(id).kind else {
                    return None;
                };
                let title = tree.node(id).children.iter().find_map(|&child| {
                    matches!(tree.node(child).kind, NodeKind::Title)
                        .then(|| text_content(tree, child))
                })?;
                Some(crate::toctree::TocEntry {
                    docname: format!("{docname}#{ids}"),
                    title,
                    children: collect(tree, id, docname),
                })
            })
            .collect()
    }

    let Ok(tree) = env.get_doctree(docname) else {
        return Vec::new();
    };
    let mut sections = collect(&tree, tree.root(), docname);
    if sections
        .first()
        .is_some_and(|entry| env.titles.get(docname) == Some(&entry.title))
    {
        let root = sections.remove(0);
        sections = root.children;
    }
    sections
}

fn apply_smartquotes(tree: &mut Doctree) {
    fn visit(tree: &mut Doctree, id: NodeId, literal: bool) {
        let literal = literal
            || matches!(
                tree.node(id).kind,
                NodeKind::Literal
                    | NodeKind::LiteralBlock { .. }
                    | NodeKind::Math { .. }
                    | NodeKind::MathBlock { .. }
            );
        if !literal {
            if let NodeKind::Text(text) = &mut tree.node_mut(id).kind {
                *text = smartquote_text(text);
            }
        }
        let children = tree.node(id).children.clone();
        for child in children {
            visit(tree, child, literal);
        }
    }

    visit(tree, tree.root(), false);
}

fn smartquote_text(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut index = 0;
    while index < chars.len() {
        if index + 2 < chars.len() && chars[index..index + 3] == ['.', '.', '.'] {
            out.push('…');
            index += 3;
            continue;
        }
        if index + 1 < chars.len() && chars[index..index + 2] == ['-', '-'] {
            out.push('—');
            index += 2;
            continue;
        }
        let character = chars[index];
        if character == '\'' || character == '"' {
            let previous = chars.get(index.wrapping_sub(1)).copied();
            let next = chars.get(index + 1).copied();
            let apostrophe = character == '\'' && previous.is_some_and(char::is_alphanumeric)
                && next.is_some_and(char::is_alphanumeric);
            if apostrophe || previous.is_some_and(|value| !value.is_whitespace()) {
                out.push(if character == '\'' { '’' } else { '”' });
            } else if next.is_some_and(|value| !value.is_whitespace()) {
                out.push(if character == '\'' { '‘' } else { '“' });
            } else {
                out.push(if character == '\'' { '’' } else { '”' });
            }
        } else {
            out.push(character);
        }
        index += 1;
    }
    out
}

/// Current [`EnvPersisted::version`]. Bump on any breaking change to that
/// struct's shape so an on-disk file saved by a previous `sphinxdocrs`
/// version is detected as stale (treated as absent) rather than
/// misinterpreted by `serde_json` (which would otherwise silently accept
/// a structurally-compatible-but-semantically-different old file).
pub const ENV_PERSISTED_VERSION: u32 = 3;

/// On-disk snapshot of the parts of [`BuildEnvironment`] that must
/// survive between separate `sphinx-build-rs` invocations for
/// incremental rebuild (**H8a**) to work: which documents were read and
/// when, their titles/toctree structure, and every domain's recovered
/// objects/labels/xrefs. Excludes anything cheaply recomputed every run
/// (`project`, `settings`) or inherently non-serializable (`events`,
/// which holds boxed closures).
///
/// Stored as JSON at `doctreedir/environment.json`
/// ([`BuildEnvironment::persisted_path`]), matching the existing
/// `Doctree::to_bytes`/`from_bytes` (**H2b**) precedent of a versioned
/// serde-json format instead of Python's pickle — an accepted deviation
/// recorded in the port plan (§3, `H8b` row).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvPersisted {
    /// Format version; see [`ENV_PERSISTED_VERSION`].
    pub version: u32,
    /// [`SphinxConfig::stable_hash`](crate::config::SphinxConfig::stable_hash)
    /// at the time this snapshot was saved.
    pub config_hash: u64,
    pub all_docs: HashMap<String, i64>,
    pub dependencies: HashMap<String, HashSet<String>>,
    pub included: HashMap<String, HashSet<String>>,
    pub reread_always: HashSet<String>,
    pub metadata: HashMap<String, HashMap<String, String>>,
    pub titles: HashMap<String, String>,
    pub longtitles: HashMap<String, String>,
    pub toc_num_entries: HashMap<String, usize>,
    pub toc_secnumbers: HashMap<String, HashMap<String, Vec<u32>>>,
    pub toctree_includes: HashMap<String, Vec<String>>,
    pub files_to_rebuild: HashMap<String, HashSet<String>>,
    pub glob_toctrees: HashSet<String>,
    pub numbered_toctrees: HashSet<String>,
    pub domaindata: HashMap<String, HashMap<String, String>>,
    pub std_domain: StdDomain,
    pub js_domain: JsDomain,
    pub lean_domain: LeanDomain,
    pub py_domain: PyDomain,
    pub rst_domain: RstDomain,
    pub rust_domain: RustDomain,
    pub source_snapshots: HashMap<String, AnalysisSnapshot>,
    pub pending_xrefs: HashMap<String, Vec<PendingXref>>,
    pub indexentries: HashMap<String, Vec<IndexEntry>>,
}

/// Always-excluded path patterns, matching upstream `sphinx.project.EXCLUDE_PATHS`.
const PROJECT_EXCLUDE_PATHS: &[&str] = &["**/_sources", ".#*", "**/.#*", "*.lproj/**"];

/// Validate a docname: no empty/`..`/absolute path components. Shared by
/// the doctree store so a malformed or hostile docname can't escape
/// `doctreedir`.
fn sanitize_docname(docname: &str) -> Result<(), BuildError> {
    if docname.is_empty() {
        return Err(BuildError::Other("docname must not be empty".into()));
    }
    for component in docname.split('/') {
        if component.is_empty() || component == ".." || component.starts_with('/') {
            return Err(BuildError::Other(format!(
                "invalid docname component {component:?} in {docname:?}"
            )));
        }
    }
    Ok(())
}

/// Current time in microseconds since the Unix epoch, for `all_docs`
/// read-time bookkeeping.
fn now_micros() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0)
}

/// `path`'s modification time in microseconds since the Unix epoch, for
/// [`BuildEnvironment::get_outdated`]'s mtime comparison. `None` when the
/// file doesn't exist or its mtime can't be read (treated as "always
/// outdated" by the caller, the safe default).
fn mtime_micros(path: &Path) -> Option<i64> {
    std::fs::metadata(path)
        .ok()?
        .modified()
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|d| d.as_micros() as i64)
}

#[derive(Debug, Clone)]
struct YamlTocEntry {
    target: String,
    title: Option<String>,
}

fn yaml_toc_entries(source: &str) -> Result<Vec<YamlTocEntry>, BuildError> {
    let data: serde_yaml::Value = serde_yaml::from_str(source)
        .map_err(|e| BuildError::Other(format!("invalid YAML toctree: {e}")))?;
    let mapping = data
        .as_mapping()
        .ok_or_else(|| BuildError::Other("YAML toctree content must be a mapping".into()))?;
    let mut entries = Vec::new();
    for key in ["root", "chapters", "sections"] {
        if let Some(value) = mapping.get(serde_yaml::Value::String(key.into())) {
            collect_yaml_toc_entries(value, &mut entries);
        }
    }
    if let Some(parts) = mapping.get(serde_yaml::Value::String("parts".into())) {
        if let Some(parts) = parts.as_sequence() {
            for part in parts {
                if let Some(part_mapping) = part.as_mapping() {
                    for key in ["chapters", "sections"] {
                        if let Some(value) = part_mapping.get(serde_yaml::Value::String(key.into()))
                        {
                            collect_yaml_toc_entries(value, &mut entries);
                        }
                    }
                }
            }
        }
    }
    Ok(entries)
}

fn collect_yaml_toc_entries(value: &serde_yaml::Value, entries: &mut Vec<YamlTocEntry>) {
    if let Some(target) = value.as_str() {
        entries.push(YamlTocEntry {
            target: target.into(),
            title: None,
        });
        return;
    }
    if let Some(values) = value.as_sequence() {
        for value in values {
            collect_yaml_toc_entries(value, entries);
        }
        return;
    }
    let Some(mapping) = value.as_mapping() else {
        return;
    };
    if mapping
        .get(serde_yaml::Value::String("build".into()))
        .and_then(serde_yaml::Value::as_bool)
        == Some(false)
    {
        return;
    }
    let target = mapping
        .get(serde_yaml::Value::String("file".into()))
        .or_else(|| mapping.get(serde_yaml::Value::String("url".into())))
        .and_then(serde_yaml::Value::as_str);
    if let Some(target) = target {
        entries.push(YamlTocEntry {
            target: target.into(),
            title: mapping
                .get(serde_yaml::Value::String("title".into()))
                .and_then(serde_yaml::Value::as_str)
                .map(String::from),
        });
    }
    for key in ["sections", "subsections"] {
        if let Some(value) = mapping.get(serde_yaml::Value::String(key.into())) {
            collect_yaml_toc_entries(value, entries);
        }
    }
}

fn yaml_docname(target: &str) -> Option<String> {
    if target.starts_with('/') || target.contains("://") {
        return None;
    }
    [".ipynb", ".rst", ".md", ".txt"]
        .iter()
        .find_map(|suffix| target.strip_suffix(suffix))
        .or(Some(target))
        .map(|target| target.trim_start_matches("./").to_string())
}

fn yaml_toc_root_and_children(
    source: &str,
    default_root: &str,
) -> Result<Option<(String, Vec<String>)>, BuildError> {
    let data: serde_yaml::Value = serde_yaml::from_str(source)
        .map_err(|e| BuildError::Other(format!("invalid YAML toctree: {e}")))?;
    let has_root = data
        .as_mapping()
        .and_then(|mapping| mapping.get(serde_yaml::Value::String("root".into())))
        .is_some();
    let entries = yaml_toc_entries(source)?;
    if entries.is_empty() {
        return Ok(None);
    }
    let (root, children) = if has_root {
        let Some(root) = yaml_docname(&entries[0].target) else {
            return Ok(None);
        };
        (root, &entries[1..])
    } else {
        (default_root.to_string(), &entries[..])
    };
    let children = children
        .iter()
        .filter_map(|entry| yaml_docname(&entry.target))
        .collect();
    Ok(Some((root, children)))
}

fn yaml_option_lines(source: &str) -> Result<Vec<String>, BuildError> {
    let data: serde_yaml::Value = serde_yaml::from_str(source)
        .map_err(|e| BuildError::Other(format!("invalid YAML toctree: {e}")))?;
    let Some(mapping) = data.as_mapping() else {
        return Ok(Vec::new());
    };
    let mut lines = Vec::new();
    for options in [
        Some(mapping),
        mapping
            .get(serde_yaml::Value::String("options".into()))
            .and_then(serde_yaml::Value::as_mapping),
    ]
    .into_iter()
    .flatten()
    {
        for key in [
            "maxdepth",
            "caption",
            "glob",
            "hidden",
            "includehidden",
            "numbered",
            "titlesonly",
            "reversed",
        ] {
            let Some(value) = options.get(serde_yaml::Value::String(key.into())) else {
                continue;
            };
            if key != "maxdepth" && key != "caption" && value.as_bool() != Some(true) {
                continue;
            }
            let rendered = value
                .as_str()
                .map(String::from)
                .or_else(|| value.as_i64().map(|number| number.to_string()))
                .unwrap_or_else(|| "".into());
            lines.push(if rendered.is_empty() {
                format!(":{key}:")
            } else {
                format!(":{key}: {rendered}")
            });
        }
    }
    Ok(lines)
}

fn dedent_yaml(lines: &[&str]) -> String {
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| line.get(indent..).unwrap_or("").to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn expand_yaml_toctree_directives(source: &str, srcdir: &Path) -> Result<String, BuildError> {
    let lines: Vec<&str> = source.lines().collect();
    let mut output = String::new();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        let Some(argument) = [".. toctreeyml::", ".. toctreeyaml::"]
            .iter()
            .find_map(|prefix| trimmed.strip_prefix(prefix))
        else {
            output.push_str(line);
            output.push('\n');
            index += 1;
            continue;
        };

        let body_start = index + 1;
        let mut body_end = body_start;
        while body_end < lines.len() {
            let body_line = lines[body_end];
            let body_indent = body_line.len() - body_line.trim_start().len();
            if !body_line.trim().is_empty() && body_indent <= indent {
                break;
            }
            body_end += 1;
        }
        let body = dedent_yaml(&lines[body_start..body_end]);
        let argument = argument.trim();
        let toc_source = if !argument.is_empty() {
            let path = srcdir.join(argument);
            std::fs::read_to_string(&path).map_err(|e| {
                BuildError::Other(format!(
                    "failed to read YAML toctree {}: {e}",
                    path.display()
                ))
            })?
        } else if !body.trim().is_empty() {
            body
        } else {
            let path = srcdir.join("_toc.yml");
            std::fs::read_to_string(&path).map_err(|e| {
                BuildError::Other(format!(
                    "failed to read YAML toctree {}: {e}",
                    path.display()
                ))
            })?
        };
        let entries = yaml_toc_entries(&toc_source)?;
        let prefix = &line[..indent];
        output.push_str(prefix);
        output.push_str(".. toctree::\n");
        for option in yaml_option_lines(&toc_source)? {
            output.push_str(prefix);
            output.push_str("   ");
            output.push_str(&option);
            output.push('\n');
        }
        output.push('\n');
        for entry in entries {
            output.push_str(prefix);
            output.push_str("   ");
            if let Some(title) = entry.title {
                output.push_str(&title);
                output.push_str(" <");
                output.push_str(&entry.target);
                output.push_str(">\n");
            } else {
                output.push_str(&entry.target);
                output.push('\n');
            }
        }
        index = body_end;
    }
    Ok(output)
}

/// Resolve a `.. toctree::` entry written relative to the directory
/// containing `base` into a full, project-root-relative docname.
///
/// Mirrors `sphinx.util.docname_join(base, other)`
/// (`posixpath.normpath(posixpath.join('#' + base, '..', other))[1:]`):
/// entries are relative to `base`'s *parent directory*, not the project
/// root, so `docname_join("tutorial/index", "getting-started")` is
/// `"tutorial/getting-started"` while `docname_join("index",
/// "usage/installation")` stays `"usage/installation"`. `..`/`.` segments
/// in `other` are normalized against `base`'s directory the same way.
pub(crate) fn docname_join(base: &str, other: &str) -> String {
    let mut segments: Vec<&str> = match base.rfind('/') {
        Some(idx) => base[..idx].split('/').collect(),
        None => Vec::new(),
    };
    for seg in other.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            seg => segments.push(seg),
        }
    }
    segments.join("/")
}

/// Recursive parent->child walk collecting every node reachable from
/// `id` (`id` itself first, then its subtree, pre-order) — the same
/// order [`BuildEnvironment::resolve_xref_nodes`]'s `0..nodes_len()`
/// forward pass visits, for a tree freshly parsed and never mutated.
/// Kept as a standalone helper (rather than folded back into
/// `resolve_xref_nodes`, which uses the cheaper `0..nodes_len()` range
/// directly — see that method's own doc comment for why) documenting
/// and test-verifying the difference between the two: unlike a raw
/// `0..nodes_len()` range, this only returns nodes still attached to the
/// tree, skipping detached/orphaned arena slots (e.g. old `Text`
/// children a prior `resolve_xref_nodes` pass left behind via
/// `Doctree::detach`). Test-only (see `tests::collect_ids_*` below) —
/// `#[cfg(test)]` rather than `#[allow(dead_code)]` since it has no
/// production caller.
#[cfg(test)]
fn collect_ids(tree: &Doctree, id: NodeId, out: &mut Vec<NodeId>) {
    out.push(id);
    for &child in &tree.node(id).children {
        collect_ids(tree, child, out);
    }
}

/// Directive names whose body is *not* recursively parsed as
/// reStructuredText by real docutils — source code / math text is taken
/// verbatim, so a nested-looking `.. toctree::`/`.. include::` shown
/// *inside* one of these (e.g. an example snippet in a
/// `.. code-block:: rst`) is just text, not a real directive.
const OPAQUE_LITERAL_DIRECTIVES: &[&str] =
    &["code-block", "code", "sourcecode", "math", "literalinclude"];

/// Blanks out (line-count-preserving, so this remains safe to run before
/// any line-indexed scan) every code-block/literal-block body in
/// `source`, so [`scan_toctree_entries`]/[`scan_include_entries`] never
/// mistake an *illustrative* directive shown inside one for a real one.
/// Handles both explicit opaque directives (`.. code-block:: rst`, ...)
/// and the plain `::` paragraph-literal-block marker docutils recognizes.
fn strip_opaque_literal_blocks(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut out: Vec<&str> = vec![""; lines.len()];
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();

        let is_opaque_directive = trimmed.strip_prefix("..").is_some_and(|rest| {
            let rest = rest.trim_start();
            OPAQUE_LITERAL_DIRECTIVES.iter().any(|name| {
                rest.strip_prefix(name)
                    .is_some_and(|r| r.trim_start().starts_with("::"))
            })
        });
        // A paragraph ending in `::` (and not itself a directive line)
        // also introduces a literal block for its following indented
        // lines — the standard rst "expanded marker" literal-block form.
        let is_literal_marker = !trimmed.starts_with("..") && trimmed.ends_with("::");

        if is_opaque_directive || is_literal_marker {
            out[i] = line;
            i += 1;
            while i < lines.len() && lines[i].trim().is_empty() {
                i += 1;
            }
            while i < lines.len() {
                let body_line = lines[i];
                if body_line.trim().is_empty() {
                    i += 1;
                    continue;
                }
                let body_indent = body_line.len() - body_line.trim_start().len();
                if body_indent <= indent {
                    break;
                }
                i += 1;
            }
            continue;
        }

        out[i] = line;
        i += 1;
    }
    out.join("\n")
}

/// Very small text-level scan for `.. toctree::` directive bodies,
/// extracting the listed docnames.
///
/// `docutilsrs`'s parser does not yet produce a structural toctree node
/// (planned alongside the domain work in **H3a**), so the read phase
/// recovers entries directly from source text: after a `.. toctree::`
/// line, every non-blank line indented further than the directive is
/// treated as an entry, except option lines (`:maxdepth:`, `:glob:`,
/// etc.) which start with `:`. Explicit `Title <docname>` entries retain
/// only the target, matching the native toctree resolver's data model.
fn scan_toctree_entries(source: &str) -> Vec<String> {
    scan_toctree_entries_with_titles(source)
        .into_iter()
        .map(|(entry, _)| entry)
        .collect()
}

fn scan_toctree_entries_with_titles(source: &str) -> Vec<(String, Option<String>)> {
    scan_toctree_entries_with_titles_mode(source, true)
}

fn scan_toctree_entries_preserving_suffixes(source: &str) -> Vec<String> {
    scan_toctree_entries_with_titles_mode(source, false)
        .into_iter()
        .map(|(entry, _)| entry)
        .collect()
}

fn scan_toctree_entries_with_titles_mode(
    source: &str,
    strip_source_suffixes: bool,
) -> Vec<(String, Option<String>)> {
    let mut entries = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        if let Some(rest) = trimmed.strip_prefix("..") {
            let rest = rest.trim_start();
            if rest.starts_with("toctree::") {
                let indent = lines[i].len() - trimmed.len();
                let mut directive_entries = Vec::new();
                let mut hidden = false;
                i += 1;
                while i < lines.len() {
                    let line = lines[i];
                    if line.trim().is_empty() {
                        i += 1;
                        continue;
                    }
                    let line_indent = line.len() - line.trim_start().len();
                    if line_indent <= indent {
                        break;
                    }
                    let body = line.trim();
                    if body == ":hidden:" {
                        hidden = true;
                    } else if !body.starts_with(':') {
                        let (target, title) = body
                            .strip_suffix('>')
                            .and_then(|body| {
                                body.rsplit_once(" <")
                                    .map(|(title, target)| (target, Some(title.to_string())))
                            })
                            .unwrap_or((body, None));
                        let target = if strip_source_suffixes {
                            [".ipynb", ".rst", ".md", ".txt"]
                                .iter()
                                .find_map(|suffix| target.strip_suffix(suffix))
                                .unwrap_or(target)
                        } else {
                            target
                        };
                        directive_entries.push((target.to_string(), title));
                    }
                    i += 1;
                }
                if !hidden {
                    entries.extend(directive_entries);
                }
                continue;
            }
        }
        i += 1;
    }
    entries
}

/// Text-level scan for `.. include:: <path>` directives, used to record
/// extra read-phase dependencies (`env.dependencies`).
fn scan_include_entries(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|l| {
            l.trim_start()
                .strip_prefix(".. include::")
                .map(|rest| rest.trim().to_string())
        })
        .filter(|s| !s.is_empty())
        .collect()
}

fn expand_rst_includes(source: &str, base_dir: &Path, encoding: &str) -> String {
    fn expand(source: &str, base_dir: &Path, encoding: &str, depth: usize) -> String {
        if depth >= 16 {
            return source.to_string();
        }
        let mut out = Vec::new();
        let mut lines = source.lines().peekable();
        while let Some(line) = lines.next() {
            let trimmed = line.trim_start();
            let Some(path) = trimmed.strip_prefix(".. include::").map(str::trim) else {
                out.push(line.to_string());
                continue;
            };
            if path.is_empty() {
                out.push(line.to_string());
                continue;
            }
            let include_path = base_dir.join(path);
            let Ok(included) = read_source_file(&include_path, encoding) else {
                out.push(line.to_string());
                continue;
            };
            let indent = &line[..line.len() - trimmed.len()];
            let expanded = expand(
                &included,
                include_path.parent().unwrap_or(base_dir),
                encoding,
                depth + 1,
            );
            for included_line in expanded.lines() {
                if included_line.is_empty() {
                    out.push(String::new());
                } else {
                    out.push(format!("{indent}{included_line}"));
                }
            }
            if expanded.is_empty() && lines.peek().is_none() {
                out.push(String::new());
            }
        }
        out.join("\n")
    }

    expand(source, base_dir, encoding, 0)
}

fn apply_module_section_ids(tree: &mut Doctree, source: &str) {
    let noindex_modules = source
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim_start();
            let name = trimmed
                .strip_prefix(".. module::")
                .or_else(|| trimmed.strip_prefix(".. py:module::"))?
                .trim()
                .to_string();
            if name.is_empty() {
                return None;
            }
            let noindex = source
                .lines()
                .skip(index + 1)
                .take_while(|option| option.trim().is_empty() || option.starts_with(' '))
                .any(|option| option.trim() == ":noindex:");
            noindex.then_some(name)
        })
        .collect::<HashSet<_>>();

    fn module_name(tree: &Doctree, id: NodeId) -> Option<String> {
        match &tree.node(id).kind {
            NodeKind::ObjectDescription { classes, sig_text, .. }
                if classes.split_whitespace().any(|class| class == "module") => sig_text
                    .split_once(' ')
                    .map(|(_, name)| name.trim().to_string())
                    .filter(|name| !name.is_empty()),
            _ => None,
        }
    }

    fn visit(
        tree: &mut Doctree,
        parent: NodeId,
        active_module: Option<String>,
        noindex_modules: &HashSet<String>,
    ) {
        let children = tree.node(parent).children.clone();
        let mut active_module = active_module;
        for child in children {
            if matches!(tree.node(child).kind, NodeKind::Section { .. }) {
                if let Some(module) = &active_module {
                    if let NodeKind::Section { ids, .. } = &mut tree.node_mut(child).kind {
                        *ids = format!("module-{module}");
                    }
                }
                if let Some(module) = tree.node(child).children.iter().find_map(|&id| {
                    module_name(tree, id).filter(|name| !noindex_modules.contains(name))
                }) {
                    if let NodeKind::Section { ids, .. } = &mut tree.node_mut(child).kind {
                        *ids = format!("module-{module}");
                    }
                }
                visit(tree, child, None, noindex_modules);
                active_module = None;
            } else if let Some(module) = module_name(tree, child) {
                if !noindex_modules.contains(&module) {
                    active_module = Some(module);
                }
            } else {
                active_module = None;
            }
        }
    }

    visit(tree, tree.root(), None, &noindex_modules);
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SphinxConfig;
    use crate::source_analysis::{
        DeclarationKind, SourceAnalysisRequest, SourceDeclaration, SourcePosition, SourceSpan,
        Visibility,
    };

    fn make_env() -> BuildEnvironment {
        let config = SphinxConfig::new_defaults();
        let project = EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]);
        BuildEnvironment::new(config, project, "/tmp/src", "/tmp/doctrees")
    }

    fn source_snapshot() -> AnalysisSnapshot {
        let declaration = SourceDeclaration::new(
            SourceLanguage::Rust,
            "test-backend",
            "crate::api::answer",
            "::",
            DeclarationKind::Function,
            Some("answer() -> i32".to_owned()),
            Some("i32".to_owned()),
            "Returns the answer.",
            Visibility::Public,
            SourceSpan::new(
                "src/lib.rs",
                SourcePosition { byte: 0, line: 1, column: 0 },
                None,
            ),
        );
        AnalysisSnapshot::new(
            SourceLanguage::Rust,
            "test-backend",
            "1",
            "/tmp/src",
            "hash",
            vec![declaration],
            Vec::new(),
        )
    }

    #[test]
    fn new_env_config_status_unset() {
        let env = make_env();
        assert_eq!(env.config_status, CONFIG_UNSET);
    }

    #[test]
    fn new_env_all_docs_empty() {
        let env = make_env();
        assert!(env.all_docs.is_empty());
    }

    #[test]
    fn source_snapshots_feed_domains_and_persist_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let srcdir = dir.path().join("src");
        let doctreedir = dir.path().join("doctrees");
        std::fs::create_dir_all(&srcdir).unwrap();
        let project = EnvProject::new(&srcdir, &[(".rst", "restructuredtext")]);
        let mut env = BuildEnvironment::new(
            SphinxConfig::new_defaults(),
            project.clone(),
            &srcdir,
            &doctreedir,
        );
        let request = SourceAnalysisRequest::new(&srcdir);
        let json_path = srcdir.join("fixture.json");
        std::fs::write(&json_path, b"initial source input").unwrap();
        let mut request = request;
        request.selected.push(json_path.clone());
        let mut snapshot = source_snapshot();
        snapshot.source_root = srcdir.to_string_lossy().into_owned();
        snapshot.source_hash = source_input_hash(&request, SourceLanguage::Rust).unwrap();
        snapshot.set_request_identity(&request);
        env.note_source_snapshot("api", snapshot).unwrap();
        assert_eq!(env.rust_domain.source_objects().len(), 1);
        assert_eq!(env.source_snapshots.len(), 1);
        assert_eq!(env.domaindata["rust"]["backend"], "test-backend");
        assert!(env.source_snapshot_matches_request("api", &request));
        let mut changed_request = request.clone();
        changed_request.include_private = true;
        assert!(!env.source_snapshot_matches_request("api", &changed_request));

        let persisted = env.to_persisted();
        let mut restored = BuildEnvironment::new(
            SphinxConfig::new_defaults(),
            project,
            &srcdir,
            &doctreedir,
        );
        restored.apply_persisted(persisted);
        assert_eq!(restored.rust_domain.source_objects().len(), 1);
        assert!(restored.source_snapshots.contains_key("api"));
        assert!(restored.source_snapshot_matches_request("api", &request));

        std::fs::write(&json_path, b"changed source input").unwrap();
        assert!(!restored.source_snapshot_matches_request("api", &request));

        restored.remove_doc("api");
        assert!(restored.rust_domain.source_objects().is_empty());
        assert!(restored.source_snapshots.is_empty());
    }

    #[cfg(feature = "rust-source-analysis")]
    #[test]
    fn provider_cache_identity_preserves_request_identity_and_toolchain_invalidation() {
        use crate::source_analysis::rust::RustdocJsonAnalyzer;

        let fixture_root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/rust");
        let json_path = fixture_root.parent().unwrap().join("rustdoc.json");
        let mut request = SourceAnalysisRequest::new(&fixture_root);
        request.selected.push(json_path);
        let analyzer = RustdocJsonAnalyzer::new(Some("fixture-toolchain-a".into()));
        let mut env = make_env();

        env.analyze_source("api", &analyzer, &request).unwrap();

        let snapshot = env.source_snapshots.get("api").unwrap();
        assert_eq!(snapshot.request_identity, request.cache_identity());
        assert_eq!(snapshot.provider_identity, analyzer.cache_identity(&request));
        assert_eq!(
            env.domaindata["rust"]["provider_identity"],
            analyzer.cache_identity(&request)
        );
        assert!(env.source_snapshot_matches_request("api", &request));
        assert!(env.source_snapshot_matches_provider("api", &analyzer, &request));
        let changed_toolchain = RustdocJsonAnalyzer::new(Some("fixture-toolchain-b".into()));
        assert!(!env.source_snapshot_matches_provider("api", &changed_toolchain, &request));

        let mut restored = make_env();
        restored.apply_persisted(env.to_persisted());
        assert!(restored.source_snapshot_matches_request("api", &request));
        assert!(restored.source_snapshot_matches_provider("api", &analyzer, &request));
        assert!(!restored.source_snapshot_matches_provider("api", &changed_toolchain, &request));
    }

    #[cfg(all(feature = "rust-source-analysis", feature = "lsp-source-analysis", unix))]
    #[test]
    fn cold_lsp_provider_initializes_identity_before_persisted_cache_match() {
        use crate::source_docs::lsp_backend::{LspServerConfig, LspSnapshotProvider};
        use crate::source_analysis::SourceLanguage;

        let directory = tempfile::tempdir().unwrap();
        let workspace = directory.path().join("workspace");
        let doctreedir = directory.path().join("doctrees");
        std::fs::create_dir_all(&workspace).unwrap();
        let source = workspace.join("lib.rs");
        std::fs::write(&source, "pub fn answer() -> i32 { 42 }\n").unwrap();
        let fake_server =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/fake_lsp.py");
        let config = LspServerConfig {
            command: vec![
                "python3".into(),
                fake_server.to_string_lossy().into_owned(),
                "symbols".into(),
            ],
            workspace_root: workspace.clone(),
            language: SourceLanguage::Rust,
            request_timeout: std::time::Duration::from_secs(10),
            allow_fallback: false,
            environment: Vec::new(),
            max_message_bytes: 1024 * 1024,
        };
        let request = {
            let mut request = SourceAnalysisRequest::new(&workspace);
            request.selected.push(source);
            request
        };
        let project = EnvProject::new(&workspace, &[(".rst", "restructuredtext")]);
        let mut env = BuildEnvironment::new(
            SphinxConfig::new_defaults(),
            project.clone(),
            &workspace,
            &doctreedir,
        );
        let original_provider = LspSnapshotProvider::new(config.clone());
        env.analyze_source("api", &original_provider, &request).unwrap();
        let persisted = env.to_persisted();
        let mut restored = BuildEnvironment::new(
            SphinxConfig::new_defaults(),
            project,
            &workspace,
            &doctreedir,
        );
        restored.apply_persisted(persisted);

        let cold_provider = LspSnapshotProvider::new(config);
        assert!(restored.source_snapshot_matches_provider("api", &cold_provider, &request));
    }

    #[test]
    fn module_directive_assigns_following_section_anchor() {
        let env = make_env();
        let tree = env
            .parse_source(
                "sandbox",
                ".. module:: jinja2.seccomp\n\nSeccomp Filtering API\n~~~~~~~~~~~~~~~~~~~~~\n",
            )
            .unwrap();
        let section = tree
            .node(tree.root())
            .children
            .iter()
            .find(|&&id| matches!(tree.node(id).kind, NodeKind::Section { .. }))
            .copied()
            .expect("section node");
        assert!(matches!(
            tree.node(section).kind,
            NodeKind::Section { ref ids, .. } if ids == "module-jinja2.seccomp"
        ));
    }

    #[test]
    fn automodule_docstring_code_block_is_highlighted() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("demo.py"),
            "\"\"\"Demo.\n\n.. code-block:: python\n\n   def f():\n       return 1\n\"\"\"\n",
        )
        .unwrap();
        let project = EnvProject::new(dir.path(), &[(".rst", "restructuredtext")]);
        let mut env = BuildEnvironment::new(
            SphinxConfig::new_defaults(),
            project,
            dir.path(),
            dir.path().join("doctrees"),
        );

        env.read_one_with_source("index", ".. automodule:: demo\n")
            .unwrap();
        let tree = env.get_doctree("index").unwrap();
        assert!((0..tree.nodes_len()).any(|id| {
            matches!(
                &tree.node(id).kind,
                NodeKind::Inline { classes } if classes == "k"
            )
        }));
    }

    #[test]
    fn expand_autodoc_handles_missing_modules_and_package_modules() {
        let dir = tempfile::tempdir().unwrap();
        let project = EnvProject::new(dir.path(), &[(".rst", "restructuredtext")]);
        let env = BuildEnvironment::new(
            SphinxConfig::new_defaults(),
            project,
            dir.path(),
            dir.path().join("doctrees"),
        );

        assert!(env.expand_autodoc(".. automodule:: missing\n").is_none());

        std::fs::create_dir(dir.path().join("package")).unwrap();
        std::fs::write(
            dir.path().join("package/__init__.py"),
            "\"\"\"Package docs.\"\"\"\n\ndef answer():\n    return 42\n",
        )
        .unwrap();
        let expanded = env
            .expand_autodoc(".. automodule:: package\n   :members:\n\n")
            .expect("package automodule should expand");
        assert!(expanded.contains("Package docs."));
    }

    #[test]
    fn document_title_prefers_document_title_and_skips_empty_titles() {
        let mut tree = Doctree::new_document("title");
        tree.set_kind(
            tree.root(),
            NodeKind::Document {
                source: String::new(),
                ids: String::new(),
                names: String::new(),
                title: "Document title".into(),
            },
        );
        assert_eq!(
            BuildEnvironment::document_title_from_tree(&tree).as_deref(),
            Some("Document title")
        );

        let mut empty = Doctree::new_document("empty");
        let section = empty.append(empty.root(), NodeKind::Section {
            ids: String::new(),
            names: String::new(),
            classes: String::new(),
        });
        empty.append(section, NodeKind::Paragraph);
        assert_eq!(BuildEnvironment::document_title_from_tree(&empty), None);
    }

    #[test]
    fn highlight_directive_sets_language_for_unlabeled_code_block() {
        let mut env = make_env();
        env.read_one_with_source(
            "index",
            ".. highlight:: python\n\n.. code-block::\n\n   return 1\n",
        )
        .unwrap();
        let tree = env.get_doctree("index").unwrap();
        assert!((0..tree.nodes_len()).any(|id| {
            matches!(
                &tree.node(id).kind,
                NodeKind::Inline { classes } if classes == "k"
            )
        }));
    }

    #[test]
    fn highlighting_matrix_produces_token_classes_for_representative_languages() {
        let cases = [
            ("python", "def answer():\n    return 42"),
            ("javascript", "function answer() { return 42; }"),
            ("rust", "fn answer() -> i32 { 42 }"),
            ("json", "{\"answer\": 42}"),
            ("html", "<p>answer</p>"),
            ("css", "body { color: red; }"),
            ("bash", "echo answer"),
            ("sql", "SELECT 42;"),
            ("yaml", "answer: 42"),
            ("markdown", "**answer**"),
        ];

        for (language, code) in cases {
            let mut env = make_env();
            let source = format!(".. code-block:: {language}\n\n   {code}\n");
            env.read_one_with_source(language, &source).unwrap();
            let tree = env.get_doctree(language).unwrap();
            assert!(
                (0..tree.nodes_len()).any(|id| matches!(
                    &tree.node(id).kind,
                    NodeKind::Inline { classes } if !classes.is_empty()
                )),
                "no token classes produced for {language}"
            );
        }
    }

    #[test]
    fn highlight_state_applies_until_replaced_and_preserves_trailing_newline() {
        let env = make_env();
        let rewritten = env
            .apply_highlight_language(
                ".. highlight:: python\n\n.. code-block::\n\n   return 1\n\n.. highlight:: rust\n\n.. code-block::\n\n   fn main() {}\n",
            )
            .unwrap();
        assert!(rewritten.contains(".. code-block:: python"));
        assert!(rewritten.contains(".. code-block:: rust"));
        assert!(rewritten.ends_with('\n'));
    }

    #[test]
    fn highlight_state_ignores_nested_directive_examples() {
        let env = make_env();
        let rewritten = env
            .apply_highlight_language(
                ".. highlight:: python\n\n.. code-block:: rst\n\n   .. code-block::\n\n      return 1\n",
            )
            .unwrap();
        assert!(rewritten.contains("   .. code-block::\n"));
    }

    #[test]
    fn highlight_state_handles_none_and_noop_inputs() {
        let env = make_env();
        assert!(env.apply_highlight_language("Plain text").is_none());

        let rewritten = env
            .apply_highlight_language(".. highlight:: none\n\n.. code-block::\n\n   text")
            .unwrap();
        assert!(rewritten.contains(".. code-block::\n"));
        assert!(!rewritten.contains(".. code-block:: none"));
    }

    #[test]
    fn parse_source_rejects_invalid_notebooks() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.project
            .docname_to_path
            .insert("broken".into(), "broken.ipynb".into());
        let error = env.parse_source("broken", "not json").unwrap_err().to_string();
        assert!(error.contains("invalid notebook"));
    }

    #[test]
    fn read_one_reports_missing_source() {
        let (_tmp, mut env) = make_env_with_tempdir();
        let error = env.read_one("missing").unwrap_err().to_string();
        assert!(error.contains("failed to read"));
    }

    #[test]
    fn read_docs_with_empty_selection_keeps_existing_state() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.read_one_with_source("index", "Index\n=====\n").unwrap();
        let read = env.read_docs(Vec::new(), None).unwrap();
        assert!(read.is_empty());
        assert!(env.is_doc_read("index"));
        assert!(env.has_stored_doctree("index"));
    }

    #[test]
    fn record_doc_read() {
        let mut env = make_env();
        env.record_doc_read("index", 1_000_000);
        assert!(env.is_doc_read("index"));
        assert!(!env.is_doc_read("other"));
    }

    #[test]
    fn set_and_get_title() {
        let mut env = make_env();
        env.set_title("index", "Welcome");
        assert_eq!(env.get_title("index"), Some("Welcome"));
        assert!(env.get_title("missing").is_none());
    }

    #[test]
    fn note_dependency() {
        let mut env = make_env();
        env.note_dependency("index", "api/module.rst");
        assert!(env.dependencies["index"].contains("api/module.rst"));
    }

    #[test]
    fn clear_temp_data() {
        let mut env = make_env();
        env.temp_data.insert("key".into(), "val".into());
        env.ref_context.insert("module".into(), "os".into());
        env.clear_temp_data();
        assert!(env.temp_data.is_empty());
        assert!(env.ref_context.is_empty());
    }

    #[test]
    fn config_status_label_unset() {
        let env = make_env();
        assert_eq!(env.config_status_label(), "config unset");
    }

    #[test]
    fn set_config_status() {
        let mut env = make_env();
        env.set_config_status(CONFIG_NEW, "new config");
        assert_eq!(env.config_status, CONFIG_NEW);
        assert_eq!(env.config_status_label(), "new config");
    }

    #[test]
    fn config_status_changed() {
        let mut env = make_env();
        env.set_config_status(CONFIG_CHANGED, "");
        assert_eq!(env.config_status_label(), "config changed");
    }

    #[test]
    fn default_settings_has_halt_level() {
        let settings = default_settings();
        assert_eq!(settings.get("halt_level").map(String::as_str), Some("5"));
    }

    #[test]
    fn default_settings_has_input_encoding() {
        let settings = default_settings();
        assert_eq!(
            settings.get("input_encoding").map(String::as_str),
            Some("utf-8-sig")
        );
    }

    #[test]
    fn parse_doc_accepts_latin1_source() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("index.rst"), b"Caf\xFC\n====\n").unwrap();
        let project = EnvProject::new(dir.path(), &[(".rst", "restructuredtext")]);
        let mut raw_config = HashMap::new();
        raw_config.insert(
            "source_encoding".to_string(),
            crate::config::ConfigVal::Str("latin-1".to_string()),
        );
        let env = BuildEnvironment::new(
            SphinxConfig::new(raw_config, HashMap::new()),
            project,
            dir.path(),
            dir.path().join("doctrees"),
        );

        let tree = env.parse_doc("index").unwrap();
        assert!(
            tree.node(tree.root())
                .children
                .iter()
                .any(|&id| matches!(tree.node(id).kind, NodeKind::Section { .. }))
        );
    }

    #[test]
    fn srcdir_and_doctreedir() {
        let env = make_env();
        assert_eq!(env.srcdir, PathBuf::from("/tmp/src"));
        assert_eq!(env.doctreedir, PathBuf::from("/tmp/doctrees"));
    }

    #[test]
    fn toc_num_entries_starts_empty() {
        let env = make_env();
        assert!(env.toc_num_entries.is_empty());
    }

    #[test]
    fn glob_toctrees_starts_empty() {
        let env = make_env();
        assert!(env.glob_toctrees.is_empty());
    }

    #[test]
    fn domaindata_starts_empty() {
        let env = make_env();
        assert!(env.domaindata.is_empty());
    }

    // ── H8a: get_outdated / remove_doc ────────────────────────────────────────

    fn make_env_with_tempdir() -> (tempfile::TempDir, BuildEnvironment) {
        let tmp = tempfile::TempDir::new().unwrap();
        let srcdir = tmp.path().join("src");
        let doctreedir = tmp.path().join("doctrees");
        std::fs::create_dir_all(&srcdir).unwrap();
        std::fs::create_dir_all(&doctreedir).unwrap();
        let config = SphinxConfig::new_defaults();
        let project = EnvProject::new(&srcdir, &[(".rst", "restructuredtext")]);
        let env = BuildEnvironment::new(config, project, &srcdir, &doctreedir);
        (tmp, env)
    }

    #[test]
    fn find_files_prefers_longest_suffix_and_skips_unknown_files() {
        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(env.srcdir.join("guide.rst.txt"), "Guide\n=====\n").unwrap();
        std::fs::write(env.srcdir.join("guide.txt"), "Duplicate\n=========\n").unwrap();
        std::fs::write(env.srcdir.join("ignored.md"), "ignored").unwrap();
        env.config.set(
            "source_suffix",
            ConfigVal::Map(vec![
                (".txt".into(), ConfigVal::Str("restructuredtext".into())),
                (".rst.txt".into(), ConfigVal::Str("myst".into())),
            ]),
        );

        env.find_files().unwrap();

        assert_eq!(env.found_docs().len(), 1);
        assert!(env.found_docs().contains("guide"));
        assert_eq!(
            env.project.docname_to_path.get("guide").map(String::as_str),
            Some("guide.rst.txt")
        );
    }

    #[test]
    fn register_yaml_notebooks_adds_targets_titles_and_root_toctree() {
        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(
            env.srcdir.join("_toc.yml"),
            "root: index\nchapters:\n  - file: notebook.ipynb\n    title: Notebook Chapter\n",
        )
        .unwrap();
        std::fs::write(
            env.srcdir.join("notebook.ipynb"),
            r##"{"cells":[],"metadata":{},"nbformat":4,"nbformat_minor":5}"##,
        )
        .unwrap();

        env.register_yaml_notebooks().unwrap();

        assert!(env.found_docs().contains("notebook"));
        assert_eq!(
            env.project.docname_to_path.get("notebook").map(String::as_str),
            Some("notebook.ipynb")
        );
        assert_eq!(
            env.toctree_includes.get("index"),
            Some(&vec!["notebook".to_string()])
        );
        assert_eq!(
            env.longtitles.get("notebook").map(String::as_str),
            Some("Notebook Chapter")
        );

        // A second registration sees the existing docname and exercises the
        // duplicate-target branch without changing the recorded path.
        env.register_yaml_notebooks().unwrap();
        assert_eq!(env.found_docs().iter().filter(|doc| *doc == "notebook").count(), 1);
    }

    #[test]
    fn register_yaml_notebooks_reads_source_directives_and_reports_read_errors() {
        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(
            env.srcdir.join("custom.yml"),
            "root: index\nchapters:\n  - file: inline.ipynb\n",
        )
        .unwrap();
        std::fs::write(env.srcdir.join("index.rst"), ".. toctreeyml:: custom.yml\n").unwrap();
        std::fs::write(
            env.srcdir.join("inline.ipynb"),
            r##"{"cells":[],"metadata":{},"nbformat":4,"nbformat_minor":5}"##,
        )
        .unwrap();
        env.project
            .docname_to_path
            .insert("index".into(), "index.rst".into());

        env.register_yaml_notebooks().unwrap();
        assert!(env.found_docs().contains("inline"));

        let (_tmp, mut env) = make_env_with_tempdir();
        env.project
            .docname_to_path
            .insert("missing".into(), "missing.rst".into());
        let error = env.register_yaml_notebooks().unwrap_err().to_string();
        assert!(error.contains("failed to read"));
    }

    #[test]
    fn register_yaml_notebooks_reports_unreadable_toc() {
        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::create_dir(env.srcdir.join("_toc.yml")).unwrap();
        let error = env.register_yaml_notebooks().unwrap_err().to_string();
        assert!(error.contains("failed to read"));
    }

    #[test]
    fn get_outdated_reports_found_but_unread_docname_as_added() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.project.docnames.insert("index".to_string());
        let (added, changed, removed) = env.get_outdated(false);
        assert_eq!(added, vec!["index".to_string()]);
        assert!(changed.is_empty());
        assert!(removed.is_empty());
    }

    #[test]
    fn get_outdated_reports_unchanged_read_docname_as_neither() {
        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(env.srcdir.join("index.rst"), "Index\n=====\n").unwrap();
        env.project.docnames.insert("index".to_string());
        // Record a read time far in the future so the file's real mtime
        // is never newer than it, regardless of test execution speed.
        let far_future = now_micros() + 60_000_000_000; // +60,000s
        env.record_doc_read("index", far_future);
        let (added, changed, removed) = env.get_outdated(false);
        assert!(added.is_empty());
        assert!(changed.is_empty(), "got changed: {changed:?}");
        assert!(removed.is_empty());
    }

    #[test]
    fn get_outdated_reports_source_newer_than_last_read_as_changed() {
        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(env.srcdir.join("index.rst"), "Index\n=====\n").unwrap();
        env.project.docnames.insert("index".to_string());
        // Record a read time far in the past so the file's real mtime is
        // always newer than it.
        env.record_doc_read("index", 0);
        let (added, changed, removed) = env.get_outdated(false);
        assert!(added.is_empty());
        assert_eq!(changed, vec!["index".to_string()]);
        assert!(removed.is_empty());
    }

    #[test]
    fn get_outdated_reports_previously_read_now_unfound_as_removed() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.record_doc_read("gone", now_micros());
        let (added, changed, removed) = env.get_outdated(false);
        assert!(added.is_empty());
        assert!(changed.is_empty());
        assert_eq!(removed, vec!["gone".to_string()]);
    }

    #[test]
    fn get_outdated_config_changed_forces_every_read_doc_as_changed() {
        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(env.srcdir.join("index.rst"), "Index\n=====\n").unwrap();
        env.project.docnames.insert("index".to_string());
        env.record_doc_read("index", now_micros() + 60_000_000_000);
        let (added, changed, removed) = env.get_outdated(true);
        assert!(added.is_empty());
        assert_eq!(changed, vec!["index".to_string()]);
        assert!(removed.is_empty());
    }

    #[test]
    fn get_outdated_reread_always_forces_changed() {
        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(env.srcdir.join("index.rst"), "Index\n=====\n").unwrap();
        env.project.docnames.insert("index".to_string());
        env.record_doc_read("index", now_micros() + 60_000_000_000);
        env.reread_always.insert("index".to_string());
        let (_added, changed, _removed) = env.get_outdated(false);
        assert_eq!(changed, vec!["index".to_string()]);
    }

    #[test]
    fn remove_doc_purges_titles_and_toctree_state() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.record_doc_read("index", 0);
        env.set_title("index", "Welcome");
        env.note_toctree("index", vec!["guide".to_string()]);
        env.remove_doc("index");
        assert!(!env.all_docs.contains_key("index"));
        assert!(env.get_title("index").is_none());
        assert!(!env.toctree_includes.contains_key("index"));
        assert!(
            !env.files_to_rebuild
                .get("guide")
                .is_some_and(|s| s.contains("index"))
        );
    }

    // ── docname_join / toctree entry qualification ─────────────────────────────

    #[test]
    fn docname_join_qualifies_entry_against_base_directory() {
        assert_eq!(
            docname_join("tutorial/index", "getting-started"),
            "tutorial/getting-started"
        );
        assert_eq!(
            docname_join("tutorial/index", "more/details"),
            "tutorial/more/details"
        );
    }

    #[test]
    fn docname_join_root_level_base_leaves_entry_unqualified() {
        assert_eq!(
            docname_join("index", "usage/installation"),
            "usage/installation"
        );
        assert_eq!(docname_join("index", "about"), "about");
    }

    #[test]
    fn docname_join_normalizes_dotdot_segments() {
        assert_eq!(
            docname_join("tutorial/sub/index", "../other"),
            "tutorial/other"
        );
    }

    #[test]
    fn read_one_with_source_qualifies_toctree_entries_relative_to_docname() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.read_one_with_source(
            "tutorial/index",
            "Build your first project\n========================\n\n.. toctree::\n\n   getting-started\n   first-steps\n",
        )
        .unwrap();
        assert_eq!(
            env.toctree_includes.get("tutorial/index"),
            Some(&vec![
                "tutorial/getting-started".to_string(),
                "tutorial/first-steps".to_string(),
            ])
        );
    }

    #[test]
    fn read_one_with_source_root_level_toctree_entries_unqualified() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.read_one_with_source(
            "index",
            "Welcome\n=======\n\n.. toctree::\n\n   usage/installation\n",
        )
        .unwrap();
        assert_eq!(
            env.toctree_includes.get("index"),
            Some(&vec!["usage/installation".to_string()])
        );
    }

    #[test]
    fn read_one_with_source_ignores_toctree_shown_inside_code_block_example() {
        // Regression test: a documentation page that *illustrates*
        // `.. toctree::` syntax inside a `.. code-block:: rst` example
        // (exactly as `sphinx/doc/usage/quickstart.rst` does) must not
        // have that example misread as a second, real toctree.
        let (_tmp, mut env) = make_env_with_tempdir();
        env.read_one_with_source(
            "usage/quickstart",
            "Quickstart\n==========\n\nReal content here.\n\n.. code-block:: rst\n\n   .. toctree::\n      :maxdepth: 2\n\n      usage/installation\n      usage/quickstart\n      ...\n\nMore real content.\n",
        )
        .unwrap();
        assert!(
            !env.toctree_includes.contains_key("usage/quickstart"),
            "toctree example inside a code-block must not be recorded as real: {:?}",
            env.toctree_includes.get("usage/quickstart")
        );
    }

    // ── H8b: environment persistence ──────────────────────────────────────────

    #[test]
    fn load_persisted_returns_none_when_absent() {
        let (_tmp, env) = make_env_with_tempdir();
        assert!(env.load_persisted().is_none());
    }

    #[test]
    fn save_and_load_persisted_round_trips_core_state() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.record_doc_read("index", 42);
        env.set_title("index", "Welcome");
        env.note_toctree("index", vec!["guide".to_string()]);
        env.save_persisted().unwrap();

        let loaded = env.load_persisted().expect("just-saved env should load");
        assert_eq!(loaded.all_docs.get("index"), Some(&42));
        assert_eq!(
            loaded.titles.get("index").map(String::as_str),
            Some("Welcome")
        );
        assert_eq!(
            loaded.toctree_includes.get("index"),
            Some(&vec!["guide".to_string()])
        );
        assert_eq!(loaded.config_hash, env.config.stable_hash());
    }

    #[test]
    fn apply_persisted_restores_state_onto_a_fresh_env() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.record_doc_read("index", 42);
        env.set_title("index", "Welcome");
        let persisted = env.to_persisted();

        let config = SphinxConfig::new_defaults();
        let project = EnvProject::new(&env.srcdir, &[(".rst", "restructuredtext")]);
        let mut fresh = BuildEnvironment::new(config, project, &env.srcdir, &env.doctreedir);
        assert!(fresh.all_docs.is_empty());

        let hash = fresh.apply_persisted(persisted);
        assert_eq!(hash, env.config.stable_hash());
        assert_eq!(fresh.all_docs.get("index"), Some(&42));
        assert_eq!(fresh.get_title("index"), Some("Welcome"));
    }

    #[test]
    fn load_persisted_returns_none_for_version_mismatch() {
        let (_tmp, env) = make_env_with_tempdir();
        let mut persisted = env.to_persisted();
        persisted.version = ENV_PERSISTED_VERSION + 1;
        let bytes = serde_json::to_vec(&persisted).unwrap();
        std::fs::write(env.persisted_path(), bytes).unwrap();
        assert!(env.load_persisted().is_none());
    }

    #[test]
    fn collect_ids_visits_root_then_children_preorder() {
        let mut tree = Doctree::new_document("test");
        let root = tree.root();
        let p1 = tree.append(root, NodeKind::Paragraph);
        let t1 = tree.append(p1, NodeKind::Text("hello".into()));
        let p2 = tree.append(root, NodeKind::Paragraph);
        let t2 = tree.append(p2, NodeKind::Text("world".into()));

        let mut ids = Vec::new();
        collect_ids(&tree, root, &mut ids);

        assert_eq!(ids, vec![root, p1, t1, p2, t2]);
    }

    #[test]
    fn collect_ids_excludes_detached_subtrees() {
        let mut tree = Doctree::new_document("test");
        let root = tree.root();
        let p1 = tree.append(root, NodeKind::Paragraph);
        let t1 = tree.append(p1, NodeKind::Text("kept".into()));
        let p2 = tree.append(root, NodeKind::Paragraph);
        tree.append(p2, NodeKind::Text("dropped".into()));

        // Detaching `p2` removes it (and its child) from `root`'s
        // children, so a reachability walk from `root` must not surface
        // either of them — unlike a raw `0..nodes_len()` arena scan,
        // which still visits their (now-orphaned) slots.
        tree.detach(p2);

        let mut ids = Vec::new();
        collect_ids(&tree, root, &mut ids);

        assert_eq!(ids, vec![root, p1, t1]);
    }

    #[test]
    fn parser_for_path_prefers_longest_suffix_and_defaults_to_rst() {
        let mut config = SphinxConfig::new_defaults();
        config.set(
            "source_suffix",
            ConfigVal::Map(vec![
                (".txt".into(), ConfigVal::Str("restructuredtext".into())),
                (".rst.txt".into(), ConfigVal::Str("myst".into())),
            ]),
        );
        let project = EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]);
        let env = BuildEnvironment::new(config, project, "/tmp/src", "/tmp/doctrees");
        assert_eq!(env.parser_for_path(Path::new("guide.rst.txt")), "myst");
        assert_eq!(
            env.parser_for_path(Path::new("guide.unknown")),
            "restructuredtext"
        );
    }

    #[test]
    fn parse_source_rejects_unknown_parser() {
        let mut config = SphinxConfig::new_defaults();
        config.set(
            "source_suffix",
            ConfigVal::Map(vec![(".foo".into(), ConfigVal::Str("unknown".into()))]),
        );
        let project = EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]);
        let env = BuildEnvironment::new(config, project, "/tmp/src", "/tmp/doctrees");
        assert!(env.parse_source("guide", "body").is_err());
    }

    #[test]
    fn parse_source_rejects_invalid_notebook_json() {
        let mut config = SphinxConfig::new_defaults();
        config.set(
            "source_suffix",
            ConfigVal::Map(vec![(".ipynb".into(), ConfigVal::Str("myst".into()))]),
        );
        let project = EnvProject::new("/tmp/src", &[(".ipynb", "myst")]);
        let env = BuildEnvironment::new(config, project, "/tmp/src", "/tmp/doctrees");
        let error = env.parse_source("broken.ipynb", "not-json").unwrap_err().to_string();
        assert!(error.contains("invalid notebook"));
    }

    #[test]
    fn read_one_and_read_all_report_missing_sources() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.project.docnames.insert("missing".into());
        assert!(env.read_one("missing").is_err());
        assert!(env.read_all().is_err());
    }

    #[test]
    fn read_all_with_events_emits_source_and_doctree_payloads() {
        use crate::app_events::{AppEventManager, EventArg};
        use std::cell::RefCell;
        use std::rc::Rc;

        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(env.srcdir.join("index.rst"), "Index\n=====\n\nBody.\n").unwrap();
        env.find_files().unwrap();
        let events = AppEventManager::shared();
        let source_count = Rc::new(RefCell::new(0usize));
        let source_count_ref = Rc::clone(&source_count);
        events.borrow_mut().connect("source-read", 0, move |args| {
            if matches!(args.get(1), Some(EventArg::StrList(_))) {
                *source_count_ref.borrow_mut() += 1;
            }
            Ok(())
        });
        let doctree_count = Rc::new(RefCell::new(0usize));
        let doctree_count_ref = Rc::clone(&doctree_count);
        events.borrow_mut().connect("doctree-read", 0, move |args| {
            if matches!(args.first(), Some(EventArg::Doctree(_))) {
                *doctree_count_ref.borrow_mut() += 1;
            }
            Ok(())
        });

        let read = env.read_all_with_events(&events).unwrap();
        assert_eq!(read, vec!["index"]);
        assert_eq!(*source_count.borrow(), 1);
        assert_eq!(*doctree_count.borrow(), 1);
        assert_eq!(events.borrow().emitted(), &["source-read", "doctree-read"]);
    }

    #[test]
    fn read_all_with_events_propagates_source_listener_errors() {
        use crate::app_events::AppEventManager;

        let (_tmp, mut env) = make_env_with_tempdir();
        std::fs::write(env.srcdir.join("index.rst"), "Index\n=====\n").unwrap();
        env.find_files().unwrap();
        let events = AppEventManager::shared();
        events.borrow_mut().connect("source-read", 0, |_args| {
            Err(crate::app_events::EventError("listener failed".into()))
        });

        let error = env.read_all_with_events(&events).unwrap_err().to_string();
        assert!(error.contains("listener failed"));
        assert!(!env.has_stored_doctree("index"));
    }

    #[test]
    fn read_one_populates_domain_inventories_from_source() {
        let (_tmp, mut env) = make_env_with_tempdir();
        env.read_one_with_source(
            "index",
            ".. _label:\n\n.. rst:directive:: toctree\n\n.. glossary::\n\n   widget\n      A thing.\n\n.. py:module:: pkg.mod\n\n.. py:function:: greet(name)\n\n.. js:module:: widgets\n\n.. js:function:: render(el)\n\n.. index:: pair: foo; bar\n\nSee :ref:`label`.\n",
        )
        .unwrap();

        assert!(env.std_domain.anonlabels.contains_key("label"));
        assert!(env.std_domain.terms.contains_key("widget"));
        assert!(env
            .domain_objects()
            .iter()
            .any(|(domain, entry)| domain == "rst" && entry.name == "toctree"));
        assert!(env
            .domain_objects()
            .iter()
            .any(|(domain, entry)| domain == "py" && entry.name == "pkg.mod.greet"));
        assert!(env
            .domain_objects()
            .iter()
            .any(|(domain, entry)| domain == "js" && entry.name == "widgets.render"));
        assert!(!env.indexentries.get("index").unwrap().is_empty());
        assert_eq!(env.pending_xrefs.get("index").unwrap().len(), 1);
    }

    #[test]
    fn resolve_references_covers_explicit_shortened_and_unresolved_targets() {
        let mut env = make_env();
        env.std_domain
            .note_label("target", "index", "target-id", "Target title");
        env.pending_xrefs.insert(
            "index".into(),
            vec![
                crate::domains::PendingXref {
                    domain: "std".into(),
                    reftype: "ref".into(),
                    target: "target".into(),
                    explicit_title: Some("Custom title".into()),
                    line: 1,
                    shorten: false,
                },
                crate::domains::PendingXref {
                    domain: "std".into(),
                    reftype: "ref".into(),
                    target: "pkg.target".into(),
                    explicit_title: None,
                    line: 2,
                    shorten: true,
                },
                crate::domains::PendingXref {
                    domain: "unknown".into(),
                    reftype: "ref".into(),
                    target: "missing".into(),
                    explicit_title: None,
                    line: 3,
                    shorten: false,
                },
            ],
        );
        let resolutions = env.resolve_references("index");
        assert!(matches!(
            &resolutions[0],
            crate::domains::XrefResolution::Resolved { target, .. }
                if target.title == "Custom title"
        ));
        assert!(matches!(
            &resolutions[1],
            crate::domains::XrefResolution::Unresolved { .. }
        ));
        assert!(matches!(
            &resolutions[2],
            crate::domains::XrefResolution::Unresolved { warning, .. }
                if warning.contains("missing")
        ));
        assert!(env.resolve_references("absent").is_empty());
        assert_eq!(env.resolve_all_references().len(), 1);
    }

    #[test]
    fn resolve_xref_nodes_rewrites_links_and_honors_disabled_and_extlinks() {
        let mut env = make_env();
        env.std_domain
            .note_label("target", "index", "target-id", "Target title");
        env.config.set(
            "extlinks",
            ConfigVal::Map(vec![
                (
                    "issue".into(),
                    ConfigVal::List(vec![
                        ConfigVal::Str("https://tracker.test/%s".into()),
                        ConfigVal::Str("Issue %s".into()),
                    ]),
                ),
            ]),
        );

        let mut tree = Doctree::new_document("index");
        let root = tree.root();
        let ref_id = tree.append(root, NodeKind::Inline { classes: "ref".into() });
        tree.append(ref_id, NodeKind::Text("target".into()));
        let disabled = tree.append(root, NodeKind::Inline { classes: "ref".into() });
        tree.append(disabled, NodeKind::Text("!target".into()));
        let external = tree.append(root, NodeKind::Inline { classes: "issue".into() });
        tree.append(external, NodeKind::Text("42".into()));
        let unknown = tree.append(root, NodeKind::Inline { classes: "unknown".into() });
        tree.append(unknown, NodeKind::Text("text".into()));

        env.resolve_xref_nodes(&mut tree, "index");
        assert!(matches!(&tree.node(ref_id).kind, NodeKind::Reference { refuri, .. } if refuri == "#target-id"));
        let span_id = tree.node(ref_id).children[0];
        assert!(matches!(&tree.node(span_id).kind, NodeKind::Inline { classes } if classes == "std std-ref"));
        assert!(matches!(tree.node(disabled).kind, NodeKind::Inline { .. }));
        assert!(matches!(&tree.node(external).kind, NodeKind::Reference { refuri, classes, .. } if refuri == "https://tracker.test/42" && classes.contains("extlink-issue")));
        assert!(matches!(tree.node(unknown).kind, NodeKind::Inline { .. }));
    }

    #[test]
    fn resolve_toctree_nodes_handles_hidden_caption_depth_and_nested_entries() {
        let mut env = make_env();
        env.set_title("guide", "Guide");
        env.set_title("guide/intro", "Introduction");
        env.note_toctree("index", vec!["guide".into()]);
        env.note_toctree("guide", vec!["guide/intro".into()]);

        let mut tree = Doctree::new_document("index");
        let visible = tree.append(
            tree.root(),
            NodeKind::Toctree {
                caption: Some("Contents".into()),
                maxdepth: 0,
                hidden: false,
                entries: vec!["guide".into()],
            },
        );
        let shallow = tree.append(
            tree.root(),
            NodeKind::Toctree {
                caption: None,
                maxdepth: 1,
                hidden: false,
                entries: vec!["guide".into()],
            },
        );
        let hidden = tree.append(
            tree.root(),
            NodeKind::Toctree {
                caption: Some("Hidden".into()),
                maxdepth: 1,
                hidden: true,
                entries: vec!["guide".into()],
            },
        );

        env.resolve_toctree_nodes(&mut tree, "index");
        assert!(matches!(tree.node(visible).kind, NodeKind::Container { .. }));
        assert!(tree.node(visible).children.iter().any(|id| matches!(tree.node(*id).kind, NodeKind::Caption)));
        assert!(tree.node(visible).children.iter().any(|id| matches!(tree.node(*id).kind, NodeKind::BulletList { .. })));
        assert!(matches!(tree.node(shallow).kind, NodeKind::Container { .. }));
        assert!(matches!(tree.node(hidden).kind, NodeKind::Comment));
    }

    #[test]
    fn doctree_store_rejects_traversal_and_corruption() {
        let (_tmp, env) = make_env_with_tempdir();
        assert!(env.doctree_path("../escape").is_err());
        std::fs::create_dir_all(&env.doctreedir).unwrap();
        std::fs::write(env.doctree_path("broken").unwrap(), b"not-json").unwrap();
        let error = env.get_doctree("broken").unwrap_err().to_string();
        assert!(error.contains("corrupt doctree"));
    }
}

#[cfg(test)]
mod yaml_and_scan_helper_tests {
    use super::*;

    #[test]
    fn sanitize_docname_rejects_empty_and_traversal_and_absolute() {
        assert!(sanitize_docname("index").is_ok());
        assert!(sanitize_docname("guide/intro").is_ok());
        assert!(sanitize_docname("").is_err());
        assert!(sanitize_docname("guide/../etc").is_err());
        assert!(sanitize_docname("/etc/passwd").is_err());
        assert!(sanitize_docname("guide//intro").is_err());
    }

    #[test]
    fn mtime_micros_returns_none_for_missing_file() {
        assert!(mtime_micros(Path::new("/no/such/file/anywhere")).is_none());
    }

    #[test]
    fn mtime_micros_returns_some_for_existing_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        assert!(mtime_micros(tmp.path()).is_some());
    }

    #[test]
    fn yaml_toc_entries_reads_root_chapters_sections_and_parts() {
        let src = r#"
root: intro
chapters:
  - guide
  - file: reference
    sections:
      - reference/api
parts:
  - caption: Part One
    chapters:
      - part1/ch1
"#;
        let entries = yaml_toc_entries(src).unwrap();
        let targets: Vec<&str> = entries.iter().map(|e| e.target.as_str()).collect();
        assert!(targets.contains(&"intro"));
        assert!(targets.contains(&"guide"));
        assert!(targets.contains(&"reference"));
        assert!(targets.contains(&"reference/api"));
        assert!(targets.contains(&"part1/ch1"));
    }

    #[test]
    fn yaml_toc_entries_requires_mapping() {
        let err = yaml_toc_entries("- just\n- a\n- list\n").unwrap_err();
        assert!(err.to_string().contains("must be a mapping"));
    }

    #[test]
    fn yaml_toc_entries_rejects_invalid_yaml() {
        let err = yaml_toc_entries("chapters: [unterminated\n").unwrap_err();
        assert!(err.to_string().contains("invalid YAML toctree"));
    }

    #[test]
    fn collect_yaml_toc_entries_skips_build_false_and_reads_title_and_subsections() {
        let src = r#"
root: index
sections:
  - file: skip-me
    build: false
  - file: keep-me
    title: Keep Me
    subsections:
      - nested/page
"#;
        let entries = yaml_toc_entries(src).unwrap();
        let targets: Vec<&str> = entries.iter().map(|e| e.target.as_str()).collect();
        assert!(!targets.contains(&"skip-me"));
        assert!(targets.contains(&"keep-me"));
        assert!(targets.contains(&"nested/page"));
        let keep = entries.iter().find(|e| e.target == "keep-me").unwrap();
        assert_eq!(keep.title.as_deref(), Some("Keep Me"));
    }

    #[test]
    fn collect_yaml_toc_entries_prefers_file_over_url() {
        let src = "root: index\nsections:\n  - file: from-file\n    url: from-url\n";
        let entries = yaml_toc_entries(src).unwrap();
        assert!(entries.iter().any(|e| e.target == "from-file"));
        assert!(!entries.iter().any(|e| e.target == "from-url"));
    }

    #[test]
    fn collect_yaml_toc_entries_uses_url_when_file_absent() {
        let src = "root: index\nsections:\n  - url: from-url\n";
        let entries = yaml_toc_entries(src).unwrap();
        assert!(entries.iter().any(|e| e.target == "from-url"));
    }

    #[test]
    fn yaml_helpers_skip_non_mapping_values_and_report_missing_default_file() {
        let entries = yaml_toc_entries(
            "parts:\n  - 7\n  - chapters: [guide]\nsections:\n  - 42\n",
        )
        .unwrap();
        assert!(entries.iter().any(|entry| entry.target == "guide"));

        let tmp = tempfile::TempDir::new().unwrap();
        let source = ".. toctreeyml::\n   root: index\noutside\n";
        let expanded = expand_yaml_toctree_directives(source, tmp.path()).unwrap();
        assert!(expanded.contains(".. toctree::"));

        let err = expand_yaml_toctree_directives(".. toctreeyml::\n", tmp.path()).unwrap_err();
        assert!(err.to_string().contains("_toc.yml"));
    }

    #[test]
    fn collect_yaml_toc_entries_ignores_mapping_without_file_or_url() {
        let src = "root: index\nsections:\n  - title: No target here\n";
        let entries = yaml_toc_entries(src).unwrap();
        assert_eq!(entries.len(), 1); // just the root
    }

    #[test]
    fn yaml_docname_strips_known_suffixes_and_dot_slash() {
        assert_eq!(yaml_docname("./guide.rst").as_deref(), Some("guide"));
        assert_eq!(yaml_docname("guide.md").as_deref(), Some("guide"));
        assert_eq!(yaml_docname("guide.ipynb").as_deref(), Some("guide"));
        assert_eq!(yaml_docname("guide.txt").as_deref(), Some("guide"));
        assert_eq!(yaml_docname("guide").as_deref(), Some("guide"));
    }

    #[test]
    fn yaml_docname_rejects_absolute_and_external_targets() {
        assert_eq!(yaml_docname("/etc/passwd"), None);
        assert_eq!(yaml_docname("https://example.org/page"), None);
    }

    #[test]
    fn yaml_toc_root_and_children_uses_explicit_root() {
        let src = "root: intro\nchapters:\n  - guide\n  - reference\n";
        let (root, children) = yaml_toc_root_and_children(src, "fallback").unwrap().unwrap();
        assert_eq!(root, "intro");
        assert_eq!(children, vec!["guide".to_string(), "reference".to_string()]);
    }

    #[test]
    fn yaml_toc_root_and_children_falls_back_to_default_root_without_root_key() {
        let src = "chapters:\n  - guide\n  - reference\n";
        let (root, children) = yaml_toc_root_and_children(src, "fallback").unwrap().unwrap();
        assert_eq!(root, "fallback");
        assert_eq!(children, vec!["guide".to_string(), "reference".to_string()]);
    }

    #[test]
    fn yaml_toc_root_and_children_returns_none_for_empty_toc() {
        assert!(yaml_toc_root_and_children("chapters: []\n", "fallback")
            .unwrap()
            .is_none());
    }

    #[test]
    fn yaml_toc_root_and_children_returns_none_when_root_entry_is_external() {
        let src = "root: https://example.org\nchapters:\n  - guide\n";
        assert!(yaml_toc_root_and_children(src, "fallback").unwrap().is_none());
    }

    #[test]
    fn yaml_option_lines_reads_top_level_and_nested_options_block() {
        let src = r#"
root: index
maxdepth: 2
caption: My Caption
glob: true
hidden: false
options:
  numbered: true
  titlesonly: true
"#;
        let lines = yaml_option_lines(src).unwrap();
        assert!(lines.contains(&":maxdepth: 2".to_string()));
        assert!(lines.contains(&":caption: My Caption".to_string()));
        assert!(lines.contains(&":glob:".to_string()));
        assert!(!lines.iter().any(|l| l.starts_with(":hidden")));
        assert!(lines.contains(&":numbered:".to_string()));
        assert!(lines.contains(&":titlesonly:".to_string()));
    }

    #[test]
    fn yaml_option_lines_returns_empty_for_non_mapping() {
        assert_eq!(yaml_option_lines("- a\n- b\n").unwrap(), Vec::<String>::new());
    }

    #[test]
    fn dedent_yaml_uses_minimum_indent_and_ignores_blank_lines() {
        let lines = vec!["    root: index", "", "    chapters:", "      - guide"];
        let out = dedent_yaml(&lines);
        assert_eq!(out, "root: index\n\nchapters:\n  - guide");
    }

    #[test]
    fn dedent_yaml_handles_all_blank_lines() {
        assert_eq!(dedent_yaml(&["", "  ", ""]), "\n  \n");
    }

    #[test]
    fn expand_yaml_toctree_directives_reads_referenced_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("_toc.yml"),
            "root: index\nchapters:\n  - guide\n",
        )
        .unwrap();
        let source = ".. toctreeyml::\n";
        let expanded = expand_yaml_toctree_directives(source, tmp.path()).unwrap();
        assert!(expanded.contains(".. toctree::"));
        assert!(expanded.contains("guide"));
    }

    #[test]
    fn expand_yaml_toctree_directives_reads_named_argument_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("custom.yml"),
            "root: index\nchapters:\n  - guide\n",
        )
        .unwrap();
        let source = ".. toctreeyaml:: custom.yml\n";
        let expanded = expand_yaml_toctree_directives(source, tmp.path()).unwrap();
        assert!(expanded.contains("guide"));
    }

    #[test]
    fn expand_yaml_toctree_directives_uses_inline_body_with_titles() {
        let tmp = tempfile::TempDir::new().unwrap();
        let source = ".. toctreeyml::\n\n   root: index\n   chapters:\n     - title: Guide Title\n       file: guide\n";
        let expanded = expand_yaml_toctree_directives(source, tmp.path()).unwrap();
        assert!(expanded.contains("Guide Title <guide>"));
    }

    #[test]
    fn expand_yaml_toctree_directives_errors_when_referenced_file_missing() {
        let tmp = tempfile::TempDir::new().unwrap();
        let source = ".. toctreeyml:: missing.yml\n";
        let err = expand_yaml_toctree_directives(source, tmp.path()).unwrap_err();
        assert!(err.to_string().contains("failed to read YAML toctree"));
    }

    #[test]
    fn expand_yaml_toctree_directives_passes_through_non_directive_lines() {
        let tmp = tempfile::TempDir::new().unwrap();
        let source = "Some text\n\nMore text\n";
        let expanded = expand_yaml_toctree_directives(source, tmp.path()).unwrap();
        assert_eq!(expanded, source);
    }

    #[test]
    fn strip_opaque_literal_blocks_blanks_code_block_body_but_keeps_directive_line() {
        let source = "before\n\n.. code-block:: rst\n\n   .. toctree::\n      hidden\n\nafter\n";
        let stripped = strip_opaque_literal_blocks(source);
        assert!(stripped.contains(".. code-block:: rst"));
        assert!(!stripped.contains("hidden"));
        assert!(stripped.contains("before"));
        assert!(stripped.contains("after"));
    }

    #[test]
    fn strip_opaque_literal_blocks_handles_plain_literal_marker() {
        let source = "Example::\n\n   .. toctree::\n      fake-entry\n\nafter\n";
        let stripped = strip_opaque_literal_blocks(source);
        assert!(!stripped.contains("fake-entry"));
        assert!(stripped.contains("after"));
    }

    #[test]
    fn strip_opaque_literal_blocks_ignores_non_opaque_directives() {
        let source = ".. note::\n\n   still here\n";
        let stripped = strip_opaque_literal_blocks(source);
        assert!(stripped.contains("still here"));
    }

    #[test]
    fn scan_toctree_entries_with_titles_strips_suffixes_and_explicit_titles() {
        let source = ".. toctree::\n   :maxdepth: 2\n\n   guide.rst\n   Custom Title <reference.md>\n";
        let entries = scan_toctree_entries_with_titles(source);
        assert_eq!(entries[0], ("guide".to_string(), None));
        assert_eq!(
            entries[1],
            ("reference".to_string(), Some("Custom Title".to_string()))
        );
        assert_eq!(scan_toctree_entries(source), vec!["guide", "reference"]);
    }

    #[test]
    fn scan_toctree_entries_stops_at_dedented_line() {
        let source = ".. toctree::\n\n   guide\n\nnot-an-entry\n";
        assert_eq!(scan_toctree_entries(source), vec!["guide"]);
    }

    #[test]
    fn scan_toctree_entries_omits_hidden_entries() {
        let source = ".. toctree::\n   visible\n\n.. toctree::\n   :hidden:\n\n   hidden\n";
        assert_eq!(scan_toctree_entries(source), vec!["visible"]);
    }

    #[test]
    fn scan_include_entries_extracts_paths_and_ignores_unrelated_lines() {
        let source = "text\n.. include:: shared/header.rst\nmore text\n   .. include::  shared/footer.rst  \n";
        let entries = scan_include_entries(source);
        assert_eq!(entries, vec!["shared/header.rst", "shared/footer.rst"]);
    }

    #[test]
    fn scan_include_entries_returns_empty_when_none_present() {
        assert!(scan_include_entries("just some text\n").is_empty());
    }
}
