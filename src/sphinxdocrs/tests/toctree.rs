//! Integration tests for the `TocTree` adapter (Tier **H5d**) end-to-end
//! through `BuildEnvironment::read_all` + `sphinxdocrs::toctree`.
//!
//! Mirrors the resolvable subset of
//! `sphinx/tests/test_environment/test_environment_toctree.py`.

use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};
use sphinxdocrs::toctree;
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
fn read_all_then_resolve_nested_toctree() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", ".. toctree::\n\n   guide\n   reference\n"),
        ("guide", "Guide\n=====\n\n.. toctree::\n\n   guide/intro\n"),
        ("guide/intro", "Introduction\n============\n\nBody.\n"),
        ("reference", "Reference\n=========\n\nBody.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let toc = toctree::global_toctree_for_doc(&env, 0);
    assert_eq!(toc.len(), 2);
    assert_eq!(toc[0].docname, "guide");
    assert_eq!(toc[0].title, "Guide");
    assert_eq!(toc[0].children[0].docname, "guide/intro");
    assert_eq!(toc[0].children[0].title, "Introduction");
    assert_eq!(toc[1].docname, "reference");
}

#[test]
fn resolve_max_depth_two_includes_one_nested_level() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", ".. toctree::\n   :maxdepth: 2\n\n   guide\n"),
        ("guide", "Guide\n=====\n\n.. toctree::\n\n   guide/intro\n"),
        ("guide/intro", "Introduction\n============\n\nBody.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let toc = toctree::global_toctree_for_doc(&env, 2);
    assert_eq!(toc.len(), 1);
    assert_eq!(toc[0].docname, "guide");
    assert_eq!(toc[0].children.len(), 1);
    assert_eq!(toc[0].children[0].docname, "guide/intro");
}

#[test]
fn resolve_preserves_a_document_listed_by_multiple_parents() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", ".. toctree::\n\n   alpha\n   beta\n"),
        ("alpha", "Alpha\n=====\n\n.. toctree::\n\n   shared\n"),
        ("beta", "Beta\n====\n\n.. toctree::\n\n   shared\n"),
        ("shared", "Shared\n======\n\nBody.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let toc = toctree::global_toctree_for_doc(&env, 0);
    assert_eq!(toc[0].children[0].docname, "shared");
    assert_eq!(toc[1].children[0].docname, "shared");
}

#[test]
fn docname_not_in_any_toctree_is_flagged_by_check_consistency() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", ".. toctree::\n\n   guide\n"),
        ("guide", "Guide\n=====\n\nBody.\n"),
        ("orphan", "Orphan\n======\n\nNot linked from anywhere.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let warnings = env.check_consistency();
    assert!(
        warnings
            .iter()
            .any(|w| w.contains("orphan") && w.contains("isn't included in any toctree"))
    );
    // The toctree adapter agrees: "orphan" has no toctree of its own and
    // isn't reachable from the root either.
    assert!(toctree::get_toc_for(&env, "orphan").is_empty());
}

#[test]
fn secnumbers_numbers_documents_in_toctree_order() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("index", ".. toctree::\n   :numbered:\n\n   a\n   b\n"),
        ("a", "A\n=\n\nBody.\n"),
        ("b", "B\n=\n\nBody.\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();
    // `:numbered:` option-line parsing isn't part of the H2c toctree
    // scanner (it only recovers entry docnames) — note the container
    // explicitly, mirroring what a `:numbered:` toctree directive would
    // register upstream.
    env.numbered_toctrees.insert("index".to_string());

    let nums = toctree::secnumbers(&env);
    assert_eq!(nums.get("a"), Some(&vec![1]));
    assert_eq!(nums.get("b"), Some(&vec![2]));
}

#[test]
fn project_yaml_toc_discovers_and_resolves_notebooks() {
    let src = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Project docs\n============\n").unwrap();
    std::fs::write(
        src.path().join("_toc.yml"),
        "root: index\nchapters:\n  - file: notebook.ipynb\n    title: Notebook chapter\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("notebook.ipynb"),
        r##"{"cells":[{"cell_type":"markdown","metadata":{"id":"cell-1","language":"markdown"},"source":["# Notebook chapter\n","\n","Body.\n"]}],"metadata":{},"nbformat":4,"nbformat_minor":5}"##,
    )
    .unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();
    assert!(env.found_docs().contains("notebook"));

    let toc = toctree::global_toctree_for_doc(&env, 0);
    assert_eq!(toc[0].docname, "notebook");
    assert_eq!(toc[0].title, "Notebook chapter");
}

#[test]
fn inline_toctreeyml_expands_notebook_entry() {
    let src = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Project docs\n============\n\n.. toctreeyml::\n\n   chapters:\n     - file: notebook.ipynb\n       title: Inline chapter\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("notebook.ipynb"),
        r##"{"cells":[{"cell_type":"markdown","metadata":{"id":"cell-1","language":"markdown"},"source":["# Inline chapter\n"]}],"metadata":{},"nbformat":4,"nbformat_minor":5}"##,
    )
    .unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();

    assert_eq!(
        env.toctree_includes.get("index"),
        Some(&vec!["notebook".to_string()])
    );
    let toc = toctree::global_toctree_for_doc(&env, 0);
    assert_eq!(toc.len(), 1);
    assert_eq!(toc[0].docname, "notebook");
    assert_eq!(toc[0].title, "Inline chapter");
}
