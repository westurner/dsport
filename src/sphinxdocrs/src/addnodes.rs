//! `sphinxdocrs::addnodes` — Rust port of `sphinx.addnodes`.
//!
//! Document tree nodes that Sphinx defines on top of those in docutils.
//!
//! ## What is ported
//!
//! | upstream class | Rust type | notes |
//! | --- | --- | --- |
//! | `translatable` | [`Translatable`] | trait for translatable nodes |
//! | `not_smartquotable` | [`NotSmartquotable`] | marker trait |
//! | `toctree` | [`Toctree`] | TOC tree with entries, caption, glob, etc. |
//! | `desc` | [`Desc`] | object description container |
//! | `desc_signature` | [`DescSignature`] | single signature |
//! | `desc_signature_line` | [`DescSignatureLine`] | line in multiline sig |
//! | `desc_content` | [`DescContent`] | description body |
//! | `desc_inline` | [`DescInline`] | inline signature fragment |
//! | `desc_name` | [`DescName`] | main object name |
//! | `desc_addname` | [`DescAddname`] | additional name prefix |
//! | `desc_type` | [`DescType`] | return type / object type |
//! | `desc_returns` | [`DescReturns`] | return annotation (`->`) |
//! | `desc_parameterlist` | [`DescParameterList`] | parameter list |
//! | `desc_type_parameter_list` | [`DescTypeParameterList`] | type param list |
//! | `desc_parameter` | [`DescParameter`] | single parameter |
//! | `desc_type_parameter` | [`DescTypeParameter`] | single type parameter |
//! | `desc_optional` | [`DescOptional`] | optional parameter group |
//! | `desc_annotation` | [`DescAnnotation`] | non-PEP-3107 annotation |
//! | `desc_sig_element` base | [`DescSigElement`] | sig leaf node base |
//! | `desc_sig_space` | [`DescSigSpace`] | whitespace in sig |
//! | `desc_sig_name` | [`DescSigName`] | identifier in sig |
//! | `desc_sig_operator` | [`DescSigOperator`] | operator in sig |
//! | `desc_sig_punctuation` | [`DescSigPunctuation`] | punctuation in sig |
//! | `desc_sig_keyword` | [`DescSigKeyword`] | keyword in sig |
//! | `desc_sig_keyword_type` | [`DescSigKeywordType`] | builtin-type keyword |
//! | `desc_sig_literal_number` | [`DescSigLiteralNumber`] | numeric literal |
//! | `desc_sig_literal_string` | [`DescSigLiteralString`] | string literal |
//! | `desc_sig_literal_char` | [`DescSigLiteralChar`] | char literal |
//! | `SIG_ELEMENTS` | [`SIG_ELEMENTS`] | set of `_sig_element=True` names |
//! | `versionmodified` | [`VersionModified`] | added/changed/deprecated/removed |
//! | `seealso` | [`SeeAlso`] | "see also" admonition |
//! | `productionlist` | [`ProductionList`] | grammar production list |
//! | `production` | [`Production`] | single grammar production rule |
//! | `index` | [`Index`] | index entry node |
//! | `centered` | [`Centered`] | deprecated centered-text node |
//! | `acks` | [`Acks`] | special "acks" list node |
//! | `hlist` | [`Hlist`] | horizontal list |
//! | `hlistcol` | [`HlistCol`] | column in a horizontal list |
//! | `compact_paragraph` | [`CompactParagraph`] | compact paragraph |
//! | `glossary` | [`Glossary`] | glossary node |
//! | `only` | [`Only`] | conditional-inclusion node |
//! | `start_of_file` | [`StartOfFile`] | marks start of a new file |
//! | `highlightlang` | [`HighlightLang`] | highlight language settings |
//! | `tabular_col_spec` | [`TabularColSpec`] | LaTeX tabular columns |
//! | `pending_xref` | [`PendingXref`] | unresolved cross-reference |
//! | `pending_xref_condition` | [`PendingXrefCondition`] | conditional xref |
//! | `number_reference` | [`NumberReference`] | number-based reference |
//! | `download_reference` | [`DownloadReference`] | download reference |
//! | `literal_emphasis` | [`LiteralEmphasis`] | emphasis without smartquotes |
//! | `literal_strong` | [`LiteralStrong`] | strong without smartquotes |
//! | `manpage` | [`Manpage`] | manpage reference |
//!
//! Trait obligations (`preserve_original_messages`, `apply_translated_message`,
//! `extract_original_messages`) are modelled on the [`Translatable`] trait.

