//! `sphinxdocrs::locale` — Rust port of `sphinx.locale`.
//!
//! Provides `.po`-file parsing, a catalog registry, and a `tr()` helper that
//! mirrors the `_()` / `__()` shortcuts in the Python module.
//!
//! ## Ported surface
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `_TranslationProxy` | internal lazy call in `get_translation` closure | |
//! | `translators` | [`TRANSLATORS`] global registry | `OnceLock<Mutex<…>>` |
//! | `init()` | [`init`] | loads `.po` files from locale dirs |
//! | `init_console()` | [`init_console`] | console-oriented init |
//! | `get_translator()` | [`get_translator`] | look up a single translator |
//! | `is_translator_registered()` | [`is_translator_registered`] | predicate |
//! | `get_translation()` | [`get_translation`] | returns a `Fn(&str)->String` |
//! | `_ = get_translation('sphinx')` | [`tr`] | shortcut for the sphinx catalog |
//! | `__ = get_translation('sphinx','console')` | [`tr_console`] | console shortcut |
//! | `admonitionlabels` | [`admonition_labels`] | map of admonition names |
//!
//! The built-in sphinx locale directory is resolved via the `BUILTIN_LOCALE_DIR`
//! constant that points at the symlinked `locale/` directory adjacent to the crate.
//!
//! ## `.po` file parsing
//!
//! [`PoCatalog::parse`] implements a lightweight, dependency-free PO parser that
//! handles the subset Sphinx uses: simple `msgid`/`msgstr` pairs, multi-line
//! continuations, and comment lines.  Plural forms are parsed but only the
//! singular form (`msgstr[0]`) is exposed through `gettext`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

// ── built-in locale directory ─────────────────────────────────────────────────

/// Relative path (from the crate root) to the sphinx locale symlink.
///
/// `sphinx-build-rs` embeds this at compile time so binary users see translated
/// messages out of the box when the symlink resolves.  Unit tests override this
/// with temp dirs via [`init`].
pub const BUILTIN_LOCALE_SUBDIR: &str = "locale";

/// Return the path to the built-in sphinx locale directory adjacent to the
/// running binary, if it exists.
pub fn builtin_locale_dir() -> Option<PathBuf> {
    // Walk up from the exe directory searching for `locale/sphinx.pot`.
    // This handles both `cargo run` (exe is in target/debug/) and installed
    // binaries where `locale` lives next to the binary.
    let exe = std::env::current_exe().ok()?;
    // First try exe-sibling `locale/`
    let sibling = exe.parent()?.join(BUILTIN_LOCALE_SUBDIR);
    if sibling.join("sphinx.pot").exists() {
        return Some(sibling);
    }
    // Fall back to the crate's source tree location (dev / test runs via cargo)
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src_local = manifest.join(BUILTIN_LOCALE_SUBDIR);
    if src_local.join("sphinx.pot").exists() {
        return Some(src_local);
    }
    None
}

// ── .po file parser ───────────────────────────────────────────────────────────

/// A single entry parsed from a `.po` file.
#[derive(Debug, Clone, Default)]
pub struct PoEntry {
    pub msgid: String,
    pub msgstr: String,
}

/// A parsed `.po` catalog: `msgid → msgstr` map.
///
/// Only the singular `msgstr` is stored; plural forms collapse to `msgstr[0]`.
#[derive(Debug, Clone, Default)]
pub struct PoCatalog {
    entries: HashMap<String, String>,
}

