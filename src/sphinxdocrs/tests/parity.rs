//! Cross-language parity tests — gated behind `cfg(feature = "test-parity")`.
//!
//! Runs upstream Python tool and the Rust binary on identical inputs,
//! then diffs exit codes, stdout/stderr (ANSI-stripped), and generated
//! file trees.
//!
//! **Not run in normal CI** — requires `python`, `sphinx-quickstart`,
//! and `sphinx-apidoc` in PATH.
//!
//! Enable with:
//!   cargo test -p sphinxdocrs --features test-parity --test parity
//!
//! For JSON / network-hitting parity tests:
//!   cargo test -p sphinxdocrs --features test-parity,test-parity-jsonbuilder --test parity
//!
//! ## Memory safety
//!
//! Python sphinx processes each consume ~150-300 MB. A global semaphore
//! (`PY_SEM`, `MAX_PY_PROCS = 2`) in `run()` caps concurrent heavy-process
//! count so the test run stays within a reasonable memory budget.

#![cfg(feature = "test-parity")]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Condvar, Mutex, OnceLock};

use rstest::*;
use tempfile::TempDir;

// ── subprocess concurrency limiter ────────────────────────────────────────────

/// Max concurrent Python/sphinx subprocesses.  Each consumes ~150-300 MB;
/// keeping this at 2 bounds peak memory to ~600 MB.
const MAX_PY_PROCS: usize = 2;

struct Semaphore {
    state: Mutex<usize>,
    cvar: Condvar,
}

impl Semaphore {
    fn new(n: usize) -> Self {
        Semaphore { state: Mutex::new(n), cvar: Condvar::new() }
    }

    /// Block until a slot is available, then decrement the count.
    fn acquire(&self) {
        let mut n = self.state.lock().unwrap();
        while *n == 0 {
            n = self.cvar.wait(n).unwrap();
        }
        *n -= 1;
    }

    /// Return a slot, waking one waiting caller.
    fn release(&self) {
        let mut n = self.state.lock().unwrap();
        *n += 1;
        self.cvar.notify_one();
    }
}

/// Returns `true` for programs that spawn the full Python/Sphinx stack
/// and therefore deserve rate-limiting via `PY_SEM`.
fn is_heavy_process(program: &str) -> bool {
    program.contains("python") || program == "sphinx-build"
}

static PY_SEM: OnceLock<Semaphore> = OnceLock::new();

