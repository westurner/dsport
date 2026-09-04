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
        "index-docling",
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

#[test]
fn index_command_runs_markdown_transform_in_process() {
    let directory = tempdir().expect("temporary directory");
    let source = directory.path().join("source");
    let artifact = directory.path().join("index.json");
    fs::create_dir_all(&source).unwrap();
    fs::write(
        source.join("guide.md"),
        "# Guide\n\n```rust\nfn main() {}\n```\n",
    )
    .unwrap();

    run(docindex().args([
        "index",
        source.to_str().unwrap(),
        "--output",
        artifact.to_str().unwrap(),
        "--transform-markdown",
        "--transform-cell-split",
        "m1",
    ]));
    let artifact_text = fs::read_to_string(artifact).unwrap();
    assert!(artifact_text.contains("sphinx_nb"));
    assert!(artifact_text.contains("fn main"));
}

#[test]
fn index_docling_applies_toc_metadata_and_selection() {
    let directory = tempdir().expect("temporary directory");
    let source = directory.path().join("docling");
    let toc = directory.path().join("_toc.yml");
    let artifact = directory.path().join("docling-index.json");
    fs::create_dir_all(source.join("papers")).unwrap();
    fs::write(
        source.join("papers/one.json"),
        r#"{"schema_name":"DoclingDocument","texts":[{"label":"title","text":"Paper One"},{"text":"Indexed body"}]}"#,
    )
    .unwrap();
    fs::write(
        source.join("skip.json"),
        r#"{"schema_name":"DoclingDocument","texts":[{"text":"Skip body"}]}"#,
    )
    .unwrap();
    fs::write(
        &toc,
        "docindex:\n  paths:\n    - path: papers/*.json\n      index: true\n      build: false\n      link: true\n      metadata:\n        tags: [paper]\n",
    )
    .unwrap();

    run(docindex().args([
        "index-docling",
        "--source",
        source.to_str().unwrap(),
        "--toc",
        toc.to_str().unwrap(),
        "--output",
        artifact.to_str().unwrap(),
    ]));
    let artifact_text = fs::read_to_string(artifact).unwrap();
    assert!(artifact_text.contains("Paper One"));
    assert!(artifact_text.contains("Indexed body"));
    assert!(artifact_text.contains("paper"));
    assert!(!artifact_text.contains("Skip body"));
}

#[test]
fn literal_index_docling_flag_runs_docling_ingestion() {
    let directory = tempdir().expect("temporary directory");
    let source = directory.path().join("document.json");
    let artifact = directory.path().join("index.json");
    fs::write(
        &source,
        r#"{"schema_name":"DoclingDocument","name":"Document","texts":[{"text":"Indexed"}]}"#,
    )
    .unwrap();

    run(docindex().args([
        "--index-docling",
        "--source",
        source.to_str().unwrap(),
        "--output",
        artifact.to_str().unwrap(),
    ]));
    assert!(fs::read_to_string(artifact).unwrap().contains("Indexed"));
}
