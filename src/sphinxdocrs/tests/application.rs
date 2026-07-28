//! Integration tests for `sphinxdocrs::application::SphinxApp`.
//!
//! Mirrors the pure-Rust-testable subset of
//! `sphinx/tests/test_application.py`.

use std::collections::HashMap;

use sphinxdocrs::application::{
    AppError, NATIVE_BUILDER_CLASSES, NATIVE_BUILDERS, SphinxApp, is_native_builder,
};

// ── helper ────────────────────────────────────────────────────────────────────

fn make_src_with_docs(docs: &[(&str, &str)]) -> tempfile::TempDir {
    let tmp = tempfile::TempDir::new().unwrap();
    for (name, content) in docs {
        // create subdirs if needed
        let path: std::path::PathBuf = name
            .split('/')
            .collect::<std::path::PathBuf>()
            .with_extension("rst");
        if let Some(parent) = tmp.path().join(&path).parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(tmp.path().join(path), content).unwrap();
    }
    tmp
}

// ── NATIVE_BUILDERS ───────────────────────────────────────────────────────────

#[test]
fn native_builders_contains_html() {
    assert!(NATIVE_BUILDERS.contains(&"html"));
}

#[test]
fn is_native_builder_html_true() {
    assert!(is_native_builder("html"));
}

#[test]
fn is_native_builder_latex_false() {
    // latex is now native
    assert!(is_native_builder("latex"));
}

#[test]
fn is_native_builder_epub_false() {
    assert!(!is_native_builder("epub"));
}

#[test]
fn is_native_builder_unknown_false() {
    // "xml" became native in H7a; use a builder name that stays non-native.
    assert!(!is_native_builder("qthelp"));
}

#[test]
fn is_native_builder_json_true() {
    assert!(is_native_builder("json"));
}

#[test]
fn is_native_builder_h7a_builders_true() {
    assert!(is_native_builder("text"));
    assert!(is_native_builder("xml"));
    assert!(is_native_builder("pseudoxml"));
}

#[test]
fn is_native_builder_h7b_builders_true() {
    assert!(is_native_builder("dirhtml"));
    assert!(is_native_builder("singlehtml"));
}

/// Every entry of `NATIVE_BUILDER_CLASSES` must be reachable through
/// `NATIVE_BUILDERS`, and vice versa.
#[test]
fn native_builder_classes_match_native_builders() {
    let from_classes: Vec<&str> = NATIVE_BUILDER_CLASSES.iter().map(|(n, _)| *n).collect();
    assert_eq!(from_classes, NATIVE_BUILDERS.to_vec());
}