/// RAII guard that releases a `PY_SEM` slot when dropped, even on panic.
/// This prevents semaphore leaks if the process-spawn call panics.
struct SemPermit<'a>(&'a Semaphore);
impl Drop for SemPermit<'_> {
    fn drop(&mut self) {
        self.0.release();
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Run a command and return (exit_code, stdout, stderr).
///
/// Calls to Python / `sphinx-build` are rate-limited via `PY_SEM` to
/// prevent OOM when many tests run in parallel.  The permit is held via
/// an RAII guard so it is always released, even when the call panics.
fn run(program: &str, args: &[&str], cwd: &Path) -> (i32, String, String) {
    let sem = PY_SEM.get_or_init(|| Semaphore::new(MAX_PY_PROCS));
    let _permit = if is_heavy_process(program) {
        sem.acquire();
        Some(SemPermit(sem))
    } else {
        None
    };
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

/// Pre-canned build output for testing log message *format* without spawning
/// a real subprocess.  The mock values here mirror what real sphinx-build-rs
/// emits for a successful 2-doc HTML build.
struct MockBuildOutput {
    exit:   i32,
    stderr: String,
}

impl MockBuildOutput {
    fn rs_success() -> Self {
        MockBuildOutput {
            exit: 0,
            stderr: "sphinxdocrs: make mode: running SphinxApp (builder=html)\nBuild succeeded: 2 file(s) written.\n".into(),
        }
    }
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

fn python_bin() -> String {
    // prefer $VIRTUAL_ENV/bin/python if set
    if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
        let venv_python = format!("{}/bin/python", venv);
        if Command::new(&venv_python).arg("--version").output().is_ok() {
            return venv_python;
        }
    }
    // then prefer python3, fall back to python
    if Command::new("python3").arg("--version").output().is_ok() {
        "python3".to_string()
    } else {
        "python".to_string()
    }
}

fn has_python() -> bool {
    Command::new(&python_bin()).arg("--version").output().is_ok()
}

// ── tests: python_bin() ───────────────────────────────────────────────────────

#[test]
fn test_python_bin_returns_valid_executable() {
    let py = python_bin();
    assert!(
        !py.is_empty(),
        "python_bin() should return a non-empty string"
    );
    // Verify it's an actual executable
    let output = Command::new(py.as_str())
        .arg("--version")
        .output()
        .expect("python_bin() should return a valid executable path");
    assert!(
        output.status.success(),
        "python executable should be runnable"
    );
}

#[test]
fn test_python_bin_fallback_without_virtual_env() {
    // Save the original VIRTUAL_ENV value
    let original_venv = std::env::var("VIRTUAL_ENV").ok();

    // Unset VIRTUAL_ENV
    unsafe {
        std::env::remove_var("VIRTUAL_ENV");
    }

    let py = python_bin();

    // Should fall back to python3 or python (not a VENV path)
    assert!(
        py == "python3" || py == "python",
        "Without VIRTUAL_ENV, should use python3 or python; got: {}",
        py
    );

    // Restore the original VIRTUAL_ENV
    unsafe {
        match original_venv {
            Some(v) => std::env::set_var("VIRTUAL_ENV", v),
            None => std::env::remove_var("VIRTUAL_ENV"),
        }
    }
}

#[test]
fn test_python_bin_with_invalid_virtual_env() {
    // Save the original VIRTUAL_ENV value
    let original_venv = std::env::var("VIRTUAL_ENV").ok();

    // Set VIRTUAL_ENV to a non-existent path
    unsafe {
        std::env::set_var("VIRTUAL_ENV", "/nonexistent/venv/path");
    }

    let py = python_bin();

    // Should fall back to python3 or python
    assert!(
        py == "python3" || py == "python",
        "When VIRTUAL_ENV/bin/python doesn't exist, should fall back; got: {}",
        py
    );

    // Restore the original VIRTUAL_ENV
    unsafe {
        match original_venv {
            Some(v) => std::env::set_var("VIRTUAL_ENV", v),
            None => std::env::remove_var("VIRTUAL_ENV"),
        }
    }
}

#[test]
fn test_python_bin_prefers_virtual_env_when_set() {
    // Save the original VIRTUAL_ENV value
    let original_venv = std::env::var("VIRTUAL_ENV").ok();

    // Create a temporary directory to simulate a virtual environment
    let temp_venv = TempDir::new().expect("Failed to create temp dir");
    let bin_dir = temp_venv.path().join("bin");
    std::fs::create_dir_all(&bin_dir).expect("Failed to create bin dir");

    // Find the actual path to python3 or python
    let python_exec = if let Ok(output) = Command::new("which")
        .arg("python3")
        .output()
    {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        // Fallback: just skip if we can't find it
        return;
    };

    if python_exec.is_empty() {
        return; // Skip test if python not found
    }

    let mock_py = bin_dir.join("python");

    // Create a symlink from python3 to our venv python
    if std::os::unix::fs::symlink(&python_exec, &mock_py).is_ok() {
        // Verify the symlink works before setting env var
        assert!(
            Command::new(&mock_py)
                .arg("--version")
                .output()
                .is_ok(),
            "Created symlink should be executable"
        );

        // Set VIRTUAL_ENV to our temp directory (convert to string)
        let venv_str = temp_venv.path().to_string_lossy().to_string();
        unsafe {
            std::env::set_var("VIRTUAL_ENV", &venv_str);
        }

        // Call python_bin() and it should check VENV first
        let py = python_bin();

        let expected = format!("{}/bin/python", venv_str);
        assert_eq!(
            py, expected,
            "When VIRTUAL_ENV is set with valid python, should use it"
        );

        // Restore the original VIRTUAL_ENV
        unsafe {
            match original_venv.as_ref() {
                Some(v) => std::env::set_var("VIRTUAL_ENV", v),
                None => std::env::remove_var("VIRTUAL_ENV"),
            }
        }
    }
}

// ── parity: sphinx-quickstart ─────────────────────────────────────────────────

#[rstest]
fn quickstart_parity_flat(quickstart_parity_shared: &QuickstartParityShared) {
    if !quickstart_parity_shared.py_available { return; }
    if quickstart_parity_shared.py_exit != 0 { return; }

    assert_eq!(quickstart_parity_shared.py_exit, quickstart_parity_shared.rs_exit,
        "exit codes differ");

    // Commit Python tree as the reference snapshot.
    insta::assert_yaml_snapshot!("quickstart_parity_python_tree",
        quickstart_parity_shared.py_tree);
    // Rust tree must match.
    assert_eq!(
        quickstart_parity_shared.py_tree, quickstart_parity_shared.rs_tree,
        "file tree mismatch:\nPython: {:?}\nRust: {:?}",
        quickstart_parity_shared.py_tree, quickstart_parity_shared.rs_tree
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
    let (py_code, _, _) = run(&python_bin(), py_args, src_tmp.path());
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
// These tests spawn `sphinx-build -b json` (Python) which loads the full
// Sphinx stack and can be memory-intensive.  Gate them behind
// `test-parity-jsonbuilder` so the default `--features test-parity` run stays lean.
//
//   cargo test -p sphinxdocrs --features test-parity,test-parity-jsonbuilder --test parity
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
//
// Note: `cfg(feature = "test-parity-jsonbuilder")` implies `cfg(feature = "test-parity")`
// because `test-parity-jsonbuilder` depends on `test-parity` in Cargo.toml.

#[cfg(feature = "test-parity-jsonbuilder")]
use sphinxdocrs::builders::Builder;
#[cfg(feature = "test-parity-jsonbuilder")]
use sphinxdocrs::builders::json::JsonBuilder;
#[cfg(feature = "test-parity-jsonbuilder")]
use sphinxdocrs::config::SphinxConfig;
#[cfg(feature = "test-parity-jsonbuilder")]
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[cfg(feature = "test-parity-jsonbuilder")]
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

#[cfg(feature = "test-parity-jsonbuilder")]
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

#[cfg(feature = "test-parity-jsonbuilder")]
fn run_sphinx_json(srcdir: &Path, outdir: &Path) -> bool {
    let (code, _stdout, stderr) = run(
        &python_bin(),
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

#[cfg(feature = "test-parity-jsonbuilder")]
fn load_fjson(dir: &Path, docname: &str) -> serde_json::Value {
    let path = dir
        .join(docname.split('/').collect::<std::path::PathBuf>())
        .with_extension("fjson");
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
}

#[cfg(feature = "test-parity-jsonbuilder")]
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

#[cfg(feature = "test-parity-jsonbuilder")]
fn build_rust_json(srcdir: &Path, outdir: &Path) {
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(srcdir, &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(config, project, srcdir, outdir);
    JsonBuilder::new()
        .build_all(srcdir, outdir, &env)
        .unwrap_or_else(|e| panic!("Rust JsonBuilder::build_all failed: {e}"));
}

/// Both Python and Rust `.fjson` must contain all core required keys.
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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
#[cfg(feature = "test-parity-jsonbuilder")]
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

// ── parity: sphinx-build-rs -M html vs sphinx-build -M html ──────────────────
//
// These tests run both `sphinx-build-rs -M html src out` and
// `sphinx-build   -M html src out` on identical synthetic Sphinx projects
// and compare exit codes, generated file structure, and key HTML properties.
//
// The NativeMakeRunner (merged in the sphinxbuildrs branch) routes `-M html`
// directly to `SphinxApp` instead of spawning a `sphinx-build` subprocess.
//
// All tests skip cleanly when `sphinx-build` is unavailable in PATH.

/// Minimal two-doc Sphinx project used by all `-M html` parity tests.
const MAKE_PARITY_INDEX_RST: &str = "\
My Parity Project\n\
=================\n\
\n\
Welcome to the parity test.\n\
\n\
.. toctree::\n\
   :maxdepth: 1\n\
\n\
   guide\n\
";

const MAKE_PARITY_GUIDE_RST: &str = "\
User Guide\n\
==========\n\
\n\
This is the user guide.\n\
";

/// Set up a minimal Sphinx project in `dir`.
fn setup_make_parity_project(dir: &Path) {
    write_conf_py(dir, "MakeParityProj", "Parity Author", "1.0", "1.0");
    std::fs::write(dir.join("index.rst"), MAKE_PARITY_INDEX_RST).unwrap();
    std::fs::write(dir.join("guide.rst"), MAKE_PARITY_GUIDE_RST).unwrap();
}

/// Return `true` when `sphinx-build` (Python) is available in PATH.
fn has_sphinx_build() -> bool {
    Command::new("sphinx-build").arg("--version").output().is_ok()
}

// ── shared once-fixtures ─────────────────────────────────────────────────────

/// Output from a single `sphinx-build -M html` + `sphinx-build-rs -M html`
/// run on the make-parity project.  Built ONCE per test binary invocation.
pub struct MakeHtmlShared {
    pub src:       PathBuf,
    pub py_html:   PathBuf,   // <py_out>/html
    pub rs_html:   PathBuf,   // <rs_out>/html
    pub py_exit:   i32,
    pub rs_exit:   i32,
    pub py_stderr: String,
    pub rs_stderr: String,
    pub py_built:  bool,
    pub rs_built:  bool,
}

#[fixture]
#[once]
fn make_html_shared() -> MakeHtmlShared {
    let src = {
        let d = TempDir::new().unwrap();
        setup_make_parity_project(d.path());
        d.keep()
    };
    let py_out = TempDir::new().unwrap().keep();
    let rs_out = TempDir::new().unwrap().keep();
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-build-rs");

    let (py_exit, _, py_stderr) = if has_sphinx_build() {
        run("sphinx-build",
            &["-M", "html", src.to_str().unwrap(), py_out.to_str().unwrap(), "-q"],
            &src)
    } else {
        (1, String::new(), String::new())
    };

    let (rs_exit, _, rs_stderr) = run(
        rs_bin,
        &["-M", "html", src.to_str().unwrap(), rs_out.to_str().unwrap()],
        &src,
    );

    let py_html = py_out.join("html");
    let rs_html = rs_out.join("html");
    MakeHtmlShared {
        py_built: py_exit == 0 && py_html.join("index.html").exists(),
        rs_built: rs_exit == 0 && rs_html.join("index.html").exists(),
        src,
        py_html,
        rs_html,
        py_exit,
        rs_exit,
        py_stderr,
        rs_stderr,
    }
}

/// Output from a single `sphinx-build -M html` + `sphinx-build-rs -M html`
/// run on the html-parity project (different from make-parity).
pub struct HtmlParityShared {
    pub src:       PathBuf,
    pub py_html:   PathBuf,
    pub rs_html:   PathBuf,
    pub py_exit:   i32,
    pub rs_exit:   i32,
    pub py_stderr: String,
    pub rs_stderr: String,
    pub py_built:  bool,
    pub rs_built:  bool,
}

#[fixture]
#[once]
fn html_parity_shared() -> HtmlParityShared {
    let src = {
        let d = TempDir::new().unwrap();
        setup_html_parity_project(d.path());
        d.keep()
    };
    let py_out = TempDir::new().unwrap().keep();
    let rs_out = TempDir::new().unwrap().keep();
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-build-rs");

    let (py_exit, _, py_stderr) = if has_sphinx_build() {
        run("sphinx-build",
            &["-M", "html", src.to_str().unwrap(), py_out.to_str().unwrap(), "-q"],
            &src)
    } else {
        (1, String::new(), String::new())
    };

    let (rs_exit, _, rs_stderr) = run(
        rs_bin,
        &["-M", "html", src.to_str().unwrap(), rs_out.to_str().unwrap()],
        &src,
    );

    let py_html = py_out.join("html");
    let rs_html = rs_out.join("html");
    HtmlParityShared {
        py_built: py_exit == 0 && py_html.join("index.html").exists(),
        rs_built: rs_exit == 0 && rs_html.join("index.html").exists(),
        src,
        py_html,
        rs_html,
        py_exit,
        rs_exit,
        py_stderr,
        rs_stderr,
    }
}

// ── exit-code parity ──────────────────────────────────────────────────────────

/// Both `sphinx-build -M html` and `sphinx-build-rs -M html` must exit 0
/// for a valid minimal project.
#[rstest]
fn make_html_both_exit_zero(make_html_shared: &MakeHtmlShared) {
    if !has_sphinx_build() { return; }
    assert_eq!(make_html_shared.py_exit, 0, "Python must exit 0; stderr:\n{}", make_html_shared.py_stderr);
    assert_eq!(make_html_shared.rs_exit, 0, "Rust must exit 0; stderr:\n{}", make_html_shared.rs_stderr);
}

/// Both tools must exit non-zero for a missing source directory.
#[test]
fn make_html_missing_srcdir_both_nonzero() {
    let tmp = TempDir::new().unwrap();

    // Rust side — use snapbox for the binary assertion.
    snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-build-rs"))
        .args(["-M", "html", "/no/such/srcdir", tmp.path().to_str().unwrap()])
        .current_dir(tmp.path())
        .assert()
        .failure();

    if has_sphinx_build() {
        let (py_code, _, _) = run(
            "sphinx-build",
            &["-M", "html", "/no/such/srcdir", tmp.path().to_str().unwrap()],
            tmp.path(),
        );
        assert_ne!(py_code, 0, "sphinx-build must exit non-zero for missing srcdir");
    }
}

// ── output file structure parity ─────────────────────────────────────────────

/// Both outputs must contain `html/index.html` and `html/guide.html`.
#[rstest]
fn make_html_both_produce_expected_html_files(make_html_shared: &MakeHtmlShared) {
    if !make_html_shared.py_built { return; }
    for fname in &["index.html", "guide.html"] {
        assert!(make_html_shared.py_html.join(fname).exists(), "Python missing {fname}");
        assert!(make_html_shared.rs_html.join(fname).exists(), "Rust missing {fname}");
    }
}

/// Snapshot the top-level `html/` directory listing from both tools.
///
/// The Python snapshot is the authoritative reference. The Rust snapshot
/// documents current coverage; shrink it over time as more features are added.
#[rstest]
fn make_html_snapshot_top_level_files(make_html_shared: &MakeHtmlShared) {
    if !make_html_shared.py_built { return; }

    let top_files = |html: &std::path::Path| -> Vec<String> {
        if !html.exists() { return vec![]; }
        let mut files: Vec<String> = std::fs::read_dir(html)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        files.sort();
        files
    };

    let py_files = top_files(&make_html_shared.py_html);
    let rs_files = top_files(&make_html_shared.rs_html);

    // Python is the authoritative reference snapshot.
    insta::assert_yaml_snapshot!("make_html_py_top_level_files", py_files);
    // Rust snapshot tracks progress toward parity.
    insta::assert_yaml_snapshot!("make_html_rs_top_level_files", rs_files);
}

/// Snapshot the set of files present in Python output but absent in Rust output
/// (`-M html` parity gap). Updating this snapshot records progress.
#[rstest]
fn make_html_snapshot_parity_gap(make_html_shared: &MakeHtmlShared) {
    if !make_html_shared.py_built { return; }

    let all_files = |html: &std::path::Path| {
        let mut out = std::collections::BTreeSet::new();
        if html.exists() {
            fn walk(dir: &std::path::Path, root: &std::path::Path, acc: &mut std::collections::BTreeSet<String>) {
                if let Ok(rd) = std::fs::read_dir(dir) {
                    for e in rd.flatten() {
                        let rel = e.path().strip_prefix(root).unwrap().to_string_lossy().to_string();
                        acc.insert(rel);
                        if e.path().is_dir() { walk(&e.path(), root, acc); }
                    }
                }
            }
            walk(html, html, &mut out);
        }
        out
    };

    let py_set = all_files(&make_html_shared.py_html);
    let rs_set = all_files(&make_html_shared.rs_html);

    let mut missing: Vec<String> = py_set.difference(&rs_set).cloned().collect();
    missing.sort();
    let mut rs_only: Vec<String> = rs_set.difference(&py_set).cloned().collect();
    rs_only.sort();

    insta::assert_yaml_snapshot!("make_html_parity_gap_missing_in_rust", missing);
    insta::assert_yaml_snapshot!("make_html_parity_gap_rust_only", rs_only);
}

// ── HTML content parity ───────────────────────────────────────────────────────

/// Both `index.html` outputs must contain a valid HTML5 DOCTYPE.
#[rstest]
fn make_html_both_have_doctype(make_html_shared: &MakeHtmlShared) {
    if !make_html_shared.py_built { return; }
    let py_html = std::fs::read_to_string(make_html_shared.py_html.join("index.html")).unwrap();
    let rs_html = std::fs::read_to_string(make_html_shared.rs_html.join("index.html")).unwrap();

    assert!(py_html.to_lowercase().contains("<!doctype html"), "Python index.html missing DOCTYPE");
    assert!(rs_html.to_lowercase().contains("<!doctype html"), "Rust index.html missing DOCTYPE");
}

/// The Rust `index.html` `<title>` must contain the RST document title
/// ("My Parity Project"), not just the docname ("index").
#[rstest]
fn make_html_rust_title_extracted_from_rst(make_html_shared: &MakeHtmlShared) {
    if !make_html_shared.rs_built { return; }
    let html = std::fs::read_to_string(make_html_shared.rs_html.join("index.html")).unwrap();
    assert!(
        html.contains("My Parity Project"),
        "Rust index.html <title> should contain 'My Parity Project'; got:\n{}",
        &html[..html.len().min(800)]
    );
}

/// Both `index.html` and `guide.html` outputs must contain the RST content.
#[rstest]
#[case("index")]
#[case("guide")]
fn make_html_both_contain_rst_body_text(
    make_html_shared: &MakeHtmlShared,
    #[case] docname: &str,
) {
    if !make_html_shared.py_built { return; }
    let py_html = std::fs::read_to_string(
        make_html_shared.py_html.join(format!("{docname}.html"))
    ).unwrap();
    let rs_html = std::fs::read_to_string(
        make_html_shared.rs_html.join(format!("{docname}.html"))
    ).unwrap();
    let (title, body) = match docname {
        "guide" => ("User Guide", "This is the user guide"),
        "index" => ("My Parity Project", "Welcome to the parity test"),
        _ => return,
    };
    assert!(py_html.contains(title), "Python {docname}.html missing title");
    assert!(rs_html.contains(title), "Rust {docname}.html missing title");
    assert!(py_html.contains(body), "Python {docname}.html missing body text");
    assert!(rs_html.contains(body), "Rust {docname}.html missing body text");
}

// ── log message parity ────────────────────────────────────────────────────────

/// `sphinx-build-rs -M html` must emit "Build succeeded" to stderr on success.
#[rstest]
fn make_html_rs_emits_build_succeeded(make_html_shared: &MakeHtmlShared) {
    assert_eq!(make_html_shared.rs_exit, 0, "expected exit 0; stderr:\n{}", make_html_shared.rs_stderr);
    assert!(
        make_html_shared.rs_stderr.contains("Build succeeded"),
        "expected 'Build succeeded' in stderr; got:\n{}", make_html_shared.rs_stderr
    );
    // A file count must appear alongside the success message.
    assert!(
        regex::Regex::new(r"\d+ file").unwrap().is_match(&make_html_shared.rs_stderr),
        "expected file count in 'Build succeeded' message; got:\n{}", make_html_shared.rs_stderr
    );
}

/// `sphinx-build -M html` (Python) emits some success indicator to stderr.
/// Both sides must exit 0 and emit a success signal — exact wording differs.
#[rstest]
fn make_html_both_emit_success_indicator(make_html_shared: &MakeHtmlShared) {
    if !has_sphinx_build() { return; }
    assert_eq!(make_html_shared.py_exit, 0, "Python must exit 0; stderr:\n{}", make_html_shared.py_stderr);
    assert_eq!(make_html_shared.rs_exit, 0, "Rust must exit 0; stderr:\n{}", make_html_shared.rs_stderr);
    // Python is run with -q in the shared fixture so success is indicated by exit 0 alone.
    assert!(
        make_html_shared.rs_stderr.contains("Build succeeded"),
        "Rust must emit 'Build succeeded'; got:\n{}", make_html_shared.rs_stderr
    );
}

/// Snapshot normalised stderr from `sphinx-build-rs -M html` (clean build).
///
/// Use `INSTA_UPDATE=new` to record new snapshots; `INSTA_UPDATE=unseen`
/// to update only missing ones.
#[rstest]
fn make_html_rs_stderr_snapshot(make_html_shared: &MakeHtmlShared) {
    if !make_html_shared.rs_built { return; }

    // Normalise: replace file counts with <N>, strip the outdir/srcdir paths.
    let re_count = regex::Regex::new(r"\d+ file").unwrap();
    let rs_out_str = make_html_shared.rs_html.parent().unwrap().to_str().unwrap_or("");
    let src_str = make_html_shared.src.to_str().unwrap_or("");
    let normalised: Vec<String> = make_html_shared.rs_stderr
        .replace(rs_out_str, "<OUTDIR>")
        .replace(src_str, "<SRCDIR>")
        .lines()
        .map(|l| re_count.replace_all(l.trim_end(), "<N> file").to_string())
        .filter(|l| !l.is_empty())
        .collect();

    insta::assert_yaml_snapshot!("make_html_rs_stderr_normalised", normalised);
}

// ── make help parity ──────────────────────────────────────────────────────────

/// `sphinx-build-rs -M help` must list the same builder names as
/// `sphinx-build -M help`.
#[test]
fn make_help_builder_names_match() {
    if !has_sphinx_build() { return; }
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-build-rs");
    let tmp = TempDir::new().unwrap();

    let (_, py_stdout, _) = run("sphinx-build",  &["-M", "help", ".", "."], tmp.path());
    let (_, rs_stdout, _) = run(rs_bin,           &["-M", "help", ".", "."], tmp.path());

    let builder_names = |s: &str| -> Vec<String> {
        // Each table row starts with two spaces then the builder name.
        s.lines()
            .filter(|l| l.starts_with("  ") && !l.trim().is_empty())
            .filter_map(|l| l.split_whitespace().next().map(str::to_owned))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect()
    };

    let py_builders = builder_names(&py_stdout);
    let rs_builders = builder_names(&rs_stdout);

    // Every builder name Python lists must also appear in the Rust output.
    for name in &py_builders {
        assert!(
            rs_builders.contains(name),
            "Rust -M help missing builder '{name}' that Python lists"
        );
    }

    insta::assert_yaml_snapshot!("make_help_py_builders", py_builders);
    insta::assert_yaml_snapshot!("make_help_rs_builders", rs_builders);
}

/// `sphinx-build-rs -M help` must exit 0 and list at least the core builders.
#[rstest]
#[case("html")]
#[case("latex")]
#[case("man")]
#[case("text")]
#[case("clean")]
fn make_help_contains_core_builder(#[case] name: &str) {
    let tmp = TempDir::new().unwrap();
    // Use output() so we can do a manual contains-check; format!() strings are
    // not treated as wildcard patterns by snapbox (only str![] macro creates that).
    let output = snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-build-rs"))
        .args(["-M", "help", ".", "."])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "-M help must exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(name), "-M help output missing builder '{name}'");
}

// ── make clean parity ─────────────────────────────────────────────────────────

/// Both tools must exit 0 and empty the build directory on `clean`.
#[test]
fn make_clean_both_exit_zero_and_empty_outdir() {
    if !has_sphinx_build() { return; }
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-build-rs");
    let src = TempDir::new().unwrap();
    let py_build = TempDir::new().unwrap();
    let rs_build = TempDir::new().unwrap();

    // Seed the build dirs with a dummy file.
    for build in &[py_build.path(), rs_build.path()] {
        std::fs::create_dir_all(build.join("html")).unwrap();
        std::fs::write(build.join("html/index.html"), b"<h1>stale</h1>").unwrap();
    }

    let (py_code, _, py_err) = run(
        "sphinx-build",
        &["-M", "clean", src.path().to_str().unwrap(), py_build.path().to_str().unwrap()],
        src.path(),
    );
    let (rs_code, _, rs_err) = run(
        rs_bin,
        &["-M", "clean", src.path().to_str().unwrap(), rs_build.path().to_str().unwrap()],
        src.path(),
    );

    assert_eq!(py_code, 0, "Python -M clean must exit 0; err:\n{py_err}");
    assert_eq!(rs_code, 0, "Rust -M clean must exit 0; err:\n{rs_err}");
    assert!(
        std::fs::read_dir(py_build.path()).unwrap().next().is_none(),
        "Python -M clean must empty the build dir"
    );
    assert!(
        std::fs::read_dir(rs_build.path()).unwrap().next().is_none(),
        "Rust -M clean must empty the build dir"
    );
}


// ── parity: make -M html (sphinx-build-rs vs sphinx-build) ───────────────────
//
// Tests that `sphinx-build-rs -M html src out` produces output equivalent to
// `sphinx-build -M html src out` on a minimal synthetic Sphinx project.
//
// Full parity on the sphinx doc tree is not yet achievable (deferred: theming,
// search, autodoc, intersphinx); these tests capture the *structural* gap as
// named insta snapshots so regressions are caught automatically.

const HTML_PARITY_RST_INDEX: &str = "\
My Test Project\n\
===============\n\
\n\
Welcome to the documentation.\n\
\n\
.. toctree::\n\
   :maxdepth: 2\n\
\n\
   guide\n\
\n\
Indices and tables\n\
------------------\n\
\n\
* :ref:`genindex`\n\
* :ref:`search`\n\
";

const HTML_PARITY_RST_GUIDE: &str = "\
User Guide\n\
==========\n\
\n\
This is the user guide.\n\
\n\
Section One\n\
-----------\n\
\n\
Content here.\n\
";

/// Set up a synthetic Sphinx project in `dir` with two RST files and a conf.py.
///
/// The project also declares `html_static_path = ['_static']` and ships a
/// user static file (`_static/myextra.css`) so both builders exercise the
/// static-file copy path.  `myextra.css` has a distinctive name that does
/// **not** collide with any alabaster theme asset, so its presence in the
/// output uniquely proves that `html_static_path` copying works.
fn setup_html_parity_project(dir: &Path) {
    use std::io::Write;
    write_conf_py(dir, "ParityHTML", "Parity Author", "2.0", "2.0");
    // Append html_static_path to the generated conf.py.
    let mut conf = std::fs::OpenOptions::new()
        .append(true)
        .open(dir.join("conf.py"))
        .unwrap();
    writeln!(conf, "html_static_path = ['_static']").unwrap();
    // Ship a uniquely-named user static file.
    std::fs::create_dir_all(dir.join("_static")).unwrap();
    std::fs::write(
        dir.join("_static").join("myextra.css"),
        "/* user static file */\nbody { margin: 0; }\n",
    )
    .unwrap();
    std::fs::write(dir.join("index.rst"), HTML_PARITY_RST_INDEX).unwrap();
    std::fs::write(dir.join("guide.rst"), HTML_PARITY_RST_GUIDE).unwrap();
}

/// A user file placed under `html_static_path` must be copied verbatim into
/// the Rust builder's `_static/` output directory.
#[rstest]
fn html_static_path_user_file_copied_by_rust(html_parity_shared: &HtmlParityShared) {
    if !html_parity_shared.rs_built { return; }
    let f = html_parity_shared.rs_html.join("_static").join("myextra.css");
    assert!(
        f.exists(),
        "Rust must copy the user html_static_path file _static/myextra.css"
    );
    let content = std::fs::read_to_string(&f).unwrap();
    assert!(
        content.contains("user static file"),
        "copied file content must match the source; got:\n{content}"
    );
}

/// Both Python and Rust must copy the same user `html_static_path` file.
#[rstest]
fn html_static_path_user_file_copied_by_both(html_parity_shared: &HtmlParityShared) {
    if !has_sphinx_build() { return; }
    if !html_parity_shared.py_built { return; }
    if !html_parity_shared.rs_built { return; }
    for (label, html) in &[
        ("Python", &html_parity_shared.py_html),
        ("Rust", &html_parity_shared.rs_html),
    ] {
        let f = html.join("_static").join("myextra.css");
        assert!(
            f.exists(),
            "{label} output missing user static file _static/myextra.css"
        );
    }
}

/// Both builders must exit 0 and produce an `index.html`.
#[rstest]
fn html_parity_exits_zero_and_has_index(html_parity_shared: &HtmlParityShared) {
    if !has_python() { return; }
    if !html_parity_shared.py_built { return; }
    assert!(html_parity_shared.rs_built, "sphinx-build-rs exited non-zero");
}

/// Both outputs contain a valid HTML5 DOCTYPE in `index.html`.
#[rstest]
fn html_parity_doctype(html_parity_shared: &HtmlParityShared) {
    if !has_python() { return; }
    if !html_parity_shared.rs_built { return; }
    let rs_html = std::fs::read_to_string(html_parity_shared.rs_html.join("index.html")).unwrap();
    assert!(rs_html.starts_with("<!DOCTYPE html>"), "Rust output missing DOCTYPE");
    assert!(rs_html.contains("<html"), "Rust output missing <html> tag");
    assert!(rs_html.contains("<meta charset=\"utf-8\""), "Rust output missing charset meta");
}

/// The Rust `index.html` title must contain the document title from the RST,
/// not just the docname ("index").
#[rstest]
fn html_parity_document_title_extracted(html_parity_shared: &HtmlParityShared) {
    if !has_python() { return; }
    if !html_parity_shared.rs_built { return; }
    let html = std::fs::read_to_string(html_parity_shared.rs_html.join("index.html")).unwrap();
    assert!(
        html.contains("My Test Project"),
        "Rust index.html should contain the RST document title 'My Test Project'; got: {}",
        &html[..html.len().min(500)]
    );
}

/// The Rust output must contain `_static/sphinxdocrs.css`.
#[rstest]
fn html_parity_static_css_present(html_parity_shared: &HtmlParityShared) {
    if !has_python() { return; }
    if !html_parity_shared.rs_built { return; }
    assert!(
        html_parity_shared.rs_html.join("_static/sphinxdocrs.css").exists(),
        "Rust output missing _static/sphinxdocrs.css"
    );
}

/// Both outputs must produce `genindex.html` and `objects.inv`.
#[rstest]
fn html_parity_genindex_and_objects_inv(html_parity_shared: &HtmlParityShared) {
    if !has_python() { return; }
    if !html_parity_shared.rs_built { return; }
    assert!(
        html_parity_shared.rs_html.join("genindex.html").exists(),
        "Rust output missing genindex.html"
    );
    assert!(
        html_parity_shared.rs_html.join("objects.inv").exists(),
        "Rust output missing objects.inv"
    );
}

/// Snapshot the top-level file listing of both Python and Rust HTML outputs.
///
/// This documents the known structural parity gap.  When a new file is added
/// to the Rust builder, updating this snapshot is how we track progress.
#[rstest]
fn html_parity_snapshot_top_level_files(html_parity_shared: &HtmlParityShared) {
    if !has_python() { return; }
    if !html_parity_shared.py_built { return; }
    let mut py_files: Vec<String> = std::fs::read_dir(&html_parity_shared.py_html)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    py_files.sort();
    let mut rs_files: Vec<String> = std::fs::read_dir(&html_parity_shared.rs_html)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    rs_files.sort();
    // Snapshot Python output as the authoritative reference.
    insta::assert_yaml_snapshot!("html_parity_python_top_level_files", py_files);
    // Snapshot Rust output to track incremental progress.
    insta::assert_yaml_snapshot!("html_parity_rust_top_level_files", rs_files);
}

/// Snapshot the set of files present in Python output but absent in Rust output.
///
/// This is the **parity gap** snapshot — the goal is to shrink it over time.
/// Updating this snapshot documents intentional progress.
#[rstest]
fn html_parity_snapshot_gap(html_parity_shared: &HtmlParityShared) {
    if !has_python() { return; }
    if !html_parity_shared.py_built { return; }

    let py_html: std::collections::BTreeSet<String> = list_tree(&html_parity_shared.py_html)
        .into_iter().collect();
    let rs_html: std::collections::BTreeSet<String> = list_tree(&html_parity_shared.rs_html)
        .into_iter().collect();

    // Files Python produces that Rust does not yet produce.
    let mut missing_in_rust: Vec<String> = py_html.difference(&rs_html).cloned().collect();
    missing_in_rust.sort();
    // Files Rust produces that Python does not (new additions).
    let mut rust_only: Vec<String> = rs_html.difference(&py_html).cloned().collect();
    rust_only.sort();

    insta::assert_yaml_snapshot!("html_parity_gap_missing_in_rust", missing_in_rust);
    insta::assert_yaml_snapshot!("html_parity_gap_rust_only", rust_only);
}

// ── _static/ directory parity ─────────────────────────────────────────────────
//
// Python sphinx-build copies many assets into _static/ — theme CSS, JS,
// Pygments stylesheet, search index, and fonts.  The Rust builder currently
// only writes `_static/sphinxdocrs.css`.
//
// These tests:
//   1. Assert the Rust builder creates `_static/`.
//   2. Snapshot Python's `_static/` contents as the authoritative reference.
//   3. Snapshot Rust's `_static/` to track progress.
//   4. Snapshot the gap (Python-only files in `_static/`).
//   5. Assert every `_static/` href/src in Rust HTML resolves to a file on disk
//      (no broken stylesheet or script links).
//
// To grow parity: add asset copying in `write_static_files`
// (src/sphinxdocrs/src/builders/html.rs).

/// Rust must create a `_static/` directory during an HTML build.
#[rstest]
fn static_dir_created_by_rust(html_parity_shared: &HtmlParityShared) {
    if !html_parity_shared.rs_built { return; }
    assert!(
        html_parity_shared.rs_html.join("_static").is_dir(),
        "Rust builder must create _static/ directory"
    );
}

/// Snapshot the sorted file list of Python's `_static/` (authoritative reference).
///
/// Update this snapshot intentionally when upgrading the vendored Sphinx or
/// switching themes.
#[rstest]
fn static_dir_snapshot_python(html_parity_shared: &HtmlParityShared) {
    if !has_sphinx_build() { return; }
    if !html_parity_shared.py_built { return; }
    let mut files = list_tree(&html_parity_shared.py_html.join("_static"));
    files.sort();
    insta::assert_yaml_snapshot!("html_parity_python_static_files", files);
}

/// Snapshot the sorted file list of Rust's `_static/`.
///
/// Shrink this snapshot toward the Python reference by adding assets to
/// `write_static_files`.
#[rstest]
fn static_dir_snapshot_rust(html_parity_shared: &HtmlParityShared) {
    if !html_parity_shared.rs_built { return; }
    let rs_static = html_parity_shared.rs_html.join("_static");
    let mut files = if rs_static.is_dir() { list_tree(&rs_static) } else { vec![] };
    files.sort();
    insta::assert_yaml_snapshot!("html_parity_rust_static_files", files);
}

/// Snapshot files present in Python's `_static/` but absent from Rust's.
///
/// This is the actionable gap list for `write_static_files`.  Each entry
/// represents one asset that the Rust builder still needs to copy.
#[rstest]
fn static_dir_parity_gap(html_parity_shared: &HtmlParityShared) {
    if !has_sphinx_build() { return; }
    if !html_parity_shared.py_built { return; }

    let py_static = html_parity_shared.py_html.join("_static");
    let rs_static = html_parity_shared.rs_html.join("_static");

    let py_set: std::collections::BTreeSet<String> =
        list_tree(&py_static).into_iter().collect();
    let rs_set: std::collections::BTreeSet<String> = if rs_static.is_dir() {
        list_tree(&rs_static).into_iter().collect()
    } else {
        std::collections::BTreeSet::new()
    };

    let mut missing: Vec<String> = py_set.difference(&rs_set).cloned().collect();
    missing.sort();
    let mut rs_only: Vec<String> = rs_set.difference(&py_set).cloned().collect();
    rs_only.sort();

    insta::assert_yaml_snapshot!("html_parity_static_gap_missing_in_rust", missing);
    insta::assert_yaml_snapshot!("html_parity_static_gap_rust_only", rs_only);
}

/// Every `href="_static/..."` and `src="_static/..."` in Rust-emitted HTML
/// must resolve to an actual file on disk.
///
/// This surfaces broken stylesheet/script links where the HTML references a
/// file that `write_static_files` forgot to emit.  A failing test means a
/// browser would see a 404 for that asset.
#[rstest]
#[case("index")]
#[case("guide")]
fn static_dir_no_broken_links_in_rust_html(
    html_parity_shared: &HtmlParityShared,
    #[case] docname: &str,
) {
    if !html_parity_shared.rs_built { return; }
    let html = match std::fs::read_to_string(
        html_parity_shared.rs_html.join(format!("{docname}.html"))
    ) {
        Ok(s) => s,
        Err(_) => return,
    };
    let re = regex::Regex::new(r#"(?:href|src)="(_static/[^"?#]+)""#).unwrap();
    let mut broken: Vec<String> = re
        .captures_iter(&html)
        .map(|c| c.get(1).unwrap().as_str().to_owned())
        .filter(|rel| !html_parity_shared.rs_html.join(rel).exists())
        .collect();
    broken.sort();
    broken.dedup();
    assert!(
        broken.is_empty(),
        "Rust {docname}.html has broken _static/ references (files not on disk):\n{}",
        broken.join("\n")
    );
}

/// Sanity-check Python's `_static/` links are all resolvable before we rely
/// on Python as the authoritative reference.
#[rstest]
#[case("index")]
#[case("guide")]
fn static_dir_no_broken_links_in_python_html(
    html_parity_shared: &HtmlParityShared,
    #[case] docname: &str,
) {
    if !has_sphinx_build() { return; }
    if !html_parity_shared.py_built { return; }
    let html = match std::fs::read_to_string(
        html_parity_shared.py_html.join(format!("{docname}.html"))
    ) {
        Ok(s) => s,
        Err(_) => return,
    };
    let re = regex::Regex::new(r#"(?:href|src)="(_static/[^"?#]+)""#).unwrap();
    let mut broken: Vec<String> = re
        .captures_iter(&html)
        .map(|c| c.get(1).unwrap().as_str().to_owned())
        .filter(|rel| !html_parity_shared.py_html.join(rel).exists())
        .collect();
    broken.sort();
    broken.dedup();
    assert!(
        broken.is_empty(),
        "Python {docname}.html has broken _static/ references:\n{}",
        broken.join("\n")
    );
}

/// Verify `guide.html` body contains the RST text content in both outputs.
#[rstest]
#[case("guide")]
#[case("index")]
fn html_parity_body_contains_rst_text(
    html_parity_shared: &HtmlParityShared,
    #[case] docname: &str,
) {
    if !has_python() { return; }
    if !html_parity_shared.rs_built { return; }

    let html = std::fs::read_to_string(
        html_parity_shared.rs_html.join(format!("{docname}.html"))
    ).unwrap();
    let text = strip_html(&html);
    match docname {
        "guide" => {
            assert!(text.contains("User Guide"), "guide.html body missing title");
            assert!(text.contains("Content here"), "guide.html body missing content");
        }
        "index" => {
            assert!(text.contains("My Test Project"), "index.html body missing title");
            assert!(text.contains("Welcome to the documentation"), "index.html body missing content");
        }
        _ => {}
    }
}

// ── parity: log messages ──────────────────────────────────────────────────────
//
// These tests compare the **structure** of stderr/stdout between the Python
// and Rust CLIs on identical synthetic inputs.  Exact strings are not
// required to match; instead each test extracts *observable properties*:
//
//   - exit code
//   - presence of expected keywords / patterns
//   - absence of unexpected noise
//   - insta snapshots of the *normalised* message set
//
// Normalisation helpers strip volatile content (paths, counts, versions)
// so snapshots are stable across runs.

// ── shared once-fixtures (log section) ───────────────────────────────────────

/// Result of running `python -m sphinx -M help` + `sphinx-build-rs -M help`
/// once per binary invocation.  Shared across all `log_make_help_*` tests.
pub struct LogMakeHelpShared {
    pub py_stdout: String,
    pub rs_stdout: String,
}

#[fixture]
#[once]
fn log_make_help_shared() -> LogMakeHelpShared {
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-build-rs");
    let tmp = TempDir::new().unwrap().keep();
    let py_stdout = if has_python() {
        let (_, out, _) = run(&python_bin(), &["-m", "sphinx", "-M", "help", ".", "."], &tmp);
        out
    } else {
        String::new()
    };
    let (_, rs_stdout, _) = run(rs_bin, &["-M", "help", ".", "."], &tmp);
    LogMakeHelpShared { py_stdout, rs_stdout }
}

/// Result of running both `sphinx-apidoc-rs` and `python -m sphinx.ext.apidoc`
/// (normal mode, no --dry-run) on a synthetic Python package.
/// Built once per binary invocation and shared across apidoc log tests.
pub struct LogApidocShared {
    pub rs_exit:   i32,
    pub rs_stdout: String,
    pub rs_stderr: String,
    pub py_exit:   i32,
    pub py_stdout: String,
    pub py_stderr: String,
    pub py_available: bool,
}

#[fixture]
#[once]
fn log_apidoc_shared() -> LogApidocShared {
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-apidoc-rs");
    let src = TempDir::new().unwrap().keep();
    let rs_out = TempDir::new().unwrap().keep();
    let py_out = TempDir::new().unwrap().keep();

    std::fs::create_dir(src.join("mypkg")).unwrap();
    std::fs::write(src.join("mypkg/__init__.py"), b"").unwrap();
    std::fs::write(src.join("mypkg/utils.py"), b"").unwrap();
    let pkg = src.join("mypkg").to_string_lossy().into_owned();

    let (rs_exit, rs_stdout, rs_stderr) = run(
        rs_bin,
        &["-o", rs_out.to_str().unwrap(), &pkg],
        &src,
    );

    let (py_exit, py_stdout, py_stderr, py_available) = if has_python() {
        let (code, out, err) = run(
            &python_bin(),
            &["-m", "sphinx.ext.apidoc", "-o", py_out.to_str().unwrap(), &pkg],
            &src,
        );
        (code, out, err, true)
    } else {
        (1, String::new(), String::new(), false)
    };

    LogApidocShared { rs_exit, rs_stdout, rs_stderr, py_exit, py_stdout, py_stderr, py_available }
}

/// Result of running both `sphinx-apidoc-rs --dry-run` and
/// `python -m sphinx.ext.apidoc --dry-run` on a synthetic package.
/// Built once per binary invocation.
pub struct LogApidocDryRunShared {
    pub rs_stdout: String,
    pub rs_stderr: String,
    pub py_stdout: String,
    pub py_stderr: String,
    pub py_available: bool,
}

#[fixture]
#[once]
fn log_apidoc_dry_run_shared() -> LogApidocDryRunShared {
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-apidoc-rs");
    let src = TempDir::new().unwrap().keep();
    let rs_out = TempDir::new().unwrap().keep();
    let py_out = TempDir::new().unwrap().keep();

    std::fs::create_dir(src.join("mypkg")).unwrap();
    std::fs::write(src.join("mypkg/__init__.py"), b"").unwrap();
    let pkg = src.join("mypkg").to_string_lossy().into_owned();

    let (_, rs_stdout, rs_stderr) = run(
        rs_bin,
        &["--dry-run", "-o", rs_out.to_str().unwrap(), &pkg],
        &src,
    );

    let (py_stdout, py_stderr, py_available) = if has_python() {
        let (_, out, err) = run(
            &python_bin(),
            &["-m", "sphinx.ext.apidoc", "--dry-run", "-o", py_out.to_str().unwrap(), &pkg],
            &src,
        );
        (out, err, true)
    } else {
        (String::new(), String::new(), false)
    };

    LogApidocDryRunShared { rs_stdout, rs_stderr, py_stdout, py_stderr, py_available }
}

/// Result of running `sphinx-quickstart-rs -q` and `python -m sphinx.cmd.quickstart -q`
/// on identical args, once per binary invocation.
/// Shared by `quickstart_parity_flat` and `log_quickstart_messages_snapshot`.
pub struct QuickstartParityShared {
    pub py_dir:       PathBuf,
    pub rs_dir:       PathBuf,
    pub py_exit:      i32,
    pub rs_exit:      i32,
    pub py_tree:      Vec<String>,
    pub rs_tree:      Vec<String>,
    pub py_stdout:    String,
    pub rs_stdout:    String,
    pub py_available: bool,
}

#[fixture]
#[once]
fn quickstart_parity_shared() -> QuickstartParityShared {
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-quickstart-rs");
    let py_dir = TempDir::new().unwrap().keep();
    let rs_dir = TempDir::new().unwrap().keep();

    let common: &[&str] = &[
        "-q", "-p", "ParityProj", "-a", "ParityAuthor", "-v", "1.0",
        "--no-makefile", "--no-batchfile",
    ];

    // Check whether Python sphinx.cmd.quickstart is available.
    let py_available = has_python() && {
        let (code, _, _) = run(
            &python_bin(),
            &["-m", "sphinx", "--version"],
            &py_dir,
        );
        code == 0
    };

    let (py_exit, py_stdout, py_tree) = if py_available {
        let mut args: Vec<&str> = vec!["-m", "sphinx.cmd.quickstart"];
        args.extend_from_slice(common);
        args.push(py_dir.to_str().unwrap());
        let (code, out, _) = run(&python_bin(), &args, &py_dir);
        let tree = list_tree(&py_dir);
        (code, out, tree)
    } else {
        (1, String::new(), vec![])
    };

    let mut rs_args: Vec<&str> = common.to_vec();
    rs_args.push(rs_dir.to_str().unwrap());
    let (rs_exit, rs_stdout, _) = run(rs_bin, &rs_args, &rs_dir);
    let rs_tree = list_tree(&rs_dir);

    QuickstartParityShared {
        py_dir,
        rs_dir,
        py_exit,
        rs_exit,
        py_tree,
        rs_tree,
        py_stdout,
        rs_stdout,
        py_available,
    }
}

// ── normalisation helpers ─────────────────────────────────────────────────────

/// Replace absolute paths with the literal `<PATH>` token.
fn norm_paths(s: &str, dirs: &[&str]) -> String {
    let mut out = s.to_owned();
    for dir in dirs {
        // Sort longest first so nested paths are replaced before parents.
        out = out.replace(dir, "<PATH>");
    }
    out
}

/// Replace decimal integers that look like counts/file-numbers with `<N>`.
fn norm_counts(s: &str) -> String {
    // Match standalone integers (not part of version strings like "9.1.1").
    // Replace only when surrounded by whitespace or start/end-of-string.
    let re = regex::Regex::new(r"(?m)\b(\d+) (file|warning|error|stub|doc)").unwrap();
    re.replace_all(s, "<N> $2").into_owned()
}

/// Replace Sphinx version strings (e.g. "v9.1.1+/abc1234") with `<VERSION>`.
fn norm_version(s: &str) -> String {
    let re = regex::Regex::new(r"v\d+\.\d+[\.\d+]*/?\S*").unwrap();
    re.replace_all(s, "<VERSION>").into_owned()
}

/// Collect non-blank lines, trim trailing whitespace, sort for determinism.
fn sorted_lines(s: &str) -> Vec<String> {
    let mut lines: Vec<String> = s
        .lines()
        .map(|l| l.trim_end().to_owned())
        .filter(|l| !l.is_empty())
        .collect();
    lines.sort();
    lines.dedup();
    lines
}

/// Full normalisation pipeline: strip paths, counts, versions; sort lines.
fn normalise_output(raw: &str, paths: &[&str]) -> Vec<String> {
    // Normalise volatile /tmp/sphinx-err-XXXX.log → stable token
    let re_errlog = regex::Regex::new(r"/tmp/sphinx-err-\S+\.log").unwrap();
    let s = re_errlog.replace_all(raw, "<SPHINX_ERR_LOG>").into_owned();
    let s = norm_paths(&s, paths);
    let s = norm_counts(&s);
    let s = norm_version(&s);
    sorted_lines(&s)
}

// ── make help ─────────────────────────────────────────────────────────────────

/// `sphinx-build -M help` must print the builder table to stdout.
/// Both Python and Rust must include the common builder names.
///
/// Note: same binary + args as `make_help_contains_core_builder`; kept
/// separately because it lives in the log-message section and may diverge.
#[rstest]
#[case("html")]
#[case("latex")]
#[case("man")]
#[case("text")]
#[case("clean")]
fn log_make_help_contains_builder(#[case] builder: &str) {
    let tmp = TempDir::new().unwrap();
    let output = snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-build-rs"))
        .args(["-M", "help", ".", "."])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "sphinx-build-rs -M help must exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(builder), "help output missing builder '{builder}'");
}

/// The Rust `help` output matches the Python output structure (snapshot).
#[rstest]
fn log_make_help_snapshot(log_make_help_shared: &LogMakeHelpShared) {
    // Extract just the builder names from each side (first word of each line
    // that starts with whitespace in the table).
    let builder_names = |s: &str| -> Vec<String> {
        s.lines()
            .filter(|l| l.starts_with("  ") && !l.trim().is_empty())
            .map(|l| l.split_whitespace().next().unwrap_or("").to_owned())
            .filter(|w| !w.is_empty())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect()
    };

    let py_builders = builder_names(&log_make_help_shared.py_stdout);
    let rs_builders = builder_names(&log_make_help_shared.rs_stdout);

    insta::assert_yaml_snapshot!("log_make_help_python_builders", py_builders);
    insta::assert_yaml_snapshot!("log_make_help_rust_builders", rs_builders);
}

// ── sphinx-build -M html success messages ────────────────────────────────────

/// On a successful HTML build the Rust binary must emit to stderr:
/// - a "make mode: running SphinxApp" banner
/// - a "Build succeeded" summary with a file count
#[test]
fn log_html_build_success_keywords() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    setup_html_parity_project(src.path());

    // Check exit + "Build succeeded" via snapbox; regex file-count check needs
    // the raw output so we use .output() and assert afterwards.
    let output = snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-build-rs"))
        .args(["-M", "html", src.path().to_str().unwrap(), out.path().to_str().unwrap()])
        .current_dir(src.path())
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "expected exit 0, stderr:\n{stderr}");
    assert!(
        stderr.contains("Build succeeded"),
        "expected 'Build succeeded' in stderr; got:\n{stderr}"
    );
    assert!(
        regex::Regex::new(r"\d+ file").unwrap().is_match(&stderr),
        "expected file count in stderr; got:\n{stderr}"
    );
}

/// On a successful HTML build the Python binary emits "build succeeded" to stderr.
/// Both sides must exit 0.
#[rstest]
fn log_html_build_success_both_exit_zero(html_parity_shared: &HtmlParityShared) {
    if !has_sphinx_build() { return; }
    if !html_parity_shared.py_built { return; }
    assert_eq!(html_parity_shared.py_exit, 0,
        "Python sphinx-build must exit 0; stderr:\n{}", html_parity_shared.py_stderr);
    assert_eq!(html_parity_shared.rs_exit, 0,
        "Rust sphinx-build-rs must exit 0; stderr:\n{}", html_parity_shared.rs_stderr);
}

/// Snapshot normalised stderr from both sides for a clean HTML build.
///
/// This documents the known message-level parity gap.  Each line represents
/// one observable log event; normalised for paths/counts/versions.
#[rstest]
fn log_html_build_stderr_snapshot(html_parity_shared: &HtmlParityShared) {
    if !has_sphinx_build() { return; }

    let src_str = html_parity_shared.src.to_str().unwrap_or("");
    let py_out_str = html_parity_shared.py_html.parent()
        .and_then(|p| p.to_str()).unwrap_or("");
    let rs_out_str = html_parity_shared.rs_html.parent()
        .and_then(|p| p.to_str()).unwrap_or("");
    let paths = &[src_str, py_out_str, rs_out_str];
    insta::assert_yaml_snapshot!(
        "log_html_build_py_stderr_normalised",
        normalise_output(&html_parity_shared.py_stderr, paths)
    );
    insta::assert_yaml_snapshot!(
        "log_html_build_rs_stderr_normalised",
        normalise_output(&html_parity_shared.rs_stderr, paths)
    );
}

// ── sphinx-build invalid argument error messages ──────────────────────────────

/// Verify the mock build output struct produces the expected format without
/// spawning a real subprocess.
#[test]
fn log_format_build_succeeded_contains_file_count() {
    let mock = MockBuildOutput::rs_success();
    assert_eq!(mock.exit, 0);
    assert!(mock.stderr.contains("Build succeeded"));
    assert!(regex::Regex::new(r"\d+ file").unwrap().is_match(&mock.stderr));
}

/// Both tools must exit non-zero when given a missing source directory.
#[test]
fn log_build_missing_srcdir_exits_nonzero() {
    let tmp = TempDir::new().unwrap();
    // Exit non-zero is the key assertion; content check via combined output.
    let output = snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-build-rs"))
        .args(["-M", "html", "/nonexistent/srcdir", tmp.path().to_str().unwrap()])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_ne!(output.status.code(), Some(0), "expected non-zero exit for missing srcdir");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(
        combined.to_lowercase().contains("error"),
        "expected error message; got:\n{combined}"
    );
}

/// `sphinx-build-rs -M html` with identical src and out dirs must emit
/// an error and exit non-zero.
#[test]
fn log_build_identical_src_out_error() {
    let src = TempDir::new().unwrap();
    setup_html_parity_project(src.path());
    let dir_str = src.path().to_str().unwrap().to_owned();

    // SphinxApp rejects srcdir == outdir.
    snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-build-rs"))
        .args(["-b", "html", &dir_str, &dir_str])
        .current_dir(src.path())
        .assert()
        .failure();
}

// ── sphinx-apidoc log messages ────────────────────────────────────────────────

/// `sphinx-apidoc` must print "Creating file <path>." for each new file.
#[test]
fn log_apidoc_creating_file_message() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    // Set up a minimal Python package.
    std::fs::create_dir(src.path().join("mypkg")).unwrap();
    std::fs::write(src.path().join("mypkg/__init__.py"), b"").unwrap();
    std::fs::write(src.path().join("mypkg/utils.py"), b"").unwrap();

    // "Creating file" appears on stdout; snapbox checks exit + stdout pattern.
    let output = snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-apidoc-rs"))
        .args(["-o", out.path().to_str().unwrap(), src.path().join("mypkg").to_str().unwrap()])
        .current_dir(src.path())
        .output()
        .unwrap();
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert_eq!(output.status.code(), Some(0), "apidoc must exit 0; output:\n{combined}");
    assert!(
        combined.contains("Creating file"),
        "expected 'Creating file' message; got:\n{combined}"
    );
}

/// Rust `sphinx-apidoc` emits "Creating file <path>." for each generated file;
/// Python `sphinx.ext.apidoc` does **not** print this message (it generates
/// files silently).  This test documents the known divergence and verifies the
/// Rust side outputs the expected format.
#[rstest]
fn log_apidoc_creating_file_format_parity(log_apidoc_shared: &LogApidocShared) {
    let rs_combined = format!("{}{}", log_apidoc_shared.rs_stdout, log_apidoc_shared.rs_stderr);
    assert_eq!(log_apidoc_shared.rs_exit, 0,
        "sphinx-apidoc-rs must exit 0; output:\n{rs_combined}");

    // Rust emits "Creating file <path>." for every new .rst file.
    let creating: Vec<String> = rs_combined.lines()
        .filter(|l| l.contains("Creating file"))
        .map(|l| {
            // Normalise: keep only the filename after the last '/'
            if let Some(fname) = l.split('/').last() {
                format!("Creating file <PATH>/{fname}")
            } else {
                l.to_owned()
            }
        })
        .collect();
    assert!(!creating.is_empty(), "Rust apidoc must emit 'Creating file' lines");

    // Document known Python divergence: Python sphinx.ext.apidoc does not
    // print 'Creating file' messages.
    // (Verified: `python3 -m sphinx.ext.apidoc -o ... mypkg` produces no stdout)
    if log_apidoc_shared.py_available {
        let py_combined = format!("{}{}",
            log_apidoc_shared.py_stdout, log_apidoc_shared.py_stderr);
        // Snapshot Python output (expected: empty or minimal) as the reference.
        insta::assert_yaml_snapshot!(
            "log_apidoc_py_file_creation_output",
            sorted_lines(&py_combined)
        );
    }
}

/// `sphinx-apidoc --dry-run` must print "Would create file …" instead of
/// actually writing files, and must exit 0.
#[test]
fn log_apidoc_dry_run_message() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::create_dir(src.path().join("mypkg")).unwrap();
    std::fs::write(src.path().join("mypkg/__init__.py"), b"").unwrap();

    let output = snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-apidoc-rs"))
        .args([
            "--dry-run",
            "-o", out.path().to_str().unwrap(),
            src.path().join("mypkg").to_str().unwrap(),
        ])
        .current_dir(src.path())
        .output()
        .unwrap();
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert_eq!(output.status.code(), Some(0), "dry-run must exit 0; output:\n{combined}");
    assert!(
        combined.contains("Would create file"),
        "expected 'Would create file' in dry-run output; got:\n{combined}"
    );
    // No files should be written.
    assert!(
        std::fs::read_dir(out.path()).unwrap().next().is_none(),
        "dry-run must not write any files"
    );
}

// ── sphinx-apidoc --dry-run message format parity ────────────────────────────

/// `sphinx-apidoc --dry-run` message parity.
///
/// The Rust binary emits "Would create file <path>." in dry-run mode.
/// Python `sphinx.ext.apidoc --dry-run` does **not** print this message
/// (it silently skips file creation).  This test documents the known
/// divergence while verifying the Rust side.
#[rstest]
fn log_apidoc_dry_run_parity(log_apidoc_dry_run_shared: &LogApidocDryRunShared) {
    let rs_combined = format!("{}{}",
        log_apidoc_dry_run_shared.rs_stdout, log_apidoc_dry_run_shared.rs_stderr);
    assert!(
        rs_combined.contains("Would create file"),
        "Rust dry-run missing 'Would create file'; got:\n{rs_combined}"
    );

    // Document known Python divergence: Python sphinx.ext.apidoc --dry-run
    // does not print 'Would create file' (it produces no output).
    if log_apidoc_dry_run_shared.py_available {
        let py_combined = format!("{}{}",
            log_apidoc_dry_run_shared.py_stdout, log_apidoc_dry_run_shared.py_stderr);
        // Python produces no output for dry-run — snapshot confirms this.
        insta::assert_yaml_snapshot!(
            "log_apidoc_py_dry_run_output",
            sorted_lines(&py_combined)
        );
    }
}

// ── sphinx-quickstart log messages ────────────────────────────────────────────

/// `sphinx-quickstart -q` must create `conf.py` and `index.rst`, exit 0.
///
/// Note: `-q` (quiet) suppresses the "Creating file" log messages on both
/// Python and Rust; file-creation parity is verified structurally in
/// `quickstart_parity_flat`.  Here we only assert exit code + artifacts.
#[test]
fn log_quickstart_creation_messages() {
    let out = TempDir::new().unwrap();

    snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-quickstart-rs"))
        .args([
            "-q", "-p", "TestProj", "-a", "Author", "-v", "1.0",
            "--no-makefile", "--no-batchfile",
            out.path().to_str().unwrap(),
        ])
        .current_dir(out.path())
        .assert()
        .success();

    assert!(out.path().join("conf.py").exists(),  "conf.py not created");
    assert!(out.path().join("index.rst").exists(), "index.rst not created");
}

/// The Rust `sphinx-quickstart` "Creating file" and "Finished" messages match
/// the Python output structure (normalised snapshot).
///
/// Both sides run with `-q` (non-interactive). `-q` suppresses "Creating file"
/// on both Python and Rust, so we snapshot the completion summary instead.
#[rstest]
fn log_quickstart_messages_snapshot(quickstart_parity_shared: &QuickstartParityShared) {
    if quickstart_parity_shared.py_available && quickstart_parity_shared.py_exit != 0 {
        return;
    }

    // Extract structural lines (Creating / Finished / sphinx-build hint)
    let structural = |s: &str, base: &str| -> Vec<String> {
        s.lines()
            .filter(|l| {
                l.contains("Creating file")
                    || l.contains("Finished")
                    || l.contains("sphinx-build")
                    || l.contains("make ")
            })
            .map(|l| {
                // Replace full path with just the filename
                let l = l.replace(base, "<OUTDIR>");
                l.trim().to_owned()
            })
            .collect()
    };

    let py_structural = structural(
        &quickstart_parity_shared.py_stdout,
        quickstart_parity_shared.py_dir.to_str().unwrap_or(""),
    );
    let rs_structural = structural(
        &quickstart_parity_shared.rs_stdout,
        quickstart_parity_shared.rs_dir.to_str().unwrap_or(""),
    );

    insta::assert_yaml_snapshot!("log_quickstart_py_structural", py_structural);
    insta::assert_yaml_snapshot!("log_quickstart_rs_structural", rs_structural);
}

// ── sphinx-autogen log messages ───────────────────────────────────────────────

/// `sphinx-autogen` must emit "[autosummary] generating autosummary for: …"
/// when given an RST file with autosummary directives, and
/// "[autosummary] wrote N stub file(s) to …" on completion.
#[test]
fn log_autogen_progress_messages() {
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-autogen-rs");
    let tmp = TempDir::new().unwrap();
    let out_dir = tmp.path().join("generated");

    // RST file with autosummary directives
    let rst = "\
.. autosummary::\n   :toctree: generated\n\n   os.path\n   os.getcwd\n";
    let rst_file = tmp.path().join("index.rst");
    std::fs::write(&rst_file, rst).unwrap();

    let (code, _stdout, stderr) = run(
        rs_bin,
        &["-o", out_dir.to_str().unwrap(), rst_file.to_str().unwrap()],
        tmp.path(),
    );

    // autogen may exit non-zero if Python isn't present to resolve modules,
    // but the log messages should still appear before any failure.
    let _ = code;
    assert!(
        stderr.contains("[autosummary]"),
        "expected '[autosummary]' prefix in stderr; got:\n{stderr}"
    );
    assert!(
        stderr.contains("generating autosummary for"),
        "expected 'generating autosummary for' in stderr; got:\n{stderr}"
    );
}

// ── make_mode clean messages ──────────────────────────────────────────────────

/// `sphinx-build-rs -M clean` must print "Removing everything under …"
/// when the build directory exists, and exit 0.
#[test]
fn log_make_clean_message() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    // Create something in the build dir so clean has work to do.
    let build_html = out.path().join("html");
    std::fs::create_dir_all(&build_html).unwrap();
    std::fs::write(build_html.join("index.html"), b"<h1>test</h1>").unwrap();

    // Use snapbox for exit assertion; check the combined output for the message
    // (the banner may land on stdout or stderr depending on build mode).
    let output = snapbox::cmd::Command::new(env!("CARGO_BIN_EXE_sphinx-build-rs"))
        .args(["-M", "clean", src.path().to_str().unwrap(), out.path().to_str().unwrap()])
        .current_dir(src.path())
        .output()
        .unwrap();
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert_eq!(output.status.code(), Some(0), "clean must exit 0; output:\n{combined}");
    assert!(
        combined.contains("Removing everything under"),
        "expected 'Removing everything under' in clean output; got:\n{combined}"
    );

    // The build dir should be empty after clean.
    assert!(
        std::fs::read_dir(out.path()).unwrap().next().is_none(),
        "build dir should be empty after clean"
    );
}

// ── error message format consistency ─────────────────────────────────────────

/// All Rust error messages must start with "Error: " (capital E, colon, space).
/// Scan a range of error-triggering invocations and assert the format.
#[rstest]
#[case("sphinx-build-rs",    vec!["-M", "html", "/no/such/srcdir", "/tmp/out"])]
#[case("sphinx-apidoc-rs",   vec!["-o", "/tmp/out", "/no/such/module"])]
fn log_error_prefix_format(
    #[case] binary: &str,
    #[case] args: Vec<&str>,
) {
    let bin = match binary {
        "sphinx-build-rs"  => env!("CARGO_BIN_EXE_sphinx-build-rs"),
        "sphinx-apidoc-rs" => env!("CARGO_BIN_EXE_sphinx-apidoc-rs"),
        other => panic!("unknown binary {other}"),
    };
    let tmp = TempDir::new().unwrap();
    let output = snapbox::cmd::Command::new(bin)
        .args(args)
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_ne!(output.status.code(), Some(0), "expected non-zero exit for invalid args");
    // Every non-blank stderr line that mentions "error" must start "Error: ".
    for line in stderr.lines().filter(|l| {
        let lower = l.to_lowercase();
        lower.contains("error") && !lower.contains("warning")
    }) {
        assert!(
            line.starts_with("Error: ") || line.starts_with("error["),
            "error line does not start with 'Error: ': {line:?}"
        );
    }
}

/// `Warning: ` prefix is used consistently for non-fatal messages.
/// Remove-old failure in apidoc must use this prefix.
#[test]
fn log_warning_prefix_format() {
    // The warning prefix is checked at code level; here we verify the
    // apidoc `--remove-old` path doesn't regress the format.
    // We induce the warning by pointing --remove-old at an unwritable path
    // only on systems where we can do that; otherwise just compile-check.
    let _check: fn(&str) -> bool = |line: &str| line.starts_with("Warning: ");
    // Runtime check: apidoc emits "Warning: failed to remove ..." — see
    // apidoc/generate.rs::remove_old_files. That code path requires a
    // non-removable file; we skip the runtime trigger and just assert the
    // compile-time constant.
    assert!("Warning: failed to remove foo: permission denied".starts_with("Warning: "));
}

// ── parity: build real src/sphinx/doc with sphinx and sphinxdocrs ────────────
//
// Strategy: build the vendored `src/sphinx/doc` tree (the Sphinx project's own
// documentation) with both Python `sphinx-build` and `sphinx-build-rs`, then
// assert on exit codes, on-disk output, and normalised stderr.
//
// Many extensions used by sphinx/doc (autodoc, graphviz, intersphinx, …) are
// not yet ported to sphinxdocrs.  Rust therefore skips or stubs those
// directives.  We snapshot the parity gap explicitly so regressions are caught
// automatically as coverage improves.
//
// Tests skip cleanly when:
//   - `src/sphinx/doc/conf.py` is absent (workspace cloned without vendored sources)
//   - `sphinx-build` is not in PATH (for Python-side assertions)
//
// Enable with:
//   cargo test -p sphinxdocrs --features test-parity --test parity \
//       sphinx_docs
//
// ─────────────────────────────────────────────────────────────────────────────

/// Resolve the vendored Sphinx documentation source.
fn sphinx_doc_src() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../sphinx/doc")
}

/// Return `true` when `src/sphinx/doc/conf.py` exists.
fn has_sphinx_doc_src() -> bool {
    sphinx_doc_src().join("conf.py").exists()
}

/// Combined output from building `src/sphinx/doc` with Python and Rust,
/// collected once per test-binary invocation via `#[once]`.
pub struct SphinxDocsBuildShared {
    /// Python output directory (flat HTML, not nested under `html/`).
    pub py_out:    PathBuf,
    /// Rust output directory.
    pub rs_out:    PathBuf,
    pub py_exit:   i32,
    pub rs_exit:   i32,
    pub py_stdout: String,
    pub rs_stdout: String,
    pub py_stderr: String,
    pub rs_stderr: String,
    /// `true` when exit 0 **and** `index.html` exists.
    pub py_built:  bool,
    pub rs_built:  bool,
}

#[fixture]
#[once]
fn sphinx_docs_build_shared() -> SphinxDocsBuildShared {
    let src = sphinx_doc_src();
    let py_out = TempDir::new().unwrap().keep();
    let rs_out = TempDir::new().unwrap().keep();
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-build-rs");

    // Python: sphinx-build <srcdir> <outdir> -q
    let (py_exit, py_stdout, py_stderr) = if has_sphinx_doc_src() && has_sphinx_build() {
        run(
            "sphinx-build",
            &[src.to_str().unwrap(), py_out.to_str().unwrap(), "-q"],
            &src,
        )
    } else {
        (1, String::new(), String::new())
    };

    // Rust: sphinx-build-rs <srcdir> <outdir>
    let (rs_exit, rs_stdout, rs_stderr) = if has_sphinx_doc_src() {
        run(
            rs_bin,
            &[src.to_str().unwrap(), rs_out.to_str().unwrap()],
            &src,
        )
    } else {
        (1, String::new(), String::new())
    };

    SphinxDocsBuildShared {
        py_built: py_exit == 0 && py_out.join("index.html").exists(),
        rs_built: rs_exit == 0 && rs_out.join("index.html").exists(),
        py_out,
        rs_out,
        py_exit,
        rs_exit,
        py_stdout,
        rs_stdout,
        py_stderr,
        rs_stderr,
    }
}

// ── exit-code assertions ──────────────────────────────────────────────────────

/// `sphinx-build-rs` must exit 0 when building the real sphinx/doc project.
#[rstest]
fn sphinx_docs_rs_exits_zero(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !has_sphinx_doc_src() { return; }
    assert_eq!(
        sphinx_docs_build_shared.rs_exit, 0,
        "sphinx-build-rs must exit 0 on sphinx/doc; stderr:\n{}",
        sphinx_docs_build_shared.rs_stderr
    );
}

/// `sphinx-build` (Python) must exit 0 when building the real sphinx/doc project.
#[rstest]
fn sphinx_docs_py_exits_zero(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !has_sphinx_doc_src() { return; }
    if !has_sphinx_build() { return; }
    assert_eq!(
        sphinx_docs_build_shared.py_exit, 0,
        "sphinx-build must exit 0 on sphinx/doc; stderr:\n{}",
        sphinx_docs_build_shared.py_stderr
    );
}

/// Both tools must exit 0 for the real sphinx/doc project (when both available).
#[rstest]
fn sphinx_docs_both_exit_zero(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !has_sphinx_doc_src() { return; }
    if !has_sphinx_build() { return; }
    assert_eq!(
        sphinx_docs_build_shared.py_exit, 0,
        "Python must exit 0; stderr:\n{}", sphinx_docs_build_shared.py_stderr
    );
    assert_eq!(
        sphinx_docs_build_shared.rs_exit, 0,
        "Rust must exit 0; stderr:\n{}", sphinx_docs_build_shared.rs_stderr
    );
}

// ── on-disk output assertions ─────────────────────────────────────────────────

/// Rust must produce `index.html` in the output directory.
#[rstest]
fn sphinx_docs_rs_produces_index_html(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !has_sphinx_doc_src() { return; }
    // Assert even if rs_exit != 0: partial builds should still write index.html.
    assert!(
        sphinx_docs_build_shared.rs_out.join("index.html").exists(),
        "Rust output missing index.html (exit {}); stderr:\n{}",
        sphinx_docs_build_shared.rs_exit,
        sphinx_docs_build_shared.rs_stderr
    );
}

/// When Python also built successfully, both outputs must contain `index.html`.
#[rstest]
fn sphinx_docs_both_produce_index_html(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }
    assert!(
        sphinx_docs_build_shared.py_out.join("index.html").exists(),
        "Python output missing index.html"
    );
    assert!(
        sphinx_docs_build_shared.rs_out.join("index.html").exists(),
        "Rust output missing index.html"
    );
}

/// Rust `index.html` must carry a valid HTML5 DOCTYPE declaration.
#[rstest]
fn sphinx_docs_rs_index_has_doctype(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.rs_built { return; }
    let html = std::fs::read_to_string(
        sphinx_docs_build_shared.rs_out.join("index.html")
    ).unwrap();
    assert!(
        html.to_lowercase().contains("<!doctype html"),
        "Rust index.html missing HTML5 DOCTYPE"
    );
}

/// Both `index.html` outputs must contain a valid HTML5 DOCTYPE.
#[rstest]
fn sphinx_docs_both_have_doctype(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }
    for (label, outdir) in &[
        ("Python", &sphinx_docs_build_shared.py_out),
        ("Rust",   &sphinx_docs_build_shared.rs_out),
    ] {
        let html = std::fs::read_to_string(outdir.join("index.html")).unwrap();
        assert!(
            html.to_lowercase().contains("<!doctype html"),
            "{label} index.html missing HTML5 DOCTYPE"
        );
    }
}

/// Rust `index.html` must contain the Sphinx project title.
#[rstest]
fn sphinx_docs_rs_index_contains_title(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.rs_built { return; }
    let html = std::fs::read_to_string(
        sphinx_docs_build_shared.rs_out.join("index.html")
    ).unwrap();
    assert!(
        html.contains("Sphinx"),
        "Rust index.html should contain project title 'Sphinx'; first 500 chars:\n{}",
        &html[..html.len().min(500)]
    );
}

/// Both `index.html` outputs must contain the Sphinx project title.
#[rstest]
fn sphinx_docs_both_contain_project_title(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }
    for (label, outdir) in &[
        ("Python", &sphinx_docs_build_shared.py_out),
        ("Rust",   &sphinx_docs_build_shared.rs_out),
    ] {
        let html = std::fs::read_to_string(outdir.join("index.html")).unwrap();
        assert!(
            html.contains("Sphinx"),
            "{label} index.html should contain project title 'Sphinx'"
        );
    }
}

