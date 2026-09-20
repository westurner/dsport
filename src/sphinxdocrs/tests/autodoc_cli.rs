use std::path::Path;
use std::process::Command;

fn run(binary: &str, args: &[&str]) -> std::process::Output {
    Command::new(binary)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to run {binary}: {error}"))
}

#[test]
fn renders_rust_fixture_to_native_directives() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_root = root.join("tests/fixtures/h14/rust");
    let json = root.join("tests/fixtures/h14/rustdoc.json");
    let output = run(
        env!("CARGO_BIN_EXE_sphinx-autodoc-rs"),
        &[
            "--source-kind",
            "rust",
            "--rustdoc-json",
            json.to_str().unwrap(),
            source_root.to_str().unwrap(),
        ],
    );
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let rst = String::from_utf8(output.stdout).unwrap();
    assert!(rst.contains(".. rust:struct:: fixture::api::Component"));
    assert!(rst.contains(".. rust:function:: fixture::api::answer"));
    assert!(!rst.contains("hidden_item"));
}

#[test]
fn renders_lean_fixture_and_supports_output_file() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_root = root.join("tests/fixtures/h14/lean/Demo.lean");
    let tempdir = tempfile::tempdir().unwrap();
    let output_path = tempdir.path().join("demo.rst");
    let output = run(
        env!("CARGO_BIN_EXE_sphinx-autodoc-rs"),
        &[
            "--source-kind",
            "lean",
            "--output",
            output_path.to_str().unwrap(),
            source_root.to_str().unwrap(),
        ],
    );
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let rst = std::fs::read_to_string(output_path).unwrap();
    assert!(rst.contains(".. lean:theorem:: Demo.Arithmetic.add_zero"));
    assert!(rst.contains("A proposition about addition."));
    assert!(rst.contains("theorem add_zero (left : Nat)"));
}

#[test]
fn reports_backend_failures_with_source_context() {
    let output = run(
        env!("CARGO_BIN_EXE_sphinx-autodoc-rs"),
        &["--source-kind", "rust", "/definitely/missing/source"],
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("/definitely/missing/source"));
    assert!(stderr.contains("rustdoc JSON") || stderr.contains("backend"));
}
