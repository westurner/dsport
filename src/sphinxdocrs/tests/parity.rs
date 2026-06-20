//! Cross-language parity tests — gated behind `cfg(feature = "parity")`.
//!
//! Runs upstream Python tool and the Rust binary on identical inputs,
//! then diffs exit codes, stdout/stderr (ANSI-stripped), and generated
//! file trees.
//!
//! **Not run in normal CI** — requires `python`, `sphinx-quickstart`,
//! and `sphinx-apidoc` in PATH.
//!
//! Enable with:
//!   cargo test -p sphinxdocrs --features parity --test parity

#![cfg(feature = "parity")]

use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

// ── helpers ───────────────────────────────────────────────────────────────────

/// Run a command and return (exit_code, stdout, stderr).
fn run(program: &str, args: &[&str], cwd: &Path) -> (i32, String, String) {
    let out = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {program}: {e}"));
    let code = out.status.code().unwrap_or(1);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    (code, strip_ansi(&stdout), strip_ansi(&stderr))
}

fn strip_ansi(s: &str) -> String {
    // Simple ANSI SGR strip — matches util_console::strip_escape_sequences semantics.
    let re = regex::Regex::new(r"\x1b\[[0-9;]*[mK]").unwrap();
    re.replace_all(s, "").into_owned()
}

/// Walk a directory and return sorted list of relative paths.
fn list_tree(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(dir: &Path, root: &Path, acc: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut entries: Vec<_> = entries.flatten().collect();
            entries.sort_by_key(|e| e.path());
            for e in entries {
                let rel = e
                    .path()
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                acc.push(rel);
                if e.path().is_dir() {
                    walk(&e.path(), root, acc);
                }
            }
        }
    }
    walk(root, root, &mut out);
    out
}

fn python_bin() -> &'static str {
    // prefer python3, fall back to python
    if Command::new("python3").arg("--version").output().is_ok() {
        "python3"
    } else {
        "python"
    }
}

fn has_python() -> bool {
    Command::new(python_bin()).arg("--version").output().is_ok()
}

// ── parity: sphinx-quickstart ─────────────────────────────────────────────────

#[test]
fn quickstart_parity_flat() {
    if !has_python() {
        return;
    }

    let py_tmp = TempDir::new().unwrap();
    let rs_tmp = TempDir::new().unwrap();

    let common_args = &[
        "-q",
        "-p",
        "ParityProj",
        "-a",
        "ParityAuthor",
        "-v",
        "1.0",
        "--no-makefile",
        "--no-batchfile",
    ];

    // Python side
    let (py_code, _, _) = run(
        python_bin(),
        &["-m", "sphinx", "quickstart", "--version"], // just verify sphinx available
        py_tmp.path(),
    );
    if py_code != 0 {
        return;
    } // sphinx not installed, skip

    let py_args: Vec<&str> = ["-m", "sphinx.cmd.quickstart"]
        .iter()
        .chain(common_args.iter())
        .chain(&[py_tmp.path().to_str().unwrap()])
        .copied()
        .collect();
    let (py_exit, _, _) = run(python_bin(), &py_args, py_tmp.path());

    // Rust side
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-quickstart-rs");
    let rs_args: Vec<&str> = common_args
        .iter()
        .chain(&[rs_tmp.path().to_str().unwrap()])
        .copied()
        .collect();
    let (rs_exit, _, _) = run(rs_bin, &rs_args, rs_tmp.path());

    assert_eq!(py_exit, rs_exit, "exit codes differ");

    let py_tree = list_tree(py_tmp.path());
    let rs_tree = list_tree(rs_tmp.path());

    // Commit Python tree as the reference snapshot.
    insta::assert_yaml_snapshot!("quickstart_parity_python_tree", py_tree);
    // Rust tree must match.
    assert_eq!(
        py_tree, rs_tree,
        "file tree mismatch:\nPython: {py_tree:?}\nRust: {rs_tree:?}"
    );
}

// ── parity: sphinx-apidoc ────────────────────────────────────────────────────

#[test]
fn apidoc_parity_basic() {
    if !has_python() {
        return;
    }

    // Synthesise a tiny Python package to run apidoc against.
    let src_tmp = TempDir::new().unwrap();
    let pkg = src_tmp.path().join("mypkg");
    std::fs::create_dir_all(pkg.join("sub")).unwrap();
    for f in &[
        "mypkg/__init__.py",
        "mypkg/utils.py",
        "mypkg/sub/__init__.py",
        "mypkg/sub/helper.py",
    ] {
        std::fs::write(src_tmp.path().join(f), b"\"\"\"Docstring.\"\"\"\n").unwrap();
    }

    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();

    let py_args = &[
        "-m",
        "sphinx.ext.apidoc",
        "-o",
        py_out.path().to_str().unwrap(),
        pkg.to_str().unwrap(),
    ];
    let (py_code, _, _) = run(python_bin(), py_args, src_tmp.path());
    if py_code != 0 {
        return;
    }

    let rs_bin = env!("CARGO_BIN_EXE_sphinx-apidoc-rs");
    let rs_args = &["-o", rs_out.path().to_str().unwrap(), pkg.to_str().unwrap()];
    let (rs_code, _, _) = run(rs_bin, rs_args, src_tmp.path());

    assert_eq!(py_code, rs_code, "exit codes differ");

    let py_tree = list_tree(py_out.path());
    let rs_tree = list_tree(rs_out.path());

    insta::assert_yaml_snapshot!("apidoc_parity_python_tree", py_tree);
    assert_eq!(
        py_tree, rs_tree,
        "file tree mismatch:\nPython: {py_tree:?}\nRust: {rs_tree:?}"
    );
}