#[cfg(test)]
use std::collections::HashSet;
// ── SIG_ELEMENTS registry ─────────────────────────────────────────────────────

/// Names of all `desc_sig_element` subclasses that registered with
/// `_sig_element=True`.
///
/// Mirrors `sphinx.addnodes.SIG_ELEMENTS`.
pub static SIG_ELEMENTS: &[&str] = &[
    "desc_sig_space",
    "desc_sig_name",
    "desc_sig_operator",
    "desc_sig_punctuation",
    "desc_sig_keyword",
    "desc_sig_keyword_type",
    "desc_sig_literal_number",
    "desc_sig_literal_string",
    "desc_sig_literal_char",
];

// ── Traits ────────────────────────────────────────────────────────────────────

/// Marker trait for nodes that support message translation.
///
/// Mirrors the `translatable` abstract base class in `sphinx.addnodes`.
pub trait Translatable {
    /// Preserve original translatable messages.
    fn preserve_original_messages(&mut self);
    /// Apply a translated message in place of its original.
    fn apply_translated_message(&mut self, original: &str, translated: &str);
    /// Return the list of translatable original messages.
    fn extract_original_messages(&self) -> Vec<String>;
}

/// Marker trait for nodes that do not support smart-quotes.
///
/// Mirrors `not_smartquotable.support_smartquotes = False`.
pub trait NotSmartquotable {
    fn support_smartquotes() -> bool {
        false
    }
}

// ── toctree ───────────────────────────────────────────────────────────────────

/// Node for inserting a "TOC tree" (the `.. toctree::` directive).
///
/// Mirrors `sphinx.addnodes.toctree`.
///
/// `entries` is a list of `(title, docname)` pairs where `title` may be
/// empty when the document's own title should be used.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Toctree {
    /// `[(title, docname)]`
    pub entries: Vec<(String, String)>,
    /// Optional caption.
    pub caption: Option<String>,
    /// Whether glob patterns are allowed.
    pub glob: bool,
    /// Include hidden entries.
    pub hidden: bool,
    /// Maximum depth (`-1` = unlimited).
    pub maxdepth: i32,
    /// Numbered entries (depth, `0` = no numbering).
    pub numbered: i32,
    /// Include only directives.
    pub includefiles: Vec<String>,
    /// Original (pre-translation) entries, preserved for gettext.
    pub rawentries: Vec<String>,
    /// Original caption before translation.
    pub rawcaption: Option<String>,
}

impl Translatable for Toctree {
    fn preserve_original_messages(&mut self) {
        self.rawentries.clear();
        for (title, _) in &self.entries {
            if !title.is_empty() {
                self.rawentries.push(title.clone());
            }
        }
        if let Some(cap) = &self.caption {
            self.rawcaption = Some(cap.clone());
        }
    }

    fn apply_translated_message(&mut self, original: &str, translated: &str) {
        for (title, _) in &mut self.entries {
            if title == original {
                *title = translated.to_string();
            }
        }
        if self.rawcaption.as_deref() == Some(original) {
            self.caption = Some(translated.to_string());
        }
    }

    fn extract_original_messages(&self) -> Vec<String> {
        let mut msgs = self.rawentries.clone();
        if let Some(rc) = &self.rawcaption {
            msgs.push(rc.clone());
        }
        msgs
    }
}

// ── desc ──────────────────────────────────────────────────────────────────────