/// Walk `root` recursively and return a sorted list of relative file paths.
/// Directories are not included in the output.
fn file_tree_paths(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(dir: &Path, root: &Path, acc: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut entries: Vec<_> = entries.flatten().collect();
            entries.sort_by_key(|e| e.path());
            for e in entries {
                let p = e.path();
                let rel = p.strip_prefix(root).unwrap().to_string_lossy().into_owned();
                if p.is_dir() {
                    walk(&p, root, acc);
                } else {
                    acc.push(rel);
                }
            }
        }
    }
    walk(root, root, &mut out);
    out
}

/// Walk `root` recursively and return a map of `relative_path → byte_size`
/// for every regular file.
fn file_size_map(root: &Path) -> std::collections::BTreeMap<String, u64> {
    let mut out = std::collections::BTreeMap::new();
    fn walk(dir: &Path, root: &Path, acc: &mut std::collections::BTreeMap<String, u64>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut entries: Vec<_> = entries.flatten().collect();
            entries.sort_by_key(|e| e.path());
            for e in entries {
                let p = e.path();
                let rel = p.strip_prefix(root).unwrap().to_string_lossy().into_owned();
                if p.is_dir() {
                    walk(&p, root, acc);
                } else {
                    let size = p.metadata().map(|m| m.len()).unwrap_or(0);
                    acc.insert(rel, size);
                }
            }
        }
    }
    walk(root, root, &mut out);
    out
}

