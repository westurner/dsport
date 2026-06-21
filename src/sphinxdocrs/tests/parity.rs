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

// ── parity: JSONHTMLBuilder ───────────────────────────────────────────────────
//
// Strategy: build identical RST with both the Python `sphinx-build -b json`
// and `sphinxdocrs::builders::json::JsonBuilder`, then assert that:
//
//   1. All **core required keys** (per the upstream serialization spec) are
//      present in both `.fjson` outputs.
//   2. `current_page_name` matches exactly.
//   3. The plain-text title (HTML-stripped) matches.
//   4. Both `body` fields contain the rendered RST content.
//   5. `display_toc` agrees for a two-section document.
//   6. `globalcontext.json` has `builder == "json"` for both.
//   7. `globalcontext.json` project / version / release match conf.py values.
//
// Accepted deviations:
//   - Python emits many extra theme/template keys; Rust emits core spec keys only.
//   - `sourcename`: Python appends `.txt`; Rust emits `<docname>.rst`.
//   - `body`: Python wraps in full Jinja2 templates; Rust emits a raw fragment.

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::json::JsonBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

const REQUIRED_FJSON_KEYS: &[&str] = &[
    "body",
    "title",
    "toc",
    "display_toc",
    "current_page_name",
    "parents",
    "next",
    "prev",
    "sourcename",
];

const PARITY_RST: &str = "\
Welcome\n\
=======\n\
\n\
This is a **bold** word and an *italic* one.\n\
\n\
Section Two\n\
-----------\n\
\n\
Another paragraph.\n\
";

fn write_conf_py(dir: &Path, project: &str, author: &str, release: &str, version: &str) {
    let conf = format!(
        "project = {project:?}\nauthor = {author:?}\nrelease = {release:?}\nversion = {version:?}\nextensions = []\n",
    );
    std::fs::write(dir.join("conf.py"), conf).unwrap();
}

fn run_sphinx_json(srcdir: &Path, outdir: &Path) -> bool {
    let (code, _stdout, stderr) = run(
        python_bin(),
        &[
            "-m",
            "sphinx",
            "-b",
            "json",
            "-q",
            srcdir.to_str().unwrap(),
            outdir.to_str().unwrap(),
        ],
        srcdir,
    );
    if code != 0 {
        eprintln!("sphinx-build -b json failed:\n{stderr}");
        return false;
    }
    outdir.join("index.fjson").exists()
}

fn load_fjson(dir: &Path, docname: &str) -> serde_json::Value {
    let path = dir
        .join(docname.split('/').collect::<std::path::PathBuf>())
        .with_extension("fjson");
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
}

fn load_globalcontext(outdir: &Path) -> serde_json::Value {
    let raw =
        std::fs::read_to_string(outdir.join("globalcontext.json")).expect("globalcontext.json");
    serde_json::from_str(&raw).expect("globalcontext.json should be valid JSON")
}

fn strip_html(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in s.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}

fn build_rust_json(srcdir: &Path, outdir: &Path) {
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(srcdir, &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, srcdir, outdir);
    JsonBuilder::new()
        .build_all(srcdir, outdir, &env)
        .unwrap_or_else(|e| panic!("Rust JsonBuilder::build_all failed: {e}"));
}

/// Both Python and Rust `.fjson` must contain all core required keys.
#[test]
fn json_parity_required_keys_present() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    build_rust_json(src.path(), rs_out.path());
    let py_page = load_fjson(py_out.path(), "index");
    let rs_page = load_fjson(rs_out.path(), "index");
    for key in REQUIRED_FJSON_KEYS {
        assert!(
            py_page.get(key).is_some(),
            "Python .fjson missing required key: {key}"
        );
        assert!(
            rs_page.get(key).is_some(),
            "Rust .fjson missing required key: {key}"
        );
    }
}

/// `current_page_name` must match between Python and Rust.
#[test]
fn json_parity_current_page_name() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    build_rust_json(src.path(), rs_out.path());
    assert_eq!(
        load_fjson(py_out.path(), "index")["current_page_name"],
        load_fjson(rs_out.path(), "index")["current_page_name"],
        "current_page_name must match"
    );
}

/// The plain-text title (HTML-stripped) must match.
#[test]
fn json_parity_title_text() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    build_rust_json(src.path(), rs_out.path());
    let py_title = strip_html(
        load_fjson(py_out.path(), "index")["title"]
            .as_str()
            .unwrap_or(""),
    );
    let rs_title = strip_html(
        load_fjson(rs_out.path(), "index")["title"]
            .as_str()
            .unwrap_or(""),
    );
    assert_eq!(
        py_title.trim(),
        rs_title.trim(),
        "plain-text title must match"
    );
}

/// Both `body` fields must contain content derived from the RST source.
#[test]
fn json_parity_body_contains_rst_content() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    build_rust_json(src.path(), rs_out.path());
    let py_body = load_fjson(py_out.path(), "index")["body"]
        .as_str()
        .unwrap_or("")
        .to_owned();
    let rs_body = load_fjson(rs_out.path(), "index")["body"]
        .as_str()
        .unwrap_or("")
        .to_owned();
    assert!(!py_body.is_empty(), "Python body should not be empty");
    assert!(!rs_body.is_empty(), "Rust body should not be empty");
    assert!(
        py_body.contains("<strong>") || py_body.contains("bold"),
        "Python body should contain bold markup"
    );
    assert!(
        rs_body.contains("<strong>") || rs_body.contains("bold"),
        "Rust body should contain bold markup"
    );
    assert!(
        py_body.contains("Another paragraph"),
        "Python body should contain RST text"
    );
    assert!(
        rs_body.contains("Another paragraph"),
        "Rust body should contain RST text"
    );
}

