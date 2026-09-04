use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_PATH: AtomicU64 = AtomicU64::new(0);

fn temporary_path(extension: &str) -> PathBuf {
    let sequence = NEXT_PATH.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "rdfhdt-cli-{}-{sequence}.{extension}",
        std::process::id()
    ))
}

fn run_with_stdin(arguments: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rdfhdt"))
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("rdfhdt binary should start");
    child
        .stdin
        .take()
        .expect("rdfhdt stdin should be available")
        .write_all(input.as_bytes())
        .expect("rdfhdt should accept stdin");
    child
        .wait_with_output()
        .expect("rdfhdt process should finish")
}

fn run(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rdfhdt"))
        .args(arguments)
        .output()
        .expect("rdfhdt binary should start")
}

#[test]
fn export_import_supports_stdin_and_stdout() {
    let hdt_path = temporary_path("hdt");
    let input = "<https://example.test/s> <https://example.test/p> \"ok\" .\n";

    let export = run_with_stdin(
        &[
            "export",
            "-",
            hdt_path.to_str().unwrap(),
            "--input-format",
            "nt",
        ],
        input,
    );
    assert!(export.status.success(), "export failed: {export:?}");

    let import = run(&[
        "import",
        hdt_path.to_str().unwrap(),
        "-",
        "--output-format",
        "nt",
    ]);
    fs::remove_file(&hdt_path).expect("temporary HDT should be removable");
    assert!(import.status.success(), "import failed: {import:?}");
    assert_eq!(String::from_utf8(import.stdout).unwrap(), input);
}

#[test]
fn export_rejects_named_graph_by_default() {
    let hdt_path = temporary_path("hdt");
    let input = "@prefix ex: <https://example.test/> . ex:g { ex:s ex:p ex:o . }\n";

    let export = run_with_stdin(
        &[
            "export",
            "-",
            hdt_path.to_str().unwrap(),
            "--input-format",
            "trig",
        ],
        input,
    );
    let _ = fs::remove_file(&hdt_path);
    assert!(!export.status.success());
    assert!(String::from_utf8_lossy(&export.stderr).contains("example.test/g"));
}

#[test]
fn export_flattens_named_graph_when_requested() {
    let hdt_path = temporary_path("hdt");
    let input = "@prefix ex: <https://example.test/> . ex:g { ex:s ex:p ex:o . }\n";

    let export = run_with_stdin(
        &[
            "export",
            "-",
            hdt_path.to_str().unwrap(),
            "--input-format",
            "trig",
            "--graph-policy",
            "flatten",
        ],
        input,
    );
    assert!(export.status.success(), "export failed: {export:?}");

    let import = run(&[
        "import",
        hdt_path.to_str().unwrap(),
        "-",
        "--output-format",
        "nt",
    ]);
    fs::remove_file(&hdt_path).expect("temporary HDT should be removable");
    assert!(import.status.success(), "import failed: {import:?}");
    assert_eq!(
        String::from_utf8(import.stdout).unwrap(),
        "<https://example.test/s> <https://example.test/p> <https://example.test/o> .\n"
    );
}

#[test]
fn hdtq_round_trip_preserves_named_graphs() {
    let hdtq_path = temporary_path("hdtq");
    let input = concat!(
        "<https://example.test/s> <https://example.test/p> <https://example.test/o> ",
        "<https://example.test/g> .\n",
        "<https://example.test/s2> <https://example.test/p> \"default\" .\n",
    );

    let export = run_with_stdin(
        &[
            "export",
            "-",
            hdtq_path.to_str().unwrap(),
            "--input-format",
            "nq",
            "--output-format",
            "hdtq",
            "--annotation-mode",
            "at",
        ],
        input,
    );
    assert!(export.status.success(), "export failed: {export:?}");

    let import = run(&[
        "import",
        hdtq_path.to_str().unwrap(),
        "-",
        "--input-format",
        "hdtq",
        "--output-format",
        "nq",
    ]);
    fs::remove_file(&hdtq_path).expect("temporary HDTQ should be removable");
    assert!(import.status.success(), "import failed: {import:?}");
    let output = String::from_utf8(import.stdout).unwrap();
    assert!(output.contains("<https://example.test/g>"));
    assert!(output.contains("\"default\""));
}

#[test]
fn help_describes_formats_and_graph_storage() {
    let help = run(&["--help"]);
    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("RDF formats: jsonld, n3, nq, nt, rdf, trig, ttl"));
    assert!(help.contains("--output-format hdtq"));

    let export_help = run(&["export", "--help"]);
    assert!(export_help.status.success());
    let export_help = String::from_utf8(export_help.stdout).unwrap();
    assert!(export_help.contains("reject"));
    assert!(export_help.contains("flatten"));
    assert!(export_help.contains("ag"));
    assert!(export_help.contains("at"));
    assert!(export_help.contains("there is no `store` value"));
}