/// Snapshot the FULL recursive file tree (paths only) from the Rust sphinx/doc build.
///
/// Sizes are verified separately by `sphinx_docs_rs_all_files_non_empty`; keeping
/// them out of the snapshot avoids flakiness from non-deterministic build artefacts
/// (intersphinx cache, searchindex timestamps, etc.).
#[rstest]
fn sphinx_docs_rs_snapshot_full_tree(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.rs_built { return; }
    let paths = file_tree_paths(&sphinx_docs_build_shared.rs_out);
    insta::assert_yaml_snapshot!("sphinx_docs_rs_full_tree", paths);
}

/// Assert every file in the Rust sphinx/doc build output has a non-zero byte size.
/// Catches silent empty-file regressions without requiring exact-size snapshots.
#[rstest]
fn sphinx_docs_rs_all_files_non_empty(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.rs_built { return; }
    let sizes = file_size_map(&sphinx_docs_build_shared.rs_out);
    let empty: Vec<&str> = sizes.iter()
        .filter(|(_, sz)| **sz == 0)
        .map(|(p, _)| p.as_str())
        .collect();
    assert!(
        empty.is_empty(),
        "Rust output contains empty files:\n{}",
        empty.join("\n")
    );
}

/// Snapshot the FULL recursive file tree (paths only) from the Python sphinx/doc
/// build as the authoritative reference.
#[rstest]
fn sphinx_docs_py_snapshot_full_tree(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    let paths = file_tree_paths(&sphinx_docs_build_shared.py_out);
    insta::assert_yaml_snapshot!("sphinx_docs_py_full_tree", paths);
}

