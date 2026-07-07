//! CLI-boundary tests for `sphinx-quickstart-rs` using snapbox + rstest.
//!
//! Invokes the binary as an external process and asserts on exit codes,
//! command output, and generated filesystem layout.  Internal-Rust
//! unit/component tests remain in `quickstart.rs`.

use rstest::*;
use snapbox::cmd::Command;

fn bin() -> Command {
    Command::new(snapbox::cmd::cargo_bin!("sphinx-quickstart-rs"))
}

// ── destpath positional argument ──────────────────────────────────────────────

/// Both a pre-existing destpath and an absent one work: all core files appear.
///
/// Cases:
/// - `preexisting_dir` — destpath directory already exists before the run.
/// - `auto_create_dir` — destpath is absent; the binary must create it.
#[rstest]
#[case::preexisting_dir("ProjA", "Alice", true)]
#[case::auto_create_dir("ProjB", "Bob", false)]
fn destpath_writes_core_files(
    #[case] project: &str,
    #[case] author: &str,
    #[case] pre_create: bool,
) {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("output");
    if pre_create {
        std::fs::create_dir_all(&dest).unwrap();
    }

    bin()
        .args(["-q", "-p", project, "-a", author, dest.to_str().unwrap()])
        .assert()
        .success();

    assert!(dest.is_dir(), "destpath dir should exist after run");
    for fname in ["conf.py", "index.rst", "Makefile", "make.bat"] {
        assert!(dest.join(fname).exists(), "{fname} not found in destpath");
    }
}

/// The project name from `-p` is embedded in the generated `conf.py`.
///
/// Verifies this holds across different project names and destpath locations.
/// Uses `snapbox::assert_data_eq!` with a `[..]` wildcard to confirm the
/// `project = ` declaration is present, then an exact contains-check for the
/// dynamic name.
#[rstest]
#[case("SnapProject", "Carol")]
#[case("AnotherProject", "Dave")]
fn destpath_conf_py_contains_project_name(#[case] project: &str, #[case] author: &str) {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("snap_dest");

    bin()
        .args(["-q", "-p", project, "-a", author, dest.to_str().unwrap()])
        .assert()
        .success();

    let conf = std::fs::read_to_string(dest.join("conf.py")).unwrap();
    // Structural check: the `project = ` assignment line must be present.
    let project_line = conf
        .lines()
        .find(|l| l.starts_with("project"))
        .unwrap_or("");
    snapbox::assert_data_eq!(project_line, snapbox::str!["project = [..]"]);
    // Content check: the declared project name must match what was passed in.
    assert!(
        conf.contains(project),
        "conf.py does not mention project name '{project}'"
    );
}

/// Omitting destpath defaults to the current working directory.
#[test]
fn default_destpath_is_current_dir() {
    let tmp = tempfile::tempdir().unwrap();

    bin()
        .args(["-q", "-p", "CwdTest", "-a", "Eve"])
        .current_dir(tmp.path())
        .assert()
        .success();

    assert!(tmp.path().join("conf.py").exists(), "conf.py not found in cwd");
    assert!(tmp.path().join("index.rst").exists(), "index.rst not found in cwd");
}

// ── interactive mode ──────────────────────────────────────────────────────────

/// Running without `-q` enters interactive mode.
///
/// The first prompt must show the positional destpath argument as its default
/// value: `> Root path for the documentation [testpath]: `.
///
/// stdin is piped with answers that accept all defaults plus the two required
/// fields (project name, author).  Prompt order mirrors `ask_user`:
///   path · sep · dot · project · author · version · release · language ·
///   suffix · master · 10 extensions · makefile · batchfile
#[test]
fn interactive_destpath_shown_as_default() {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("testpath");

    // One answer per prompt, accepting all defaults except required fields.
    let stdin = [
        "\n",            // path      — accept default (= "testpath")
        "n\n",           // sep       — no separate source/build dir
        "\n",            // dot       — accept "_"
        "MyProject\n",   // project   — required, no default
        "Me\n",          // author    — required, no default
        "\n",            // version   — accept ""
        "\n",            // release   — accept "" (= version)
        "\n",            // language  — accept "en"
        "\n",            // suffix    — accept ".rst"
        "\n",            // master    — accept "index"
        // 10 extension prompts (autodoc … githubpages) — all "n"
        "n\n", "n\n", "n\n", "n\n", "n\n",
        "n\n", "n\n", "n\n", "n\n", "n\n",
        "\n",            // makefile  — accept "y"
        "\n",            // batchfile — accept "y"
    ]
    .concat();

    let output = bin()
        .arg("testpath")          // relative name — shown as-is in the prompt default
        .current_dir(tmp.path())  // so the binary's cwd is the tempdir
        .stdin(stdin)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Welcome banner.
    assert!(
        stdout.contains("Welcome to the Sphinx quickstart utility"),
        "missing welcome banner;\nstdout:\n{stdout}"
    );

    // The path prompt must show the positional arg as the bracketed default.
    assert!(
        stdout.contains("Root path for the documentation [testpath]"),
        "destpath not shown as prompt default;\nstdout:\n{stdout}"
    );

    assert_eq!(
        output.status.code(),
        Some(0),
        "expected exit 0;\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Files must be generated at the destpath.
    assert!(dest.join("conf.py").exists(), "conf.py not generated");
    assert!(dest.join("index.rst").exists(), "index.rst not generated");
}
