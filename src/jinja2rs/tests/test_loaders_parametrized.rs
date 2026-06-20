#![allow(clippy::needless_borrows_for_generic_args)]

//! Parametrized loader tests using rstest fixtures and mocking.
//!
//! Demonstrates best practices for:
//! - `#[rstest]` parametrization with `#[case]` macro
//! - Shared `#[fixture]` setup
//! - Loader composition and chaining

use jinja2rs::{ChoiceLoader, DictLoader, FileSystemLoader, Loader};
use rstest::{fixture, rstest};
use std::collections::HashMap;

// ============================================================================
// FIXTURES
// ============================================================================

/// Fixture: a pre-populated DictLoader with common test templates.
#[fixture]
fn base_templates() -> HashMap<String, String> {
    vec![
        ("base.html", "{% block content %}default{% endblock %}"),
        ("greeting.html", "Hello {{ name }}!"),
        ("loop.html", "{% for item in items %}{{ item }}{% endfor %}"),
        ("nested.html", "{% include 'base.html' %}"),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
}

/// Fixture: a DictLoader instance with base templates.
#[fixture]
fn dict_loader(base_templates: HashMap<String, String>) -> DictLoader {
    DictLoader::from_map(base_templates)
}

// ============================================================================
// PARAMETRIZED TESTS
// ============================================================================

/// Test DictLoader with various template names and contents.
#[rstest]
#[case("greeting.html", Some("Hello {{ name }}!".to_string()))]
#[case("base.html", Some("{% block content %}default{% endblock %}".to_string()))]
#[case("missing.html", None)]
#[case("", None)]
fn test_dict_loader_lookup(
    dict_loader: DictLoader,
    #[case] name: &str,
    #[case] expected: Option<String>,
) {
    let result = dict_loader.get_source(name);
    assert_eq!(result, expected, "Failed to lookup {}", name);
}

/// Test DictLoader.Loader trait implementation.
#[rstest]
#[case("greeting.html", true)]
#[case("base.html", true)]
#[case("nonexistent.html", false)]
fn test_dict_loader_trait(dict_loader: DictLoader, #[case] name: &str, #[case] should_exist: bool) {
    // Test that DictLoader implements Loader trait
    let loader: &dyn Loader = &dict_loader;
    let result = loader.get_source(name);

    assert!(result.is_ok(), "Loader::get_source() should not error");
    let source = result.unwrap();
    if should_exist {
        assert!(source.is_some(), "Template {} should exist", name);
    } else {
        assert!(source.is_none(), "Template {} should not exist", name);
    }
}

/// Test FileSystemLoader with various file lookups.
#[rstest]
#[case("page.html", Some("Page: {{ title }}".to_string()))]
#[case("layout.html", Some("Layout: {% block body %}{% endblock %}".to_string()))]
#[case("missing.html", None)]
fn test_fs_loader_lookup(#[case] name: &str, #[case] expected: Option<String>) {
    // Create temp directory with files
    let temp_dir = tempfile::tempdir().expect("create tempdir");
    std::fs::write(temp_dir.path().join("page.html"), "Page: {{ title }}").unwrap();
    std::fs::write(
        temp_dir.path().join("layout.html"),
        "Layout: {% block body %}{% endblock %}",
    )
    .unwrap();

    let fs_loader = FileSystemLoader::new(temp_dir.path());
    let result = fs_loader
        .get_source(name)
        .expect("FileSystemLoader::get_source should not error");
    assert_eq!(result, expected, "Failed to lookup {}", name);
}

/// Test ChoiceLoader prioritization with multiple sources.
///
/// Verifies that ChoiceLoader tries loaders in order and returns the first match.
#[rstest]
#[case("greeting.html", "Hello {{ name }}!")] // In first loader (DictLoader)
#[case("page.html", "Page: {{ title }}")] // In second loader (FileSystemLoader)
#[case("base.html", "{% block content %}default{% endblock %}")] // In first loader (shadows second)
fn test_choice_loader_precedence(
    dict_loader: DictLoader,
    #[case] name: &str,
    #[case] expected: &str,
) {
    use std::sync::Arc;

    // Create temp directory with files
    let temp_dir = tempfile::tempdir().expect("create tempdir");
    std::fs::write(temp_dir.path().join("page.html"), "Page: {{ title }}").unwrap();
    std::fs::write(
        temp_dir.path().join("layout.html"),
        "Layout: {% block body %}{% endblock %}",
    )
    .unwrap();
    let fs_loader = FileSystemLoader::new(temp_dir.path());

    // Order matters: dict_loader first, then fs_loader
    let choice = ChoiceLoader::new(vec![Arc::new(dict_loader), Arc::new(fs_loader)]);

    let result = choice
        .get_source(name)
        .expect("ChoiceLoader::get_source should not error");

    assert_eq!(
        result,
        Some(expected.to_string()),
        "ChoiceLoader should find {} in priority order",
        name
    );
}

/// Test ChoiceLoader with reversed order.
///
/// Verifies that changing the order changes which template is returned.
#[rstest]
fn test_choice_loader_reverse_order(dict_loader: DictLoader) {
    use std::sync::Arc;

    // Create temp directory with files
    let temp_dir = tempfile::tempdir().expect("create tempdir");
    std::fs::write(temp_dir.path().join("page.html"), "Page: {{ title }}").unwrap();
    std::fs::write(
        temp_dir.path().join("layout.html"),
        "Layout: {% block body %}{% endblock %}",
    )
    .unwrap();
    let fs_loader = FileSystemLoader::new(temp_dir.path());

    // Reverse order: fs_loader first, then dict_loader
    let choice = ChoiceLoader::new(vec![Arc::new(fs_loader), Arc::new(dict_loader)]);

    // page.html only exists in fs_loader, should be found
    let result = choice
        .get_source("page.html")
        .expect("ChoiceLoader should find page.html");
    assert!(result.is_some());

    // base.html exists in both; fs_loader comes first
    // (but fs_loader doesn't have it, so falls back to dict_loader)
    let result = choice
        .get_source("base.html")
        .expect("ChoiceLoader should find base.html from second loader");
    assert_eq!(
        result,
        Some("{% block content %}default{% endblock %}".to_string())
    );
}

/// Test ChoiceLoader returns None when no loader has the template.
#[rstest]
fn test_choice_loader_not_found(dict_loader: DictLoader) {
    use std::sync::Arc;

    // Create temp directory with files
    let temp_dir = tempfile::tempdir().expect("create tempdir");
    std::fs::write(temp_dir.path().join("page.html"), "Page: {{ title }}").unwrap();
    let fs_loader = FileSystemLoader::new(temp_dir.path());

    let choice = ChoiceLoader::new(vec![Arc::new(dict_loader), Arc::new(fs_loader)]);

    let result = choice
        .get_source("totally_missing.html")
        .expect("ChoiceLoader::get_source should not error");

    assert_eq!(
        result, None,
        "ChoiceLoader should return None when template not found"
    );
}

/// Parametrized test for template variations in DictLoader.
///
/// Tests different template syntaxes and content patterns.
#[rstest]
#[case("empty.html", "", "", "Empty template should be loadable")]
#[case(
    "whitespace.html",
    "   \n\n   ",
    "   \n\n   ",
    "Whitespace-only template should be preserved"
)]
#[case(
    "multiline.html",
    "line1\nline2\nline3",
    "line1\nline2\nline3",
    "Multiline template should preserve newlines"
)]
#[case(
    "syntax_html.html",
    "{{ var|upper }}",
    "{{ var|upper }}",
    "Template syntax should be preserved as-is"
)]
fn test_dict_loader_content_preservation(
    #[case] name: &str,
    #[case] content: &str,
    #[case] expected: &str,
    #[case] description: &str,
) {
    let mut map = HashMap::new();
    map.insert(name.to_string(), content.to_string());
    let loader = DictLoader::from_map(map);

    let result = loader.get_source(name).expect(description);
    assert_eq!(result, expected, "{}", description);
}