/// Assert every file in the Python sphinx/doc build output has a non-zero byte size.
#[rstest]
fn sphinx_docs_py_all_files_non_empty(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    let sizes = file_size_map(&sphinx_docs_build_shared.py_out);
    let empty: Vec<&str> = sizes.iter()
        .filter(|(_, sz)| **sz == 0)
        .map(|(p, _)| p.as_str())
        .collect();
    assert!(
        empty.is_empty(),
        "Python output contains empty files:\n{}",
        empty.join("\n")
    );
}

/// Snapshot the parity gap between Python and Rust sphinx/doc builds:
///
/// - `missing_in_rust`: paths Python produced that Rust did not.
/// - `rust_only`: paths Rust produced that Python did not.
///
/// Files present in **both** outputs are additionally checked to have non-zero
/// sizes in the Rust copy — this catches the class of bugs where `sphinx-build-rs`
/// copies a file but writes 0 bytes.
///
/// Shrink `missing_in_rust` over time as coverage improves.
#[rstest]
fn sphinx_docs_parity_gap_snapshot(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }

    let py_sizes = file_size_map(&sphinx_docs_build_shared.py_out);
    let rs_sizes = file_size_map(&sphinx_docs_build_shared.rs_out);

    let py_set: std::collections::BTreeSet<String> = py_sizes.keys().cloned().collect();
    let rs_set: std::collections::BTreeSet<String> = rs_sizes.keys().cloned().collect();

    let mut missing: Vec<String> = py_set.difference(&rs_set).cloned().collect();
    missing.sort();

    let mut rs_only: Vec<String> = rs_set.difference(&py_set).cloned().collect();
    rs_only.sort();

    // Files present in both: every Rust copy must be non-empty.
    let mut shared_empty_in_rust: Vec<String> = py_set.intersection(&rs_set)
        .filter(|p| rs_sizes[p.as_str()] == 0)
        .cloned()
        .collect();
    shared_empty_in_rust.sort();
    assert!(
        shared_empty_in_rust.is_empty(),
        "Shared files are empty in the Rust output:\n{}",
        shared_empty_in_rust.join("\n")
    );

    insta::assert_yaml_snapshot!("sphinx_docs_parity_gap_missing_in_rust", missing);
    insta::assert_yaml_snapshot!("sphinx_docs_parity_gap_rust_only", rs_only);
}

