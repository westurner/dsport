//! Integration tests for `sphinxdocrs::environment::BuildEnvironment`.
//!
//! Mirrors `sphinx/tests/test_environment/` (pure-algorithm subset).

use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{
    BuildEnvironment, CONFIG_CHANGED, CONFIG_EXTENSIONS_CHANGED, CONFIG_NEW, CONFIG_OK,
    CONFIG_UNSET, EnvProject, default_settings,
};
use tempfile::TempDir;

fn make_env() -> BuildEnvironment {
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]);
    BuildEnvironment::new(config, project, "/tmp/src", "/tmp/doctrees")
}

// ── construction ─────────────────────────────────────────────────────────────

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
fn new_env_paths() {
    let env = make_env();
    assert_eq!(env.srcdir.to_str().unwrap(), "/tmp/src");
    assert_eq!(env.doctreedir.to_str().unwrap(), "/tmp/doctrees");
}

#[test]
fn new_env_project_srcdir() {
    let env = make_env();
    assert_eq!(env.project.srcdir.to_str().unwrap(), "/tmp/src");
}

// ── default settings ──────────────────────────────────────────────────────────

#[test]
fn default_settings_halt_level() {
    let s = default_settings();
    assert_eq!(s.get("halt_level").map(String::as_str), Some("5"));
}

#[test]
fn default_settings_input_encoding() {
    let s = default_settings();
    assert_eq!(
        s.get("input_encoding").map(String::as_str),
        Some("utf-8-sig")
    );
}

#[test]
fn default_settings_auto_id_prefix() {
    let s = default_settings();
    assert_eq!(s.get("auto_id_prefix").map(String::as_str), Some("id"));
}

// ── config status ─────────────────────────────────────────────────────────────

#[test]
fn config_status_new() {
    let mut env = make_env();
    env.set_config_status(CONFIG_NEW, "new config");
    assert_eq!(env.config_status, CONFIG_NEW);
    assert_eq!(env.config_status_label(), "new config");
    assert_eq!(env.config_status_extra, "new config");
}

#[test]
fn config_status_changed() {
    let mut env = make_env();
    env.set_config_status(CONFIG_CHANGED, "");
    assert_eq!(env.config_status_label(), "config changed");
}

#[test]
fn config_status_extensions_changed() {
    let mut env = make_env();
    env.set_config_status(CONFIG_EXTENSIONS_CHANGED, "");
    assert_eq!(env.config_status_label(), "extensions changed");
}

#[test]
fn config_status_ok() {
    let mut env = make_env();
    env.set_config_status(CONFIG_OK, "");
    assert_eq!(env.config_status_label(), "config OK");
}

// ── document tracking ─────────────────────────────────────────────────────────

#[test]
fn record_and_check_doc_read() {
    let mut env = make_env();
    assert!(!env.is_doc_read("index"));
    env.record_doc_read("index", 1_000_000);
    assert!(env.is_doc_read("index"));
    assert!(!env.is_doc_read("api/module"));
}

#[test]
fn record_multiple_docs() {
    let mut env = make_env();
    env.record_doc_read("index", 100);
    env.record_doc_read("guide/intro", 200);
    assert_eq!(env.all_docs.len(), 2);
}

// ── titles ────────────────────────────────────────────────────────────────────

#[test]
fn set_and_get_title() {
    let mut env = make_env();
    env.set_title("index", "Welcome");
    assert_eq!(env.get_title("index"), Some("Welcome"));
}

#[test]
fn get_title_missing_returns_none() {
    let env = make_env();
    assert!(env.get_title("nonexistent").is_none());
}

// ── dependencies ─────────────────────────────────────────────────────────────

#[test]
fn note_dependency_single() {
    let mut env = make_env();
    env.note_dependency("index", "includes/header.rst");
    assert!(env.dependencies["index"].contains("includes/header.rst"));
}

