//! Tests for `sphinxdocrs::scan` — the `--scan-requirements` implementation.
//!
//! Test categories:
//!
//! * **Unit** — `parse_conf_extensions`, `parse_conf_third_party_imports`,
//!   `find_requirements_files`, `stdlib_modules` (no I/O, use temp files).
//! * **Integration** — `scan_requirements` on real `conf.py` files in the
//!   workspace.
//! * **CLI** — run `sphinx-build-rs --scan-requirements <dir>` as a
//!   subprocess and assert exit code / stdout contents.

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use sphinxdocrs::scan::{
    PackageSource, find_requirements_files, parse_conf_extensions, parse_conf_third_party_imports,
    scan_requirements, stdlib_modules,
};

// ── helpers ───────────────────────────────────────────────────────────────────

/// Write `content` to `dir/conf.py` and return the path.
fn write_conf(dir: &Path, content: &str) -> PathBuf {
    let path = dir.join("conf.py");
    std::fs::write(&path, content).unwrap();
    path
}

/// Resolve a path relative to the workspace root.
fn ws(rel: &str) -> PathBuf {
    let manifest: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    let root = manifest.parent().unwrap().parent().unwrap();
    root.join(rel)
}

/// Path to the compiled `sphinx-build-rs` test binary.
const SPHINX_BUILD_RS: &str = env!("CARGO_BIN_EXE_sphinx-build-rs");

// ── parse_conf_extensions ─────────────────────────────────────────────────────

#[test]
fn extensions_empty_list() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(dir.path(), "extensions = []\n");
    assert!(parse_conf_extensions(&conf).is_empty());
}

#[test]
fn extensions_single_line() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(
        dir.path(),
        r#"extensions = ['sphinx.ext.autodoc', "sphinx.ext.todo"]"#,
    );
    let exts = parse_conf_extensions(&conf);
    assert_eq!(exts, vec!["sphinx.ext.autodoc", "sphinx.ext.todo"]);
}

#[test]
fn extensions_multi_line() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(
        dir.path(),
        "extensions = [\n    'sphinx.ext.autodoc',\n    'myst_parser',\n]\n",
    );
    let exts = parse_conf_extensions(&conf);
    assert!(exts.contains(&"sphinx.ext.autodoc".to_string()));
    assert!(exts.contains(&"myst_parser".to_string()));
}

#[test]
fn extensions_missing_conf_returns_empty() {
    let path = PathBuf::from("/nonexistent/conf.py");
    assert!(parse_conf_extensions(&path).is_empty());
}

// ── parse_conf_third_party_imports ────────────────────────────────────────────

#[test]
fn imports_skips_stdlib() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(dir.path(), "import os\nimport sys\nimport re\n");
    assert!(parse_conf_third_party_imports(&conf).is_empty());
}

#[test]
fn imports_skips_sphinx_and_docutils() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(
        dir.path(),
        "import sphinx\nfrom docutils import nodes\nfrom sphinx.ext import something\n",
    );
    assert!(parse_conf_third_party_imports(&conf).is_empty());
}

#[test]
fn imports_skips_comment_lines() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(dir.path(), "# import third_party\n");
    assert!(parse_conf_third_party_imports(&conf).is_empty());
}

#[test]
fn imports_detects_third_party() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(
        dir.path(),
        "import os\nimport alabaster\nfrom myst_parser import config as mconfig\n",
    );
    let imps = parse_conf_third_party_imports(&conf);
    assert!(
        imps.contains(&"alabaster".to_string()),
        "alabaster expected; got {imps:?}"
    );
    assert!(
        imps.contains(&"myst_parser".to_string()),
        "myst_parser expected; got {imps:?}"
    );
    assert!(!imps.contains(&"os".to_string()));
}

#[test]
fn imports_inline_comment_stripped() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(dir.path(), "import alabaster # nice theme\n");
    let imps = parse_conf_third_party_imports(&conf);
    assert_eq!(imps, vec!["alabaster"]);
}

#[test]
fn imports_dotted_module_top_level_only() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(dir.path(), "import some_pkg.sub.module\n");
    let imps = parse_conf_third_party_imports(&conf);
    assert_eq!(imps, vec!["some_pkg"]);
}

#[test]
fn imports_from_relative_skipped() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(
        dir.path(),
        "from . import utils\nfrom .helpers import foo\n",
    );
    assert!(parse_conf_third_party_imports(&conf).is_empty());
}

