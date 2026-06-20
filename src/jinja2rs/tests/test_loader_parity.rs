#![allow(clippy::needless_borrows_for_generic_args)]


//! Parity tests for jinja2rs loaders against upstream Jinja2
//!
//! Tests filesystem loading, dict loading, choice loading, and related functionality.
//! Each test is tagged as:
//! - **exact** — byte-for-byte identical behavior
//! - **accepted deviation** — documented difference
//! - **pending** — known gap

use jinja2rs::environment::Environment;
use jinja2rs::loaders::{ChoiceLoader, DictLoader, FileSystemLoader, Loader};
use rstest::rstest;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_dict_loader_get_source_exact() {
    let mut templates = HashMap::new();
    templates.insert(
        "template.html".to_string(),
        "<h1>{{ title }}</h1>".to_string(),
    );
    templates.insert(
        "email.txt".to_string(),
        "From: {{ sender }}\nTo: {{ recipient }}".to_string(),
    );

    let loader = DictLoader::from_map(templates);

    let result = loader.get_source("template.html");
    assert_eq!(result, Some("<h1>{{ title }}</h1>".to_string()));

    let result = loader.get_source("email.txt");
    assert_eq!(
        result,
        Some("From: {{ sender }}\nTo: {{ recipient }}".to_string())
    );
}

#[test]
fn test_dict_loader_missing_template_exact() {
    let mut templates = HashMap::new();
    templates.insert("exists.html".to_string(), "content".to_string());
    let loader = DictLoader::from_map(templates);

    let result = loader.get_source("missing.html");
    assert!(result.is_none());
}

#[test]
fn test_dict_loader_empty_exact() {
    let loader = DictLoader::new();

    let result = loader.get_source("any.html");
    assert!(result.is_none());
}

#[test]
fn test_dict_loader_duplicate_templates_exact() {
    // Later templates override earlier ones
    let mut templates = HashMap::new();
    templates.insert("template.html".to_string(), "second".to_string());
    let loader = DictLoader::from_map(templates);

    let result = loader.get_source("template.html");
    assert_eq!(result, Some("second".to_string()));
}

#[test]
fn test_choice_loader_exact() {
    let mut templates1 = HashMap::new();
    templates1.insert(
        "base.html".to_string(),
        "<html>{% block content %}{% endblock %}</html>".to_string(),
    );

    let mut templates2 = HashMap::new();
    templates2.insert(
        "page.html".to_string(),
        "{% extends 'base.html' %}{% block content %}Page{% endblock %}".to_string(),
    );

    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);

    let loader = ChoiceLoader::new(vec![Arc::new(dict1), Arc::new(dict2)]);

    let result = loader.get_source("base.html").unwrap().unwrap();
    assert!(result.contains("block content"));

    let result = loader.get_source("page.html").unwrap().unwrap();
    assert!(result.contains("extends"));
}

#[test]
fn test_choice_loader_priority_exact() {
    // First loader's template should take priority
    let mut templates1 = HashMap::new();
    templates1.insert("template.html".to_string(), "from_first_loader".to_string());

    let mut templates2 = HashMap::new();
    templates2.insert(
        "template.html".to_string(),
        "from_second_loader".to_string(),
    );

    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);

    let loader = ChoiceLoader::new(vec![Arc::new(dict1), Arc::new(dict2)]);

    let result = loader.get_source("template.html").unwrap().unwrap();
    assert_eq!(result, "from_first_loader");
}

#[test]
fn test_choice_loader_fallback_exact() {
    // Should fall back to second loader if first doesn't have template
    let mut templates1 = HashMap::new();
    templates1.insert("first.html".to_string(), "content1".to_string());

    let mut templates2 = HashMap::new();
    templates2.insert("second.html".to_string(), "content2".to_string());

    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);

    let loader = ChoiceLoader::new(vec![Arc::new(dict1), Arc::new(dict2)]);

    let result = loader.get_source("second.html").unwrap().unwrap();
    assert_eq!(result, "content2");
}

