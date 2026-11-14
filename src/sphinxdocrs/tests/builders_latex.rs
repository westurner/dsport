use std::collections::HashMap;
use tempfile::TempDir;

use sphinxdocrs::application::SphinxApp;

fn build_project(configuration: &str) -> (TempDir, TempDir) {
    let root = TempDir::new().unwrap();
    let source = root.path().join("src");
    let output = TempDir::new().unwrap();
    let doctrees = root.path().join("doctrees");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&doctrees).unwrap();
    std::fs::write(source.join("conf.py"), configuration).unwrap();
    std::fs::write(
        source.join("index.rst"),
        "Parity Matrix\n=============\n\nA default LaTeX project.\n",
    )
    .unwrap();

    let mut app =
        SphinxApp::new(&source, output.path(), &doctrees, "latex", HashMap::new()).unwrap();
    app.build().unwrap();
    (root, output)
}

#[test]
fn default_latex_documents_write_sphinx_project_file() {
    let (_root, output) = build_project("project = 'Parity Matrix'\nauthor = 'Parity'\n");

    let latex = std::fs::read_to_string(output.path().join("paritymatrix.tex")).unwrap();
    assert!(latex.starts_with("\\documentclass{sphinxmanual}\n"));
    assert!(latex.contains("\\title{Parity Matrix}"));
    assert!(latex.contains("\\author{Parity}"));
    assert!(latex.contains("A default LaTeX project."));
    assert!(!output.path().join("index.tex").exists());
}

#[test]
fn explicitly_empty_latex_documents_remains_empty() {
    let (_root, output) =
        build_project("project = 'Parity Matrix'\nauthor = 'Parity'\nlatex_documents = []\n");

    assert!(!output.path().join("paritymatrix.tex").exists());
    assert!(!output.path().join("index.tex").exists());
}
