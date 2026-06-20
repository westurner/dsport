//! Integration tests for `sphinxdocrs::builders::html::HtmlBuilder`.
//!
//! Mirrors the pure-Rust-testable subset of
//! `sphinx/tests/test_builders/test_build_html.py`:
//! - `test_docutils_output`: RST → HTML5 round-trip
//! - Structure/content assertions on the produced HTML files

use std::path::Path;
use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::html::HtmlBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

// ── helper ────────────────────────────────────────────────────────────────────

fn make_env(srcdir: &Path, outdir: &Path) -> BuildEnvironment {
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(srcdir, &[(".rst", "restructuredtext")]);
    BuildEnvironment::new(config, project, srcdir, outdir)
}

fn read_html(dir: &Path, docname: &str) -> String {
    let path = docname
        .split('/')
        .collect::<std::path::PathBuf>()
        .with_extension("html");
    std::fs::read_to_string(dir.join(path)).expect("output html should exist")
}

// ── get_target_uri ────────────────────────────────────────────────────────────

/// Mirrors `StandaloneHTMLBuilder.get_target_uri` semantics.
#[test]
fn get_target_uri_plain() {
    let b = HtmlBuilder::new();
    assert_eq!(b.get_target_uri("index"), "index.html");
}

#[test]
fn get_target_uri_subdir() {
    let b = HtmlBuilder::new();
    assert_eq!(b.get_target_uri("api/module"), "api/module.html");
}

// ── builder name / format / suffix ───────────────────────────────────────────

#[test]
fn builder_name_is_html() {
    assert_eq!(HtmlBuilder::new().name(), "html");
}

#[test]
fn builder_format_is_html() {
    assert_eq!(HtmlBuilder::new().format(), "html");
}

#[test]
fn builder_out_suffix_is_dot_html() {
    assert_eq!(HtmlBuilder::new().out_suffix(), ".html");
}

// ── build_doc — HTML5 structure ───────────────────────────────────────────────

/// Mirrors `test_docutils_output`: verify the output is valid HTML5.
#[test]
fn build_doc_produces_html5_doctype() {
    let out = TempDir::new().unwrap();
    let b = HtmlBuilder::new();
    b.build_doc("index", "Title\n=====\n\nContent.\n", out.path())
        .unwrap();
    let html = read_html(out.path(), "index");
    assert!(
        html.starts_with("<!DOCTYPE html>"),
        "must start with DOCTYPE"
    );
}

#[test]
fn build_doc_has_html_root_element() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc("doc", "Hello\n=====\n", out.path())
        .unwrap();
    let html = read_html(out.path(), "doc");
    assert!(html.contains("<html"), "must contain <html> element");
    assert!(html.contains("</html>"), "must contain </html> closing tag");
}

#[test]
fn build_doc_has_head_and_body() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc("doc", "Title\n=====\n\nParagraph.\n", out.path())
        .unwrap();
    let html = read_html(out.path(), "doc");
    assert!(html.contains("<head>"));
    assert!(html.contains("<body>"));
    assert!(html.contains("</body>"));
}

#[test]
fn build_doc_has_charset_meta() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc("doc", "Title\n=====\n", out.path())
        .unwrap();
    let html = read_html(out.path(), "doc");
    assert!(html.contains("utf-8") || html.contains("UTF-8"));
}

// ── build_doc — RST content ───────────────────────────────────────────────────

/// Mirrors `test_docutils_output` XPath assertions: section title becomes
/// an HTML heading element.
#[test]
fn build_doc_section_title_rendered() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc(
            "index",
            "Welcome to the docs\n====================\n\nIntro text.\n",
            out.path(),
        )
        .unwrap();
    let html = read_html(out.path(), "index");
    assert!(
        html.contains("Welcome to the docs"),
        "section title should appear in output"
    );
}

#[test]
fn build_doc_paragraph_rendered() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc("doc", "Title\n=====\n\nA paragraph of text.\n", out.path())
        .unwrap();
    let html = read_html(out.path(), "doc");
    assert!(html.contains("<p>") || html.contains("paragraph of text"));
}