#[test]
fn test_choice_loader_missing_exact() {
    let mut templates1 = HashMap::new();
    templates1.insert("a.html".to_string(), "content".to_string());

    let mut templates2 = HashMap::new();
    templates2.insert("b.html".to_string(), "content".to_string());

    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);

    let loader = ChoiceLoader::new(vec![Arc::new(dict1), Arc::new(dict2)]);

    let result = loader.get_source("c.html").unwrap();
    assert!(result.is_none());
}

#[test]
fn test_choice_loader_empty_exact() {
    let loader = ChoiceLoader::new(vec![]);

    let result = loader.get_source("any.html").unwrap();
    assert!(result.is_none());
}

#[test]
fn test_filesystem_loader_from_temp_exact() {
    use std::fs;
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();
    let template_path = tmpdir.path().join("template.html");
    fs::write(&template_path, "<h1>{{ title }}</h1>").unwrap();

    let loader = FileSystemLoader::new(tmpdir.path());
    let result = loader.get_source("template.html").unwrap().unwrap();
    assert_eq!(result, "<h1>{{ title }}</h1>");
}

#[test]
fn test_filesystem_loader_missing_file_exact() {
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();
    let loader = FileSystemLoader::new(tmpdir.path());

    let result = loader.get_source("missing.html").unwrap();
    assert!(result.is_none());
}

#[test]
fn test_filesystem_loader_subdirectory_exact() {
    use std::fs;
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();
    let subdir = tmpdir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();
    fs::write(subdir.join("nested.html"), "nested content").unwrap();

    let loader = FileSystemLoader::new(tmpdir.path());
    let result = loader.get_source("subdir/nested.html").unwrap().unwrap();
    assert_eq!(result, "nested content");
}

#[rstest]
#[case("../../../etc/passwd", "relative traversal (3 levels)")]
#[case("../../etc/passwd", "relative traversal (2 levels)")]
#[case("../etc/passwd", "relative traversal (1 level)")]
#[case("/etc/passwd", "absolute path bypass")]
#[case("subdir/../../etc/passwd", "traversal through sub-component")]
#[case("./../../etc/passwd", "dot-slash prefix traversal")]
#[case("subdir/../../../etc/passwd", "traversal out of subdir and base")]
#[case("good\0../../../etc/passwd", "null-byte injection")]
#[case("..", "parent directory bare")]
#[case(".", "current directory bare")]
#[case("", "empty name")]
fn test_filesystem_loader_invalid_path_safe_exact(#[case] attack: &str, #[case] _label: &str) {
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();
    let loader = FileSystemLoader::new(tmpdir.path());

    // The only correct result for any traversal attempt is None.
    // A content-based check is not a security guarantee.
    let result = loader.get_source(attack).unwrap();
    assert_eq!(
        result, None,
        "path traversal was not blocked for: {attack:?}"
    );
}

/// Positive control: ensure the loader still serves legitimate files.
/// Without this a buggy "always returns None" implementation would
/// silently pass every traversal case above.
#[test]
fn test_filesystem_loader_legitimate_file_still_works() {
    use std::fs;
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();
    fs::write(tmpdir.path().join("ok.html"), "OK").unwrap();
    let loader = FileSystemLoader::new(tmpdir.path());

    assert_eq!(loader.get_source("ok.html").unwrap().as_deref(), Some("OK"));
}

/// Symlinks that escape the base directory must be blocked.
///
/// This is the canonical traversal attacker scenario: an attacker who can
/// drop a file into the templates directory plants a symlink that points
/// outside the templates root.
#[cfg(unix)]
#[test]
fn test_filesystem_loader_blocks_escaping_symlink() {
    use std::fs;
    use std::os::unix::fs::symlink;
    use tempfile::TempDir;

    let outside = TempDir::new().unwrap();
    fs::write(outside.path().join("secret"), "PWNED").unwrap();

    let tmpdir = TempDir::new().unwrap();
    symlink(
        outside.path().join("secret"),
        tmpdir.path().join("escape.html"),
    )
    .unwrap();

    let loader = FileSystemLoader::new(tmpdir.path());
    let result = loader.get_source("escape.html").unwrap();
    assert_eq!(
        result, None,
        "escaping symlink was followed out of the base directory"
    );
}