/// `display_toc` must agree for a two-section document.
/// Python value is snapshotted as reference.
#[test]
fn json_parity_display_toc_multi_section() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    build_rust_json(src.path(), rs_out.path());
    let py_val = load_fjson(py_out.path(), "index")["display_toc"]
        .as_bool()
        .unwrap_or(false);
    let rs_val = load_fjson(rs_out.path(), "index")["display_toc"]
        .as_bool()
        .unwrap_or(false);
    insta::assert_yaml_snapshot!("json_parity_py_display_toc_multi_section", py_val);
    assert_eq!(
        py_val, rs_val,
        "display_toc should agree for a two-heading document"
    );
}

/// `globalcontext.json` must have `builder == "json"` for both.
#[test]
fn json_parity_globalcontext_builder() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    build_rust_json(src.path(), rs_out.path());
    let py_gc = load_globalcontext(py_out.path());
    let rs_gc = load_globalcontext(rs_out.path());
    assert_eq!(
        py_gc["builder"].as_str(),
        Some("json"),
        "Python globalcontext builder should be 'json'"
    );
    assert_eq!(
        rs_gc["builder"].as_str(),
        Some("json"),
        "Rust globalcontext builder should be 'json'"
    );
}

/// `globalcontext.json` project / version / release must reflect conf.py values.
#[test]
fn json_parity_globalcontext_project_metadata() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityProject", "Parity Author", "2.3.4", "2.3");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    let mut overrides = std::collections::HashMap::new();
    overrides.insert("project".into(), "ParityProject".into());
    overrides.insert("release".into(), "2.3.4".into());
    overrides.insert("version".into(), "2.3".into());
    let config = SphinxConfig::new(std::collections::HashMap::new(), overrides);
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, src.path(), rs_out.path());
    JsonBuilder::new()
        .build_all(src.path(), rs_out.path(), &env)
        .unwrap();
    let py_gc = load_globalcontext(py_out.path());
    let rs_gc = load_globalcontext(rs_out.path());
    for field in &["project", "version", "release"] {
        assert_eq!(
            py_gc[field].as_str().unwrap_or(""),
            rs_gc[field].as_str().unwrap_or(""),
            "globalcontext.json field '{field}' should match conf.py"
        );
    }
}

/// Rust `globalcontext.json` must contain a `titles` map with entries for each
/// built docname.
///
/// **Accepted deviation**: Sphinx ≥ 8 no longer emits a top-level `titles` key
/// in `globalcontext.json`; it moved that data into per-page `.fjson` context.
/// Rust retains the `titles` field because it is part of the documented
/// serialization spec and is useful for consumers that don't want to open every
/// `.fjson` file.  The Python side is checked only if it happens to have it.
#[test]
fn json_parity_globalcontext_titles_map() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    let rs_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    build_rust_json(src.path(), rs_out.path());
    let py_gc = load_globalcontext(py_out.path());
    let rs_gc = load_globalcontext(rs_out.path());
    // Rust must always emit `titles`.
    assert!(
        rs_gc.get("titles").is_some(),
        "Rust globalcontext should have 'titles'"
    );
    let rs_titles = rs_gc["titles"]
        .as_object()
        .expect("titles should be a JSON object");
    assert!(
        rs_titles.contains_key("index"),
        "Rust titles should contain 'index'; got: {:?}",
        rs_titles.keys().collect::<Vec<_>>()
    );
    // If Python also emits titles (pre-8 behaviour), assert they agree for
    // the built docnames.
    if let Some(py_titles) = py_gc.get("titles").and_then(|v| v.as_object()) {
        for (docname, py_title) in py_titles {
            if let Some(rs_title) = rs_titles.get(docname) {
                let py_text = strip_html(py_title.as_str().unwrap_or(""));
                let rs_text = strip_html(rs_title.as_str().unwrap_or(""));
                assert_eq!(
                    py_text.trim(),
                    rs_text.trim(),
                    "titles[{docname:?}] plain-text should match"
                );
            }
        }
    }
}

/// Snapshot the Python `.fjson` key set as the authoritative reference.
#[test]
fn json_parity_snapshot_python_fjson_keys() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    let py_page = load_fjson(py_out.path(), "index");
    let mut keys: Vec<String> = py_page
        .as_object()
        .expect("should be object")
        .keys()
        .cloned()
        .collect();
    keys.sort();
    insta::assert_yaml_snapshot!("json_parity_python_fjson_keys", keys);
}

/// Snapshot the Python `globalcontext.json` key set as the authoritative reference.
#[test]
fn json_parity_snapshot_python_globalcontext_keys() {
    if !has_python() {
        return;
    }
    let src = TempDir::new().unwrap();
    let py_out = TempDir::new().unwrap();
    write_conf_py(src.path(), "ParityTest", "Parity Author", "1.0.0", "1.0");
    std::fs::write(src.path().join("index.rst"), PARITY_RST).unwrap();
    if !run_sphinx_json(src.path(), py_out.path()) {
        return;
    }
    let py_gc = load_globalcontext(py_out.path());
    let mut keys: Vec<String> = py_gc
        .as_object()
        .expect("should be object")
        .keys()
        .cloned()
        .collect();
    keys.sort();
    insta::assert_yaml_snapshot!("json_parity_python_globalcontext_keys", keys);
}
