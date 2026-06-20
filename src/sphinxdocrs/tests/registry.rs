//! Integration tests for `sphinxdocrs::registry` — P2 subset.
//!
//! Mirrors the behaviour described in `sphinx.registry.SphinxComponentRegistry`
//! and verified against the upstream Python test suite.

use std::collections::HashMap;
use std::path::PathBuf;

use rstest::*;
use sphinxdocrs::registry::{RegistryError, SphinxComponentRegistry};

// ── helpers ────────────────────────────────────────────────────────────────

fn reg() -> SphinxComponentRegistry {
    SphinxComponentRegistry::new()
}

// ── source_suffix ─────────────────────────────────────────────────────────

#[test]
fn source_suffix_register_and_retrieve() {
    let mut r = reg();
    r.add_source_suffix(".rst", "restructuredtext", false)
        .unwrap();
    r.add_source_suffix(".md", "myst", false).unwrap();
    assert_eq!(
        r.source_suffix.get(".rst").map(String::as_str),
        Some("restructuredtext")
    );
    assert_eq!(r.source_suffix.get(".md").map(String::as_str), Some("myst"));
}

#[rstest]
#[case(".rst")]
#[case(".md")]
#[case(".txt")]
fn source_suffix_duplicate_rejected(#[case] suffix: &str) {
    let mut r = reg();
    r.add_source_suffix(suffix, "first", false).unwrap();
    let err = r.add_source_suffix(suffix, "second", false).unwrap_err();
    assert!(
        matches!(&err, RegistryError::Duplicate(msg) if msg.contains(suffix)),
        "expected Duplicate for {suffix}: {err}"
    );
    // Original value preserved
    assert_eq!(r.source_suffix[suffix], "first");
}

#[test]
fn source_suffix_override_allowed() {
    let mut r = reg();
    r.add_source_suffix(".rst", "restructuredtext", false)
        .unwrap();
    r.add_source_suffix(".rst", "myst", true).unwrap();
    assert_eq!(r.source_suffix[".rst"], "myst");
}

// ── source_parser ─────────────────────────────────────────────────────────

#[test]
fn source_parser_register_and_get() {
    let mut r = reg();
    r.add_source_parser("sphinx.parsers.RSTParser", &["restructuredtext"], false)
        .unwrap();
    assert_eq!(
        r.get_source_parser("restructuredtext").unwrap(),
        "sphinx.parsers.RSTParser"
    );
}

#[test]
fn source_parser_multi_filetype() {
    let mut r = reg();
    r.add_source_parser("myst_parser.Parser", &["myst", "markdown"], false)
        .unwrap();
    assert_eq!(r.get_source_parser("myst").unwrap(), "myst_parser.Parser");
    assert_eq!(
        r.get_source_parser("markdown").unwrap(),
        "myst_parser.Parser"
    );
}

#[test]
fn source_parser_duplicate_rejected() {
    let mut r = reg();
    r.add_source_parser("sphinx.parsers.RSTParser", &["restructuredtext"], false)
        .unwrap();
    let err = r
        .add_source_parser("other.Parser", &["restructuredtext"], false)
        .unwrap_err();
    assert!(matches!(err, RegistryError::Duplicate(_)));
}

#[test]
fn source_parser_override_replaces() {
    let mut r = reg();
    r.add_source_parser("sphinx.parsers.RSTParser", &["restructuredtext"], false)
        .unwrap();
    r.add_source_parser("custom.Parser", &["restructuredtext"], true)
        .unwrap();
    assert_eq!(
        r.get_source_parser("restructuredtext").unwrap(),
        "custom.Parser"
    );
}

#[test]
fn source_parser_not_found_error() {
    let r = reg();
    let err = r.get_source_parser("notype").unwrap_err();
    assert!(matches!(&err, RegistryError::NotFound(m) if m.contains("notype")));
}

#[test]
fn get_source_parsers_map() {
    let mut r = reg();
    r.add_source_parser("P1", &["t1"], false).unwrap();
    r.add_source_parser("P2", &["t2"], false).unwrap();
    let map = r.get_source_parsers();
    assert_eq!(map.len(), 2);
    assert_eq!(map["t1"], "P1");
    assert_eq!(map["t2"], "P2");
}

// ── transforms ────────────────────────────────────────────────────────────

#[test]
fn transforms_empty_by_default() {
    let r = reg();
    assert!(r.get_transforms().is_empty());
    assert!(r.get_post_transforms().is_empty());
}

#[test]
fn transforms_ordered() {
    let mut r = reg();
    r.add_transform("T1");
    r.add_transform("T2");
    r.add_transform("T3");
    assert_eq!(r.get_transforms(), &["T1", "T2", "T3"]);
}

#[test]
fn post_transforms_ordered() {
    let mut r = reg();
    r.add_post_transform("PT1");
    r.add_post_transform("PT2");
    assert_eq!(r.get_post_transforms(), &["PT1", "PT2"]);
}

#[test]
fn transforms_and_post_transforms_independent() {
    let mut r = reg();
    r.add_transform("T");
    r.add_post_transform("PT");
    assert_eq!(r.get_transforms().len(), 1);
    assert_eq!(r.get_post_transforms().len(), 1);
    assert_ne!(r.get_transforms()[0], r.get_post_transforms()[0]);
}