#[test]
fn imports_deduplicated() {
    let dir = TempDir::new().unwrap();
    let conf = write_conf(
        dir.path(),
        "import alabaster\nimport alabaster\nfrom alabaster import theme\n",
    );
    let imps = parse_conf_third_party_imports(&conf);
    assert_eq!(imps.iter().filter(|m| m.as_str() == "alabaster").count(), 1);
}

// ── stdlib_modules ────────────────────────────────────────────────────────────

#[test]
fn stdlib_contains_common_modules() {
    let s = stdlib_modules();
    for m in &[
        "os",
        "sys",
        "re",
        "io",
        "json",
        "pathlib",
        "typing",
        "__future__",
    ] {
        assert!(s.contains(*m), "{m} should be in stdlib set");
    }
}

#[test]
fn stdlib_does_not_contain_third_party() {
    let s = stdlib_modules();
    for m in &["alabaster", "myst_parser", "sphinxcontrib_websupport"] {
        assert!(!s.contains(*m), "{m} should not be in stdlib set");
    }
}

// ── find_requirements_files ───────────────────────────────────────────────────

#[test]
fn requirements_files_found_in_docs_root() {
    let root = TempDir::new().unwrap();
    let docs = root.path().join("docs");
    std::fs::create_dir_all(&docs).unwrap();
    std::fs::write(docs.join("requirements.txt"), "sphinx\n").unwrap();
    std::fs::write(root.path().join("pyproject.toml"), "[project]\n").unwrap();

    let found = find_requirements_files(&docs, root.path());
    let names: Vec<_> = found
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap())
        .collect();
    assert!(
        names.contains(&"requirements.txt"),
        "docs/requirements.txt expected; got {names:?}"
    );
    assert!(
        names.contains(&"pyproject.toml"),
        "root/pyproject.toml expected; got {names:?}"
    );
}

#[test]
fn requirements_files_empty_when_none_exist() {
    let root = TempDir::new().unwrap();
    let docs = root.path().join("docs");
    std::fs::create_dir_all(&docs).unwrap();
    let found = find_requirements_files(&docs, root.path());
    assert!(found.is_empty());
}

// ── scan_requirements (integration) ──────────────────────────────────────────

/// Scan a minimal conf.py with no extensions — all-present, no files.
#[test]
fn scan_minimal_conf() {
    let dir = TempDir::new().unwrap();
    write_conf(dir.path(), "extensions = []\nproject = 'test'\n");
    let result = scan_requirements(dir.path(), dir.path());
    assert!(
        result.all_present(),
        "empty extensions should be all-present"
    );
    assert!(result.missing().is_empty());
}

/// Scan a conf.py that imports a definitely-importable stdlib module that is
/// *not* filtered (shouldn't happen, but guards the pipeline).
/// Instead we check a known-good third-party package (sphinx itself re-exports
/// nothing we test here); instead, just verify the report format is correct.
#[test]
fn scan_report_format() {
    let dir = TempDir::new().unwrap();
    write_conf(dir.path(), "extensions = []\n");
    let result = scan_requirements(dir.path(), dir.path());
    let report = result.report();
    assert!(
        report.contains("Requirements scan:"),
        "report header missing: {report}"
    );
    assert!(report.contains("status:"), "status line missing: {report}");
}

/// Extensions containing a clearly missing package name should show as MISSING.
#[test]
fn scan_detects_missing_extension() {
    let dir = TempDir::new().unwrap();
    write_conf(
        dir.path(),
        "extensions = ['__definitely_not_installed_xyz_abc_123__']\n",
    );
    let result = scan_requirements(dir.path(), dir.path());
    assert!(!result.all_present());
    assert_eq!(result.missing().len(), 1);
    assert_eq!(
        result.missing()[0].name,
        "__definitely_not_installed_xyz_abc_123__"
    );
    assert_eq!(result.missing()[0].source, PackageSource::Extension);
    assert!(result.report().contains("MISSING"));
}

/// Third-party import that is not installed should appear as MISSING with
/// source = Import.
#[test]
fn scan_detects_missing_import() {
    let dir = TempDir::new().unwrap();
    write_conf(
        dir.path(),
        "import __definitely_not_installed_xyz_abc_123__\n",
    );
    let result = scan_requirements(dir.path(), dir.path());
    let missing = result.missing();
    assert!(!missing.is_empty());
    let pkg = missing
        .iter()
        .find(|p| p.name == "__definitely_not_installed_xyz_abc_123__");
    assert!(
        pkg.is_some(),
        "missing package not found in result: {result:?}"
    );
    assert_eq!(pkg.unwrap().source, PackageSource::Import);
}

