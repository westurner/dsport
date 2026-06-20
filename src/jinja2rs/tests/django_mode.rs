//! Integration tests for Django template compatibility mode.
//!
//! These tests verify that `Environment::with_django_mode` renders Django
//! templates identically to (or deliberately close to) the Python Django
//! template engine output.

use jinja2rs::{DjangoMode, Environment};
use serde_json::json;

// ── Filter smoke tests ────────────────────────────────────────────────────────

#[test]
fn django_upper_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ text|upper }}", json!({"text": "hello world"}))
            .unwrap(),
        "HELLO WORLD"
    );
}

#[test]
fn django_lower_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ text|lower }}", json!({"text": "HELLO"}))
            .unwrap(),
        "hello"
    );
}

#[test]
fn django_capfirst_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ text|capfirst }}", json!({"text": "hello world"}))
            .unwrap(),
        "Hello world"
    );
}

#[test]
fn django_title_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ text|title }}", json!({"text": "hello world"}))
            .unwrap(),
        "Hello World"
    );
}

#[test]
fn django_slugify_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ title|slugify }}", json!({"title": "Hello, World!"}))
            .unwrap(),
        "hello-world"
    );
}

#[test]
fn django_truncatewords_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    let result = env
        .render_str(
            "{{ text|truncatewords(2) }}",
            json!({"text": "one two three four five"}),
        )
        .unwrap();
    assert_eq!(result, "one two\u{2026}");
}

#[test]
fn django_truncatechars_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    let result = env
        .render_str(
            "{{ text|truncatechars(6) }}",
            json!({"text": "Hello, World!"}),
        )
        .unwrap();
    assert_eq!(result, "Hello\u{2026}");
}

#[test]
fn django_add_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ count|add(5) }}", json!({"count": 10}))
            .unwrap(),
        "15"
    );
}

#[test]
fn django_floatformat_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ pi|floatformat(2) }}", json!({"pi": 3.14159_f64}))
            .unwrap(),
        "3.14"
    );
}

#[test]
fn django_pluralize_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ n }} comment{{ n|pluralize }}", json!({"n": 1}),)
            .unwrap(),
        "1 comment"
    );
    assert_eq!(
        env.render_str("{{ n }} comment{{ n|pluralize }}", json!({"n": 5}),)
            .unwrap(),
        "5 comments"
    );
}

#[test]
fn django_first_last_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ items|first }}", json!({"items": [1, 2, 3]}))
            .unwrap(),
        "1"
    );
    assert_eq!(
        env.render_str("{{ items|last }}", json!({"items": [1, 2, 3]}))
            .unwrap(),
        "3"
    );
}

#[test]
fn django_join_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str(
            r#"{{ items|join(", ") }}"#,
            json!({"items": ["a", "b", "c"]}),
        )
        .unwrap(),
        "a, b, c"
    );
}

#[test]
fn django_length_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ items|length }}", json!({"items": [1, 2, 3, 4]}))
            .unwrap(),
        "4"
    );
}

#[test]
fn django_yesno_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ v|yesno }}", json!({"v": true})).unwrap(),
        "yes"
    );
    assert_eq!(
        env.render_str("{{ v|yesno }}", json!({"v": false}))
            .unwrap(),
        "no"
    );
}

#[test]
fn django_default_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str(r#"{{ name|default("Anonymous") }}"#, json!({"name": ""}))
            .unwrap(),
        "Anonymous"
    );
    assert_eq!(
        env.render_str(
            r#"{{ name|default("Anonymous") }}"#,
            json!({"name": "Alice"})
        )
        .unwrap(),
        "Alice"
    );
}

#[test]
fn django_striptags_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ html|striptags }}", json!({"html": "<b>bold</b>"}))
            .unwrap(),
        "bold"
    );
}

#[test]
fn django_urlencode_filter() {
    let env = Environment::with_django_mode(DjangoMode::default());
    assert_eq!(
        env.render_str("{{ s|urlencode }}", json!({"s": "hello world"}))
            .unwrap(),
        "hello%20world"
    );
}

// ── Control flow ──────────────────────────────────────────────────────────────

#[test]
fn django_for_loop() {
    let env = Environment::with_django_mode(DjangoMode::default());
    let result = env
        .render_str(
            "{% for item in items %}{{ item }}{% if not loop.last %}, {% endif %}{% endfor %}",
            json!({"items": ["a", "b", "c"]}),
        )
        .unwrap();
    assert_eq!(result, "a, b, c");
}

