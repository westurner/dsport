#![allow(clippy::needless_borrows_for_generic_args)]


//! Integration snapshot tests for jinja2rs.
//!
//! Each test renders a template and asserts the output against an insta
//! snapshot, providing byte-parity documentation vs. Python Jinja2.
//!
//! Snapshot files live in `tests/snapshots/`.
//! Accept new snapshots with:  `cargo insta review -p jinja2rs`

use insta::assert_snapshot;
use jinja2rs::Environment;
use rstest::rstest;
use serde_json::json;

// ---------------------------------------------------------------------------
// Basic rendering parity
// ---------------------------------------------------------------------------

#[test]
fn test_variable_interpolation() {
    let env = Environment::new();
    let out = env
        .render_str("Hello, {{ name }}!", json!({"name": "Sphinx"}))
        .unwrap();
    assert_snapshot!(out, @"Hello, Sphinx!");
}

#[test]
fn test_for_loop() {
    let env = Environment::new();
    let out = env
        .render_str(
            "{% for item in items %}{{ item }}{% if not loop.last %},{% endif %}{% endfor %}",
            json!({"items": ["a", "b", "c"]}),
        )
        .unwrap();
    assert_snapshot!(out, @"a,b,c");
}

#[test]
fn test_if_else() {
    let env = Environment::new();
    let src = "{% if show %}visible{% else %}hidden{% endif %}";
    let out_true = env.render_str(src, json!({"show": true})).unwrap();
    let out_false = env.render_str(src, json!({"show": false})).unwrap();
    assert_snapshot!(out_true, @"visible");
    assert_snapshot!(out_false, @"hidden");
}

#[test]
fn test_block_inheritance() {
    let mut env = Environment::new();
    env.add_template(
        "base.html",
        "<!DOCTYPE html><html><head>{% block head %}{% endblock %}</head><body>{% block body %}{% endblock %}</body></html>",
    ).unwrap();
    env.add_template(
        "child.html",
        r#"{% extends "base.html" %}{% block head %}<title>{{ title }}</title>{% endblock %}{% block body %}<h1>{{ title }}</h1>{% endblock %}"#,
    ).unwrap();
    let tmpl = env.get_template("child.html").unwrap();
    let out = tmpl.render(json!({"title": "Hello"})).unwrap();
    assert_snapshot!(out, @"<!DOCTYPE html><html><head><title>Hello</title></head><body><h1>Hello</h1></body></html>");
}

// ---------------------------------------------------------------------------
// Filters
// ---------------------------------------------------------------------------

#[rstest]
#[case("true", "true")]
#[case("false", "false")]
#[case("1", "true")]
#[case("yes", "true")]
#[case("no", "false")]
fn test_tobool_filter(#[case] input: &str, #[case] expected: &str) {
    let env = Environment::new();
    let out = env
        .render_str("{{ val|tobool }}", json!({"val": input}))
        .unwrap();
    assert_eq!(out, expected);
}

#[rstest]
#[case("42", "42")]
#[case("-7", "-7")]
#[case("nope", "0")]
fn test_toint_filter(#[case] input: &str, #[case] expected: &str) {
    let env = Environment::new();
    let out = env
        .render_str("{{ val|toint }}", json!({"val": input}))
        .unwrap();
    assert_eq!(out, expected);
}

#[rstest]
#[case(0i64, "0")]
#[case(320i64, "320px")]
fn test_todim_filter_int(#[case] input: i64, #[case] expected: &str) {
    let env = Environment::new();
    let out = env
        .render_str("{{ val|todim }}", json!({"val": input}))
        .unwrap();
    assert_eq!(out, expected);
}

#[test]
fn test_todim_filter_undefined() {
    let env = Environment::new();
    let out = env.render_str("{{ none|todim }}", json!({})).unwrap();
    assert_snapshot!(out, @"initial");
}

// ---------------------------------------------------------------------------
// Auto-escape
// ---------------------------------------------------------------------------

#[test]
fn test_html_autoescaping() {
    let mut env = Environment::new();
    env.add_template("escaped.html", "{{ content }}").unwrap();
    let tmpl = env.get_template("escaped.html").unwrap();
    let out = tmpl
        .render(json!({"content": "<script>alert(1)</script>"}))
        .unwrap();
    // minijinja auto-escapes .html templates by default
    assert_snapshot!(out, @"&lt;script&gt;alert(1)&lt;&#x2f;script&gt;");
}

// ---------------------------------------------------------------------------
// Sandbox
// ---------------------------------------------------------------------------

#[test]
fn test_sandbox_undefined_strict() {
    let env = jinja2rs::SandboxedEnvironment::new();
    let result = env.render_str("{{ undefined_var }}", json!({}));
    assert!(result.is_err(), "strict mode should error on undefined");
}

// ---------------------------------------------------------------------------
// IdGen global
// ---------------------------------------------------------------------------

#[test]
fn test_idgen_in_template() {
    let mut env = Environment::new();
    env.add_template(
        "ids.html",
        "{{ idgen.next() }}-{{ idgen.next() }}-{{ idgen.current() }}",
    )
    .unwrap();
    let tmpl = env.get_template("ids.html").unwrap();
    let out = tmpl.render(json!({})).unwrap();
    assert_snapshot!(out, @"1-2-2");
}

// ---------------------------------------------------------------------------
// Sphinx glue integration
// ---------------------------------------------------------------------------

#[test]
fn test_sphinx_glue_render_from_dir() {
    use jinja2rs::sphinx_glue::BuiltinTemplateLoader;
    use std::fs;

    let dir = tempfile::tempdir().expect("tempdir");
    fs::write(dir.path().join("page.html"), "Doc: {{ docname }}").unwrap();

    let loader = BuiltinTemplateLoader::new(vec![dir.path().to_path_buf()], vec![]);
    let out = loader
        .render("page.html", json!({"docname": "index"}))
        .unwrap();
    assert_snapshot!(out, @"Doc: index");
}