#[test]
fn duplicate_transform_names_allowed() {
    // upstream does not de-duplicate transforms
    let mut r = reg();
    r.add_transform("T");
    r.add_transform("T");
    assert_eq!(r.get_transforms().len(), 2);
}

// ── CSS assets ────────────────────────────────────────────────────────────

#[test]
fn css_file_added() {
    let mut r = reg();
    r.add_css_file("custom.css", HashMap::new());
    assert_eq!(r.css_files.len(), 1);
    assert_eq!(r.css_files[0].filename, "custom.css");
}

#[test]
fn css_file_with_attributes() {
    let mut r = reg();
    let attrs = HashMap::from([("media".to_string(), "print".to_string())]);
    r.add_css_file("print.css", attrs.clone());
    assert_eq!(r.css_files[0].attributes["media"], "print");
}

#[test]
fn css_files_ordered() {
    let mut r = reg();
    r.add_css_file("a.css", HashMap::new());
    r.add_css_file("b.css", HashMap::new());
    assert_eq!(r.css_files[0].filename, "a.css");
    assert_eq!(r.css_files[1].filename, "b.css");
}

// ── JS assets ─────────────────────────────────────────────────────────────

#[test]
fn js_file_named() {
    let mut r = reg();
    r.add_js_file(Some("app.js"), HashMap::new());
    assert_eq!(r.js_files[0].filename, Some("app.js".to_string()));
}

#[test]
fn js_file_none_filename() {
    let mut r = reg();
    r.add_js_file(None::<&str>, HashMap::new());
    assert_eq!(r.js_files[0].filename, None);
}

// ── static directories ────────────────────────────────────────────────────

#[test]
fn static_dir_registered() {
    let mut r = reg();
    r.add_static_dir("/ext/static");
    assert_eq!(r.static_dirs[0], PathBuf::from("/ext/static"));
}

#[test]
fn static_dirs_ordered() {
    let mut r = reg();
    r.add_static_dir("/a");
    r.add_static_dir("/b");
    assert_eq!(r.static_dirs, [PathBuf::from("/a"), PathBuf::from("/b")]);
}

// ── LaTeX packages ────────────────────────────────────────────────────────

#[test]
fn latex_package_basic() {
    let mut r = reg();
    assert!(
        r.add_latex_package("amsmath", None::<&str>, false)
            .is_none()
    );
    assert!(r.has_latex_package("amsmath"));
}

#[test]
fn latex_package_with_options() {
    let mut r = reg();
    r.add_latex_package("geometry", Some("margin=1in"), false);
    assert_eq!(r.latex_packages[0].1, Some("margin=1in".to_string()));
}

#[test]
fn latex_package_after_hyperref() {
    let mut r = reg();
    r.add_latex_package("amsmath", None::<&str>, false);
    r.add_latex_package("bookmark", None::<&str>, true);
    assert_eq!(r.latex_packages.len(), 1);
    assert_eq!(r.latex_packages_after_hyperref.len(), 1);
    assert_eq!(r.latex_packages_after_hyperref[0].0, "bookmark");
}

#[test]
fn latex_package_duplicate_warns_non_fatal() {
    let mut r = reg();
    r.add_latex_package("pkg", None::<&str>, false);
    let warn = r.add_latex_package("pkg", None::<&str>, false);
    assert!(matches!(warn, Some(RegistryError::Warning(_))));
    // Package still added (upstream does not deduplicate)
    assert_eq!(r.latex_packages.len(), 2);
}

#[test]
fn has_latex_package_false_when_absent() {
    let r = reg();
    assert!(!r.has_latex_package("missing"));
}

#[test]
fn has_latex_package_checks_after_hyperref() {
    let mut r = reg();
    r.add_latex_package("bm", None::<&str>, true);
    assert!(r.has_latex_package("bm"));
}

// ── HTML themes ───────────────────────────────────────────────────────────

#[test]
fn html_theme_registered() {
    let mut r = reg();
    r.add_html_theme("mytheme", "/path/to/mytheme");
    assert_eq!(r.html_themes["mytheme"], PathBuf::from("/path/to/mytheme"));
}

#[test]
fn html_theme_overwritten_by_later_add() {
    let mut r = reg();
    r.add_html_theme("t", "/old");
    r.add_html_theme("t", "/new");
    assert_eq!(r.html_themes["t"], PathBuf::from("/new"));
}

// ── empty registry ────────────────────────────────────────────────────────

#[test]
fn new_registry_all_empty() {
    let r = SphinxComponentRegistry::new();
    assert!(r.source_suffix.is_empty());
    assert!(r.source_parsers.is_empty());
    assert!(r.transforms.is_empty());
    assert!(r.post_transforms.is_empty());
    assert!(r.css_files.is_empty());
    assert!(r.js_files.is_empty());
    assert!(r.static_dirs.is_empty());
    assert!(r.latex_packages.is_empty());
    assert!(r.latex_packages_after_hyperref.is_empty());
    assert!(r.html_themes.is_empty());
}
