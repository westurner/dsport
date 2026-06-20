//! `sphinxdocrs::registry` — Rust port of `sphinx.registry.SphinxComponentRegistry`.
//!
//! **P2 subset**: pure data-structure operations that carry no builder,
//! domain, or environment dependency:
//!
//! * Source suffix / parser registration (`add_source_suffix`,
//!   `add_source_parser`, `get_source_parser`, `get_source_parsers`).
//! * Transform / post-transform lists.
//! * Asset registration: CSS files, JS files, static directories.
//! * LaTeX package management.
//! * HTML theme registration.
//!
//! **Deferred to P3** (requires builders / domains / translators): the
//! `builders`, `domains`, `translators`, `translation_handlers`,
//! `enumerable_nodes`, `html_math_renderers` maps; `load_extension`;
//! `create_source_parser`; `create_builder`; `create_domains`.
//!
//! The Rust `source_parsers` map stores **parser class names** (`String`)
//! rather than Python class objects, because we don't have Python class
//! types at this level. The PyO3 bridge (P3) will resolve class names to
//! actual Python types when constructing a parser instance.

use std::collections::HashMap;
use std::path::PathBuf;

// ── errors ────────────────────────────────────────────────────────────────────

/// Errors raised by registry operations.
///
/// Mirrors the `sphinx.errors.ExtensionError` / `SphinxError` surface seen
/// from `SphinxComponentRegistry`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RegistryError {
    /// A registration attempted to overwrite an existing entry without the
    /// `override` flag — matches upstream `ExtensionError` messages.
    #[error("{0}")]
    Duplicate(String),
    /// A look-up failed because the key was never registered.
    #[error("{0}")]
    NotFound(String),
    /// A warning-level event (not fatal — upstream logs a `logger.warning`).
    #[error("{0}")]
    Warning(String),
}

// ── CSS / JS asset records ────────────────────────────────────────────────────

/// A single CSS file entry: `(filename, attributes)`.
///
/// Mirrors upstream `css_files: list[tuple[str, dict[str, Any]]]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssFile {
    pub filename: String,
    pub attributes: HashMap<String, String>,
}

/// A single JS file entry: `(filename, attributes)`.
///
/// Mirrors upstream `js_files: list[tuple[str | None, dict[str, Any]]]`.
/// The filename may be `None` for inline script entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsFile {
    pub filename: Option<String>,
    pub attributes: HashMap<String, String>,
}

// ── SphinxComponentRegistry ───────────────────────────────────────────────────

/// Rust port of `sphinx.registry.SphinxComponentRegistry` — P2 subset.
///
/// All fields are plain Rust types so this struct can be created without
/// an active Python interpreter.
#[derive(Debug, Default)]
pub struct SphinxComponentRegistry {
    // ── source suffix / parser ───────────────────────────────────────────────
    /// Maps file suffix (e.g. `".rst"`) to filetype string
    /// (e.g. `"restructuredtext"`).
    ///
    /// Mirrors `source_suffix: dict[str, str]`.
    pub source_suffix: HashMap<String, String>,

    /// Maps filetype string to parser class name.
    ///
    /// In Python this stores the actual `type[Parser]` object.  In the P2
    /// Rust port we store the fully-qualified class name
    /// (e.g. `"sphinx.parsers.RSTParser"`) and resolve it via PyO3 in P3.
    ///
    /// Mirrors `source_parsers: dict[str, type[Parser]]`.
    pub source_parsers: HashMap<String, String>,

    // ── transforms ───────────────────────────────────────────────────────────
    /// Additional transforms — ordered list of transform class names.
    ///
    /// Mirrors `transforms: list[type[Transform]]`.
    pub transforms: Vec<String>,

    /// Post-transforms — applied after the builder's writing phase.
    ///
    /// Mirrors `post_transforms: list[type[Transform]]`.
    pub post_transforms: Vec<String>,

    // ── HTML / JS / CSS assets ────────────────────────────────────────────────
    /// CSS files added by extensions.
    pub css_files: Vec<CssFile>,

    /// JS files added by extensions.
    pub js_files: Vec<JsFile>,

