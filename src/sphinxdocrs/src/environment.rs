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
use std::path::{Path, PathBuf};

use docutilsrs::doctree::{Doctree, NodeId, NodeKind};
use serde::{Deserialize, Serialize};

use crate::builders::BuildError;
use crate::config::{ConfigVal, SphinxConfig};
use crate::domains::{
    IndexEntry, JsDomain, ObjectEntry, PendingXref, PyDomain, RstDomain, StdDomain, XrefResolution,
    scan,
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
        let settings = default_settings();
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
        let source = std::fs::read_to_string(&path)
            .map_err(|e| BuildError::Other(format!("failed to read {}: {e}", path.display())))?;
        Ok(docutilsrs::parse_rst_with_source(&source, docname))
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
            let source = std::fs::read_to_string(&path).map_err(|e| {
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
        let source = std::fs::read_to_string(&path)
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
        let expanded_source = self.expand_autodoc(source);
        let source = expanded_source.as_deref().unwrap_or(source);
        let highlighted_source = self.apply_highlight_language(source);
        let parse_source = highlighted_source.as_deref().unwrap_or(source);
        let tree = docutilsrs::parse_rst_with_source(parse_source, docname);

        let title = match &tree.node(tree.root()).kind {
            NodeKind::Document { title, .. } if !title.is_empty() => title.clone(),
            _ => docname.rsplit('/').next().unwrap_or(docname).to_string(),
        };
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
            if trimmed.starts_with(".. highlight::") {
                let next = trimmed[14..].trim();
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
                    let indent = &line[..line.len() - trimmed.len()];
                    output.push(format!("{indent}{prefix} {language}"));
                    changed = true;
                    continue;
                }
            }
            output.push(line.to_string());
        }

        changed.then(|| output.join("\n"))
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
                "rst" => self
                    .rst_domain
                    .resolve_xref(self, docname, &reftype, &target),
                "py" => self
                    .py_domain
                    .resolve_xref(self, docname, &reftype, &target),
                "js" => self
                    .js_domain
                    .resolve_xref(self, docname, &reftype, &target),
                _ => None,
            };

            let inner_class = match domain.as_str() {
                "rst" => format!("xref rst rst-{reftype}"),
                "py" => format!("xref py py-{reftype}"),
                "js" => format!("xref js js-{reftype}"),
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
                        target.rsplit('.').next().unwrap_or(&target).to_string()
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
                            classes: inner_class,
                        },
                    );
                    tree.append(id, NodeKind::Text(title));
                }
                continue;
            };

            let title = if let Some(t) = &explicit_title {
                t.clone()
            } else if shorten {
                target.rsplit('.').next().unwrap_or(&target).to_string()
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
                    classes: inner_class,
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
            if let Some(caption) = caption {
                let p = tree.append(id, NodeKind::Paragraph);
                let span = tree.append(
                    p,
                    NodeKind::Inline {
                        classes: "caption-text".to_string(),
                    },
                );
                tree.append(span, NodeKind::Text(caption));
            }
            // Entries are written relative to the directory containing
            // *this* document (e.g. `usage/restructuredtext/index.rst`
            // listing `basics` really means `usage/restructuredtext/basics`),
            // matching `scan_toctree_entries`'s own qualification above.
            let entries: Vec<String> = entries
                .into_iter()
                .map(|entry| docname_join(docname, &entry))
                .collect();
            let depth = if maxdepth <= 0 { 0 } else { maxdepth as usize };
            let resolved = crate::toctree::resolve_from_entries(self, &entries, depth);
            if !resolved.is_empty() {
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
            let target_uri = builder.get_target_uri(&entry.docname);
            let href = relative_uri(base_uri, &target_uri);
            let refnode = tree.append(
                item,
                NodeKind::Reference {
                    name: String::new(),
                    refuri: href,
                    anonymous: false,
                    classes: "reference internal".to_string(),
                },
            );
            tree.append(refnode, NodeKind::Text(entry.title.clone()));
            if !entry.children.is_empty() {
                Self::append_toc_entries(tree, item, &entry.children, builder, base_uri);
            }
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
            .map(|d| format!("{d}: document isn't included in any toctree"))
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
        self.rst_domain.clear_doc(docname);
        self.py_domain.clear_doc(docname);
        self.js_domain.clear_doc(docname);
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
            rst_domain: self.rst_domain.clone(),
            py_domain: self.py_domain.clone(),
            js_domain: self.js_domain.clone(),
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

/// Current [`EnvPersisted::version`]. Bump on any breaking change to that
/// struct's shape so an on-disk file saved by a previous `sphinxdocrs`
/// version is detected as stale (treated as absent) rather than
/// misinterpreted by `serde_json` (which would otherwise silently accept
/// a structurally-compatible-but-semantically-different old file).
pub const ENV_PERSISTED_VERSION: u32 = 1;

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
    pub rst_domain: RstDomain,
    pub py_domain: PyDomain,
    pub js_domain: JsDomain,
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
fn docname_join(base: &str, other: &str) -> String {
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
/// etc.) which start with `:`. The `Title <docname>` and `:glob:` forms
/// are not expanded yet — accepted deviation until **H3a** lands a real
/// toctree node.
fn scan_toctree_entries(source: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        if let Some(rest) = trimmed.strip_prefix("..") {
            let rest = rest.trim_start();
            if rest.starts_with("toctree::") {
                let indent = lines[i].len() - trimmed.len();
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
                    if !body.starts_with(':') {
                        entries.push(body.to_string());
                    }
                    i += 1;
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

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SphinxConfig;

    fn make_env() -> BuildEnvironment {
        let config = SphinxConfig::new_defaults();
        let project = EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]);
        BuildEnvironment::new(config, project, "/tmp/src", "/tmp/doctrees")
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
                NodeKind::Inline { classes } if classes == "keyword"
            )
        }));
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
                NodeKind::Inline { classes } if classes == "keyword"
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
}