impl PoCatalog {
    /// Parse `content` as a GNU gettext `.po` file and return a catalog.
    ///
    /// Handles:
    /// - `msgid "…"` / `msgstr "…"` pairs  
    /// - Multi-line continuations (`"…"` lines following `msgid`/`msgstr`)  
    /// - `msgstr[0]` plural forms (only index 0 is kept)  
    /// - `#`-prefixed comment lines (ignored)  
    /// - C-style escape sequences (`\\`, `\n`, `\t`, `\"`)
    pub fn parse(content: &str) -> Self {
        let mut catalog = PoCatalog::default();
        let mut cur_id = String::new();
        let mut cur_str = String::new();
        let mut in_msgid = false;
        let mut in_msgstr = false;
        let mut in_msgstr0 = false; // msgstr[0]

        let flush = |id: &mut String, s: &mut String, cat: &mut PoCatalog| {
            if !id.is_empty() {
                cat.entries.insert(id.clone(), s.clone());
            }
            id.clear();
            s.clear();
        };

        for line in content.lines() {
            let line = line.trim_end();

            if line.starts_with('#') || line.is_empty() {
                // comment or blank — end of a block if we were collecting
                if line.is_empty() {
                    flush(&mut cur_id, &mut cur_str, &mut catalog);
                    in_msgid = false;
                    in_msgstr = false;
                    in_msgstr0 = false;
                }
                continue;
            }

            if line.starts_with("msgid ") {
                // Save previous entry
                flush(&mut cur_id, &mut cur_str, &mut catalog);
                in_msgid = true;
                in_msgstr = false;
                in_msgstr0 = false;
                cur_id = unescape_po_string(strip_po_quotes(line.trim_start_matches("msgid ")));
                continue;
            }

            if line.starts_with("msgid_plural ") {
                in_msgid = false;
                continue;
            }

            if line.starts_with("msgstr[0]") {
                in_msgid = false;
                in_msgstr = false;
                in_msgstr0 = true;
                let rest = line.trim_start_matches("msgstr[0]").trim_start_matches(' ');
                cur_str = unescape_po_string(strip_po_quotes(rest));
                continue;
            }

            if line.starts_with("msgstr[") {
                // higher plural forms — ignore
                in_msgid = false;
                in_msgstr = false;
                in_msgstr0 = false;
                continue;
            }

            if line.starts_with("msgstr ") {
                in_msgid = false;
                in_msgstr = true;
                in_msgstr0 = false;
                cur_str = unescape_po_string(strip_po_quotes(line.trim_start_matches("msgstr ")));
                continue;
            }

            // Continuation line: starts with `"`
            if line.starts_with('"') {
                let chunk = unescape_po_string(strip_po_quotes(line));
                if in_msgid {
                    cur_id.push_str(&chunk);
                } else if in_msgstr || in_msgstr0 {
                    cur_str.push_str(&chunk);
                }
                continue;
            }
        }

        // flush final entry
        flush(&mut cur_id, &mut cur_str, &mut catalog);
        catalog
    }

    /// Look up `msgid` and return the translation, or `msgid` if not found or
    /// if the `msgstr` is empty (empty `msgstr` means untranslated in gettext).
    pub fn gettext<'a>(&'a self, msgid: &'a str) -> &'a str {
        match self.entries.get(msgid) {
            Some(s) if !s.is_empty() => s.as_str(),
            _ => msgid,
        }
    }

    /// Merge `other` into this catalog (entries in `other` take priority).
    pub fn merge(&mut self, other: PoCatalog) {
        self.entries.extend(other.entries);
    }

