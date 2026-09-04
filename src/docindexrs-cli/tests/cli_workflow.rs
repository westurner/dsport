use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn docindex() -> Command {
    Command::new(env!("CARGO_BIN_EXE_docindex"))
}

fn run(command: &mut Command) -> String {
    let output = command.output().expect("run docindex");
    assert!(
        output.status.success(),
        "docindex failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("UTF-8 command output")
}

fn write_html(directory: &Path) {
    fs::create_dir_all(directory).expect("create source directory");
    fs::write(
        directory.join("guide.html"),
        "<html><head><title>Rust Guide</title></head><body><h1>Rust Guide</h1><p>Build an index and search this guide.</p></body></html>",
    )
    .expect("write source document");
}

#[test]
fn help_lists_the_complete_workflow() {
    let output = run(docindex().arg("--help"));
    for command in [
        "index-chats",
        "index-html",
        "search",
        "status",
        "list-indices",
        "clear-index",
        "delete-index",
        "update-synonyms",
        "show-synonyms",
        "suggest-synonyms",
        "export-synonyms",
        "generate-glossary",
        "export-hdt",
    ] {
        assert!(output.contains(command), "missing {command} in help");
    }
}

#[test]
fn artifact_index_search_and_hdt_export_round_trip() {
    let directory = tempdir().expect("temporary directory");
    let source = directory.path().join("source");
    let artifact = directory.path().join("index.json");
    let hdt = directory.path().join("index.hdt");
    write_html(&source);

    let output = run(docindex().args([
        "index",
        source.to_str().unwrap(),
        "--output",
        artifact.to_str().unwrap(),
    ]));
    assert!(output.contains("indexed 1/1 documents"));
    assert!(artifact.is_file());

    let output =
        run(docindex().args(["search", artifact.to_str().unwrap(), "Rust", "--limit", "1"]));
    assert!(output.contains("Rust Guide"));

    run(docindex().args([
        "export-hdt",
        artifact.to_str().unwrap(),
        hdt.to_str().unwrap(),
    ]));
    assert!(fs::metadata(hdt).expect("HDT metadata").len() > 0);
}

#[test]
fn oxirs_backend_workflow_persists_and_manages_indexes() {
    let directory = tempdir().expect("temporary directory");
    let source = directory.path().join("source");
    let storage = directory.path().join("storage");
    let synonyms = directory.path().join("synonyms.yaml");
    write_html(&source);
    fs::write(&synonyms, "rust:\n  - rust-lang\n").expect("write synonyms");

    run(docindex().args([
        "--oxirs-storage-path",
        storage.to_str().unwrap(),
        "index-html",
        "--source",
        source.to_str().unwrap(),
        "--index",
        "sphinx",
    ]));
    let output = run(docindex().args([
        "--oxirs-storage-path",
        storage.to_str().unwrap(),
        "search",
        "Rust",
        "--index",
        "sphinx",
    ]));
    assert!(output.contains("Rust Guide"));

    run(docindex().args([
        "--oxirs-storage-path",
        storage.to_str().unwrap(),
        "update-synonyms",
        "--file",
        synonyms.to_str().unwrap(),
        "--indices",
        "sphinx",
    ]));
    let output = run(docindex().args([
        "--oxirs-storage-path",
        storage.to_str().unwrap(),
        "show-synonyms",
        "--index",
        "sphinx",
    ]));
    assert!(output.contains("rust-lang"));

    let output = run(docindex().args([
        "--oxirs-storage-path",
        storage.to_str().unwrap(),
        "clear-index",
        "--index",
        "sphinx",
        "--confirm",
    ]));
    assert!(output.contains("cleared"));
    let output = run(docindex().args([
        "--oxirs-storage-path",
        storage.to_str().unwrap(),
        "delete-index",
        "--index",
        "sphinx",
        "--confirm",
    ]));
    assert!(output.contains("deleted: true"));
}

#[test]
fn destructive_commands_require_confirmation() {
    let output = docindex()
        .args(["clear-index", "--index", "all"])
        .output()
        .expect("run docindex");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--confirm"));
}
