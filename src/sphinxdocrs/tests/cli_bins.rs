//! CLI-boundary coverage for the native Sphinx binary entrypoints.
//!
//! These tests deliberately spawn Cargo's binary targets so coverage is
//! attributed to `src/bin/*.rs`, not only to the library helpers.

use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

const APIDOC_RS: &str = env!("CARGO_BIN_EXE_sphinx-apidoc-rs");
const AUTOGEN_RS: &str = env!("CARGO_BIN_EXE_sphinx-autogen-rs");
const BUILD_RS: &str = env!("CARGO_BIN_EXE_sphinx-build-rs");
const ERRORS_RS: &str = env!("CARGO_BIN_EXE_sphinx-errors-rs");

fn run(binary: &str, args: &[&str], cwd: &Path) -> Output {
    Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap_or_else(|error| panic!("failed to run {binary}: {error}"))
}

fn assert_exit(output: &Output, expected: i32) {
    assert_eq!(
        output.status.code(),
        Some(expected),
        "unexpected exit status; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn write_package(root: &Path) -> std::path::PathBuf {
    let package = root.join("mypkg");
    std::fs::create_dir_all(package.join("sub")).unwrap();
    for relative in [
        "__init__.py",
        "core.py",
        "excluded.py",
        "sub/__init__.py",
        "sub/helper.py",
    ] {
        std::fs::write(package.join(relative), b"").unwrap();
    }
    package
}

#[test]
fn apidoc_covers_validation_generation_exclusion_and_remove_old() {
    let tmp = TempDir::new().unwrap();
    let package = write_package(tmp.path());
    let output_dir = tmp.path().join("apidoc");

    let invalid = run(
        APIDOC_RS,
        &["-o", output_dir.to_str().unwrap(), "missing"],
        tmp.path(),
    );
    assert_exit(&invalid, 1);
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("not a directory"));

    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::write(output_dir.join("stale.rst"), b"stale").unwrap();
    let generated = run(
        APIDOC_RS,
        &[
            "-o",
            output_dir.to_str().unwrap(),
            "--force",
            "--remove-old",
            package.to_str().unwrap(),
            "*/excluded.py",
            "excluded?.py",
            "module[abc].py",
        ],
        tmp.path(),
    );
    assert_exit(&generated, 0);
    assert!(output_dir.join("modules.rst").is_file());
    assert!(output_dir.join("mypkg.rst").is_file());
    assert!(!output_dir.join("stale.rst").exists());
    assert!(!output_dir.to_string_lossy().contains("excluded"));
    assert!(!output_dir.join("mypkg.excluded.rst").exists());
}