/// `SphinxApp::new` registers every native builder under its own type,
/// not all under `HtmlBuilder`.
#[test]
fn new_registers_each_builder_under_its_own_class() {
    let src = make_src_with_docs(&[("index", "Title\n=====\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let doctrees = tempfile::TempDir::new().unwrap();
    let app = SphinxApp::new(
        src.path(),
        out.path().join("build"),
        doctrees.path(),
        "html",
        HashMap::new(),
    )
    .unwrap();

    for (name, class) in NATIVE_BUILDER_CLASSES {
        assert_eq!(app.registry.get_builder(name), Some(*class), "{name}");
    }
}

/// `-b json` runs natively and emits `.fjson` pages plus `globalcontext.json`.
#[test]
fn build_json_writes_fjson_pages() {
    let src = make_src_with_docs(&[("index", "Title\n=====\n\nHello.\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let doctrees = tempfile::TempDir::new().unwrap();
    let outdir = out.path().join("build");

    let mut app =
        SphinxApp::new(src.path(), &outdir, doctrees.path(), "json", HashMap::new()).unwrap();
    let result = app.build().unwrap();

    assert_eq!(result.written, 1);
    let page = outdir.join("index.fjson");
    assert!(page.exists(), "index.fjson not written");
    let ctx: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&page).unwrap()).unwrap();
    assert_eq!(ctx["current_page_name"], "index");
    assert_eq!(ctx["sourcename"], "index.rst");
    assert!(ctx["body"].as_str().unwrap().contains("Hello."));
    assert!(outdir.join("globalcontext.json").exists());
}

// ── SphinxApp::new — path validation ─────────────────────────────────────────

/// Mirrors test that `Sphinx()` raises `ApplicationError` for missing srcdir.
#[test]
fn new_missing_srcdir_raises_app_error() {
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let err = SphinxApp::new(
        "/no/such/path/xyz_9273",
        out.path(),
        dt.path(),
        "html",
        HashMap::new(),
    )
    .unwrap_err();
    assert!(
        matches!(err, AppError::InvalidPath(_)),
        "expected InvalidPath: {err}"
    );
    assert!(err.to_string().contains("Cannot find source directory"));
}

/// Mirrors test that `Sphinx()` raises `ApplicationError` when src == out.
#[test]
fn new_identical_src_and_out_raises_app_error() {
    let tmp = tempfile::TempDir::new().unwrap();
    let err = SphinxApp::new(
        tmp.path(),
        tmp.path(),
        tmp.path().join("dt"),
        "html",
        HashMap::new(),
    )
    .unwrap_err();
    assert!(matches!(err, AppError::InvalidPath(_)));
    assert!(err.to_string().contains("cannot be identical"));
}

/// `SphinxApp::new` creates `outdir` when it doesn't exist.
#[test]
fn new_creates_missing_outdir() {
    let src = make_src_with_docs(&[("index", "Home\n====\n")]);
    let base = tempfile::TempDir::new().unwrap();
    let out = base.path().join("_build");
    let dt = base.path().join(".doctrees");
    assert!(!out.exists());
    SphinxApp::new(src.path(), &out, &dt, "html", HashMap::new()).unwrap();
    assert!(out.exists(), "outdir should be created");
}

// ── SphinxApp constructor ─────────────────────────────────────────────────────

#[test]
fn new_stores_buildername() {
    let src = make_src_with_docs(&[("index", "T\n=\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let app = SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    assert_eq!(app.buildername, "html");
}

#[test]
fn new_stores_resolved_paths() {
    let src = make_src_with_docs(&[("index", "T\n=\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let app = SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    assert!(app.srcdir.is_absolute());
    assert!(app.outdir.is_absolute());
    assert!(app.doctreedir.is_absolute());
}

#[test]
fn new_supports_native_html() {
    let src = make_src_with_docs(&[("index", "T\n=\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let app = SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    assert!(app.supports_native());
}

// ── SphinxApp::build ──────────────────────────────────────────────────────────

/// Mirrors `test_application.py`: build() writes output files.
#[test]
fn build_html_writes_index_html() {
    let src = make_src_with_docs(&[("index", "Welcome\n=======\n\nHomepage content.\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    let result = app.build().unwrap();
    assert_eq!(result.written, 1);
    assert!(out.path().join("index.html").exists());
}

#[test]
fn build_html_content_is_valid_html5() {
    let src = make_src_with_docs(&[("index", "Title\n=====\n\nParagraph.\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app.build().unwrap();
    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.contains("<html"));
    assert!(html.contains("</html>"));
}

#[test]
fn build_html_renders_rst_content() {
    let src = make_src_with_docs(&[(
        "index",
        "Welcome to Sphinx\n=================\n\nThis is **important**.\n",
    )]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app.build().unwrap();
    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(html.contains("Welcome to Sphinx") || html.contains("important"));
}

#[test]
fn build_html_multi_doc_project() {
    let src = make_src_with_docs(&[
        ("index", "Home\n====\n\nWelcome.\n"),
        ("about", "About\n=====\n\nInfo.\n"),
        ("guide/intro", "Intro\n=====\n\nContent.\n"),
    ]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    let result = app.build().unwrap();
    assert_eq!(result.written, 3);
    assert!(out.path().join("index.html").exists());
    assert!(out.path().join("about.html").exists());
    assert!(out.path().join("guide").join("intro.html").exists());
}

/// `build()` returns `AppError::UnknownBuilder` for non-native builders.
#[test]
fn build_unknown_builder_returns_error() {
    let src = make_src_with_docs(&[("index", "T\n=\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let mut app =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app.buildername = "epub".into();
    let err = app.build().unwrap_err();
    assert!(matches!(err, AppError::UnknownBuilder(_)));
    assert!(err.to_string().contains("epub"));
}

// ── config wiring ─────────────────────────────────────────────────────────────

/// The app config defaults to `language = "en"`.
#[test]
fn app_config_default_language() {
    let src = make_src_with_docs(&[("index", "T\n=\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let app = SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    assert_eq!(app.config.language(), "en");
}

/// Command-line overrides are passed through to the config.
#[test]
fn app_config_override_project() {
    let src = make_src_with_docs(&[("index", "T\n=\n")]);
    let out = tempfile::TempDir::new().unwrap();
    let dt = tempfile::TempDir::new().unwrap();
    let mut overrides = HashMap::new();
    overrides.insert("project".into(), "MyProject".into());
    let app = SphinxApp::new(src.path(), out.path(), dt.path(), "html", overrides).unwrap();
    assert_eq!(app.config.project(), "MyProject");
}