    /// Static source directories registered by extensions.
    pub static_dirs: Vec<PathBuf>,

    // ── LaTeX packages ────────────────────────────────────────────────────────
    /// LaTeX packages added before `hyperref`.
    ///
    /// Each entry is `(package_name, options_string | None)`.
    pub latex_packages: Vec<(String, Option<String>)>,

    /// LaTeX packages added after `hyperref`.
    pub latex_packages_after_hyperref: Vec<(String, Option<String>)>,

    // ── HTML themes ───────────────────────────────────────────────────────────
    /// HTML themes: name → filesystem path.
    ///
    /// Mirrors `html_themes: dict[str, str]`.
    pub html_themes: HashMap<String, PathBuf>,
}

impl SphinxComponentRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    // ── source suffix ─────────────────────────────────────────────────────────

    /// Register a source-file suffix mapping.
    ///
    /// Mirrors `SphinxComponentRegistry.add_source_suffix(suffix, filetype, override)`.
    ///
    /// # Errors
    /// Returns `RegistryError::Duplicate` when `suffix` is already registered
    /// and `override_` is `false`.
    pub fn add_source_suffix(
        &mut self,
        suffix: impl Into<String>,
        filetype: impl Into<String>,
        override_: bool,
    ) -> Result<(), RegistryError> {
        let suffix = suffix.into();
        let filetype = filetype.into();
        if self.source_suffix.contains_key(&suffix) && !override_ {
            return Err(RegistryError::Duplicate(format!(
                "source_suffix {suffix:?} is already registered"
            )));
        }
        self.source_suffix.insert(suffix, filetype);
        Ok(())
    }

    // ── source parser ─────────────────────────────────────────────────────────

    /// Register a parser for one or more file types.
    ///
    /// In Python, `add_source_parser(ParserClass)` reads `ParserClass.supported`
    /// to get the list of supported filetypes.  In this Rust P2 port the caller
    /// supplies the filetypes explicitly (since we don't load Python classes).
    ///
    /// Mirrors `SphinxComponentRegistry.add_source_parser(parser, override)`.
    ///
    /// # Errors
    /// Returns `RegistryError::Duplicate` if any filetype is already registered
    /// and `override_` is `false`.
    pub fn add_source_parser(
        &mut self,
        parser_name: impl Into<String>,
        filetypes: &[impl AsRef<str>],
        override_: bool,
    ) -> Result<(), RegistryError> {
        let parser_name = parser_name.into();
        for ft in filetypes {
            let ft = ft.as_ref();
            if self.source_parsers.contains_key(ft) && !override_ {
                return Err(RegistryError::Duplicate(format!(
                    "source_parser for {ft:?} is already registered"
                )));
            }
            self.source_parsers
                .insert(ft.to_string(), parser_name.clone());
        }
        Ok(())
    }

    /// Look up the parser class name for a given filetype.
    ///
    /// Mirrors `SphinxComponentRegistry.get_source_parser(filetype)`.
    ///
    /// # Errors
    /// Returns `RegistryError::NotFound` when no parser is registered.
    pub fn get_source_parser(&self, filetype: &str) -> Result<&str, RegistryError> {
        self.source_parsers
            .get(filetype)
            .map(String::as_str)
            .ok_or_else(|| {
                RegistryError::NotFound(format!("Source parser for {filetype} not registered"))
            })
    }

    /// Return a reference to the full source-parser map.
    ///
    /// Mirrors `SphinxComponentRegistry.get_source_parsers()`.
    pub fn get_source_parsers(&self) -> &HashMap<String, String> {
        &self.source_parsers
    }

    // ── transforms ────────────────────────────────────────────────────────────

    /// Append a transform to the transform list.
    ///
    /// Mirrors `SphinxComponentRegistry.add_transform(transform)`.
    pub fn add_transform(&mut self, transform_name: impl Into<String>) {
        self.transforms.push(transform_name.into());
    }

    /// Return a slice of registered transform class names.
    ///
    /// Mirrors `SphinxComponentRegistry.get_transforms()`.
    pub fn get_transforms(&self) -> &[String] {
        &self.transforms
    }

    /// Append a post-transform to the post-transform list.
    ///
    /// Mirrors `SphinxComponentRegistry.add_post_transform(transform)`.
    pub fn add_post_transform(&mut self, transform_name: impl Into<String>) {
        self.post_transforms.push(transform_name.into());
    }

    /// Return a slice of registered post-transform class names.
    ///
    /// Mirrors `SphinxComponentRegistry.get_post_transforms()`.
    pub fn get_post_transforms(&self) -> &[String] {
        &self.post_transforms
    }

    // ── CSS / JS / static assets ──────────────────────────────────────────────

    /// Add a CSS file.
    ///
    /// `attributes` is a `key → value` map of HTML `<link>` attributes
    /// (e.g. `{"media": "print"}`).
    ///
    /// Mirrors `SphinxComponentRegistry.add_css_files(filename, **attributes)`.
    pub fn add_css_file(
        &mut self,
        filename: impl Into<String>,
        attributes: HashMap<String, String>,
    ) {
        self.css_files.push(CssFile {
            filename: filename.into(),
            attributes,
        });
    }

    /// Add a JS file.
    ///
    /// `filename` may be `None` for an inline script blob (matches upstream
    /// `js_files: list[tuple[str | None, dict]]`).
    ///
    /// Mirrors `SphinxComponentRegistry.add_js_file(filename, **attributes)`.
    pub fn add_js_file(
        &mut self,
        filename: Option<impl Into<String>>,
        attributes: HashMap<String, String>,
    ) {
        self.js_files.push(JsFile {
            filename: filename.map(Into::into),
            attributes,
        });
    }

    /// Register a static source directory.
    ///
    /// Mirrors `SphinxComponentRegistry.add_static_dir(path)`.
    pub fn add_static_dir(&mut self, path: impl Into<PathBuf>) {
        self.static_dirs.push(path.into());
    }

    // ── LaTeX packages ────────────────────────────────────────────────────────

    /// Return `true` if a LaTeX package with `name` has already been added
    /// (in either the normal or after-hyperref list).
    ///
    /// Mirrors `SphinxComponentRegistry.has_latex_package(name)`.
    pub fn has_latex_package(&self, name: &str) -> bool {
        self.latex_packages
            .iter()
            .chain(&self.latex_packages_after_hyperref)
            .any(|(n, _)| n == name)
    }

    /// Register a LaTeX package.
    ///
    /// Logs a `RegistryError::Warning` (non-fatal) if the package is already
    /// present — matches upstream `logger.warning`.  The warning is returned
    /// so callers can forward it to a log sink; it is not an error.
    ///
    /// Mirrors `SphinxComponentRegistry.add_latex_package(name, options, after_hyperref)`.
    pub fn add_latex_package(
        &mut self,
        name: impl Into<String>,
        options: Option<impl Into<String>>,
        after_hyperref: bool,
    ) -> Option<RegistryError> {
        let name = name.into();
        let options = options.map(Into::into);
        let warn = if self.has_latex_package(&name) {
            Some(RegistryError::Warning(format!(
                "latex package {name:?} already included"
            )))
        } else {
            None
        };
        if after_hyperref {
            self.latex_packages_after_hyperref.push((name, options));
        } else {
            self.latex_packages.push((name, options));
        }
        warn
    }

    // ── HTML themes ───────────────────────────────────────────────────────────

    /// Register an HTML theme.
    ///
    /// Mirrors `SphinxComponentRegistry.add_html_theme(name, theme_path)`.
    pub fn add_html_theme(&mut self, name: impl Into<String>, theme_path: impl Into<PathBuf>) {
        self.html_themes.insert(name.into(), theme_path.into());
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn reg() -> SphinxComponentRegistry {
        SphinxComponentRegistry::new()
    }

    // ── source_suffix ─────────────────────────────────────────────────────────

    #[test]
    fn add_source_suffix_basic() {
        let mut r = reg();
        r.add_source_suffix(".rst", "restructuredtext", false)
            .unwrap();
        assert_eq!(r.source_suffix[".rst"], "restructuredtext");
    }

    #[test]
    fn add_source_suffix_duplicate_errors() {
        let mut r = reg();
        r.add_source_suffix(".rst", "restructuredtext", false)
            .unwrap();
        let err = r.add_source_suffix(".rst", "other", false).unwrap_err();
        assert!(matches!(err, RegistryError::Duplicate(_)));
        assert!(err.to_string().contains(".rst"));
    }

    #[test]
    fn add_source_suffix_override_replaces() {
        let mut r = reg();
        r.add_source_suffix(".rst", "restructuredtext", false)
            .unwrap();
        r.add_source_suffix(".rst", "myst", true).unwrap();
        assert_eq!(r.source_suffix[".rst"], "myst");
    }

    #[test]
    fn add_source_suffix_multiple_distinct() {
        let mut r = reg();
        r.add_source_suffix(".rst", "restructuredtext", false)
            .unwrap();
        r.add_source_suffix(".md", "myst", false).unwrap();
        assert_eq!(r.source_suffix.len(), 2);
    }

    // ── source_parser ─────────────────────────────────────────────────────────

    #[test]
    fn add_get_source_parser_basic() {
        let mut r = reg();
        r.add_source_parser("sphinx.parsers.RSTParser", &["restructuredtext"], false)
            .unwrap();
        assert_eq!(
            r.get_source_parser("restructuredtext").unwrap(),
            "sphinx.parsers.RSTParser"
        );
    }

    #[test]
    fn add_source_parser_multiple_filetypes() {
        let mut r = reg();
        r.add_source_parser("myst_parser.mdit.Parser", &["myst", "markdown"], false)
            .unwrap();
        assert_eq!(
            r.get_source_parser("myst").unwrap(),
            "myst_parser.mdit.Parser"
        );
        assert_eq!(
            r.get_source_parser("markdown").unwrap(),
            "myst_parser.mdit.Parser"
        );
    }

    #[test]
    fn add_source_parser_duplicate_errors() {
        let mut r = reg();
        r.add_source_parser("sphinx.parsers.RSTParser", &["restructuredtext"], false)
            .unwrap();
        let err = r
            .add_source_parser("other.Parser", &["restructuredtext"], false)
            .unwrap_err();
        assert!(matches!(err, RegistryError::Duplicate(_)));
    }

    #[test]
    fn add_source_parser_override_replaces() {
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
    fn get_source_parser_not_found() {
        let r = reg();
        let err = r.get_source_parser("notype").unwrap_err();
        assert!(matches!(err, RegistryError::NotFound(_)));
        assert!(err.to_string().contains("notype"));
    }

    #[test]
    fn get_source_parsers_returns_map() {
        let mut r = reg();
        r.add_source_parser("sphinx.parsers.RSTParser", &["restructuredtext"], false)
            .unwrap();
        let parsers = r.get_source_parsers();
        assert!(parsers.contains_key("restructuredtext"));
    }

    // ── transforms ───────────────────────────────────────────────────────────

    #[test]
    fn add_get_transforms() {
        let mut r = reg();
        assert!(r.get_transforms().is_empty());
        r.add_transform("sphinx.transforms.compact_bullet_list.RefOnlyBulletListTransform");
        r.add_transform("sphinx.transforms.i18n.Locale");
        let ts = r.get_transforms();
        assert_eq!(ts.len(), 2);
        assert!(ts[0].contains("RefOnly"));
        assert!(ts[1].contains("Locale"));
    }

    #[test]
    fn add_get_post_transforms() {
        let mut r = reg();
        assert!(r.get_post_transforms().is_empty());
        r.add_post_transform("sphinx.transforms.post_transforms.images.ImageConverter");
        let pts = r.get_post_transforms();
        assert_eq!(pts.len(), 1);
        assert!(pts[0].contains("ImageConverter"));
    }

    #[test]
    fn transforms_and_post_transforms_are_independent() {
        let mut r = reg();
        r.add_transform("MyTransform");
        r.add_post_transform("MyPostTransform");
        assert_eq!(r.get_transforms().len(), 1);
        assert_eq!(r.get_post_transforms().len(), 1);
        assert_ne!(r.get_transforms()[0], r.get_post_transforms()[0]);
    }

    // ── CSS / JS / static assets ──────────────────────────────────────────────

    #[test]
    fn add_css_file() {
        let mut r = reg();
        r.add_css_file("custom.css", HashMap::new());
        assert_eq!(r.css_files.len(), 1);
        assert_eq!(r.css_files[0].filename, "custom.css");
    }

    #[test]
    fn add_css_file_with_attributes() {
        let mut r = reg();
        let attrs = HashMap::from([("media".to_string(), "print".to_string())]);
        r.add_css_file("print.css", attrs.clone());
        assert_eq!(r.css_files[0].attributes, attrs);
    }

    #[test]
    fn add_js_file_named() {
        let mut r = reg();
        r.add_js_file(Some("custom.js"), HashMap::new());
        assert_eq!(r.js_files[0].filename, Some("custom.js".to_string()));
    }

    #[test]
    fn add_js_file_none_filename() {
        let mut r = reg();
        r.add_js_file(None::<&str>, HashMap::new());
        assert_eq!(r.js_files[0].filename, None);
    }

    #[test]
    fn add_static_dir() {
        let mut r = reg();
        r.add_static_dir("/ext/static");
        assert_eq!(r.static_dirs.len(), 1);
        assert_eq!(r.static_dirs[0], PathBuf::from("/ext/static"));
    }

    // ── LaTeX packages ────────────────────────────────────────────────────────

    #[test]
    fn add_latex_package_basic() {
        let mut r = reg();
        let warn = r.add_latex_package("amsmath", None::<&str>, false);
        assert!(warn.is_none());
        assert!(r.has_latex_package("amsmath"));
        assert_eq!(r.latex_packages[0], ("amsmath".to_string(), None));
    }

    #[test]
    fn add_latex_package_with_options() {
        let mut r = reg();
        r.add_latex_package("geometry", Some("margin=1in"), false);
        assert_eq!(
            r.latex_packages[0],
            ("geometry".to_string(), Some("margin=1in".to_string()))
        );
    }

    #[test]
    fn add_latex_package_after_hyperref() {
        let mut r = reg();
        r.add_latex_package("hyperref", None::<&str>, false);
        r.add_latex_package("bookmark", None::<&str>, true);
        assert_eq!(r.latex_packages.len(), 1);
        assert_eq!(r.latex_packages_after_hyperref.len(), 1);
        assert_eq!(r.latex_packages_after_hyperref[0].0, "bookmark");
    }

    #[test]
    fn add_latex_package_duplicate_warns() {
        let mut r = reg();
        r.add_latex_package("amsmath", None::<&str>, false);
        let warn = r.add_latex_package("amsmath", None::<&str>, false);
        assert!(warn.is_some());
        assert!(matches!(warn.unwrap(), RegistryError::Warning(_)));
        // Both entries present (upstream does not deduplicate)
        assert_eq!(r.latex_packages.len(), 2);
    }

    #[test]
    fn has_latex_package_false_when_absent() {
        let r = reg();
        assert!(!r.has_latex_package("nonexistent"));
    }

    #[test]
    fn has_latex_package_true_after_hyperref() {
        let mut r = reg();
        r.add_latex_package("bookmark", None::<&str>, true);
        assert!(r.has_latex_package("bookmark"));
    }

    // ── HTML themes ───────────────────────────────────────────────────────────

    #[test]
    fn add_html_theme() {
        let mut r = reg();
        r.add_html_theme("mytheme", "/ext/themes/mytheme");
        assert_eq!(
            r.html_themes["mytheme"],
            PathBuf::from("/ext/themes/mytheme")
        );
    }

    #[test]
    fn add_html_theme_replaces() {
        let mut r = reg();
        r.add_html_theme("t", "/old");
        r.add_html_theme("t", "/new");
        assert_eq!(r.html_themes["t"], PathBuf::from("/new"));
    }

    // ── Default / new ────────────────────────────────────────────────────────

    #[test]
    fn new_registry_is_empty() {
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
}