/// Node for a list of object signatures and a common description.
///
/// Mirrors `sphinx.addnodes.desc`. Always carries two classes:
/// the domain name and the object type.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Desc {
    /// Domain name (e.g. `"py"`, `"c"`).
    pub domain: String,
    /// Object type within the domain (e.g. `"function"`, `"class"`).
    pub objtype: String,
    pub no_index: bool,
    pub no_index_entry: bool,
    pub no_contents_entry: bool,
    pub no_typesetting: bool,
    pub noindex: bool,
    pub noindexentry: bool,
    pub nocontentsentry: bool,
}

/// Node for a single object signature.
///
/// Mirrors `sphinx.addnodes.desc_signature`.
/// Always carries classes `["sig", "sig-object"]` plus the domain.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescSignature {
    /// The IDs attached to this signature (for HTML anchors).
    pub ids: Vec<String>,
    /// The names attached to this signature.
    pub names: Vec<String>,
    /// If `true`, this is a multi-line signature (children are
    /// `DescSignatureLine` nodes).
    pub is_multiline: bool,
    /// Child text separator for multi-line mode.
    pub domain: String,
}

/// Node for one line of a multi-line object signature.
///
/// Mirrors `sphinx.addnodes.desc_signature_line`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescSignatureLine {
    /// E.g. `"overload"`, `"first"`, etc.
    pub sphinx_line_type: String,
    /// Whether this line gets the permalink.
    pub add_permalink: bool,
}

/// Node for object description content (the body after the signature).
///
/// Mirrors `sphinx.addnodes.desc_content`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescContent;

/// Node for an inline signature fragment.
///
/// Mirrors `sphinx.addnodes.desc_inline`. Always carries
/// classes `["sig", "sig-inline"]` plus the domain.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescInline {
    pub domain: String,
}

// ── desc_* detail nodes ───────────────────────────────────────────────────────

/// Main object name within a signature.
///
/// Mirrors `desc_name`. Classes: `["sig-name", "descname"]`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescName {
    pub text: String,
}

/// Additional name prefix (module qualifier).
///
/// Mirrors `desc_addname`. Classes: `["sig-prename", "descclassname"]`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescAddname {
    pub text: String,
}

/// Type annotation or object type name.
///
/// Mirrors `desc_type`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescType {
    pub text: String,
}

/// Return-type annotation (`->` prefix on `astext`).
///
/// Mirrors `desc_returns`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescReturns {
    pub text: String,
}

impl DescReturns {
    /// Return the text with a `" -> "` prefix, mirroring `astext()`.
    pub fn astext(&self) -> String {
        format!(" -> {}", self.text)
    }
}

/// General parameter list.
///
/// Mirrors `desc_parameterlist`.
/// `child_text_separator = ", "`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescParameterList {
    pub multi_line_parameter_list: bool,
    pub multi_line_trailing_comma: bool,
}

/// Type parameter list (`[T, U, ...]`).
///
/// Mirrors `desc_type_parameter_list`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescTypeParameterList {
    pub multi_line_parameter_list: bool,
    pub multi_line_trailing_comma: bool,
}

/// A single parameter node.
///
/// Mirrors `desc_parameter`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescParameter {
    pub text: String,
}

/// A single type-parameter node.
///
/// Mirrors `desc_type_parameter`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescTypeParameter {
    pub text: String,
}

/// Optional parameter group (`[...]`).
///
/// Mirrors `desc_optional`. `child_text_separator = ", "`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescOptional {
    pub text: String,
}

impl DescOptional {
    /// Return the text wrapped in `[` `]`, mirroring `astext()`.
    pub fn astext(&self) -> String {
        format!("[{}]", self.text)
    }
}

/// Non-PEP-3107 annotation (e.g. C storage class).
///
/// Mirrors `desc_annotation`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescAnnotation {
    pub text: String,
}

// ── desc_sig_element leaf nodes ───────────────────────────────────────────────

/// Shared data for all `desc_sig_element` leaf nodes.
///
/// Mirrors the common attributes of `sphinx.addnodes.desc_sig_element`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescSigElement {
    pub rawsource: String,
    pub text: String,
    pub classes: Vec<String>,
}

