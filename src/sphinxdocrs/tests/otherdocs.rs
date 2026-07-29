//! Build-parity tests for real documentation trees — gated behind
//! `cfg(feature = "test-build-extdocs")`.
//!
//! For each docs directory that exists **and** contains both a `Makefile` and
//! a `conf.py`, the test:
//!
//! 1. Runs `make -C <docs_root> <builder> SPHINXBUILD=sphinx-build-rs` into
//!    a temporary output directory (via `SPHINXOPTS=-E -d <doctree-tmp>`).
//! 2. Runs `make -C <docs_root> <builder> SPHINXBUILD=sphinx-build` into a
//!    separate temporary output directory.
//! 3. Collects the sorted file-tree of each output dir and asserts equality.
//!
//! **Enable with:**
//!
//! ```text
//! cargo test -p sphinxdocrs --features test-build-extdocs --test otherdocs
//! ```
//!
//! The tests are parametrised via `rstest` `#[case]` so each docs directory
//! runs as an independent test case.  They are automatically skipped (via an
//! early return) when:
//!
//! - The directory does not exist on the current machine.
//! - The directory is missing a `Makefile` or `conf.py`.
//! - `sphinx-build` (upstream Python) is not found on `PATH`.
//! - `make` is not found on `PATH`.

#![cfg(feature = "test-build-extdocs")]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Condvar, Mutex, OnceLock};

use rstest::*;
use tempfile::TempDir;

use sphinxdocrs::scan::{find_requirements_files, scan_requirements};

// ── concurrency limiter ───────────────────────────────────────────────────────

/// Max concurrent sphinx-build subprocesses.  Each can consume 150–300 MB.
const MAX_BUILD_PROCS: usize = 2;

struct Semaphore {
    state: Mutex<usize>,
    cvar: Condvar,
}

impl Semaphore {
    fn new(n: usize) -> Self {
        Self {
            state: Mutex::new(n),
            cvar: Condvar::new(),
        }
    }

    fn acquire(&self) {
        let mut n = self.state.lock().unwrap();
        while *n == 0 {
            n = self.cvar.wait(n).unwrap();
        }
        *n -= 1;
    }

    fn release(&self) {
        let mut n = self.state.lock().unwrap();
        *n += 1;
        self.cvar.notify_one();
    }
}

static BUILD_SEM: OnceLock<Semaphore> = OnceLock::new();

// ── helpers ───────────────────────────────────────────────────────────────────

/// Run a command, rate-limit sphinx-build calls, return (exit_code, stdout, stderr).
fn run(program: &str, args: &[&str], cwd: &Path) -> (i32, String, String) {
    let is_heavy = program.contains("sphinx-build") || program == "make";
    let sem = BUILD_SEM.get_or_init(|| Semaphore::new(MAX_BUILD_PROCS));
    if is_heavy {
        sem.acquire();
    }
    let out = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap_or_else(|e| panic!("failed to run `{program}`: {e}"));
    if is_heavy {
        sem.release();
    }
    let code = out.status.code().unwrap_or(1);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    (code, stdout, stderr)
}

/// Walk a directory recursively and return a sorted list of relative paths.
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
                acc.push(rel.clone());
                if e.path().is_dir() {
                    walk(&e.path(), root, acc);
                }
            }
        }
    }
    walk(root, root, &mut out);
    out
}

/// Resolve a docs path given as a workspace-relative string.
///
/// The path is resolved relative to the workspace root, which is assumed to be
/// two directories above the crate manifest (src/sphinxdocrs → workspace root).
fn workspace_docs_path(rel: &str) -> PathBuf {
    // CARGO_MANIFEST_DIR is /workspaces/dsport/src/sphinxdocrs at test time.
    let manifest: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    let workspace_root = manifest
        .parent() // src/
        .and_then(|p| p.parent()) // workspace root
        .expect("cannot derive workspace root from CARGO_MANIFEST_DIR")
        .to_path_buf();
    workspace_root.join(rel)
}