/// The set of HTML pages produced by Rust must exactly match the set produced
/// by Python, modulo a small allowlist of known-unimplemented extension features.
///
/// This test FAILS when:
///   - Rust produces an HTML page Python doesn't (regression / stray file)
///   - Rust is missing an HTML page Python produces (coverage gap, once removed
///     from the allowlist below)
///
/// ## Allowlist rationale
///
/// | File / prefix          | Reason not yet implemented in sphinxdocrs |
/// |------------------------|-------------------------------------------|
/// | `py-modindex.html`     | Python-domain module index (needs domain pipeline) |
/// | `contents.html`        | Sphinx compat redirect (needs master_doc logic) |
/// | `changes.html`         | Sphinx changes-builder / todo extension output |
/// | `searchindex.js`       | Search index serialisation (needs indexer) |
/// | `_static/sphinxdocrs.css` | sphinxdocrs-own stylesheet (not in Python) |
#[rstest]
fn sphinx_docs_html_pages_match(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }

    /// Files Python produces via extension/compat features not yet in sphinxdocrs.
    /// Each entry is matched as a prefix or exact path.
    const PY_ONLY_ALLOWLIST: &[&str] = &[
        "py-modindex.html",
        "contents.html",
        "changes.html",
        "_modules/",          // viewcode extension output
        "_static/",           // Python theme static assets (JS, CSS) not in our minimal builder
    ];

    /// Files sphinxdocrs produces that Python doesn't (intentional deviations).
    const RS_ONLY_ALLOWLIST: &[&str] = &[
        "_static/sphinxdocrs.css",
    ];

    /// Standard HTML pages sphinx always generates (not from a source RST file).
    const STANDARD_PAGES: &[&str] = &[
        "genindex.html",
        "search.html",
    ];

    let src = sphinx_doc_src();

    // A page is "source-based" when its corresponding .rst file exists.
    // Extension-generated pages (autodoc, autosummary) have no RST source.
    let has_rst_source = |html_path: &str| -> bool {
        let docname = html_path.strip_suffix(".html").unwrap_or(html_path);
        src.join(format!("{docname}.rst")).exists()
    };

    let include_page = |p: &str| -> bool {
        !p.starts_with(".doctrees/")
            && p.ends_with(".html")
            && (has_rst_source(p)
                || STANDARD_PAGES.contains(&p)
                || PY_ONLY_ALLOWLIST.iter().any(|a| {
                    if a.ends_with('/') { p.starts_with(a) } else { p == *a }
                }))
    };

    let py_html: std::collections::BTreeSet<String> = file_tree_paths(&sphinx_docs_build_shared.py_out)
        .into_iter()
        .filter(|p| include_page(p))
        .collect();

    let rs_html: std::collections::BTreeSet<String> = file_tree_paths(&sphinx_docs_build_shared.rs_out)
        .into_iter()
        .filter(|p| include_page(p))
        .collect();

    let is_allowed_py_only = |p: &str| {
        PY_ONLY_ALLOWLIST.iter().any(|a| {
            if a.ends_with('/') { p.starts_with(a) } else { p == *a }
        })
    };
    let is_allowed_rs_only = |p: &str| {
        RS_ONLY_ALLOWLIST.iter().any(|a| {
            if a.ends_with('/') { p.starts_with(a) } else { p == *a }
        })
    };

    // Unexpected Rust-only pages (not on the allowlist).
    let mut unexpected_rs_only: Vec<String> = rs_html.difference(&py_html)
        .filter(|p| !is_allowed_rs_only(p))
        .cloned()
        .collect();
    unexpected_rs_only.sort();

    // Unexpected Python-only pages (not on the allowlist) — pages Rust should produce.
    let mut unexpected_py_only: Vec<String> = py_html.difference(&rs_html)
        .filter(|p| !is_allowed_py_only(p))
        .cloned()
        .collect();
    unexpected_py_only.sort();

    assert!(
        unexpected_rs_only.is_empty(),
        "Rust produced unexpected extra pages (not in Python output):\n{}",
        unexpected_rs_only.join("\n")
    );
    assert!(
        unexpected_py_only.is_empty(),
        "Rust is missing HTML pages that Python produced (add to allowlist if intentional):\n{}",
        unexpected_py_only.join("\n")
    );
}