/// Extensions that are importable should show as `ok`.
#[test]
fn scan_importable_extension_is_ok() {
    let dir = TempDir::new().unwrap();
    // `sphinx.ext.autodoc` ships with sphinx and is always importable.
    write_conf(dir.path(), "extensions = ['sphinx.ext.autodoc']\n");
    let result = scan_requirements(dir.path(), dir.path());
    // sphinx.ext.autodoc is under the sphinx namespace so it may be filtered
    // by parse_conf_extensions — extensions are NOT filtered by the
    // sphinx-prefix logic, only imports are. Confirm the entry is present.
    assert!(
        result
            .packages
            .iter()
            .any(|p| p.name == "sphinx.ext.autodoc"),
        "expected sphinx.ext.autodoc in packages"
    );
    let entry = result
        .packages
        .iter()
        .find(|p| p.name == "sphinx.ext.autodoc")
        .unwrap();
    assert!(entry.importable, "sphinx.ext.autodoc should be importable");
}

/// Scan `docs/` in the workspace root — the actual project docs.
#[test]
fn scan_workspace_docs() {
    let docs_root = ws("docs");
    if !docs_root.join("conf.py").is_file() {
        return; // skip if not available
    }
    let project_root = ws("");
    let result = scan_requirements(&docs_root, &project_root);
    // The report should be printable without panicking.
    let report = result.report();
    assert!(report.contains("Requirements scan:"));
}

// ── CLI integration ───────────────────────────────────────────────────────────

/// `--scan-requirements` exits 0 when all packages in a minimal conf.py are
/// present.
#[test]
fn cli_scan_requirements_exit0_all_present() {
    let dir = TempDir::new().unwrap();
    write_conf(dir.path(), "extensions = []\nproject = 'test'\n");
    // Provide a dummy outputdir so arg parsing succeeds.
    let out = dir.path().join("_build");
    let status = Command::new(SPHINX_BUILD_RS)
        .args([
            dir.path().to_str().unwrap(),
            out.to_str().unwrap(),
            "--scan-requirements",
        ])
        .output()
        .expect("failed to run sphinx-build-rs");
    assert_eq!(
        status.status.code(),
        Some(0),
        "expected exit 0; stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&status.stdout),
        String::from_utf8_lossy(&status.stderr),
    );
    let stdout = String::from_utf8_lossy(&status.stdout);
    assert!(stdout.contains("Requirements scan:"), "stdout: {stdout}");
    assert!(
        stdout.contains("status: all packages present"),
        "stdout: {stdout}"
    );
}

/// `--scan-requirements` exits 1 and reports MISSING when an extension cannot
/// be imported.
#[test]
fn cli_scan_requirements_exit1_missing() {
    let dir = TempDir::new().unwrap();
    write_conf(
        dir.path(),
        "extensions = ['__definitely_not_installed_xyz_abc_123__']\n",
    );
    let out = dir.path().join("_build");
    let status = Command::new(SPHINX_BUILD_RS)
        .args([
            dir.path().to_str().unwrap(),
            out.to_str().unwrap(),
            "--scan-requirements",
        ])
        .output()
        .expect("failed to run sphinx-build-rs");
    assert_eq!(
        status.status.code(),
        Some(1),
        "expected exit 1; stdout: {}",
        String::from_utf8_lossy(&status.stdout),
    );
    let stdout = String::from_utf8_lossy(&status.stdout);
    assert!(
        stdout.contains("MISSING"),
        "expected MISSING in output: {stdout}"
    );
    assert!(
        stdout.contains("__definitely_not_installed_xyz_abc_123__"),
        "expected package name in output: {stdout}"
    );
}

/// `--scan-requirements` stdout lists requirements files when found.
#[test]
fn cli_scan_requirements_lists_requirements_files() {
    let dir = TempDir::new().unwrap();
    write_conf(dir.path(), "extensions = []\n");
    std::fs::write(dir.path().join("requirements.txt"), "sphinx\n").unwrap();
    let out = dir.path().join("_build");
    let status = Command::new(SPHINX_BUILD_RS)
        .args([
            dir.path().to_str().unwrap(),
            out.to_str().unwrap(),
            "--scan-requirements",
        ])
        .output()
        .expect("failed to run sphinx-build-rs");
    let stdout = String::from_utf8_lossy(&status.stdout);
    assert!(
        stdout.contains("requirements.txt"),
        "expected requirements.txt in output: {stdout}"
    );
}
