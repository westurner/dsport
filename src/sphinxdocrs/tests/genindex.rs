//! Integration tests for the general index / module index generation
//! (Tier **H5e**) end-to-end through `BuildEnvironment::read_all` +
//! `sphinxdocrs::genindex`.

use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};
use sphinxdocrs::genindex::{build_genindex, build_modindex};
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
fn read_all_populates_genindex_from_index_directive() {
    let (_src, _dt, mut env) = make_disk_env(&[(
        "guide",
        "Guide\n=====\n\n.. index:: Widget, Gadget\n\nBody text.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let index = build_genindex(&env);
    let (_, terms) = index.iter().find(|(l, _)| l == "W").unwrap();
    assert_eq!(terms[0].name, "Widget");
    let (_, terms) = index.iter().find(|(l, _)| l == "G").unwrap();
    assert_eq!(terms[0].name, "Gadget");
}

#[test]
fn read_all_populates_modindex_from_py_modules() {
    let (_src, _dt, mut env) = make_disk_env(&[
        ("api/alpha", ".. py:module:: pkg.alpha\n"),
        ("api/beta", ".. py:module:: pkg.beta\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let index = build_modindex(&env);
    let (_, mods) = index.iter().find(|(l, _)| l == "P").unwrap();
    let names: Vec<&str> = mods.iter().map(|m| m.name.as_str()).collect();
    assert!(names.contains(&"pkg.alpha"));
    assert!(names.contains(&"pkg.beta"));
}

#[test]
fn rereading_document_clears_stale_index_entries() {
    let (src, _dt, mut env) = make_disk_env(&[("guide", ".. index:: OldTerm\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();
    assert!(
        build_genindex(&env)
            .iter()
            .any(|(_, terms)| terms.iter().any(|t| t.name == "OldTerm"))
    );

    std::fs::write(src.path().join("guide.rst"), ".. index:: NewTerm\n").unwrap();
    env.read_all().unwrap();

    let index = build_genindex(&env);
    assert!(
        !index
            .iter()
            .any(|(_, terms)| terms.iter().any(|t| t.name == "OldTerm"))
    );
    assert!(
        index
            .iter()
            .any(|(_, terms)| terms.iter().any(|t| t.name == "NewTerm"))
    );
}
