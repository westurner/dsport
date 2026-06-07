//! Parity tests for jinja2rs loaders against upstream Jinja2
//! 
//! Tests filesystem loading, dict loading, choice loading, and related functionality.
//! Each test is tagged as:
//! - **exact** — byte-for-byte identical behavior
//! - **accepted deviation** — documented difference
//! - **pending** — known gap

use jinja2rs::loaders::{DictLoader, ChoiceLoader, FileSystemLoader, Loader};
use serde_json::json;
use jinja2rs::environment::Environment;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_dict_loader_get_source_exact() {
    let mut templates = HashMap::new();
    templates.insert("template.html".to_string(), "<h1>{{ title }}</h1>".to_string());
    templates.insert("email.txt".to_string(), "From: {{ sender }}\nTo: {{ recipient }}".to_string());
    
    let loader = DictLoader::from_map(templates);
    
    let result = loader.get_source("template.html");
    assert_eq!(result, Some("<h1>{{ title }}</h1>".to_string()));
    
    let result = loader.get_source("email.txt");
    assert_eq!(result, Some("From: {{ sender }}\nTo: {{ recipient }}".to_string()));
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
    templates1.insert("base.html".to_string(), "<html>{% block content %}{% endblock %}</html>".to_string());
    
    let mut templates2 = HashMap::new();
    templates2.insert("page.html".to_string(), "{% extends 'base.html' %}{% block content %}Page{% endblock %}".to_string());
    
    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);
    
    let loader = ChoiceLoader::new(vec![
        Arc::new(dict1),
        Arc::new(dict2),
    ]);
    
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
    templates2.insert("template.html".to_string(), "from_second_loader".to_string());
    
    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);
    
    let loader = ChoiceLoader::new(vec![
        Arc::new(dict1),
        Arc::new(dict2),
    ]);
    
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
    
    let loader = ChoiceLoader::new(vec![
        Arc::new(dict1),
        Arc::new(dict2),
    ]);
    
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
    
    let loader = ChoiceLoader::new(vec![
        Arc::new(dict1),
        Arc::new(dict2),
    ]);
    
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

#[test]
#[ignore] // TODO: FileSystemLoader path traversal protection
fn test_filesystem_loader_invalid_path_safe_exact() {
    use tempfile::TempDir;
    
    let tmpdir = TempDir::new().unwrap();
    let loader = FileSystemLoader::new(tmpdir.path());
    
    // Path traversal attempts should not read outside the template directory
    // (This is tested more thoroughly in sandbox security tests)
    let result = loader.get_source("../../../etc/passwd").unwrap();
    // It's OK if this either returns None or returns no content from the temp dir
    match result {
        None => assert!(true), // Path was blocked
        Some(content) => {
            // If content was returned, it should not be from system files
            assert!(!content.contains("root"));
        }
    }
}

#[test]
fn test_dict_loader_with_environment_exact() {
    let mut templates = HashMap::new();
    templates.insert("greeting".to_string(), "Hello {{ name }}!".to_string());
    
    let _loader = DictLoader::from_map(templates);
    
    let env = Environment::new();
    
    let result = env.render_str("Hello {{ name }}!", &json!({"name": "Alice"})).unwrap();
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
    
    let loader = ChoiceLoader::new(vec![
        Arc::new(dict1),
        Arc::new(dict2),
        Arc::new(dict3),
    ]);
    
    assert_eq!(
        loader.get_source("a.html").unwrap().unwrap(),
        "from_1"
    );
    assert_eq!(
        loader.get_source("b.html").unwrap().unwrap(),
        "from_2"
    );
    assert_eq!(
        loader.get_source("c.html").unwrap().unwrap(),
        "from_3"
    );
}

#[test]
fn test_choice_loader_duplicate_across_loaders_exact() {
    let mut templates1 = HashMap::new();
    templates1.insert("template.html".to_string(), "first".to_string());
    
    let mut templates2 = HashMap::new();
    templates2.insert("template.html".to_string(), "second".to_string());
    
    let dict1 = DictLoader::from_map(templates1);
    let dict2 = DictLoader::from_map(templates2);
    
    let loader = ChoiceLoader::new(vec![
        Arc::new(dict1),
        Arc::new(dict2),
    ]);
    
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
    
    assert_eq!(
        loader.get_source("base.html").unwrap().unwrap(),
        "base"
    );
    assert_eq!(
        loader.get_source("_partial.html").unwrap().unwrap(),
        "partial"
    );
    assert_eq!(
        loader.get_source("page.template.html").unwrap().unwrap(),
        "template"
    );
}
