//! Integration tests for `sphinxdocrs::locale`.
//!
//! Mirrors `sphinx/tests/test_intl/test_locale.py`.
//! Uses rstest fixtures and mocks for isolation.

use rstest::*;
use sphinxdocrs::locale::{
    PoCatalog, clear_translators, get_translation, init, is_translator_registered,
};
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

// ── Fixture helpers ───────────────────────────────────────────────────────────

/// Write a minimal `.po` file to `{dir}/{lang}/LC_MESSAGES/{catalog}.po`.
fn write_po(dir: &Path, lang: &str, catalog: &str, content: &str) {
    let lc = dir.join(lang).join("LC_MESSAGES");
    std::fs::create_dir_all(&lc).unwrap();
    let path = lc.join(format!("{catalog}.po"));
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

/// Pre-made locale1 directory (mirrors `tests/roots/test-locale/locale1`).
#[fixture]
fn locale1() -> TempDir {
    let tmp = TempDir::new().unwrap();
    write_po(
        tmp.path(),
        "en",
        "myext",
        "msgid \"Hello world\"\nmsgstr \"HELLO WORLD\"\n",
    );
    write_po(
        tmp.path(),
        "et",
        "myext",
        "msgid \"Hello world\"\nmsgstr \"Tere maailm\"\n",
    );
    tmp
}

/// Pre-made locale2 directory (mirrors `tests/roots/test-locale/locale2`).
#[fixture]
fn locale2() -> TempDir {
    let tmp = TempDir::new().unwrap();
    write_po(
        tmp.path(),
        "en",
        "myext",
        "msgid \"Hello sphinx\"\nmsgstr \"HELLO SPHINX\"\n",
    );
    tmp
}

// ── test_init ─────────────────────────────────────────────────────────────────

/// Mirrors `test_locale.py::test_init` — progressive loading across two dirs.
#[rstest]
fn test_init(locale1: TempDir, locale2: TempDir) {
    clear_translators();

    // Not initialised yet — fallback to identity
    let ns = "test_init";
    let _ = get_translation("myext", ns);
    let translate = get_translation("myext", ns);
    assert_eq!(translate("Hello world"), "Hello world");
    assert_eq!(translate("Hello sphinx"), "Hello sphinx");
    assert_eq!(translate("Hello reST"), "Hello reST");

    // Load locale1
    init(&[locale1.path()], Some("en"), "myext", ns);
    let translate = get_translation("myext", ns);
    assert_eq!(translate("Hello world"), "HELLO WORLD");
    assert_eq!(translate("Hello sphinx"), "Hello sphinx"); // not in locale1
    assert_eq!(translate("Hello reST"), "Hello reST");

    // Load locale2 into a different (unrelated) namespace — main ns unchanged
    init(&[locale2.path()], Some("en"), "myext", "other_ns");
    let translate = get_translation("myext", ns);
    assert_eq!(translate("Hello world"), "HELLO WORLD");
    assert_eq!(translate("Hello sphinx"), "Hello sphinx"); // still unchanged in ns

    // Load locale2 in addition to locale1 in the same namespace
    init(&[locale2.path()], Some("en"), "myext", ns);
    let translate = get_translation("myext", ns);
    assert_eq!(translate("Hello world"), "HELLO WORLD");
    assert_eq!(translate("Hello sphinx"), "HELLO SPHINX"); // now loaded
    assert_eq!(translate("Hello reST"), "Hello reST"); // still missing → identity
}

// ── test_init_with_unknown_language ───────────────────────────────────────────

/// Mirrors `test_locale.py::test_init_with_unknown_language`.
#[rstest]
fn test_init_with_unknown_language(locale1: TempDir) {
    clear_translators();
    let ns = "test_unknown_lang";

    let found = init(&[locale1.path()], Some("unknown"), "myext", ns);
    assert!(!found, "should not find catalogs for 'unknown' language");

    let translate = get_translation("myext", ns);
    assert_eq!(translate("Hello world"), "Hello world");
    assert_eq!(translate("Hello sphinx"), "Hello sphinx");
    assert_eq!(translate("Hello reST"), "Hello reST");
}

// ── is_translator_registered ─────────────────────────────────────────────────

#[rstest]
fn is_registered_before_and_after_init(locale1: TempDir) {
    clear_translators();
    let ns = "test_reg_check";
    assert!(!is_translator_registered("myext", ns));
    init(&[locale1.path()], Some("en"), "myext", ns);
    assert!(is_translator_registered("myext", ns));
}

// ── PoCatalog::parse corner-cases ─────────────────────────────────────────────

#[rstest]
#[case("en", "HELLO WORLD", "Hello world")]
#[case("et", "Tere maailm", "Hello world")]
fn test_catalog_parse_by_language(
    locale1: TempDir,
    #[case] lang: &str,
    #[case] expected: &str,
    #[case] msgid: &str,
) {
    let po_path = locale1
        .path()
        .join(lang)
        .join("LC_MESSAGES")
        .join("myext.po");
    let content = std::fs::read_to_string(&po_path).unwrap();
    let cat = PoCatalog::parse(&content);
    assert_eq!(cat.gettext(msgid), expected);
}

/// Continuations and escape sequences from a real-world snippet.
#[test]
fn catalog_parses_real_world_snippet() {
    let po = r#"
# Translations template for Sphinx.
msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\n"

#: roles.py:293
#, python-format
msgid "Python Enhancement Proposals; PEP %s"
msgstr "Python Enhancement Proposals; PEP %s"

#: roles.py:354
#, python-format
msgid "invalid RFC number %s"
msgstr "ungültige RFC-Nummer %s"
"#;
    let cat = PoCatalog::parse(po);
    assert_eq!(
        cat.gettext("Python Enhancement Proposals; PEP %s"),
        "Python Enhancement Proposals; PEP %s"
    );
    assert_eq!(
        cat.gettext("invalid RFC number %s"),
        "ungültige RFC-Nummer %s"
    );
}

// ── get_translation closure semantics ────────────────────────────────────────

#[test]
fn get_translation_returns_owned_string() {
    clear_translators();
    let f = get_translation("sphinx_gttest", "ns_gt");
    let result: String = f("Some message");
    assert_eq!(result, "Some message");
}

// ── admonition_labels ────────────────────────────────────────────────────────

#[test]
fn admonition_labels_contains_all_keys() {
    use sphinxdocrs::locale::admonition_labels;
    let labels = admonition_labels();
    for key in &[
        "attention",
        "caution",
        "danger",
        "error",
        "hint",
        "important",
        "note",
        "seealso",
        "tip",
        "warning",
    ] {
        assert!(labels.contains_key(key), "missing label: {key}");
    }
}

#[test]
fn admonition_labels_fallback_to_english_when_no_catalog() {
    use sphinxdocrs::locale::admonition_labels;
    clear_translators();
    let labels = admonition_labels();
    // Without a loaded catalog the labels mirror the English msgids
    assert_eq!(labels["note"], "Note");
    assert_eq!(labels["warning"], "Warning");
}

// ── language_variant edge cases ───────────────────────────────────────────────

#[rstest]
#[case("de_DE", vec!["de_DE", "de"])]
#[case("fr", vec!["fr"])]
#[case("ca@valencia", vec!["ca@valencia", "ca"])]
#[case("en_GB", vec!["en_GB", "en"])]
fn test_language_fallback_chain(#[case] input: &str, #[case] expected: Vec<&str>) {
    // Test via init: locale1 only has "en" → loading "en_US" should fall back.
    let tmp = TempDir::new().unwrap();
    write_po(
        tmp.path(),
        expected[0],
        "myext",
        "msgid \"Hi\"\nmsgstr \"Hi_trans\"\n",
    );
    clear_translators();
    let found = init(&[tmp.path()], Some(input), "myext", "test_lang_fb");
    // At minimum the first variant file exists, so found == true
    assert!(
        found,
        "expected to find catalog for language chain from {input}"
    );
}