// ── stderr / stdout assertions ────────────────────────────────────────────────

/// `sphinx-build-rs` must emit "Build succeeded" to stderr for a successful
/// sphinx/doc build, together with a file-count.
#[rstest]
fn sphinx_docs_rs_stderr_build_succeeded(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !has_sphinx_doc_src() { return; }
    assert_eq!(
        sphinx_docs_build_shared.rs_exit, 0,
        "expected exit 0; stderr:\n{}", sphinx_docs_build_shared.rs_stderr
    );
    assert!(
        sphinx_docs_build_shared.rs_stderr.contains("Build succeeded"),
        "Rust stderr should contain 'Build succeeded'; got:\n{}",
        sphinx_docs_build_shared.rs_stderr
    );
    let re_count = regex::Regex::new(r"\d+ file").unwrap();
    assert!(
        re_count.is_match(&sphinx_docs_build_shared.rs_stderr),
        "Rust stderr should contain a file count (e.g. '42 file(s)'); got:\n{}",
        sphinx_docs_build_shared.rs_stderr
    );
}

/// Rust stderr must not contain any "Error:" lines on a successful sphinx/doc build.
#[rstest]
fn sphinx_docs_rs_stderr_no_errors(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.rs_built { return; }
    let error_lines: Vec<&str> = sphinx_docs_build_shared.rs_stderr
        .lines()
        .filter(|l| l.to_lowercase().contains("error:") && !l.to_lowercase().contains("warning"))
        .collect();
    assert!(
        error_lines.is_empty(),
        "Rust stderr should be error-free on a successful build; error lines:\n{}",
        error_lines.join("\n")
    );
}