/// Symlinks that stay inside the base directory are allowed
/// (matches upstream Jinja2 behavior).
#[cfg(unix)]
#[test]
fn test_filesystem_loader_allows_internal_symlink() {
    use std::fs;
    use std::os::unix::fs::symlink;
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();
    fs::write(tmpdir.path().join("real.html"), "REAL").unwrap();
    symlink(
        tmpdir.path().join("real.html"),
        tmpdir.path().join("link.html"),
    )
    .unwrap();

    let loader = FileSystemLoader::new(tmpdir.path());
    assert_eq!(
        loader.get_source("link.html").unwrap().as_deref(),
        Some("REAL")
    );
}

/// The base directory itself may be a symlink to the real templates directory.
///
/// Because the loader canonicalizes the base before comparing, a legitimate
/// template reached through a symlinked root must still resolve. This guards
/// against an over-zealous `starts_with` check that would reject everything
/// when the configured base differs from its canonical form.
#[cfg(unix)]
#[test]
fn test_filesystem_loader_symlinked_base_resolves() {
    use std::fs;
    use std::os::unix::fs::symlink;
    use tempfile::TempDir;

    let parent = TempDir::new().unwrap();
    let real_dir = parent.path().join("real_templates");
    fs::create_dir(&real_dir).unwrap();
    fs::write(real_dir.join("page.html"), "PAGE").unwrap();

    // The loader is rooted at a symlink pointing to the real directory.
    let link_dir = parent.path().join("link_templates");
    symlink(&real_dir, &link_dir).unwrap();

    let loader = FileSystemLoader::new(&link_dir);
    assert_eq!(
        loader.get_source("page.html").unwrap().as_deref(),
        Some("PAGE")
    );
}

/// An escaping symlinked *directory* component must block traversal through it.
///
/// This differs from the final-component symlink test: here the symlink is an
/// intermediate directory (`link -> /outside`), and the request reads a file
/// *underneath* it (`link/secret`). The canonicalized target escapes the base,
/// so the read must be refused.
#[cfg(unix)]
#[test]
fn test_filesystem_loader_blocks_escaping_symlinked_dir() {
    use std::fs;
    use std::os::unix::fs::symlink;
    use tempfile::TempDir;

    let outside = TempDir::new().unwrap();
    fs::write(outside.path().join("secret"), "PWNED").unwrap();

    let tmpdir = TempDir::new().unwrap();
    // A directory symlink inside the base that points outside it.
    symlink(outside.path(), tmpdir.path().join("link")).unwrap();

    let loader = FileSystemLoader::new(tmpdir.path());
    let result = loader.get_source("link/secret").unwrap();
    assert_eq!(
        result, None,
        "read traversed an escaping symlinked directory"
    );
}

/// Directories are always rejected, even with allow_special_files=true.
#[test]
fn test_filesystem_loader_directories_always_rejected() {
    use std::fs;
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();
    fs::create_dir(tmpdir.path().join("adir")).unwrap();

    // Default loader: a directory name resolves to None (not an Err).
    let loader = FileSystemLoader::new(tmpdir.path());
    assert_eq!(loader.get_source("adir").unwrap(), None);
    assert_eq!(loader.get_source(".").unwrap(), None);

    // Even with special files allowed, directories must still be rejected
    // rather than surfacing an IsADirectory I/O error.
    let loader_special = FileSystemLoader::new(tmpdir.path()).with_special_files(true);
    assert_eq!(loader_special.get_source("adir").unwrap(), None);
    assert_eq!(loader_special.get_source(".").unwrap(), None);
    assert_eq!(loader_special.get_source("").unwrap(), None);
}
/// By default it is false (only regular files are read).
#[test]
fn test_filesystem_loader_special_files_flag() {
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();

    // Create a loader with the default setting (allow_special_files = false)
    let loader_default = FileSystemLoader::new(tmpdir.path());
    assert!(
        !loader_default.allows_special_files(),
        "special files should be blocked by default"
    );

    // Create a loader with special files allowed
    let loader_with_special = FileSystemLoader::new(tmpdir.path()).with_special_files(true);
    assert!(
        loader_with_special.allows_special_files(),
        "special files should be allowed after with_special_files(true)"
    );

    // Verify it can be toggled back
    let loader_back_to_default = loader_with_special.with_special_files(false);
    assert!(
        !loader_back_to_default.allows_special_files(),
        "special files should be blocked after with_special_files(false)"
    );
}