#[test]
fn apidoc_covers_dry_run_full_and_output_errors() {
    let tmp = TempDir::new().unwrap();
    let package = write_package(tmp.path());
    let dry_output = tmp.path().join("dry");

    let dry_run = run(
        APIDOC_RS,
        &[
            "--dry-run",
            "--no-toc",
            "-o",
            dry_output.to_str().unwrap(),
            package.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_exit(&dry_run, 0);
    assert!(!dry_output.exists());

    let full_output = tmp.path().join("full");
    let full = run(
        APIDOC_RS,
        &[
            "--full",
            "--doc-project",
            "Full Project",
            "--doc-author",
            "An Author",
            "-o",
            full_output.to_str().unwrap(),
            package.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_exit(&full, 0);
    assert!(full_output.join("conf.py").is_file());
    assert!(full_output.join("index.rst").is_file());
    assert!(full_output.join("Makefile").is_file());

    let output_file = tmp.path().join("not-a-directory");
    std::fs::write(&output_file, b"file").unwrap();
    let error = run(
        APIDOC_RS,
        &[
            "-o",
            output_file.to_str().unwrap(),
            package.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_exit(&error, 1);
    assert!(String::from_utf8_lossy(&error.stderr).contains("cannot create"));
}

#[test]
fn apidoc_parse_error_is_reported() {
    let tmp = TempDir::new().unwrap();
    let output = run(APIDOC_RS, &[], tmp.path());
    assert_exit(&output, 2);
    assert!(String::from_utf8_lossy(&output.stderr).contains("required"));
}

#[test]
fn apidoc_python_fallback_help_is_available() {
    let tmp = TempDir::new().unwrap();
    let output = run(APIDOC_RS, &["--use-python-impl", "--help"], tmp.path());
    assert_exit(&output, 0);
}

#[test]
fn autogen_covers_empty_entries_output_selection_and_remove_old() {
    let tmp = TempDir::new().unwrap();
    let empty_source = tmp.path().join("empty.rst");
    std::fs::write(&empty_source, "Plain text.\n").unwrap();
    let empty = run(AUTOGEN_RS, &[empty_source.to_str().unwrap()], tmp.path());
    assert_exit(&empty, 0);

    let source = tmp.path().join("autosummary.rst");
    std::fs::write(
        &source,
        ".. autosummary::\n   :toctree: api\n\n   mymod\n   mypkg.MyClass\n",
    )
    .unwrap();
    let output_dir = tmp.path().join("stubs");
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::write(output_dir.join("stale.rst"), b"stale").unwrap();
    let generated = run(
        AUTOGEN_RS,
        &[
            "-o",
            output_dir.to_str().unwrap(),
            "--remove-old",
            source.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_exit(&generated, 0);
    assert!(output_dir.join("mymod.rst").is_file());
    assert!(output_dir.join("mypkg.MyClass.rst").is_file());
    assert!(!output_dir.join("stale.rst").exists());

    let fallback_source = tmp.path().join("fallback.rst");
    std::fs::write(
        &fallback_source,
        ".. autosummary::\n   :toctree: fallback\n\n   another_module\n",
    )
    .unwrap();
    let fallback = run(AUTOGEN_RS, &[fallback_source.to_str().unwrap()], tmp.path());
    assert_exit(&fallback, 0);
    assert!(tmp.path().join("fallback/another_module.rst").is_file());

    let no_tree_source = tmp.path().join("no-tree.rst");
    std::fs::write(&no_tree_source, ".. autosummary::\n\n   no_tree_module\n").unwrap();

    let missing_output = run(AUTOGEN_RS, &[no_tree_source.to_str().unwrap()], tmp.path());
    assert_exit(&missing_output, 1);
    assert!(String::from_utf8_lossy(&missing_output.stderr).contains("no output directory"));
}

#[test]
fn autogen_covers_empty_output_path_and_long_entry_list() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("many.rst");
    let entries = (0..21)
        .map(|index| format!("   module_{index}\n"))
        .collect::<String>();
    std::fs::write(&source, format!(".. autosummary::\n\n{entries}")).unwrap();

    let output = run(
        AUTOGEN_RS,
        &["-o", "", source.to_str().unwrap()],
        tmp.path(),
    );
    assert_exit(&output, 0);
    assert!(tmp.path().join("module_0.rst").is_file());
    assert!(tmp.path().join("module_20.rst").is_file());
    assert!(String::from_utf8_lossy(&output.stderr).contains("..."));
}

#[test]
fn build_covers_parse_scan_native_success_and_native_error() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("docs");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::write(
        source.join("conf.py"),
        "project = 'CLI coverage'\nmaster_doc = 'index'\nextensions = []\n",
    )
    .unwrap();
    std::fs::write(
        source.join("index.rst"),
        "CLI coverage\n===========\n\nText.\n",
    )
    .unwrap();

    let scan = run(
        BUILD_RS,
        &[
            "--scan-requirements",
            source.to_str().unwrap(),
            "unused-out",
        ],
        tmp.path(),
    );
    assert_exit(&scan, 0);

    let output_dir = tmp.path().join("html");
    let build = run(
        BUILD_RS,
        &[
            "-b",
            "html",
            "-E",
            source.to_str().unwrap(),
            output_dir.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_exit(&build, 0);
    assert!(output_dir.join("index.html").is_file());

    let malformed = tmp.path().join("missing-source");
    let failed = run(
        BUILD_RS,
        &["-b", "html", malformed.to_str().unwrap(), "bad-out"],
        tmp.path(),
    );
    assert_exit(&failed, 1);

    let missing_requirement = tmp.path().join("missing-requirement");
    std::fs::create_dir_all(&missing_requirement).unwrap();
    std::fs::write(
        missing_requirement.join("conf.py"),
        "import package_that_is_not_installed_for_coverage\n",
    )
    .unwrap();
    let requirement_scan = run(
        BUILD_RS,
        &[
            "--scan-requirements",
            missing_requirement.to_str().unwrap(),
            "missing-requirement-out",
        ],
        tmp.path(),
    );
    assert_exit(&requirement_scan, 1);

    let invalid_source = tmp.path().join("invalid-source");
    std::fs::create_dir_all(&invalid_source).unwrap();
    std::fs::write(invalid_source.join("conf.py"), "project = 'Invalid'\n").unwrap();
    std::fs::write(invalid_source.join("index.rst"), [0xff, 0xfe, 0xfd]).unwrap();
    let invalid_build = run(
        BUILD_RS,
        &[
            "-b",
            "html",
            invalid_source.to_str().unwrap(),
            "invalid-out",
        ],
        tmp.path(),
    );
    assert_exit(&invalid_build, 1);
}

#[test]
fn build_covers_make_mode_and_non_native_fallback_dispatch() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("docs");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::write(source.join("conf.py"), "project = 'Make mode'\n").unwrap();
    std::fs::write(source.join("index.rst"), "Make mode\n=========\n").unwrap();

    let make_output = tmp.path().join("make-html");
    let make = run(
        BUILD_RS,
        &[
            "-M",
            "html",
            source.to_str().unwrap(),
            make_output.to_str().unwrap(),
            "-Q",
        ],
        tmp.path(),
    );
    assert_exit(&make, 0);
    assert!(
        make_output.join("html/index.html").is_file(),
        "make stderr: {}",
        String::from_utf8_lossy(&make.stderr)
    );

    let fallback = run(
        BUILD_RS,
        &[
            "-b",
            "pickle",
            source.to_str().unwrap(),
            tmp.path().join("fallback-out").to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(fallback.status.code(), Some(2));
}

#[cfg(feature = "sqlite-error-db")]
#[test]
fn build_covers_sqlite_diagnostic_recording() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("docs");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::write(source.join("conf.py"), "project = 'SQLite build'\n").unwrap();
    std::fs::write(source.join("index.rst"), "SQLite build\n============\n").unwrap();

    let database = tmp.path().join("build-errors.db");
    let success = run(
        BUILD_RS,
        &[
            "-b",
            "html",
            "--error-db",
            database.to_str().unwrap(),
            source.to_str().unwrap(),
            tmp.path().join("html").to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_exit(&success, 0);
    assert!(database.is_file());

    let unusable_database = tmp.path().join("database-directory");
    std::fs::create_dir_all(&unusable_database).unwrap();
    let failure = run(
        BUILD_RS,
        &[
            "-b",
            "html",
            "--error-db",
            unusable_database.to_str().unwrap(),
            tmp.path().join("missing-source").to_str().unwrap(),
            tmp.path().join("failed-html").to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_exit(&failure, 1);
}

#[test]
fn build_parse_error_is_reported() {
    let tmp = TempDir::new().unwrap();
    let output = run(BUILD_RS, &[], tmp.path());
    assert_exit(&output, 2);
}

#[test]
fn build_python_fallback_help_is_available() {
    let tmp = TempDir::new().unwrap();
    let output = run(BUILD_RS, &["--use-python-impl", "--help"], tmp.path());
    assert_exit(&output, 0);
}

#[test]
fn autogen_parse_error_is_reported() {
    let tmp = TempDir::new().unwrap();
    let output = run(AUTOGEN_RS, &[], tmp.path());
    assert_exit(&output, 2);
}

#[test]
fn autogen_python_fallback_help_is_available() {
    let tmp = TempDir::new().unwrap();
    let output = run(AUTOGEN_RS, &["--use-python-impl", "--help"], tmp.path());
    assert_exit(&output, 0);
}

#[cfg(not(feature = "sqlite-error-db"))]
#[test]
fn errors_covers_default_disabled_import_and_sqlite_paths() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("build.log");
    std::fs::write(&source, "WARNING: docs/index.rst: warning\n").unwrap();
    let database = tmp.path().join("errors.db");

    let missing_source = run(ERRORS_RS, &[], tmp.path());
    assert_exit(&missing_source, 1);
    assert!(String::from_utf8_lossy(&missing_source.stderr)
        .contains("SQLite error logging is disabled"));

    let missing_db = run(
        ERRORS_RS,
        &["--format", "text", source.to_str().unwrap()],
        tmp.path(),
    );
    assert_exit(&missing_db, 1);
    assert!(
        String::from_utf8_lossy(&missing_db.stderr).contains("SQLite error logging is disabled")
    );

    let import = run(
        ERRORS_RS,
        &[
            "--format",
            "json",
            "--db",
            database.to_str().unwrap(),
            source.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_exit(&import, 1);
    assert!(String::from_utf8_lossy(&import.stderr).contains("SQLite error logging is disabled"));

    let sqlite_without_source = run(ERRORS_RS, &["--format", "sqlite"], tmp.path());
    assert_exit(&sqlite_without_source, 1);
    assert!(String::from_utf8_lossy(&sqlite_without_source.stderr)
        .contains("SQLite error logging is disabled"));

    let sqlite_path = tmp.path().join("existing.sqlite3");
    let sqlite = run(ERRORS_RS, &[sqlite_path.to_str().unwrap()], tmp.path());
    assert_exit(&sqlite, 1);
    assert!(String::from_utf8_lossy(&sqlite.stderr).contains("SQLite error logging is disabled"));
}

#[cfg(feature = "sqlite-error-db")]
#[test]
fn errors_covers_sqlite_import_listing_and_empty_review() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("build.log");
    std::fs::write(&source, "/tmp/index.rst:12:4: WARNING: bad title\n").unwrap();
    let database = tmp.path().join("errors.db");

    let import = run(
        ERRORS_RS,
        &[
            "--format",
            "text",
            "--db",
            database.to_str().unwrap(),
            &source.to_string_lossy(),
        ],
        tmp.path(),
    );
    assert_exit(&import, 0);
    assert!(String::from_utf8_lossy(&import.stdout).contains("Imported build"));

    let listing = run(ERRORS_RS, &[database.to_str().unwrap()], tmp.path());
    assert_exit(&listing, 0);
    assert!(String::from_utf8_lossy(&listing.stdout).contains("bad title"));

    let empty_review = run(
        ERRORS_RS,
        &[
            "--format",
            "sqlite",
            "--db",
            database.to_str().unwrap(),
            "--interactive",
        ],
        tmp.path(),
    );
    assert_exit(&empty_review, 0);
}