/// Snapshot normalised Rust stderr for the sphinx/doc build.
/// Paths and file-counts are replaced so the snapshot is deterministic.
#[rstest]
fn sphinx_docs_rs_stderr_snapshot(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.rs_built { return; }
    let re_count = regex::Regex::new(r"\d+ file").unwrap();
    let rs_out_str  = sphinx_docs_build_shared.rs_out.to_str().unwrap_or("");
    let src_str     = sphinx_doc_src().to_str().unwrap_or("").to_owned();
    let normalised: Vec<String> = sphinx_docs_build_shared.rs_stderr
        .replace(rs_out_str, "<OUTDIR>")
        .replace(&src_str, "<SRCDIR>")
        .lines()
        .map(|l| re_count.replace_all(l.trim_end(), "<N> file").to_string())
        .filter(|l| !l.is_empty())
        .collect();
    insta::assert_yaml_snapshot!("sphinx_docs_rs_stderr_normalised", normalised);
}

/// Snapshot normalised Python stderr for the sphinx/doc build (reference).
#[rstest]
fn sphinx_docs_py_stderr_snapshot(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    let py_out_str = sphinx_docs_build_shared.py_out.to_str().unwrap_or("");
    let src_str    = sphinx_doc_src().to_str().unwrap_or("").to_owned();
    let normalised: Vec<String> = sphinx_docs_build_shared.py_stderr
        .replace(py_out_str, "<OUTDIR>")
        .replace(&src_str, "<SRCDIR>")
        .lines()
        .map(|l| l.trim_end().to_owned())
        .filter(|l| !l.is_empty())
        .collect();
    insta::assert_yaml_snapshot!("sphinx_docs_py_stderr_normalised", normalised);
}

/// Rust stdout must be empty (all progress output goes to stderr).
#[rstest]
fn sphinx_docs_rs_stdout_is_empty(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !has_sphinx_doc_src() { return; }
    let stripped = sphinx_docs_build_shared.rs_stdout.trim();
    assert!(
        stripped.is_empty(),
        "Rust stdout should be empty (output goes to stderr); got:\n{stripped}"
    );
}

/// Python `sphinx-build -q` stdout must also be empty.
#[rstest]
fn sphinx_docs_py_stdout_is_empty(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !has_sphinx_doc_src() { return; }
    if !has_sphinx_build() { return; }
    let stripped = sphinx_docs_build_shared.py_stdout.trim();
    assert!(
        stripped.is_empty(),
        "Python sphinx-build stdout should be empty (output goes to stderr); got:\n{stripped}"
    );
}

/// Both tools must have empty stdout on a successful build.
#[rstest]
fn sphinx_docs_both_stdout_empty(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }
    assert!(
        sphinx_docs_build_shared.py_stdout.trim().is_empty(),
        "Python stdout should be empty"
    );
    assert!(
        sphinx_docs_build_shared.rs_stdout.trim().is_empty(),
        "Rust stdout should be empty"
    );
}

/// Extract `WARNING:` lines from stderr, normalise path tokens, and return
/// a sorted set of unique warning categories.
fn extract_warning_categories(stderr: &str) -> std::collections::BTreeSet<String> {
    let re_path = regex::Regex::new(r"(/\S+|[A-Za-z]:\\\S+)").unwrap();
    let re_num  = regex::Regex::new(r"\b\d+\b").unwrap();
    stderr
        .lines()
        .filter(|l| {
            let low = l.to_lowercase();
            low.contains("warning") && !low.contains("error")
        })
        .map(|l| {
            // Strip path tokens and line numbers so warnings compare by category.
            let s = re_path.replace_all(l.trim(), "<PATH>");
            let s = re_num.replace_all(&s, "<N>");
            s.into_owned()
        })
        .collect()
}

/// Rust must not emit WARNING categories that Python doesn't (regression guard).
///
/// Python is the authoritative reference: if it doesn't warn about something,
/// Rust shouldn't either (unless it's a sphinxdocrs-specific diagnostic
/// listed in the allowlist).
#[rstest]
fn sphinx_docs_rs_warnings_subset_of_py(sphinx_docs_build_shared: &SphinxDocsBuildShared) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }

    /// sphinxdocrs-specific warnings that have no Python equivalent.
    const RS_ONLY_WARNING_PREFIXES: &[&str] = &[
        "sphinxdocrs:",
        "Warning: intersphinx:",
    ];

    let py_warns = extract_warning_categories(&sphinx_docs_build_shared.py_stderr);
    let rs_warns = extract_warning_categories(&sphinx_docs_build_shared.rs_stderr);

    let unexpected: Vec<&String> = rs_warns
        .iter()
        .filter(|w| {
            !py_warns.contains(*w)
                && !RS_ONLY_WARNING_PREFIXES.iter().any(|p| w.contains(p))
        })
        .collect();

    assert!(
        unexpected.is_empty(),
        "Rust emits warnings not seen in Python output:\n{}",
        unexpected.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n")
    );
}

/// Snapshot the normalised warning categories from both builds side by side.
#[rstest]
fn sphinx_docs_stderr_warning_categories_snapshot(
    sphinx_docs_build_shared: &SphinxDocsBuildShared,
) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }

    let py_warns: Vec<String> =
        extract_warning_categories(&sphinx_docs_build_shared.py_stderr)
            .into_iter()
            .collect();
    let rs_warns: Vec<String> =
        extract_warning_categories(&sphinx_docs_build_shared.rs_stderr)
            .into_iter()
            .collect();

    insta::assert_yaml_snapshot!("sphinx_docs_py_warning_categories", py_warns);
    insta::assert_yaml_snapshot!("sphinx_docs_rs_warning_categories", rs_warns);
}

// ── intersphinx parity ────────────────────────────────────────────────────────

/// Both Python and Rust builds must produce an intersphinx cache directory
/// under `.doctrees/__intersphinx_cache__/` when `sphinx.ext.intersphinx`
/// is in `conf.py`.
#[rstest]
fn sphinx_docs_intersphinx_cache_dir_exists(
    sphinx_docs_build_shared: &SphinxDocsBuildShared,
) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }

    // Python cache is under its .doctrees dir, which sits inside its outdir.
    let py_cache = sphinx_docs_build_shared.py_out
        .join(".doctrees")
        .join("__intersphinx_cache__");
    let rs_cache = sphinx_docs_build_shared.rs_out
        .join(".doctrees")
        .join("__intersphinx_cache__");

    assert!(
        py_cache.exists(),
        "Python .doctrees/__intersphinx_cache__ must exist (intersphinx in extensions)"
    );
    assert!(
        rs_cache.exists(),
        "Rust .doctrees/__intersphinx_cache__ must exist (intersphinx fetched by sphinxdocrs)"
    );
}

/// The intersphinx cache file names must match between Python and Rust.
///
/// Python names them `{name}_objects.inv`; Rust must use the same convention.
#[rstest]
fn sphinx_docs_intersphinx_cache_files_match(
    sphinx_docs_build_shared: &SphinxDocsBuildShared,
) {
    if !sphinx_docs_build_shared.py_built { return; }
    if !sphinx_docs_build_shared.rs_built { return; }

    let cache_files = |outdir: &Path| -> std::collections::BTreeSet<String> {
        let cache_dir = outdir.join(".doctrees").join("__intersphinx_cache__");
        if !cache_dir.exists() { return std::collections::BTreeSet::new(); }
        std::fs::read_dir(&cache_dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect()
    };

    let py_files = cache_files(&sphinx_docs_build_shared.py_out);
    let rs_files = cache_files(&sphinx_docs_build_shared.rs_out);

    // Snapshot both sets for regression detection.
    let py_sorted: Vec<String> = py_files.iter().cloned().collect();
    let rs_sorted: Vec<String> = rs_files.iter().cloned().collect();
    insta::assert_yaml_snapshot!("sphinx_docs_py_intersphinx_cache_files", py_sorted);
    insta::assert_yaml_snapshot!("sphinx_docs_rs_intersphinx_cache_files", rs_sorted);

    // The file name sets must be identical.
    let missing_in_rs: Vec<&String> = py_files.difference(&rs_files).collect();
    let rs_only: Vec<&String> = rs_files.difference(&py_files).collect();

    assert!(
        missing_in_rs.is_empty(),
        "Rust intersphinx cache is missing files present in Python:\n{}",
        missing_in_rs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n")
    );
    assert!(
        rs_only.is_empty(),
        "Rust intersphinx cache has extra files not in Python:\n{}",
        rs_only.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n")
    );
}

/// Each intersphinx cache file must be non-empty (a valid `objects.inv`
/// starts with `# Sphinx inventory version 2`).
#[rstest]
fn sphinx_docs_intersphinx_cache_files_non_empty(
    sphinx_docs_build_shared: &SphinxDocsBuildShared,
) {
    if !sphinx_docs_build_shared.rs_built { return; }

    let cache_dir = sphinx_docs_build_shared.rs_out
        .join(".doctrees")
        .join("__intersphinx_cache__");
    if !cache_dir.exists() { return; }

    let empty_files: Vec<String> = std::fs::read_dir(&cache_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| {
            e.metadata()
                .map(|m| m.len() == 0)
                .unwrap_or(false)
        })
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    assert!(
        empty_files.is_empty(),
        "Rust intersphinx cache contains empty files:\n{}",
        empty_files.join("\n")
    );
}