#[test]
fn note_dependency_multiple() {
    let mut env = make_env();
    env.note_dependency("index", "a.rst");
    env.note_dependency("index", "b.rst");
    assert_eq!(env.dependencies["index"].len(), 2);
}

// ── temp_data / ref_context ───────────────────────────────────────────────────

#[test]
fn clear_temp_data_clears_both_maps() {
    let mut env = make_env();
    env.temp_data.insert("key".into(), "val".into());
    env.ref_context.insert("module".into(), "os".into());
    env.clear_temp_data();
    assert!(env.temp_data.is_empty());
    assert!(env.ref_context.is_empty());
}

// ── toc / domain maps start empty ─────────────────────────────────────────────

#[test]
fn toc_num_entries_empty() {
    let env = make_env();
    assert!(env.toc_num_entries.is_empty());
}

#[test]
fn glob_toctrees_empty() {
    let env = make_env();
    assert!(env.glob_toctrees.is_empty());
}

#[test]
fn domaindata_empty() {
    let env = make_env();
    assert!(env.domaindata.is_empty());
}

#[test]
fn found_docs_delegates_to_project() {
    let env = make_env();
    // project.docnames starts empty
    assert!(env.found_docs().is_empty());
}

// ── config wiring ─────────────────────────────────────────────────────────────

#[test]
fn env_carries_config_language() {
    let mut config = SphinxConfig::new_defaults();
    use sphinxdocrs::config::ConfigVal;
    config.set("language", ConfigVal::Str("de".into()));
    let project = EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, "/tmp/src", "/tmp/doctrees");
    assert_eq!(env.config.language(), "de");
}

// ── H2: read/write pipeline ───────────────────────────────────────────────────
//
// `find_files` (H2a), doctree store round trip (H2b), `read_all` (H2c),
// `get_and_resolve_doctree` (H2e) and `check_consistency` (H2f).

/// Build a `BuildEnvironment` backed by real temp directories containing
/// `files` (docname → RST source), returning the srcdir/doctreedir
/// `TempDir` guards alongside the env so callers can inspect the
/// filesystem after `find_files`/`read_all` run.
fn make_disk_env(files: &[(&str, &str)]) -> (TempDir, TempDir, BuildEnvironment) {
    let src = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    for (docname, source) in files {
        let path = src.path().join(format!("{docname}.rst"));
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, source).unwrap();
    }
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    (src, doctrees, env)
}

#[test]
fn find_files_discovers_rst_docs() {
    let (_src, _dt, mut env) =
        make_disk_env(&[("index", "Home\n====\n"), ("guide/intro", "Intro\n=====\n")]);
    env.find_files().unwrap();
    let mut docs: Vec<String> = env.found_docs().iter().cloned().collect();
    docs.sort();
    assert_eq!(docs, vec!["guide/intro".to_string(), "index".to_string()]);
}

#[test]
fn find_files_respects_exclude_patterns() {
    let (_src, _dt, mut env) =
        make_disk_env(&[("index", "Home\n====\n"), ("_drafts/wip", "Draft\n=====\n")]);
    use sphinxdocrs::config::ConfigVal;
    env.config.set(
        "exclude_patterns",
        ConfigVal::List(vec![ConfigVal::Str("_drafts/**".into())]),
    );
    env.find_files().unwrap();
    let docs: Vec<String> = env.found_docs().iter().cloned().collect();
    assert_eq!(docs, vec!["index".to_string()]);
}

#[test]
fn doc2path_resolves_discovered_docname() {
    let (src, _dt, mut env) = make_disk_env(&[("index", "Home\n====\n")]);
    env.find_files().unwrap();
    assert_eq!(env.doc2path("index"), src.path().join("index.rst"));
}

#[test]
fn store_and_get_doctree_round_trips() {
    let (_src, _dt, env) = make_disk_env(&[("index", "Title\n=====\n\nBody text.\n")]);
    let tree = env.parse_doc("index").unwrap();
    env.store_doctree("index", &tree).unwrap();
    assert!(env.has_stored_doctree("index"));
    let restored = env.get_doctree("index").unwrap();
    assert_eq!(
        restored.node(restored.root()).children.len(),
        tree.node(tree.root()).children.len()
    );
}