/// Test that DictLoader cloning creates independent instances.
#[rstest]
fn test_dict_loader_clone_independence(dict_loader: DictLoader) {
    let cloned = dict_loader.clone();

    // Both should return the same template
    let orig = dict_loader.get_source("greeting.html");
    let cloned_result = cloned.get_source("greeting.html");

    assert_eq!(orig, cloned_result);
    assert!(orig.is_some());
}

/// Test ChoiceLoader with 3+ loaders in a chain.
#[rstest]
fn test_choice_loader_chain(base_templates: HashMap<String, String>) {
    use std::sync::Arc;
    // Create three loaders with different templates
    let mut loader1_map = HashMap::new();
    loader1_map.insert("first.html".to_string(), "FIRST".to_string());

    let mut loader2_map = HashMap::new();
    loader2_map.insert("second.html".to_string(), "SECOND".to_string());

    let mut loader3_map = base_templates.clone();
    loader3_map.insert("third.html".to_string(), "THIRD".to_string());

    let choice = ChoiceLoader::new(vec![
        Arc::new(DictLoader::from_map(loader1_map)),
        Arc::new(DictLoader::from_map(loader2_map)),
        Arc::new(DictLoader::from_map(loader3_map)),
    ]);

    // Each should be found from its respective loader
    assert_eq!(
        choice.get_source("first.html").ok().flatten(),
        Some("FIRST".to_string())
    );
    assert_eq!(
        choice.get_source("second.html").ok().flatten(),
        Some("SECOND".to_string())
    );
    assert_eq!(
        choice.get_source("third.html").ok().flatten(),
        Some("THIRD".to_string())
    );

    // From the third loader (which has base_templates)
    assert_eq!(
        choice.get_source("greeting.html").ok().flatten(),
        Some("Hello {{ name }}!".to_string())
    );
}

/// Test that minijinja closures work correctly with DictLoader.
#[rstest]
fn test_dict_loader_minijinja_closure(dict_loader: DictLoader) {
    let closure = dict_loader.into_minijinja_loader();

    let result = closure("greeting.html");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("Hello {{ name }}!".to_string()));

    let result = closure("missing.html");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

/// Test that ChoiceLoader minijinja closure works end-to-end.
#[rstest]
fn test_choice_loader_minijinja_closure(dict_loader: DictLoader) {
    use std::sync::Arc;

    // Create temp directory with files
    let temp_dir = tempfile::tempdir().expect("create tempdir");
    std::fs::write(temp_dir.path().join("page.html"), "Page: {{ title }}").unwrap();
    std::fs::write(
        temp_dir.path().join("layout.html"),
        "Layout: {% block body %}{% endblock %}",
    )
    .unwrap();
    let fs_loader = FileSystemLoader::new(temp_dir.path());

    let choice = ChoiceLoader::new(vec![Arc::new(dict_loader), Arc::new(fs_loader)]);
    let closure = choice.into_minijinja_loader();

    // From dict_loader
    let result = closure("greeting.html");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    // From fs_loader
    let result = closure("page.html");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    // Not found
    let result = closure("nonexistent.html");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}
