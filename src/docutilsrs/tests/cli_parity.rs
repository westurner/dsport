//! CLI Parity Tests
//!
//! Compares the command-line interface of the Rust implementations against the Python originals.
//!
//! ## Environment Variables
//!
//! - `DOCUTILS_TEST_WORKSPACE_ROOT`: Override the computed workspace root path.
//!   Default: Auto-detected from `CARGO_MANIFEST_DIR`.
//!
//! - `DOCUTILS_TEST_PYTHON_BIN`: Override the Python binary path.
//!   Default: Auto-detected from `VIRTUAL_ENV/bin/python`, or falls back to `python3`.
//!
//! Example usage:
//!   ```sh
//!   DOCUTILS_TEST_WORKSPACE_ROOT=/path/to/wsroot DOCUTILS_TEST_PYTHON_BIN=/usr/bin/python3 \
//!   cargo test -p docutilsrs --test cli_parity
//!   ```

use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    // Allow override via DOCUTILS_TEST_WORKSPACE_ROOT environment variable
    if let Ok(ws_root) = std::env::var("DOCUTILS_TEST_WORKSPACE_ROOT") {
        return PathBuf::from(ws_root);
    }

    // CARGO_MANIFEST_DIR is set by cargo to the package directory (src/docutilsrs).
    // The workspace root is two levels up: src/docutilsrs -> src -> workspace root.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir)
        .parent()
        .expect("src/docutilsrs has no parent")
        .parent()
        .expect("src has no parent")
        .to_path_buf()
}

fn python_bin_path() -> String {
    // Allow override via DOCUTILS_TEST_PYTHON_BIN environment variable
    if let Ok(python_bin) = std::env::var("DOCUTILS_TEST_PYTHON_BIN") {
        return python_bin;
    }

    // Try to detect Python from VIRTUAL_ENV environment variable
    if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
        let venv_python = PathBuf::from(&venv).join("bin").join("python");
        if venv_python.exists() {
            return venv_python.to_string_lossy().to_string();
        }
    }

    // Fall back to python3
    "python3".to_string()
}

/// Run the pre-compiled Rust binary with `--help` and return its stdout.
///
/// Uses the absolute binary path produced by the current cargo build instead of
/// spawning a nested `cargo run` invocation.  Nested `cargo run` inside
/// `cargo test` causes E0460 (conflicting rmeta files) and can deadlock on
/// the cargo build lock.
fn get_rust_help(bin_path: &str) -> String {
    let output = Command::new(bin_path)
        .arg("--help")
        .output()
        .unwrap_or_else(|e| panic!("Failed to run {bin_path} --help: {e}"));
    // --help output goes to stdout on success, stderr on some tools
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if stdout.is_empty() { stderr } else { stdout }
}

fn get_python_help(bin_name: &str) -> String {
    let python_cmd = format!(
        "import docutils.core; import sys; sys.argv=['{}.py', '--help']; sys.exit(docutils.core.{}())",
        bin_name,
        bin_name.replace("-rs", "")
    );
    let output = Command::new(python_bin_path())
        .env("PYTHONPATH", workspace_root().join("src/docutils/docutils"))
        .args(["-c", &python_cmd])
        .output()
        .unwrap_or_else(|e| panic!("Failed to run python for {bin_name}: {e}"));
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn extract_options(help_text: &str) -> HashSet<String> {
    let mut options = HashSet::new();
    for line in help_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("-") {
            if trimmed.chars().all(|c| c == '-') {
                continue;
            }

            for part in trimmed.split_whitespace() {
                if !part.starts_with("-") {
                    break;
                }

                let clean = part.trim_matches(',');
                let clean = clean.split('=').next().unwrap();
                let clean = clean.trim_end_matches('.');
                let clean = clean.trim_matches('"');
                let clean = clean.trim_matches('\'');

                if clean.starts_with("-")
                    && clean.len() >= 2
                    && !clean.chars().skip(1).all(|c| c == '-')
                {
                    options.insert(clean.to_string());
                }
            }
        }
    }
    options
}

macro_rules! generate_parity_test {
    ($test_name:ident, $bin_name:expr, $py_name:expr, $bin_path:expr) => {
        #[test]
        fn $test_name() {
            let rust_help = get_rust_help($bin_path);
            let python_help = get_python_help($py_name);

            let rust_options = extract_options(&rust_help);
            let python_options = extract_options(&python_help);

            let mut missing = Vec::new();
            for opt in python_options.iter() {
                if !rust_options.contains(opt) {
                    missing.push(opt.clone());
                }
            }

            missing.sort();
            if !missing.is_empty() {
                panic!(
                    "{} is missing the following options present in {}:\n{:#?}",
                    $bin_name, $py_name, missing
                );
            }
        }
    };
}

generate_parity_test!(test_rst2html5_cli_parity,    "rst2html5-rs",    "rst2html5",    env!("CARGO_BIN_EXE_rst2html5-rs"));
generate_parity_test!(test_rst2latex_cli_parity,    "rst2latex-rs",    "rst2latex",    env!("CARGO_BIN_EXE_rst2latex-rs"));
generate_parity_test!(test_rst2man_cli_parity,      "rst2man-rs",      "rst2man",      env!("CARGO_BIN_EXE_rst2man-rs"));
generate_parity_test!(test_rst2odt_cli_parity,      "rst2odt-rs",      "rst2odt",      env!("CARGO_BIN_EXE_rst2odt-rs"));
generate_parity_test!(
    test_rst2pseudoxml_cli_parity,
    "rst2pseudoxml-rs",
    "rst2pseudoxml",
    env!("CARGO_BIN_EXE_rst2pseudoxml-rs")
);