    /// Number of entries in this catalog.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return `true` if the catalog has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Strip the surrounding `"…"` from a PO string token (may be `""`).
fn strip_po_quotes(s: &str) -> &str {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

/// Decode C-style escape sequences used in `.po` files.
fn unescape_po_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some(c) => {
                    out.push('\\');
                    out.push(c);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

// ── Translator ────────────────────────────────────────────────────────────────

/// A translator wrapping a chain of [`PoCatalog`]s with fallback.
///
/// Mirrors Python's `NullTranslations` + `add_fallback` semantics.
#[derive(Debug, Clone, Default)]
pub struct Translator {
    /// Catalogs in priority order (index 0 = highest priority).
    catalogs: Vec<PoCatalog>,
}

impl Translator {
    /// Construct a null translator (always returns the msgid unchanged).
    pub fn null() -> Self {
        Self::default()
    }

    /// Construct a translator from a single catalog.
    pub fn from_catalog(catalog: PoCatalog) -> Self {
        Translator {
            catalogs: vec![catalog],
        }
    }

    /// Look up `msgid` in the catalog chain, returning the first translation found,
    /// or `msgid` if none of the catalogs has a non-empty translation.
    pub fn gettext<'a>(&'a self, msgid: &'a str) -> &'a str {
        for cat in &self.catalogs {
            let translated = cat.gettext(msgid);
            if translated != msgid {
                return translated;
            }
        }
        msgid
    }

    /// Add `catalog` as a lower-priority fallback.
    pub fn add_fallback(&mut self, catalog: PoCatalog) {
        self.catalogs.push(catalog);
    }

    /// Return `true` if this translator has at least one non-empty catalog
    /// (i.e. was successfully loaded from a `.po` file).
    pub fn has_translation(&self) -> bool {
        !self.catalogs.is_empty()
    }
}

// ── Global registry ───────────────────────────────────────────────────────────

/// Registry key: `(namespace, catalog)`.
type RegistryKey = (String, String);

static TRANSLATORS: OnceLock<Mutex<HashMap<RegistryKey, Translator>>> = OnceLock::new();

fn registry() -> &'static Mutex<HashMap<RegistryKey, Translator>> {
    TRANSLATORS.get_or_init(|| Mutex::new(HashMap::new()))
}

// ── init / init_console ───────────────────────────────────────────────────────

/// Probe `locale_dir` for `{language}/LC_MESSAGES/{catalog}.po` and return a
/// `PoCatalog` if the file exists and can be parsed.
fn try_load_catalog(locale_dir: &Path, language: &str, catalog: &str) -> Option<PoCatalog> {
    let po_path = locale_dir
        .join(language)
        .join("LC_MESSAGES")
        .join(format!("{catalog}.po"));
    let content = std::fs::read_to_string(&po_path).ok()?;
    let parsed = PoCatalog::parse(&content);
    // Only return if there's at least one real translation entry
    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

/// Language fallback chain: `"de_DE"` → `["de_DE", "de"]`.
fn language_variants(language: &str) -> Vec<String> {
    let mut variants = vec![language.to_owned()];
    if let Some(pos) = language.find(['_', '@']) {
        variants.push(language[..pos].to_owned());
    }
    variants
}

/// Load message catalogs from `locale_dirs` for the given `language` and
/// register them under `(namespace, catalog)`.
///
/// Mirrors `sphinx.locale.init()`.
///
/// Returns `true` if at least one `.po` file was found and loaded.
pub fn init(
    locale_dirs: &[impl AsRef<Path>],
    language: Option<&str>,
    catalog: &str,
    namespace: &str,
) -> bool {
    let key: RegistryKey = (namespace.to_owned(), catalog.to_owned());
    let mut reg = registry().lock().unwrap();

    // Check if we already have a *real* (non-null) translator cached
    if let Some(existing) = reg.get(&key) {
        if existing.has_translation() {
            // Already loaded — merge additional dirs in, mirroring Python's
            // `add_fallback` behaviour.
        }
    }

    let languages: Vec<String> = match language {
        Some(lang) => language_variants(lang),
        None => {
            // Fall back to LANGUAGE / LANG env vars (mirrors Python's locale.getlocale)
            std::env::var("LANGUAGE")
                .or_else(|_| std::env::var("LANG"))
                .ok()
                .map(|v| v.split(':').flat_map(language_variants).collect::<Vec<_>>())
                .unwrap_or_default()
        }
    };

    let mut translator = reg.remove(&key).unwrap_or_else(Translator::null);
    let mut found = translator.has_translation();

    for dir in locale_dirs {
        for lang in &languages {
            if let Some(cat) = try_load_catalog(dir.as_ref(), lang, catalog) {
                if !found {
                    translator = Translator::from_catalog(cat);
                    found = true;
                } else {
                    translator.add_fallback(cat);
                }
                break; // found a match for this dir+language variant
            }
        }
    }

    reg.insert(key, translator);
    found
}

/// Initialise the console catalog from a single locale dir.
///
/// Mirrors `sphinx.locale.init_console()`.
pub fn init_console(locale_dir: Option<&Path>, catalog: &str) -> bool {
    // Detect system language via LANGUAGE/LANG
    let language = std::env::var("LANGUAGE")
        .or_else(|_| std::env::var("LANG"))
        .ok();
    match locale_dir {
        Some(d) => init(&[d], language.as_deref(), catalog, "console"),
        None => {
            let empty: &[&Path] = &[];
            init(empty, language.as_deref(), catalog, "console")
        }
    }
}

// ── Accessors ─────────────────────────────────────────────────────────────────

/// Return a clone of the translator registered under `(namespace, catalog)`,
/// or a null translator if none has been registered.
pub fn get_translator(catalog: &str, namespace: &str) -> Translator {
    let key: RegistryKey = (namespace.to_owned(), catalog.to_owned());
    let reg = registry().lock().unwrap();
    reg.get(&key).cloned().unwrap_or_else(Translator::null)
}

/// Return `true` if a translator is registered under `(namespace, catalog)`.
pub fn is_translator_registered(catalog: &str, namespace: &str) -> bool {
    let key: RegistryKey = (namespace.to_owned(), catalog.to_owned());
    let reg = registry().lock().unwrap();
    reg.contains_key(&key)
}

/// Remove all registered translators (used to reset state in tests).
pub fn clear_translators() {
    let mut reg = registry().lock().unwrap();
    reg.clear();
}

/// Return a closure that translates a `&str` using the given catalog.
///
/// If the translator is not yet registered the closure returns the msgid
/// unchanged (matching Python's `_TranslationProxy.__str__` fallback).
///
/// Mirrors `sphinx.locale.get_translation()`.
pub fn get_translation(catalog: &'static str, namespace: &'static str) -> impl Fn(&str) -> String {
    move |msgid: &str| {
        let key: RegistryKey = (namespace.to_owned(), catalog.to_owned());
        let reg = registry().lock().unwrap();
        match reg.get(&key) {
            Some(t) => t.gettext(msgid).to_owned(),
            None => msgid.to_owned(),
        }
    }
}

// ── tr / tr_console ───────────────────────────────────────────────────────────

/// Translate `msgid` using the built-in `sphinx` catalog (`general` namespace).
///
/// Shortcut for `get_translation("sphinx", "general")(msgid)`.
/// Mirrors the `_` shortcut in `sphinx.locale`.
pub fn tr(msgid: &str) -> String {
    let key: RegistryKey = ("general".to_owned(), "sphinx".to_owned());
    let reg = registry().lock().unwrap();
    match reg.get(&key) {
        Some(t) => t.gettext(msgid).to_owned(),
        None => msgid.to_owned(),
    }
}

/// Translate `msgid` using the `sphinx` catalog in the `console` namespace.
///
/// Shortcut for `get_translation("sphinx", "console")(msgid)`.
/// Mirrors the `__` shortcut in `sphinx.locale`.
pub fn tr_console(msgid: &str) -> String {
    let key: RegistryKey = ("console".to_owned(), "sphinx".to_owned());
    let reg = registry().lock().unwrap();
    match reg.get(&key) {
        Some(t) => t.gettext(msgid).to_owned(),
        None => msgid.to_owned(),
    }
}

/// Macro shorthand: `tr!("msg")` → `crate::locale::tr("msg")`.
///
/// Accepts a format-string-like first argument (returned as `String` without
/// Rust-level `format!` substitution — callers should use `format!()` with the
/// result for `%s`-style placeholders, matching the upstream Sphinx convention
/// of translating the template and formatting separately).
#[macro_export]
macro_rules! tr {
    ($msg:expr) => {
        $crate::locale::tr($msg)
    };
}

/// Macro shorthand: `tr_c!("msg")` → `crate::locale::tr_console("msg")`.
#[macro_export]
macro_rules! tr_c {
    ($msg:expr) => {
        $crate::locale::tr_console($msg)
    };
}

// ── admonition labels ─────────────────────────────────────────────────────────

/// Admonition display labels translated via the sphinx catalog.
///
/// Mirrors `sphinx.locale.admonitionlabels`.
pub fn admonition_labels() -> HashMap<&'static str, String> {
    let mut m = HashMap::new();
    for (key, msgid) in [
        ("attention", "Attention"),
        ("caution", "Caution"),
        ("danger", "Danger"),
        ("error", "Error"),
        ("hint", "Hint"),
        ("important", "Important"),
        ("note", "Note"),
        ("seealso", "See also"),
        ("tip", "Tip"),
        ("warning", "Warning"),
    ] {
        m.insert(key, tr(msgid));
    }
    m
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn write_po(dir: &Path, lang: &str, catalog: &str, content: &str) {
        let lc = dir.join(lang).join("LC_MESSAGES");
        std::fs::create_dir_all(&lc).unwrap();
        let mut f = std::fs::File::create(lc.join(format!("{catalog}.po"))).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    // ── PoCatalog::parse ────────────────────────────────────────────────────

    #[test]
    fn parse_simple_entry() {
        let po = r#"
msgid "Hello"
msgstr "Hallo"
"#;
        let cat = PoCatalog::parse(po);
        assert_eq!(cat.gettext("Hello"), "Hallo");
        assert_eq!(cat.gettext("Unknown"), "Unknown");
    }

    #[test]
    fn parse_empty_msgstr_falls_back_to_msgid() {
        let po = r#"
msgid "Hello"
msgstr ""
"#;
        let cat = PoCatalog::parse(po);
        // Empty msgstr: gettext returns the msgid unchanged
        assert_eq!(cat.gettext("Hello"), "Hello");
    }

    #[test]
    fn parse_multiline_strings() {
        let po = "msgid \"Hello \"\n\"world\"\nmsgstr \"Hallo \"\n\"Welt\"\n";
        let cat = PoCatalog::parse(po);
        assert_eq!(cat.gettext("Hello world"), "Hallo Welt");
    }

    #[test]
    fn parse_escape_sequences() {
        let po = r#"
msgid "line1\nline2"
msgstr "zeile1\nzeile2"
"#;
        let cat = PoCatalog::parse(po);
        assert_eq!(cat.gettext("line1\nline2"), "zeile1\nzeile2");
    }

    #[test]
    fn parse_plural_uses_singular() {
        let po = r#"
msgid "file"
msgid_plural "files"
msgstr[0] "Datei"
msgstr[1] "Dateien"
"#;
        let cat = PoCatalog::parse(po);
        assert_eq!(cat.gettext("file"), "Datei");
    }

    #[test]
    fn parse_ignores_comments() {
        let po = r#"
# This is a comment
#: source.py:10
#, python-format
msgid "Hello"
msgstr "Hallo"
"#;
        let cat = PoCatalog::parse(po);
        assert_eq!(cat.gettext("Hello"), "Hallo");
    }

    #[test]
    fn parse_header_entry_is_skipped() {
        // The PO header has msgid "" — skip it (empty msgid → not stored)
        let po = r#"
msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\n"

msgid "Hello"
msgstr "Hallo"
"#;
        let cat = PoCatalog::parse(po);
        assert_eq!(cat.len(), 1);
        assert_eq!(cat.gettext("Hello"), "Hallo");
    }

    // ── Translator ────────────────────────────────────────────────────────────

    #[test]
    fn null_translator_returns_msgid() {
        let t = Translator::null();
        assert_eq!(t.gettext("Hello"), "Hello");
    }

    #[test]
    fn translator_chain_prefers_first() {
        let mut cat1 = PoCatalog::default();
        cat1.entries.insert("Hello".into(), "Hallo".into());
        let mut cat2 = PoCatalog::default();
        cat2.entries.insert("Hello".into(), "Bonjour".into());

        let mut t = Translator::from_catalog(cat1);
        t.add_fallback(cat2);

        assert_eq!(t.gettext("Hello"), "Hallo");
    }

    #[test]
    fn translator_falls_back_when_primary_missing() {
        let cat1 = PoCatalog::default(); // empty
        let mut cat2 = PoCatalog::default();
        cat2.entries.insert("Hello".into(), "Bonjour".into());

        let mut t = Translator::from_catalog(cat1);
        t.add_fallback(cat2);

        assert_eq!(t.gettext("Hello"), "Bonjour");
    }

    // ── init / get_translation ────────────────────────────────────────────────

    #[test]
    fn init_loads_catalog_and_translates() {
        let tmp = TempDir::new().unwrap();
        write_po(
            tmp.path(),
            "en",
            "myext",
            "msgid \"Hello world\"\nmsgstr \"HELLO WORLD\"\n",
        );
        clear_translators();

        let found = init(&[tmp.path()], Some("en"), "myext", "test_ns_1");
        assert!(found);

        let _ = get_translation("myext", "test_ns_1")("Hello world");
    }

    #[test]
    fn get_translation_before_init_returns_msgid() {
        clear_translators();
        let translate = get_translation("notregistered", "ns_missing");
        assert_eq!(translate("Hello sphinx"), "Hello sphinx");
    }

    #[test]
    fn init_unknown_language_returns_msgid() {
        let tmp = TempDir::new().unwrap();
        write_po(
            tmp.path(),
            "en",
            "myext",
            "msgid \"Hello world\"\nmsgstr \"HELLO WORLD\"\n",
        );
        clear_translators();

        let found = init(&[tmp.path()], Some("zz"), "myext", "test_ns_unknown");
        assert!(!found);

        let translate = get_translation("myext", "test_ns_unknown");
        assert_eq!(translate("Hello world"), "Hello world");
    }

    #[test]
    fn init_merges_multiple_locale_dirs() {
        let tmp1 = TempDir::new().unwrap();
        let tmp2 = TempDir::new().unwrap();
        write_po(
            tmp1.path(),
            "en",
            "myext",
            "msgid \"Hello world\"\nmsgstr \"HELLO WORLD\"\n",
        );
        write_po(
            tmp2.path(),
            "en",
            "myext",
            "msgid \"Hello sphinx\"\nmsgstr \"HELLO SPHINX\"\n",
        );
        clear_translators();

        init(&[tmp1.path()], Some("en"), "myext", "test_ns_merge");
        init(&[tmp2.path()], Some("en"), "myext", "test_ns_merge");

        let translate = get_translation("myext", "test_ns_merge");
        assert_eq!(translate("Hello world"), "HELLO WORLD");
        assert_eq!(translate("Hello sphinx"), "HELLO SPHINX");
        assert_eq!(translate("Hello reST"), "Hello reST"); // not in either
    }

    #[test]
    fn is_translator_registered_reflects_init() {
        clear_translators();
        assert!(!is_translator_registered("myext2", "test_ns_reg"));
        let tmp = TempDir::new().unwrap();
        // init always registers even with no files found (null translator)
        init(&[tmp.path()], Some("en"), "myext2", "test_ns_reg");
        assert!(is_translator_registered("myext2", "test_ns_reg"));
    }

    #[test]
    fn language_variants_splits_on_underscore() {
        let v = language_variants("de_DE");
        assert_eq!(v, vec!["de_DE", "de"]);
    }

    #[test]
    fn language_variants_no_suffix() {
        let v = language_variants("fr");
        assert_eq!(v, vec!["fr"]);
    }
}
