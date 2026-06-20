//! Parametrized i18n (Phase 6) tests with rstest.
//!
//! Tests for gettext/ngettext translation globals.

use jinja2rs::Environment;
use jinja2rs::i18n::I18nProvider;
use rstest::{fixture, rstest};
use serde_json::json;
use std::collections::HashMap;

// ============================================================================
// FIXTURES
// ============================================================================

/// Fixture: environment with i18n support installed.
#[fixture]
fn env_with_i18n() -> Environment {
    let provider = I18nProvider::new();
    let mut env = Environment::new();
    env.install_gettext(provider);
    env
}

/// Fixture: environment with i18n and translations loaded.
#[fixture]
fn env_with_translations() -> Environment {
    let provider = I18nProvider::new();

    // Load some test translations
    let mut dict = HashMap::new();
    dict.insert("Hello".to_string(), "Hola".to_string());
    dict.insert("Good morning".to_string(), "Buenos días".to_string());
    dict.insert("Thank you".to_string(), "Gracias".to_string());
    provider.load_translations(dict);

    // Load plural forms
    let mut plurals = HashMap::new();
    plurals.insert(
        "file".to_string(),
        vec!["archivo".to_string(), "archivos".to_string()],
    );
    plurals.insert(
        "item".to_string(),
        vec!["elemento".to_string(), "elementos".to_string()],
    );
    provider.load_plural_forms(plurals);

    let mut env = Environment::new();
    env.install_gettext(provider);
    env
}

// ============================================================================
// GETTEXT BASIC TESTS
// ============================================================================

/// Test gettext with untranslated message (passthrough).
#[rstest]
#[case("Hello")]
#[case("World")]
#[case("Unknown message")]
fn test_gettext_untranslated(env_with_i18n: Environment, #[case] message: &str) {
    let template = "{{ gettext(msg) }}";
    let out = env_with_i18n
        .render_str(template, json!({"msg": message}))
        .expect("gettext should render");

    assert_eq!(out, message);
}

/// Test gettext with translated messages.
#[rstest]
#[case("Hello", "Hola")]
#[case("Good morning", "Buenos días")]
#[case("Thank you", "Gracias")]
fn test_gettext_translated(
    env_with_translations: Environment,
    #[case] msg: &str,
    #[case] expected: &str,
) {
    let template = "{{ gettext(msg) }}";
    let out = env_with_translations
        .render_str(template, json!({"msg": msg}))
        .expect("gettext should render");

    assert_eq!(out, expected);
}

/// Test gettext in templates with context.
#[rstest]
fn test_gettext_in_template(env_with_translations: Environment) {
    let template = r#"
<h1>{{ gettext("Hello") }}</h1>
<p>{{ gettext("Good morning") }}</p>
"#;
    let out = env_with_translations
        .render_str(template, json!({}))
        .expect("gettext in template should work");

    assert!(out.contains("Hola"));
    assert!(out.contains("Buenos días"));
}

/// Test gettext in loops.
#[rstest]
fn test_gettext_in_loop(env_with_translations: Environment) {
    let template = r#"
{% for greeting in greetings %}
{{ gettext(greeting) }}
{% endfor %}
"#;
    let out = env_with_translations
        .render_str(template, json!({"greetings": ["Hello", "Good morning"]}))
        .expect("gettext in loop should work");

    assert!(out.contains("Hola"));
    assert!(out.contains("Buenos días"));
}

/// Test gettext with empty message.
#[rstest]
fn test_gettext_empty_message(env_with_i18n: Environment) {
    let template = "{{ gettext('') }}";
    let out = env_with_i18n
        .render_str(template, json!({}))
        .expect("gettext with empty message should work");

    assert_eq!(out, "");
}

// ============================================================================
// NGETTEXT TESTS
// ============================================================================

/// Test ngettext with singular form (n=1).
#[rstest]
#[case(1, "file", "archivo")]
#[case(1, "item", "elemento")]
fn test_ngettext_singular(
    env_with_translations: Environment,
    #[case] n: i64,
    #[case] msg: &str,
    #[case] expected: &str,
) {
    let template = "{{ ngettext(singular, plural, count) }}";
    let out = env_with_translations
        .render_str(
            template,
            json!({"singular": msg, "plural": msg.to_string() + "s", "count": n}),
        )
        .expect("ngettext singular should render");

    assert_eq!(out, expected);
}

/// Test ngettext with plural form (n > 1).
#[rstest]
#[case(2, "file", "archivos")]
#[case(5, "item", "elementos")]
#[case(100, "file", "archivos")]
fn test_ngettext_plural(
    env_with_translations: Environment,
    #[case] n: i64,
    #[case] msg: &str,
    #[case] expected: &str,
) {
    let template = "{{ ngettext(singular, plural, count) }}";
    let out = env_with_translations
        .render_str(
            template,
            json!({"singular": msg, "plural": msg.to_string() + "s", "count": n}),
        )
        .expect("ngettext plural should render");

    assert_eq!(out, expected);
}