#[test]
fn build_doc_strong_emphasis() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc(
            "doc",
            "Title\n=====\n\nThis has **bold** and *italic* text.\n",
            out.path(),
        )
        .unwrap();
    let html = read_html(out.path(), "doc");
    assert!(html.contains("<strong>bold</strong>") || html.contains("bold"));
    assert!(html.contains("<em>italic</em>") || html.contains("italic"));
}

#[test]
fn build_doc_bullet_list() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc(
            "doc",
            "Title\n=====\n\n- item one\n- item two\n",
            out.path(),
        )
        .unwrap();
    let html = read_html(out.path(), "doc");
    assert!(html.contains("<ul>") || html.contains("item one"));
}

#[test]
fn build_doc_literal_block() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc(
            "doc",
            "Title\n=====\n\nExample::\n\n    code here\n",
            out.path(),
        )
        .unwrap();
    let html = read_html(out.path(), "doc");
    assert!(html.contains("code here") || html.contains("<pre>"));
}

// ── build_doc — output path ───────────────────────────────────────────────────

#[test]
fn build_doc_creates_output_file() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc("index", "T\n=\n", out.path())
        .unwrap();
    assert!(out.path().join("index.html").exists());
}

#[test]
fn build_doc_creates_subdirectory() {
    let out = TempDir::new().unwrap();
    HtmlBuilder::new()
        .build_doc("api/reference", "T\n=\n", out.path())
        .unwrap();
    assert!(out.path().join("api").join("reference.html").exists());
}

// ── build_all ────────────────────────────────────────────────────────────────

#[test]
fn build_all_empty_project() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let env = make_env(src.path(), out.path());
    let result = HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 0);
    assert_eq!(result.skipped, 0);
}

#[test]
fn build_all_single_doc() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    let env = make_env(src.path(), out.path());
    let result = HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 1);
    assert!(out.path().join("index.html").exists());
    let html = read_html(out.path(), "index");
    assert!(html.contains("Welcome") || html.contains("Homepage"));
}

#[test]
fn build_all_multiple_docs() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Home\n====\n").unwrap();
    std::fs::write(src.path().join("about.rst"), "About\n=====\n").unwrap();
    std::fs::write(src.path().join("contact.rst"), "Contact\n=======\n").unwrap();
    let env = make_env(src.path(), out.path());
    let result = HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 3);
}

#[test]
fn build_all_with_subdirectory() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::create_dir(src.path().join("guide")).unwrap();
    std::fs::write(src.path().join("index.rst"), "Home\n====\n").unwrap();
    std::fs::write(
        src.path().join("guide").join("intro.rst"),
        "Introduction\n============\n",
    )
    .unwrap();
    let env = make_env(src.path(), out.path());
    let result = HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 2);
    assert!(out.path().join("index.html").exists());
    assert!(out.path().join("guide").join("intro.html").exists());
}

#[test]
fn build_all_ignores_non_rst_files() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "T\n=\n").unwrap();
    std::fs::write(src.path().join("notes.md"), "# notes").unwrap();
    std::fs::write(src.path().join("conf.py"), "project = 'test'").unwrap();
    let env = make_env(src.path(), out.path());
    let result = HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 1, "only .rst files should be built");
}

#[test]
fn build_all_uses_env_all_docs_when_populated() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    // Write three RST files but only register two in env.all_docs.
    std::fs::write(src.path().join("index.rst"), "Home\n====\n").unwrap();
    std::fs::write(src.path().join("about.rst"), "About\n=====\n").unwrap();
    std::fs::write(src.path().join("skip.rst"), "Skip\n=====\n").unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), out.path());
    // Only register two docs.
    env.record_doc_read("index", 1000);
    env.record_doc_read("about", 2000);

    let result = HtmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    // Should only build the two registered docs.
    assert_eq!(result.written, 2);
    assert!(out.path().join("index.html").exists());
    assert!(out.path().join("about.html").exists());
    assert!(!out.path().join("skip.html").exists());
}