#[test]
fn test_dict_loader_with_environment_exact() {
    let mut templates = HashMap::new();
    templates.insert("greeting".to_string(), "Hello {{ name }}!".to_string());

    let _loader = DictLoader::from_map(templates);

    let env = Environment::new();

    let result = env
        .render_str("Hello {{ name }}!", &json!({"name": "Alice"}))
        .unwrap();
    assert_eq!(result, "Hello Alice!");
}

#[test]
fn test_loader_error_handling_exact() {
    let loader = DictLoader::new();

    // Should handle missing templates gracefully
    let result = loader.get_source("missing");
    assert!(result.is_none());
}

#[test]
fn test_dict_loader_multiple_access_exact() {
    let mut templates = HashMap::new();
    templates.insert("template.html".to_string(), "content".to_string());
    let loader = DictLoader::from_map(templates);

    // Multiple accesses should return same content
    let result1 = loader.get_source("template.html");
    let result2 = loader.get_source("template.html");

    assert_eq!(result1, result2);
    assert_eq!(result1, Some("content".to_string()));
}

#[test]
fn test_choice_loader_multi_level_exact() {
    let mut templates1 = HashMap::new();
    templates1.insert("a.html".to_string(), "from_1".to_string());

    let mut templates2 = HashMap::new();
    templates2.insert("b.html".to_string(), "from_2".to_string());

    let mut templates3 = HashMap::new();
    templates3.insert("c.html".to_string(), "from_3".to_string());

    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);
    let dict3 = DictLoader::from_map(templates3);

    let loader = ChoiceLoader::new(vec![Arc::new(dict1), Arc::new(dict2), Arc::new(dict3)]);

    assert_eq!(loader.get_source("a.html").unwrap().unwrap(), "from_1");
    assert_eq!(loader.get_source("b.html").unwrap().unwrap(), "from_2");
    assert_eq!(loader.get_source("c.html").unwrap().unwrap(), "from_3");
}

#[test]
fn test_choice_loader_duplicate_across_loaders_exact() {
    let mut templates1 = HashMap::new();
    templates1.insert("template.html".to_string(), "first".to_string());

    let mut templates2 = HashMap::new();
    templates2.insert("template.html".to_string(), "second".to_string());

    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);

    let loader = ChoiceLoader::new(vec![Arc::new(dict1), Arc::new(dict2)]);

    // Should use first loader's version
    let result = loader.get_source("template.html").unwrap().unwrap();
    assert_eq!(result, "first");
}

#[test]
fn test_filesystem_loader_special_names_exact() {
    use std::fs;
    use tempfile::TempDir;

    let tmpdir = TempDir::new().unwrap();

    // Create templates with various names
    fs::write(tmpdir.path().join("base.html"), "base").unwrap();
    fs::write(tmpdir.path().join("_partial.html"), "partial").unwrap();
    fs::write(tmpdir.path().join("page.template.html"), "template").unwrap();

    let loader = FileSystemLoader::new(tmpdir.path());

    assert_eq!(loader.get_source("base.html").unwrap().unwrap(), "base");
    assert_eq!(
        loader.get_source("_partial.html").unwrap().unwrap(),
        "partial"
    );
    assert_eq!(
        loader.get_source("page.template.html").unwrap().unwrap(),
        "template"
    );
}
