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
use std::path::PathBuf;

use docutilsrs::doctree::{Doctree, NodeKind};

use crate::builders::BuildError;
use crate::config::SphinxConfig;
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
        }
    }

    /// Install a handle to the app's native event bus (**H4a**).
    ///
    /// Called by [`crate::application::SphinxApp`] before dispatching to a
    /// builder so the write phase can emit `html-page-context` per page.
    pub fn set_events(&mut self, events: crate::app_events::SharedEvents) {
        self.events = EventsHandle(Some(events));
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
        self.read_all_impl(None)
    }

    /// Same as [`read_all`](Self::read_all), but emits `source-read`
    /// (before each document is parsed) and `doctree-read` (after it is
    /// stored) on `events` — the **H4a** per-document event hooks used by
    /// [`crate::application::SphinxApp::read`].
    pub fn read_all_with_events(
        &mut self,
        events: &crate::app_events::SharedEvents,
    ) -> Result<Vec<String>, BuildError> {
        self.read_all_impl(Some(events))
    }

    fn read_all_impl(
        &mut self,
        events: Option<&crate::app_events::SharedEvents>,
    ) -> Result<Vec<String>, BuildError> {
        use crate::app_events::EventArg;

        let mut docnames: Vec<String> = self.found_docs().iter().cloned().collect();
        docnames.sort();

        for docname in &docnames {
            if let Some(events) = events {
                events
                    .borrow_mut()
                    .emit("source-read", &[EventArg::Str(docname.clone())]);
            }

            let path = self.doc2path(docname);
            let source = std::fs::read_to_string(&path).map_err(|e| {
                BuildError::Other(format!("failed to read {}: {e}", path.display()))
            })?;

            let tree = docutilsrs::parse_rst_with_source(&source, docname);

            let title = match &tree.node(tree.root()).kind {
                NodeKind::Document { title, .. } if !title.is_empty() => title.clone(),
                _ => docname.rsplit('/').next().unwrap_or(docname).to_string(),
            };
            self.set_title(docname.clone(), title);

            let entries = scan_toctree_entries(&source);
            if !entries.is_empty() {
                self.note_toctree(docname.clone(), entries);
            }

            for include in scan_include_entries(&source) {
                self.note_dependency(docname.clone(), include);
            }

            self.note_domain_data(docname, &source);

            self.store_doctree(docname, &tree)?;
            self.record_doc_read(docname.clone(), now_micros());

            if let Some(events) = events {
                events
                    .borrow_mut()
                    .emit("doctree-read", &[EventArg::Str(docname.clone())]);
            }
        }

        Ok(docnames)
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
            let labelid = format!("rst-{objtype}-{}", crate::domains::normalize_id(&name));
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
}