/// Test ngettext without translations (passthrough).
#[rstest]
#[case(1, "apple", "apples", "apple")]
#[case(5, "apple", "apples", "apples")]
#[case(1, "dog", "dogs", "dog")]
#[case(3, "dog", "dogs", "dogs")]
fn test_ngettext_untranslated(
    env_with_i18n: Environment,
    #[case] n: i64,
    #[case] singular: &str,
    #[case] plural: &str,
    #[case] expected: &str,
) {
    let template = "{{ ngettext(s, p, count) }}";
    let out = env_with_i18n
        .render_str(template, json!({"s": singular, "p": plural, "count": n}))
        .expect("ngettext untranslated should work");

    assert_eq!(out, expected);
}

/// Test ngettext in loops.
#[rstest]
fn test_ngettext_in_loop(env_with_translations: Environment) {
    let template = r#"
{% for count in counts %}
{{ count }} {{ ngettext("file", "files", count) }}
{% endfor %}
"#;
    let out = env_with_translations
        .render_str(template, json!({"counts": [1, 2, 5]}))
        .expect("ngettext in loop should work");

    assert!(out.contains("1 archivo"));
    assert!(out.contains("2 archivos"));
    assert!(out.contains("5 archivos"));
}

/// Test ngettext with zero count (should use plural).
#[rstest]
fn test_ngettext_zero_count(env_with_translations: Environment) {
    let template = "{{ ngettext('file', 'files', 0) }}";
    let out = env_with_translations
        .render_str(template, json!({}))
        .expect("ngettext with zero should render");

    assert_eq!(out, "archivos");
}

/// Test combining gettext and ngettext.
#[rstest]
fn test_gettext_and_ngettext_together(env_with_translations: Environment) {
    let template = r#"
{{ gettext("Hello") }}! {{ ngettext("file", "files", count) }}
"#;
    let out = env_with_translations
        .render_str(template, json!({"count": 3}))
        .expect("gettext and ngettext together should work");

    assert!(out.contains("Hola"));
    assert!(out.contains("archivos"));
}

// ============================================================================
// I18N PROVIDER TESTS
// ============================================================================

/// Test I18nProvider translation loading.
#[rstest]
fn test_provider_load_translations(env_with_translations: Environment) {
    // Already tested implicitly through env_with_translations fixture
    let template = "{{ gettext('Thank you') }}";
    let out = env_with_translations
        .render_str(template, json!({}))
        .expect("translations should be loaded");

    assert_eq!(out, "Gracias");
}

/// Test I18nProvider plural forms loading.
#[rstest]
fn test_provider_load_plural_forms(env_with_translations: Environment) {
    // Already tested implicitly through env_with_translations fixture
    let template = "{{ ngettext('item', 'items', 1) }}";
    let out = env_with_translations
        .render_str(template, json!({}))
        .expect("plural forms should be loaded");

    assert_eq!(out, "elemento");
}

// ============================================================================
// REALISTIC I18N TEMPLATE TESTS
// ============================================================================

/// Test a realistic translated page template.
#[rstest]
fn test_realistic_translated_page(env_with_translations: Environment) {
    let template = r#"
<html>
<head>
<title>{{ gettext("Hello") }}</title>
</head>
<body>
<h1>{{ gettext("Good morning") }}</h1>
<p>{{ count }} {{ ngettext("file", "files", count) }}</p>
<footer>{{ gettext("Thank you") }}</footer>
</body>
</html>
"#;
    let out = env_with_translations
        .render_str(template, json!({"count": 5}))
        .expect("realistic page should render");

    assert!(out.contains("Hola"));
    assert!(out.contains("Buenos días"));
    assert!(out.contains("5 archivos"));
    assert!(out.contains("Gracias"));
}

/// Test i18n with conditional rendering.
#[rstest]
fn test_i18n_with_conditionals(env_with_translations: Environment) {
    let template = r#"
{% if user %}
  {{ gettext("Good morning") }}
{% else %}
  {{ gettext("Hello") }}
{% endif %}
"#;
    let out = env_with_translations
        .render_str(template, json!({"user": true}))
        .expect("i18n with conditionals should work");

    assert!(out.contains("Buenos días"));
}

/// Test i18n error handling for invalid argument count.
#[rstest]
fn test_ngettext_missing_arguments(env_with_i18n: Environment) {
    let template = "{{ ngettext('file') }}";
    let result = env_with_i18n.render_str(template, json!({}));

    // Should error due to missing arguments
    assert!(result.is_err());
}
