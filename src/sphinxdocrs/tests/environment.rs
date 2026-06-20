//! Integration tests for `sphinxdocrs::environment::BuildEnvironment`.
//!
//! Mirrors `sphinx/tests/test_environment/` (pure-algorithm subset).

use std::collections::HashMap;

use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{
    BuildEnvironment, CONFIG_CHANGED, CONFIG_EXTENSIONS_CHANGED, CONFIG_NEW, CONFIG_OK,
    CONFIG_UNSET, EnvProject, default_settings,
};

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
