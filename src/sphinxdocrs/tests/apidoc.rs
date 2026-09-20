//! `sphinx-apidoc-rs` integration tests — rstest + cargo-insta.
//!
//! Covers: parser flags, `is_initpy`, `module_join`, `is_excluded`,
//! `recurse_tree` on a synthetic Python package tree, and file-content
//! snapshots.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rstest::*;
use tempfile::TempDir;

use sphinxdocrs::apidoc::generate::{
    create_module_file, create_modules_toc_file, is_excluded, is_initpy, module_join, recurse_tree,
};
use sphinxdocrs::apidoc::parser::parse_args;
use sphinxdocrs::apidoc::settings::{ApidocOptions, SourceMode};
use sphinxdocrs::apidoc::templates::ApidocTemplates;

// ── helpers ───────────────────────────────────────────────────────────────────

fn args(s: &str) -> Vec<String> {
    s.split_whitespace().map(String::from).collect()
}

/// Build a minimal Python package tree in `root`:
/// ```text
/// mypkg/
///   __init__.py
///   utils.py
///   _private.py
///   sub/
///     __init__.py
///     helper.py
/// ```
fn make_pkg(root: &Path) {
    let pkg = root.join("mypkg");
    std::fs::create_dir_all(pkg.join("sub")).unwrap();
    for f in &[
        "mypkg/__init__.py",
        "mypkg/utils.py",
        "mypkg/_private.py",
        "mypkg/sub/__init__.py",
        "mypkg/sub/helper.py",
    ] {
        std::fs::write(root.join(f), b"").unwrap();
    }
}

#[fixture]
#[once]
fn templates() -> ApidocTemplates {
    ApidocTemplates::vendored()
}

// ── pure helper unit tests ────────────────────────────────────────────────────

#[rstest]
#[case("__init__.py", true)]
#[case("__init__.pyx", true)]
#[case("utils.py", false)]
#[case("__init___.py", false)]
fn test_is_initpy(#[case] name: &str, #[case] expected: bool) {
    assert_eq!(is_initpy(Path::new(name)), expected, "file={name}");
}

#[rstest]
#[case(&["", "mypkg", "utils"], "mypkg.utils")]
#[case(&["mypkg", ""],          "mypkg")]
#[case(&["a", "b", "c"],        "a.b.c")]
#[case(&["", ""],               "")]
fn test_module_join(#[case] parts: &[&str], #[case] expected: &str) {
    assert_eq!(module_join(parts), expected);
}

#[test]
fn test_is_excluded_matches() {
    let re = regex::Regex::new(".*test.*").unwrap();
    assert!(is_excluded(
        Path::new("/src/tests/foo.py"),
        std::slice::from_ref(&re)
    ));
    assert!(!is_excluded(Path::new("/src/mymod.py"), &[re]));
}

// ── parser ────────────────────────────────────────────────────────────────────

#[test]
fn parser_required_args() {
    let opts = parse_args(&args("-o /out /src/mypkg")).unwrap();
    assert_eq!(opts.module_path, PathBuf::from("/src/mypkg"));
    assert_eq!(opts.dest_dir, PathBuf::from("/out"));
    assert_eq!(opts.header, "mypkg");
    assert_eq!(opts.suffix, "rst");
    assert_eq!(opts.max_depth, 4);
}

#[rstest]
#[case("-e", true, false, false)]
#[case("-P", false, true, false)]
#[case("-M", false, false, true)]
fn parser_bool_flags(
    #[case] flag: &str,
    #[case] sep: bool,
    #[case] priv_: bool,
    #[case] modfirst: bool,
) {
    let opts = parse_args(&args(&format!("-o /out /src {flag}"))).unwrap();
    assert_eq!(opts.separate_modules, sep);
    assert_eq!(opts.include_private, priv_);
    assert_eq!(opts.module_first, modfirst);
}

#[test]
fn parser_no_toc_clears_toc_file() {
    let opts = parse_args(&args("-o /out /src -T")).unwrap();
    assert!(opts.toc_file.is_empty());
}

