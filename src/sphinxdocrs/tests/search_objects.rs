//! Integration tests for search-object / index-entry population
//! (Tier **H3d**) end-to-end through `BuildEnvironment::read_all` +
//! `SearchIndex`.

use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};
use sphinxdocrs::search::SearchIndex;
use tempfile::TempDir;

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
fn domain_objects_aggregates_every_domain() {
    let (_src, _dt, mut env) = make_disk_env(&[(
        "api",
        ".. glossary::\n\n   Widget\n      A thing.\n\n.. rst:directive:: toctree\n\n   Body.\n\n.. py:module:: pkg\n\n.. py:function:: greet()\n\n.. js:module:: widgets\n\n.. js:function:: render()\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let objects = env.domain_objects();
    let domains: std::collections::HashSet<&str> =
        objects.iter().map(|(d, _)| d.as_str()).collect();
    assert!(domains.contains("std"));
    assert!(domains.contains("rst"));
    assert!(domains.contains("py"));
    assert!(domains.contains("js"));
}

#[test]
fn search_index_objects_populated_from_env() {
    let (src, _dt, mut env) = make_disk_env(&[(
        "api",
        ".. py:module:: pkg\n\n.. py:function:: greet(name)\n\n   Say hi.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let outdir = TempDir::new().unwrap();
    SearchIndex::build_and_write_with_env(&env, src.path(), outdir.path(), &["api".to_string()])
        .unwrap();

    let content = std::fs::read_to_string(outdir.path().join("searchindex.js")).unwrap();
    let inner = &content["Search.setIndex(".len()..content.len() - 1];
    let json: serde_json::Value = serde_json::from_str(inner).unwrap();

    // objtypes should contain a "py:function" entry.
    let objtypes = json["objtypes"].as_object().unwrap();
    assert!(objtypes.values().any(|v| v == "py:function"));
    // objects["pkg"] should list "greet".
    let pkg_entries = json["objects"]["pkg"].as_array().unwrap();
    assert_eq!(pkg_entries[0][4], "greet");
}

#[test]
fn search_index_entries_populated_from_env() {
    let (src, _dt, mut env) = make_disk_env(&[(
        "guide",
        "Guide\n=====\n\n.. index:: single: Widget; rendering\n\nBody.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let outdir = TempDir::new().unwrap();
    SearchIndex::build_and_write_with_env(&env, src.path(), outdir.path(), &["guide".to_string()])
        .unwrap();

    let content = std::fs::read_to_string(outdir.path().join("searchindex.js")).unwrap();
    let inner = &content["Search.setIndex(".len()..content.len() - 1];
    let json: serde_json::Value = serde_json::from_str(inner).unwrap();

    let indexentries = json["indexentries"].as_object().unwrap();
    assert!(indexentries.contains_key("widget"));
    assert!(indexentries.contains_key("widget; rendering"));
}