fn has_program(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Return true if `docs_root` qualifies for the test:
/// - directory exists
/// - contains `Makefile`
/// - contains `conf.py`
fn qualifies(docs_root: &Path) -> bool {
    docs_root.is_dir()
        && docs_root.join("Makefile").is_file()
        && docs_root.join("conf.py").is_file()
}

// ── dependency detection (delegates to sphinxdocrs::scan) ────────────────────

/// Wraps [`scan_requirements`] to return a simple missing-deps summary for use
/// in the pre-flight skip check.
fn check_missing_deps_for_skip(docs_root: &Path, workspace_root: &Path) -> Option<String> {
    let result = scan_requirements(docs_root, workspace_root);
    let missing = result.missing();
    if missing.is_empty() {
        return None;
    }
    let mut msg = String::from("  missing Python packages:\n");
    for p in &missing {
        msg.push_str(&format!("    - {} ({})\n", p.name, p.source));
    }
    let req_files = find_requirements_files(docs_root, workspace_root);
    if !req_files.is_empty() {
        msg.push_str("  requirements/dependency files found:\n");
        for f in &req_files {
            msg.push_str(&format!("    {}\n", f.display()));
        }
    }
    Some(msg)
}

// ── core build helper ─────────────────────────────────────────────────────────

/// Build docs in `docs_root` using `SPHINXBUILD=<sphinx_build_bin>` via `make`.
///
/// Output is placed under `out_dir`.  BUILDDIR is overridden to a unique temp
/// dir, which also isolates the doctree cache.  We pass `-E` to force a fresh
/// build but intentionally omit `-d` because `sphinx-build -M` already derives
/// the doctree path from BUILDDIR; supplying `-d` a second time via SPHINXOPTS
/// causes clap to reject the duplicate flag.
///
/// Returns `(exit_code, stderr)`.
fn sphinx_make_build(
    docs_root: &Path,
    sphinx_build_bin: &str,
    builder: &str,
    out_dir: &Path,
) -> (i32, String) {
    let builddir = out_dir.display().to_string();

    let (code, _stdout, stderr) = run(
        "make",
        &[
            "-C",
            docs_root.to_str().unwrap(),
            builder,
            &format!("SPHINXBUILD={sphinx_build_bin}"),
            &format!("BUILDDIR={builddir}"),
            "SPHINXOPTS=-E",
        ],
        docs_root,
    );
    (code, stderr)
}

// ── parametrised test ─────────────────────────────────────────────────────────

#[rstest]
#[case("docs", "html")]
#[case("src/sphinx/doc", "html")]
#[case("src/sphinxdocrs/docs", "html")]
#[case("src/jinja2/docs", "html")]
#[case("src/MyST-Parser/docs", "html")]
#[case("src/pygments/doc", "html")]
#[case("src/pygmentsrs/docs", "html")]
fn build_otherdocs_parity(#[case] docs_rel: &str, #[case] builder: &str) {
    // ── pre-flight checks ──────────────────────────────────────────────────
    let docs_root = workspace_docs_path(docs_rel);

    if !qualifies(&docs_root) {
        eprintln!("SKIP {docs_rel}: directory missing or lacks Makefile/conf.py");
        return;
    }

    if !has_program("make") {
        eprintln!("SKIP {docs_rel}: `make` not found in PATH");
        return;
    }

    if !has_program("sphinx-build") {
        eprintln!("SKIP {docs_rel}: `sphinx-build` not found in PATH");
        return;
    }

    // ── check for missing Python dependencies ──────────────────────────────
    let workspace_root = workspace_docs_path("");
    if let Some(msg) = check_missing_deps_for_skip(&docs_root, &workspace_root) {
        eprintln!("SKIP {docs_rel}: missing Python dependencies required for docs build\n{msg}");
        return;
    }

    // ── locate sphinx-build-rs ─────────────────────────────────────────────
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-build-rs");

    // ── temp dirs ─────────────────────────────────────────────────────────
    let rs_out = TempDir::new().expect("create rs_out tempdir");
    let py_out = TempDir::new().expect("create py_out tempdir");

    // ── run sphinx-build-rs ────────────────────────────────────────────────
    let (rs_code, rs_stderr) = sphinx_make_build(&docs_root, rs_bin, builder, rs_out.path());

    // ── run sphinx-build (Python) ──────────────────────────────────────────
    let (py_code, py_stderr) =
        sphinx_make_build(&docs_root, "sphinx-build", builder, py_out.path());

    // ── assertions ────────────────────────────────────────────────────────
    // If the Python build itself failed due to missing packages (ImportError /
    // ModuleNotFoundError in stderr), report the missing packages and skip
    // rather than failing the parity assertion.
    if py_code != 0
        && (py_stderr.contains("ModuleNotFoundError") || py_stderr.contains("ImportError"))
    {
        // Collect the missing module names from the error lines.
        let missing: Vec<&str> = py_stderr
            .lines()
            .filter(|l| l.contains("ModuleNotFoundError") || l.contains("No module named"))
            .collect();
        eprintln!(
            "SKIP {docs_rel}: Python build failed due to missing packages:\n{}",
            missing.join("\n")
        );
        return;
    }

    assert_eq!(
        py_code, rs_code,
        "exit code mismatch for `{docs_rel}` builder=`{builder}`\n\
         Python exit={py_code}, Rust exit={rs_code}\n\
         --- Python stderr ---\n{py_stderr}\n\
         --- Rust stderr ---\n{rs_stderr}"
    );

    // Collect file trees from the builder sub-directory (e.g. _build/html or
    // the root when BUILDDIR was overridden to a flat temp dir).
    // Sphinx writes output to $BUILDDIR/<builder>/, so look there first.
    let rs_build_sub = rs_out.path().join(builder);
    let py_build_sub = py_out.path().join(builder);

    let rs_root = if rs_build_sub.is_dir() {
        rs_build_sub.as_path()
    } else {
        rs_out.path()
    };
    let py_root = if py_build_sub.is_dir() {
        py_build_sub.as_path()
    } else {
        py_out.path()
    };

    let rs_tree = list_tree(rs_root);
    let py_tree = list_tree(py_root);

    // If the Python build produced no files the run was a no-op (sphinx-build
    // not on PATH for make, or wrote to an unexpected location).  Skip rather
    // than recording an empty snapshot.
    if py_tree.is_empty() {
        eprintln!(
            "SKIP snapshot for `{docs_rel}` builder=`{builder}`: \
             Python output tree is empty (sphinx-build may not be on PATH for make)"
        );
        return;
    }

    // Snapshot both trees: Python = reference, Rust = current progress.
    // Snapshot failures mean Python changed (update reference) or Rust changed
    // (regressed or improved — review with `cargo insta review`).
    let snap_base = format!(
        "otherdocs__{}__{}",
        docs_rel.replace(['/', '\\', '.'], "_"),
        builder,
    );
    insta::assert_yaml_snapshot!(format!("{snap_base}_python_tree"), py_tree.clone());
    insta::assert_yaml_snapshot!(format!("{snap_base}_rust_tree"), rs_tree.clone());
}

// ── index.html content parity ─────────────────────────────────────────────────
//
// `build_otherdocs_parity` (above) only compares file *names* — it says
// nothing about whether the pages Rust does produce match Python's
// rendered *content*. This test builds the same real doc trees again and
// diffs each side's rendered `index.html` after stripping known-volatile
// substrings (the Sphinx / `sphinxdocrs` generator version strings),
// tracking the diff as an insta snapshot — the same "snapshot the current
// parity gap, shrink it over time" pattern `tests/parity.rs`'s
// `html_parity_gap_*`/`sphinx_docs_parity_gap_*` tests already use for
// structural gaps. A snapshot failure means the diff between the two
// sides changed (for better or worse) — review with `cargo insta review`.

/// Strip substrings from rendered HTML that legitimately differ between a
/// Python and a Rust build of the identical project, so this test tracks
/// real content drift rather than expected per-build noise.
fn normalize_html_for_diff(html: &str) -> String {
    // Sphinx's own generator string, e.g. "Sphinx v9.1.1+/f60972202" /
    // "Sphinx 9.1.1".
    let re_sphinx_version =
        regex::Regex::new(r"Sphinx v?\d+\.\d+(\.\d+)?(\+/[0-9a-f]+)?").expect("valid regex");
    let mut out = re_sphinx_version
        .replace_all(html, "Sphinx VERSION")
        .into_owned();
    // `theme_render.rs` reports this crate's own version as
    // `sphinx_version` in lieu of a bundled Python Sphinx version.
    out = out.replace(env!("CARGO_PKG_VERSION"), "VERSION");
    out
}

/// Render a compact unified-style diff between `a` (labelled `label_a`)
/// and `b` (labelled `label_b`), capped to `max_lines` changed lines so a
/// wildly different page (e.g. Rust's output missing autodoc/graphviz
/// content entirely) doesn't blow up the snapshot file — callers only
/// need enough to see *what kind* of drift is happening, not a full
/// byte-level transcript.
fn compact_line_diff(label_a: &str, a: &str, label_b: &str, b: &str, max_lines: usize) -> String {
    let diff = similar::TextDiff::from_lines(a, b);
    let mut out = String::new();
    let mut shown = 0usize;
    let mut truncated = false;
    for change in diff.iter_all_changes() {
        if change.tag() == similar::ChangeTag::Equal {
            continue;
        }
        if max_lines != 0 && shown >= max_lines {
            truncated = true;
            break;
        }
        let sign = match change.tag() {
            similar::ChangeTag::Delete => '-',
            similar::ChangeTag::Insert => '+',
            similar::ChangeTag::Equal => ' ',
        };
        out.push(sign);
        out.push(' ');
        out.push_str(change.value());
        if !change.value().ends_with('\n') {
            out.push('\n');
        }
        shown += 1;
    }
    if truncated {
        out.push_str("... (diff truncated) ...\n");
    }
    if out.is_empty() {
        format!("{label_a} and {label_b} are identical after normalization\n")
    } else {
        out
    }
}

#[rstest]
#[case("docs", "html")]
#[case("src/sphinx/doc", "html")]
#[case("src/sphinxdocrs/docs", "html")]
#[case("src/jinja2/docs", "html")]
#[case("src/MyST-Parser/docs", "html")]
#[case("src/pygments/doc", "html")]
#[case("src/pygmentsrs/docs", "html")]
fn build_otherdocs_index_html_content_parity(#[case] docs_rel: &str, #[case] builder: &str) {
    // ── pre-flight checks (mirrors build_otherdocs_parity) ─────────────────
    let docs_root = workspace_docs_path(docs_rel);

    if !qualifies(&docs_root) {
        eprintln!("SKIP {docs_rel}: directory missing or lacks Makefile/conf.py");
        return;
    }
    if !has_program("make") {
        eprintln!("SKIP {docs_rel}: `make` not found in PATH");
        return;
    }
    if !has_program("sphinx-build") {
        eprintln!("SKIP {docs_rel}: `sphinx-build` not found in PATH");
        return;
    }

    let workspace_root = workspace_docs_path("");
    if let Some(msg) = check_missing_deps_for_skip(&docs_root, &workspace_root) {
        eprintln!("SKIP {docs_rel}: missing Python dependencies required for docs build\n{msg}");
        return;
    }

    // ── build both sides ────────────────────────────────────────────────────
    let rs_bin = env!("CARGO_BIN_EXE_sphinx-build-rs");
    let rs_out = TempDir::new().expect("create rs_out tempdir");
    let py_out = TempDir::new().expect("create py_out tempdir");

    let (rs_code, rs_stderr) = sphinx_make_build(&docs_root, rs_bin, builder, rs_out.path());
    let (py_code, py_stderr) =
        sphinx_make_build(&docs_root, "sphinx-build", builder, py_out.path());

    if py_code != 0
        && (py_stderr.contains("ModuleNotFoundError") || py_stderr.contains("ImportError"))
    {
        let missing: Vec<&str> = py_stderr
            .lines()
            .filter(|l| l.contains("ModuleNotFoundError") || l.contains("No module named"))
            .collect();
        eprintln!(
            "SKIP {docs_rel}: Python build failed due to missing packages:\n{}",
            missing.join("\n")
        );
        return;
    }
    if py_code != 0 {
        eprintln!("SKIP {docs_rel}: Python build failed (exit {py_code}):\n{py_stderr}");
        return;
    }
    if rs_code != 0 {
        eprintln!(
            "SKIP {docs_rel}: Rust build failed (exit {rs_code}), nothing to compare:\n{rs_stderr}"
        );
        return;
    }

    // ── locate index.html on each side ──────────────────────────────────────
    let rs_build_sub = rs_out.path().join(builder);
    let py_build_sub = py_out.path().join(builder);
    let rs_root = if rs_build_sub.is_dir() {
        rs_build_sub.as_path()
    } else {
        rs_out.path()
    };
    let py_root = if py_build_sub.is_dir() {
        py_build_sub.as_path()
    } else {
        py_out.path()
    };

    let py_index = py_root.join("index.html");
    let rs_index = rs_root.join("index.html");

    if !py_index.is_file() {
        eprintln!("SKIP {docs_rel}: Python build produced no index.html");
        return;
    }
    assert!(
        rs_index.is_file(),
        "Rust build for `{docs_rel}` produced no index.html at {} even though Python did",
        rs_index.display()
    );

    let py_html = std::fs::read_to_string(&py_index)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", py_index.display()));
    let rs_html = std::fs::read_to_string(&rs_index)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", rs_index.display()));

    let py_norm = normalize_html_for_diff(&py_html);
    let rs_norm = normalize_html_for_diff(&rs_html);

    // ── diff + snapshot ──────────────────────────────────────────────────────
    let diff = compact_line_diff("python", &py_norm, "rust", &rs_norm, 0);

    let snap_name = format!(
        "otherdocs__{}__{}__index_html_diff",
        docs_rel.replace(['/', '\\', '.'], "_"),
        builder,
    );
    insta::assert_snapshot!(snap_name, diff);
}