// Concrete desc_sig_* types follow the same pattern: a newtype around
// DescSigElement.  We use a macro to avoid repetition.

macro_rules! desc_sig_node {
    ($(#[$m:meta])* $name:ident, $classes:expr) => {
        $(#[$m])*
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $name(pub DescSigElement);

        impl $name {
            /// Construct with text.
            pub fn new(text: impl Into<String>) -> Self {
                let text = text.into();
                Self(DescSigElement {
                    rawsource: text.clone(),
                    text,
                    classes: $classes.iter().map(|s: &&str| s.to_string()).collect(),
                })
            }
            /// Return the display text.
            pub fn text(&self) -> &str { &self.0.text }
            /// CSS classes.
            pub fn classes(&self) -> &[String] { &self.0.classes }
        }
    };
}

desc_sig_node!(
    /// Whitespace in a signature. Default text is `" "`.
    DescSigSpace, &["w"]
);
desc_sig_node!(
    /// Identifier in a signature.
    DescSigName, &["n"]
);
desc_sig_node!(
    /// Operator in a signature.
    DescSigOperator, &["o"]
);
desc_sig_node!(
    /// Punctuation in a signature.
    DescSigPunctuation, &["p"]
);
desc_sig_node!(
    /// General keyword in a signature.
    DescSigKeyword, &["k"]
);
desc_sig_node!(
    /// Built-in type keyword in a signature.
    DescSigKeywordType, &["kt"]
);
desc_sig_node!(
    /// Numeric literal in a signature.
    DescSigLiteralNumber, &["m"]
);
desc_sig_node!(
    /// String literal in a signature.
    DescSigLiteralString, &["s"]
);
desc_sig_node!(
    /// Character literal in a signature.
    DescSigLiteralChar, &["sc"]
);

// ── Admonition-like nodes ─────────────────────────────────────────────────────

/// Node for version-change entries.
///
/// Used by `versionadded`, `versionchanged`, `deprecated`, `versionremoved`.
///
/// Mirrors `sphinx.addnodes.versionmodified`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VersionModified {
    /// `"added"`, `"changed"`, `"deprecated"`, or `"removed"`.
    pub kind: String,
    /// Version string.
    pub version: String,
}

/// Custom "see also" admonition.
///
/// Mirrors `sphinx.addnodes.seealso`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SeeAlso;

/// Node for grammar production lists (the `.. productionlist::` directive).
///
/// Mirrors `sphinx.addnodes.productionlist`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProductionList {
    /// Optional name prefix for all productions.
    pub tokenprefix: String,
}

/// A single grammar production rule inside a [`ProductionList`].
///
/// Mirrors `sphinx.addnodes.production`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Production {
    /// Nonterminal token name (LHS).
    pub tokenname: String,
    /// Production text (RHS).
    pub text: String,
}

// ── Other directive-level nodes ───────────────────────────────────────────────

/// Node for index entries.
///
/// `entries` is a list of `(entrytype, entryname, target, ignored, key)`.
///
/// Mirrors `sphinx.addnodes.index`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Index {
    /// Each element: `(entrytype, entryname, target, ignored, key)`.
    pub entries: Vec<(String, String, String, String, Option<String>)>,
}

/// Deprecated centered-text node.
///
/// Mirrors `sphinx.addnodes.centered`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Centered {
    pub text: String,
}

/// Special "acks" list node.
///
/// Mirrors `sphinx.addnodes.acks`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Acks;

/// Node for horizontal lists.
///
/// Mirrors `sphinx.addnodes.hlist`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hlist {
    pub ncolumns: usize,
}

/// One column in a horizontal list.
///
/// Mirrors `sphinx.addnodes.hlistcol`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HlistCol;

/// Compact paragraph (never generates a `<p>` element).
///
/// Mirrors `sphinx.addnodes.compact_paragraph`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CompactParagraph {
    pub text: String,
}

