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

// ── HtmlBuilder::build_all wiring ────────────────────────────────────────────
//
// Regression tests for the H5e "wire genindex/modindex into the actual HTML
// build" gap: `genindex.html` used to be an unconditional empty stub written
// by `write_static_files`, and `py-modindex.html` was never written at all,
// even though `build_genindex`/`build_modindex` (tested above) already
// existed. `HtmlBuilder::build_all` now renders both from real `env` data.

#[test]
fn build_all_writes_real_genindex_html() {
    use sphinxdocrs::builders::{html::HtmlBuilder, Builder};

    let (src, _dt, mut env) = make_disk_env(&[(
        "guide",
        "Guide\n=====\n\n.. index:: Widget, Gadget\n\nBody text.\n",
    )]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let genindex_path = out.path().join("genindex.html");
    assert!(genindex_path.exists());
    let html = std::fs::read_to_string(genindex_path).unwrap();
    assert!(
        !html.contains("not yet generated"),
        "genindex.html should no longer be the placeholder stub"
    );
    assert!(html.contains("Widget"), "html:\n{html}");
    assert!(html.contains("Gadget"), "html:\n{html}");
    assert!(html.contains("guide.html"), "html:\n{html}");
}

#[test]
fn build_all_writes_real_py_modindex_html_when_modules_present() {
    use sphinxdocrs::builders::{html::HtmlBuilder, Builder};

    let (src, _dt, mut env) = make_disk_env(&[
        ("api/alpha", ".. py:module:: pkg.alpha\n"),
        ("api/beta", ".. py:module:: pkg.beta\n"),
    ]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let modindex_path = out.path().join("py-modindex.html");
    assert!(modindex_path.exists());
    let html = std::fs::read_to_string(modindex_path).unwrap();
    assert!(html.contains("pkg.alpha"), "html:\n{html}");
    assert!(html.contains("pkg.beta"), "html:\n{html}");
}

#[test]
fn build_all_omits_py_modindex_html_when_no_modules() {
    use sphinxdocrs::builders::{html::HtmlBuilder, Builder};

    let (src, _dt, mut env) = make_disk_env(&[("guide", "Guide\n=====\n\nBody text.\n")]);
    env.find_files().unwrap();
    env.read_all().unwrap();

    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    // genindex.html is always written (even if empty of real entries);
    // py-modindex.html is only written when there's at least one module,
    // matching upstream's `PythonModuleIndex` (which is simply never added
    // to the build if `get_objects()` is empty).
    assert!(out.path().join("genindex.html").exists());
    assert!(!out.path().join("py-modindex.html").exists());
}
