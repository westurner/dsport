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

use crate::config::SphinxConfig;

// ── project shim ─────────────────────────────────────────────────────────────

/// Minimal project descriptor for the environment.
///
/// The full PyO3-backed `Project` is in `crate::project`; this struct
/// carries the Rust-native subset needed for `BuildEnvironment`.
#[derive(Debug, Clone, Default)]
pub struct EnvProject {
    pub srcdir: PathBuf,
    pub source_suffix: Vec<(String, String)>,
    /// Known docnames (populated by `discover`).
    pub docnames: HashSet<String>,
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
        }
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