/// Glossary node.
///
/// Mirrors `sphinx.addnodes.glossary`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Glossary;

/// Conditional-inclusion node (`.. only:: tag`).
///
/// Mirrors `sphinx.addnodes.only`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Only {
    /// Tag expression (e.g. `"html"`, `"latex or html"`).
    pub expr: String,
}

// ── Meta-information nodes ────────────────────────────────────────────────────

/// Marks the start of a new source file (LaTeX builder uses this).
///
/// Mirrors `sphinx.addnodes.start_of_file`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StartOfFile {
    pub docname: String,
}

/// Sets highlight language and line-number options for subsequent code blocks.
///
/// Mirrors `sphinx.addnodes.highlightlang`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HighlightLang {
    pub lang: String,
    pub force: bool,
    pub linenothreshold: Option<usize>,
}

/// Node for specifying tabular columns (LaTeX).
///
/// Mirrors `sphinx.addnodes.tabular_col_spec`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TabularColSpec {
    pub spec: String,
}

// ── Inline nodes ──────────────────────────────────────────────────────────────

/// Cross-reference that cannot be resolved until the full build.
///
/// Mirrors `sphinx.addnodes.pending_xref`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PendingXref {
    pub refdoc: String,
    pub refdomain: String,
    pub reftype: String,
    pub reftarget: String,
    pub modname: Option<String>,
    pub classname: Option<String>,
    pub refexplicit: bool,
    pub refwarn: bool,
}

/// One possible resolution of a [`PendingXref`].
///
/// Mirrors `sphinx.addnodes.pending_xref_condition`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PendingXrefCondition {
    /// `"resolved"`, `"*"`, or a domain-specific condition string.
    pub condition: String,
    pub text: String,
}

/// Reference using a number (figure numbers, section numbers).
///
/// Mirrors `sphinx.addnodes.number_reference`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NumberReference {
    pub refuri: String,
    pub title: String,
}

/// Reference for downloadable files.
///
/// Mirrors `sphinx.addnodes.download_reference`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DownloadReference {
    pub refdoc: String,
    pub reftarget: String,
    pub filename: String,
}

/// Emphasis that suppresses smart-quotes.
///
/// Mirrors `sphinx.addnodes.literal_emphasis`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiteralEmphasis {
    pub text: String,
}
impl NotSmartquotable for LiteralEmphasis {}

/// Strong text that suppresses smart-quotes.
///
/// Mirrors `sphinx.addnodes.literal_strong`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiteralStrong {
    pub text: String,
}
impl NotSmartquotable for LiteralStrong {}