#[test]
fn django_if_elif_else() {
    let env = Environment::with_django_mode(DjangoMode::default());
    let tmpl = "{% if n > 10 %}big{% elif n > 5 %}medium{% else %}small{% endif %}";
    assert_eq!(env.render_str(tmpl, json!({"n": 15})).unwrap(), "big");
    assert_eq!(env.render_str(tmpl, json!({"n": 7})).unwrap(), "medium");
    assert_eq!(env.render_str(tmpl, json!({"n": 2})).unwrap(), "small");
}

// ── Template inheritance ──────────────────────────────────────────────────────

#[test]
fn django_template_inheritance() {
    let mut env = Environment::with_django_mode(DjangoMode::default());
    env.add_template(
        "base.html",
        "<html><head><title>{% block title %}Default{% endblock %}</title></head>\
         <body>{% block content %}{% endblock %}</body></html>",
    )
    .unwrap();
    env.add_template(
        "child.html",
        "{% extends \"base.html\" %}{% block title %}My Page{% endblock %}\
         {% block content %}<p>Hello!</p>{% endblock %}",
    )
    .unwrap();

    let result = env
        .get_template("child.html")
        .unwrap()
        .render(json!({}))
        .unwrap();
    assert!(result.contains("<title>My Page</title>"));
    assert!(result.contains("<p>Hello!</p>"));
}

// ── Auto-escaping ─────────────────────────────────────────────────────────────

#[test]
fn django_html_auto_escape() {
    let mut env = Environment::with_django_mode(DjangoMode::default());
    // .html template — auto-escape must be active
    env.add_template("page.html", "{{ content }}").unwrap();
    let result = env
        .get_template("page.html")
        .unwrap()
        .render(json!({"content": "<script>alert(1)</script>"}))
        .unwrap();
    // minijinja also escapes '/' as '&#x2f;' for XSS safety (small deviation from
    // Django which leaves '/' unescaped, but both are safe HTML)
    assert!(result.contains("&lt;script&gt;"), "< should be escaped");
    assert!(!result.contains("<script>"), "raw tag must not appear");
    assert!(result.contains("alert(1)"), "content should be present");
}

#[test]
fn django_safe_bypasses_escape() {
    let mut env = Environment::with_django_mode(DjangoMode::default());
    env.add_template("page.html", "{{ content|safe }}").unwrap();
    let result = env
        .get_template("page.html")
        .unwrap()
        .render(json!({"content": "<b>bold</b>"}))
        .unwrap();
    assert_eq!(result, "<b>bold</b>");
}

// ── DjangoMode builder ────────────────────────────────────────────────────────

#[test]
fn django_mode_builder_chaining() {
    let mode = DjangoMode::default()
        .with_app_directory("/app/accounts")
        .with_app_directory("/app/posts")
        .with_timezone("Europe/London")
        .with_locale("en-GB")
        .with_url_resolution(true);

    assert_eq!(mode.app_directories.len(), 2);
    assert_eq!(mode.timezone, "Europe/London");
    assert_eq!(mode.locale, "en-GB");
    assert!(mode.enable_url_resolution);
}

// ── App-directory loader ──────────────────────────────────────────────────────

#[test]
fn django_app_directory_loader_integration() {
    use std::fs;
    let tmp = tempfile::tempdir().unwrap();
    let tmpl_dir = tmp.path().join("templates");
    fs::create_dir_all(&tmpl_dir).unwrap();
    fs::write(
        tmpl_dir.join("greeting.html"),
        "Hello, {{ name|capfirst }}!",
    )
    .unwrap();

    let mode = DjangoMode::default().with_app_directory(tmp.path().to_path_buf());
    let env = Environment::with_django_mode(mode);

    let result = env
        .get_template("greeting.html")
        .unwrap()
        .render(json!({"name": "django"}))
        .unwrap();
    assert_eq!(result, "Hello, Django!");
}

// ── CompatMode::Django ────────────────────────────────────────────────────────

#[test]
fn compat_mode_django_via_set_compat_mode() {
    use jinja2rs::compat::{CompatMode, DjangoMode};
    let mut env = Environment::new();
    env.set_compat_mode(CompatMode::Django(DjangoMode::default()));
    // Filters should now be registered.
    let result = env
        .render_str("{{ text|slugify }}", json!({"text": "Hello World!"}))
        .unwrap();
    assert_eq!(result, "hello-world");
}