#[test]
fn get_doctree_missing_errors() {
    let (_src, _dt, env) = make_disk_env(&[]);
    assert!(!env.has_stored_doctree("nope"));
    assert!(env.get_doctree("nope").is_err());
}

#[test]
fn get_and_resolve_doctree_delegates_to_store() {
    let (_src, _dt, env) = make_disk_env(&[("index", "Title\n=====\n")]);
    let tree = env.parse_doc("index").unwrap();
    env.store_doctree("index", &tree).unwrap();
    assert!(env.get_and_resolve_doctree("index").is_ok());
}

#[test]
fn read_all_parses_and_stores_every_doc() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", "Welcome\n=======\n\nHello.\n"),
        ("about", "About\n=====\n\nInfo.\n"),
    ]);
    env.find_files().unwrap();
    let read = env.read_all().unwrap();
    assert_eq!(read, vec!["about".to_string(), "index".to_string()]);
    assert!(env.is_doc_read("index"));
    assert!(env.is_doc_read("about"));
    assert!(env.has_stored_doctree("index"));
    assert!(env.has_stored_doctree("about"));
    assert_eq!(env.get_title("index"), Some("Welcome"));
    assert_eq!(env.get_title("about"), Some("About"));
}

#[test]
fn resolve_xref_nodes_resolves_doc_pending_xref() {
    let (_src, _dt, mut env) = make_disk_env(&[
        (
            "index",
            "Welcome\n=======\n\nSee :doc:`about` or :doc:`/about`.\n",
        ),
        ("about", "About Us\n========\n\nInfo.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let mut tree = env.get_doctree("index").unwrap();
    env.resolve_xref_nodes(&mut tree, "index");

    let html = docutilsrs::html5(
        &tree,
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );

    assert!(
        html.contains("<a class=\"reference internal\" href=\"about.html\"><span class=\"doc\">About Us</span></a>"),
        "HTML output: {html}"
    );
}

#[test]
fn read_all_persists_resolved_pending_xrefs() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", "Welcome\n=======\n\nSee :doc:`about`.\n"),
        ("about", "About Us\n========\n\nInfo.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let tree = env.get_doctree("index").unwrap();
    assert!(
        (0..tree.nodes_len()).any(|id| matches!(
            tree.node(id).kind,
            docutilsrs::doctree::NodeKind::Reference { .. }
        )),
        "read_all should persist resolved references before rendering"
    );
    assert!(!(0..tree.nodes_len()).any(|id| matches!(
        tree.node(id).kind,
        docutilsrs::doctree::NodeKind::PendingXref { .. }
    )));
}

#[test]
fn read_all_notes_toctree_and_include_dependencies() {
    let (_src, _dt, mut env) = make_disk_env(&[
        (
            "index",
            ".. toctree::\n\n   about\n   guide/intro\n\n.. include:: shared.rst\n",
        ),
        ("about", "About\n=====\n"),
        ("guide/intro", "Intro\n=====\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();
    assert_eq!(env.toc_num_entries.get("index"), Some(&2));
    assert!(env.dependencies["index"].contains("shared.rst"));
}

#[test]
fn check_consistency_flags_orphan_docs() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", ".. toctree::\n\n   about\n"),
        ("about", "About\n=====\n"),
        ("orphan", "Orphan\n======\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();
    let warnings = env.check_consistency();
    assert!(warnings.iter().any(|w| w.contains("orphan")));
    assert!(!warnings.iter().any(|w| w.contains("index:")));
    assert!(!warnings.iter().any(|w| w.contains("about:")));
}

#[test]
fn check_consistency_empty_when_all_docs_in_toctree() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", ".. toctree::\n\n   about\n"),
        ("about", "About\n=====\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();
    assert!(env.check_consistency().is_empty());
}