/// Manpage reference.
///
/// Mirrors `sphinx.addnodes.manpage`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Manpage {
    pub page: String,
    pub section: u32,
    pub text: String,
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── SIG_ELEMENTS ──────────────────────────────────────────────────────────

    #[test]
    fn sig_elements_contains_expected() {
        // Mirrors test_desc_sig_element_nodes from test_addnodes.py.
        let expected = [
            "desc_sig_space",
            "desc_sig_name",
            "desc_sig_operator",
            "desc_sig_punctuation",
            "desc_sig_keyword",
            "desc_sig_keyword_type",
            "desc_sig_literal_number",
            "desc_sig_literal_string",
            "desc_sig_literal_char",
        ];
        let set: HashSet<&&str> = SIG_ELEMENTS.iter().collect();
        for name in &expected {
            assert!(set.contains(name), "SIG_ELEMENTS missing: {name}");
        }
        assert_eq!(SIG_ELEMENTS.len(), expected.len());
    }

    // ── toctree ───────────────────────────────────────────────────────────────

    #[test]
    fn toctree_default() {
        let tc = Toctree::default();
        assert!(tc.entries.is_empty());
        assert!(tc.caption.is_none());
        assert_eq!(tc.maxdepth, 0);
    }

    #[test]
    fn toctree_preserve_and_extract_messages() {
        let mut tc = Toctree {
            entries: vec![
                ("Introduction".into(), "intro".into()),
                ("".into(), "api/index".into()),
            ],
            caption: Some("Contents".into()),
            ..Default::default()
        };
        tc.preserve_original_messages();
        assert_eq!(tc.rawentries, vec!["Introduction"]);
        assert_eq!(tc.rawcaption.as_deref(), Some("Contents"));

        let msgs = tc.extract_original_messages();
        assert!(msgs.contains(&"Introduction".to_string()));
        assert!(msgs.contains(&"Contents".to_string()));
    }

    #[test]
    fn toctree_apply_translated_message() {
        let mut tc = Toctree {
            entries: vec![("Hello".into(), "page".into())],
            ..Default::default()
        };
        tc.apply_translated_message("Hello", "Hola");
        assert_eq!(tc.entries[0].0, "Hola");
    }

    #[test]
    fn toctree_apply_translated_caption() {
        let mut tc = Toctree {
            caption: Some("Table".into()),
            rawcaption: Some("Table".into()),
            ..Default::default()
        };
        tc.apply_translated_message("Table", "Tabla");
        assert_eq!(tc.caption.as_deref(), Some("Tabla"));
    }

    // ── desc_returns ──────────────────────────────────────────────────────────

    #[test]
    fn desc_returns_astext_prefix() {
        let r = DescReturns { text: "int".into() };
        assert_eq!(r.astext(), " -> int");
    }

    // ── desc_optional ─────────────────────────────────────────────────────────

    #[test]
    fn desc_optional_astext_brackets() {
        let opt = DescOptional {
            text: "x=1, y=2".into(),
        };
        assert_eq!(opt.astext(), "[x=1, y=2]");
    }

    // ── desc_sig_* constructors ───────────────────────────────────────────────

    #[test]
    fn desc_sig_name_classes() {
        let n = DescSigName::new("MyClass");
        assert_eq!(n.text(), "MyClass");
        assert!(n.classes().contains(&"n".to_string()));
    }

    #[test]
    fn desc_sig_space_default_class() {
        let s = DescSigSpace::new(" ");
        assert!(s.classes().contains(&"w".to_string()));
    }

    #[test]
    fn desc_sig_operator_class() {
        let op = DescSigOperator::new("=");
        assert!(op.classes().contains(&"o".to_string()));
    }

    #[test]
    fn desc_sig_keyword_type_class() {
        let kt = DescSigKeywordType::new("int");
        assert!(kt.classes().contains(&"kt".to_string()));
    }

    // ── literal_emphasis / literal_strong ─────────────────────────────────────

    #[test]
    fn literal_emphasis_not_smartquotable() {
        assert!(!LiteralEmphasis::support_smartquotes());
    }

    #[test]
    fn literal_strong_not_smartquotable() {
        assert!(!LiteralStrong::support_smartquotes());
    }

    // ── pending_xref ──────────────────────────────────────────────────────────

    #[test]
    fn pending_xref_defaults() {
        let xref = PendingXref::default();
        assert!(xref.refdomain.is_empty());
        assert!(!xref.refexplicit);
    }

    // ── versionmodified ───────────────────────────────────────────────────────

    #[test]
    fn versionmodified_fields() {
        let vm = VersionModified {
            kind: "added".into(),
            version: "1.2".into(),
        };
        assert_eq!(vm.kind, "added");
        assert_eq!(vm.version, "1.2");
    }

    // ── index ─────────────────────────────────────────────────────────────────

    #[test]
    fn index_default_empty() {
        let idx = Index::default();
        assert!(idx.entries.is_empty());
    }

    // ── only ─────────────────────────────────────────────────────────────────

    #[test]
    fn only_expr() {
        let o = Only {
            expr: "html or latex".into(),
        };
        assert_eq!(o.expr, "html or latex");
    }

    // ── manpage ───────────────────────────────────────────────────────────────

    #[test]
    fn manpage_fields() {
        let m = Manpage {
            page: "grep".into(),
            section: 1,
            text: "grep(1)".into(),
        };
        assert_eq!(m.section, 1);
    }
}