#[test]
fn parser_suffix_strips_dot() {
    let opts = parse_args(&args("-o /out /src -s .rst")).unwrap();
    assert_eq!(opts.suffix, "rst");
}

#[test]
fn parser_exclude_patterns() {
    let opts = parse_args(&args("-o /out /src tests setup.py")).unwrap();
    assert_eq!(opts.exclude_pattern, ["tests", "setup.py"]);
}

#[test]
fn parser_automodule_options_csv() {
    let opts = parse_args(&args(
        "-o /out /src --automodule-options members,undoc-members",
    ))
    .unwrap();
    let expected: BTreeSet<_> = ["members", "undoc-members"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(opts.automodule_options, expected);
}

#[test]
fn parser_ext_flag() {
    let opts = parse_args(&args("-o /out /src -F --ext-autodoc")).unwrap();
    assert!(opts.extensions.contains(&"sphinx.ext.autodoc".to_owned()));
}

// ── recurse_tree on real tmp directory ───────────────────────────────────────

#[rstest]
fn recurse_tree_basic_package(templates: &ApidocTemplates) {
    let src_tmp = TempDir::new().unwrap();
    let out_tmp = TempDir::new().unwrap();
    make_pkg(src_tmp.path());

    let opts = ApidocOptions {
        module_path: src_tmp.path().join("mypkg"),
        dest_dir: out_tmp.path().to_path_buf(),
        quiet: true,
        force: true,
        ..ApidocOptions::new(src_tmp.path().join("mypkg"), out_tmp.path().to_path_buf())
    };

    let (written, modules) = recurse_tree(&opts.module_path, &[], &opts, templates).unwrap();
    // Should have at least one .rst file written
    assert!(!written.is_empty(), "no files written");
    // mypkg should be in top-level modules
    assert!(
        modules.iter().any(|m| m == "mypkg"),
        "mypkg missing from modules: {modules:?}"
    );
    // All written files should exist
    for p in &written {
        assert!(p.exists(), "written file missing: {}", p.display());
    }
}

#[rstest]
fn recurse_tree_excludes_private(templates: &ApidocTemplates) {
    let src_tmp = TempDir::new().unwrap();
    let out_tmp = TempDir::new().unwrap();
    make_pkg(src_tmp.path());

    let opts = ApidocOptions {
        module_path: src_tmp.path().join("mypkg"),
        dest_dir: out_tmp.path().to_path_buf(),
        quiet: true,
        force: true,
        include_private: false,
        ..ApidocOptions::new(src_tmp.path().join("mypkg"), out_tmp.path().to_path_buf())
    };

    let (written, _) = recurse_tree(&opts.module_path, &[], &opts, templates).unwrap();
    // _private.py should not produce a written file
    let has_private = written
        .iter()
        .any(|p| p.to_string_lossy().contains("_private"));
    assert!(
        !has_private,
        "private module should be excluded: {written:?}"
    );
}

#[rstest]
fn recurse_tree_include_private(templates: &ApidocTemplates) {
    let src_tmp = TempDir::new().unwrap();
    let out_tmp = TempDir::new().unwrap();
    make_pkg(src_tmp.path());

    let opts = ApidocOptions {
        module_path: src_tmp.path().join("mypkg"),
        dest_dir: out_tmp.path().to_path_buf(),
        quiet: true,
        force: true,
        include_private: true,
        separate_modules: true, // needed to get a separate file per module
        ..ApidocOptions::new(src_tmp.path().join("mypkg"), out_tmp.path().to_path_buf())
    };

    let (written, _) = recurse_tree(&opts.module_path, &[], &opts, templates).unwrap();
    let has_private = written
        .iter()
        .any(|p| p.to_string_lossy().contains("_private"));
    assert!(
        has_private,
        "private module should be included when -P + -e: {written:?}"
    );
}

#[rstest]
fn source_fixtures_generate_native_entry_points(templates: &ApidocTemplates) {
    let output = TempDir::new().unwrap();
    let rust_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/h14/rust");
    let rust_output = output.path().join("rust");
    std::fs::create_dir_all(&rust_output).unwrap();
    let mut rust_options = ApidocOptions::new(rust_root.clone(), rust_output.clone());
    rust_options.source_mode = SourceMode::Rust;
    rust_options.cargo_manifest = Some(rust_root.join("Cargo.toml"));
    rust_options.cargo_package = Some("h14_fixture".to_owned());
    rust_options.force = true;
    rust_options.quiet = true;

    let (rust_files, rust_modules) =
        recurse_tree(&rust_root, &[], &rust_options, templates).unwrap();
    assert_eq!(rust_modules, ["h14_fixture", "h14_fixture.api"]);
    assert_eq!(rust_files.len(), 2);
    let rust_entry = std::fs::read_to_string(rust_output.join("h14_fixture.api.rst")).unwrap();
    assert!(rust_entry.contains("source-mode: rust"));
    assert!(rust_entry.contains(".. rust:module:: h14_fixture::api"));
    assert!(rust_entry.contains(":source:"));

    let lean_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/h14/lean");
    let lean_output = output.path().join("lean");
    std::fs::create_dir_all(&lean_output).unwrap();
    let mut lean_options = ApidocOptions::new(lean_root.clone(), lean_output.clone());
    lean_options.source_mode = SourceMode::Lean;
    lean_options.force = true;
    lean_options.quiet = true;

    let (lean_files, lean_modules) =
        recurse_tree(&lean_root, &[], &lean_options, templates).unwrap();
    assert_eq!(lean_modules, ["Demo", "Malformed"]);
    assert_eq!(lean_files.len(), 2);
    let lean_entry = std::fs::read_to_string(lean_output.join("Demo.rst")).unwrap();
    assert!(lean_entry.contains("source-mode: lean"));
    assert!(lean_entry.contains(".. lean:module:: Demo"));
    assert!(lean_entry.contains(":source:"));
}

#[test]
fn auto_source_mode_rejects_mixed_rust_and_lean_roots() {
    let root = TempDir::new().unwrap();
    let output = TempDir::new().unwrap();
    std::fs::write(root.path().join("lib.rs"), "pub fn answer() {}\n").unwrap();
    std::fs::write(root.path().join("Demo.lean"), "def answer := 1\n").unwrap();

    let options = ApidocOptions {
        source_mode: SourceMode::Auto,
        force: true,
        quiet: true,
        ..ApidocOptions::new(root.path().to_path_buf(), output.path().to_path_buf())
    };

    let template_set = templates();
    let error = recurse_tree(root.path(), &[], &options, &template_set).unwrap_err();
    assert!(error.to_string().contains("mixed Rust and Lean"));
}

// ── module/package file content snapshots ────────────────────────────────────

#[rstest]
fn module_file_snapshot(templates: &ApidocTemplates) {
    let tmp = TempDir::new().unwrap();
    let opts = ApidocOptions {
        dest_dir: tmp.path().to_path_buf(),
        quiet: true,
        force: true,
        ..ApidocOptions::new(PathBuf::from("/src"), tmp.path().to_path_buf())
    };
    let _path = create_module_file(Some("mypkg"), "utils", &opts, templates).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("mypkg.utils.rst")).unwrap();
    insta::assert_snapshot!(content);
}

#[rstest]
fn toc_file_snapshot(templates: &ApidocTemplates) {
    let tmp = TempDir::new().unwrap();
    let opts = ApidocOptions {
        dest_dir: tmp.path().to_path_buf(),
        quiet: true,
        force: true,
        header: "My Package".to_owned(),
        max_depth: 2,
        ..ApidocOptions::new(PathBuf::from("/src"), tmp.path().to_path_buf())
    };
    let modules = vec![
        "mypkg".to_owned(),
        "mypkg.sub".to_owned(),
        "other".to_owned(),
    ];
    let _path = create_modules_toc_file(&modules, &opts, "modules", templates).unwrap();
    let content = std::fs::read_to_string(tmp.path().join("modules.rst")).unwrap();
    insta::assert_snapshot!(content);
}

// ── help snapshot ─────────────────────────────────────────────────────────────

#[test]
fn apidoc_help_snapshot() {
    let mut cmd = sphinxdocrs::apidoc::parser::build_parser();
    insta::assert_snapshot!(cmd.render_help().to_string());
}
