//! Integration tests for `sphinxdocrs::addnodes`.
//!
//! Mirrors `sphinx/tests/test_addnodes.py`.

use sphinxdocrs::addnodes::{
    DescOptional, DescReturns, DescSigKeyword, DescSigKeywordType, DescSigLiteralChar,
    DescSigLiteralNumber, DescSigLiteralString, DescSigName, DescSigOperator, DescSigPunctuation,
    DescSigSpace, HighlightLang, Index, LiteralEmphasis, LiteralStrong, Manpage, NotSmartquotable,
    Only, PendingXref, PendingXrefCondition, SIG_ELEMENTS, StartOfFile, Toctree, Translatable,
    VersionModified,
};

// ── SIG_ELEMENTS ──────────────────────────────────────────────────────────────

/// Mirrors `test_desc_sig_element_nodes` from `test_addnodes.py`.
///
/// The test verifies that exactly the expected nine `desc_sig_*` node classes
/// are registered in `SIG_ELEMENTS`.
#[test]
fn sig_elements_exact_set() {
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
    use std::collections::HashSet;
    let actual: HashSet<&&str> = SIG_ELEMENTS.iter().collect();
    for name in &expected {
        assert!(actual.contains(name), "SIG_ELEMENTS missing: {name}");
    }
    assert_eq!(
        SIG_ELEMENTS.len(),
        expected.len(),
        "SIG_ELEMENTS has unexpected extra entries"
    );
}

// ── desc_sig_* classes ────────────────────────────────────────────────────────

#[test]
fn desc_sig_space_class_w() {
    let n = DescSigSpace::new(" ");
    assert!(n.classes().contains(&"w".to_string()));
}

#[test]
fn desc_sig_name_class_n() {
    let n = DescSigName::new("MyClass");
    assert!(n.classes().contains(&"n".to_string()));
    assert_eq!(n.text(), "MyClass");
}

#[test]
fn desc_sig_operator_class_o() {
    let n = DescSigOperator::new("*");
    assert!(n.classes().contains(&"o".to_string()));
}

#[test]
fn desc_sig_punctuation_class_p() {
    let n = DescSigPunctuation::new(",");
    assert!(n.classes().contains(&"p".to_string()));
}

#[test]
fn desc_sig_keyword_class_k() {
    let n = DescSigKeyword::new("def");
    assert!(n.classes().contains(&"k".to_string()));
}

#[test]
fn desc_sig_keyword_type_class_kt() {
    let n = DescSigKeywordType::new("int");
    assert!(n.classes().contains(&"kt".to_string()));
}

#[test]
fn desc_sig_literal_number_class_m() {
    let n = DescSigLiteralNumber::new("42");
    assert!(n.classes().contains(&"m".to_string()));
}

#[test]
fn desc_sig_literal_string_class_s() {
    let n = DescSigLiteralString::new(r#""hello""#);
    assert!(n.classes().contains(&"s".to_string()));
}

#[test]
fn desc_sig_literal_char_class_sc() {
    let n = DescSigLiteralChar::new("'a'");
    assert!(n.classes().contains(&"sc".to_string()));
}

// ── toctree translatable ──────────────────────────────────────────────────────

#[test]
fn toctree_preserve_extract_messages() {
    let mut tc = Toctree {
        entries: vec![("Guide".into(), "guide".into()), ("".into(), "api".into())],
        caption: Some("Table of Contents".into()),
        ..Default::default()
    };
    tc.preserve_original_messages();
    // Only titled entries are preserved.
    assert_eq!(tc.rawentries, vec!["Guide"]);
    assert_eq!(tc.rawcaption.as_deref(), Some("Table of Contents"));

    let msgs = tc.extract_original_messages();
    assert!(msgs.contains(&"Guide".to_string()));
    assert!(msgs.contains(&"Table of Contents".to_string()));
    // The untitled entry has no original message.
    assert!(!msgs.contains(&"api".to_string()));
}

#[test]
fn toctree_apply_translated_entry_title() {
    let mut tc = Toctree {
        entries: vec![
            ("Hello".into(), "page".into()),
            ("World".into(), "other".into()),
        ],
        ..Default::default()
    };
    tc.apply_translated_message("Hello", "Hola");
    assert_eq!(tc.entries[0].0, "Hola");
    assert_eq!(tc.entries[1].0, "World"); // unchanged
}

#[test]
fn toctree_apply_translated_caption() {
    let mut tc = Toctree {
        caption: Some("Contents".into()),
        rawcaption: Some("Contents".into()),
        ..Default::default()
    };
    tc.apply_translated_message("Contents", "Contenido");
    assert_eq!(tc.caption.as_deref(), Some("Contenido"));
}

#[test]
fn toctree_apply_translation_no_match_unchanged() {
    let mut tc = Toctree {
        entries: vec![("Hello".into(), "page".into())],
        ..Default::default()
    };
    tc.apply_translated_message("Goodbye", "Adios");
    assert_eq!(tc.entries[0].0, "Hello"); // unchanged
}

// ── desc_returns astext ───────────────────────────────────────────────────────

#[test]
fn desc_returns_astext_arrow() {
    let r = DescReturns {
        text: "None".into(),
    };
    assert_eq!(r.astext(), " -> None");
}

// ── desc_optional astext ──────────────────────────────────────────────────────

#[test]
fn desc_optional_brackets() {
    let opt = DescOptional { text: "a=1".into() };
    assert_eq!(opt.astext(), "[a=1]");
}

// ── not_smartquotable ─────────────────────────────────────────────────────────

#[test]
fn literal_emphasis_no_smartquotes() {
    assert!(!LiteralEmphasis::support_smartquotes());
}

#[test]
fn literal_strong_no_smartquotes() {
    assert!(!LiteralStrong::support_smartquotes());
}

// ── pending_xref ──────────────────────────────────────────────────────────────

#[test]
fn pending_xref_default_fields() {
    let xref = PendingXref::default();
    assert!(xref.refdomain.is_empty());
    assert!(xref.reftarget.is_empty());
    assert!(!xref.refexplicit);
    assert!(!xref.refwarn);
}

#[test]
fn pending_xref_condition_default() {
    let cond = PendingXrefCondition::default();
    assert!(cond.condition.is_empty());
}

// ── version modified ──────────────────────────────────────────────────────────

#[test]
fn versionmodified_added() {
    let vm = VersionModified {
        kind: "added".into(),
        version: "2.0".into(),
    };
    assert_eq!(vm.kind, "added");
}

#[test]
fn versionmodified_deprecated() {
    let vm = VersionModified {
        kind: "deprecated".into(),
        version: "3.0".into(),
    };
    assert_eq!(vm.kind, "deprecated");
}

// ── index ─────────────────────────────────────────────────────────────────────

#[test]
fn index_entries_tuple() {
    let idx = Index {
        entries: vec![(
            "single".into(),
            "module".into(),
            "module".into(),
            "".into(),
            None,
        )],
    };
    assert_eq!(idx.entries[0].0, "single");
}

// ── only ──────────────────────────────────────────────────────────────────────

#[test]
fn only_tag_expression() {
    let only = Only {
        expr: "html".into(),
    };
    assert_eq!(only.expr, "html");
}

// ── highlightlang ─────────────────────────────────────────────────────────────

#[test]
fn highlightlang_fields() {
    let h = HighlightLang {
        lang: "python".into(),
        force: true,
        linenothreshold: Some(5),
    };
    assert_eq!(h.lang, "python");
    assert_eq!(h.linenothreshold, Some(5));
}

// ── start_of_file ─────────────────────────────────────────────────────────────

#[test]
fn start_of_file_docname() {
    let sof = StartOfFile {
        docname: "index".into(),
    };
    assert_eq!(sof.docname, "index");
}

// ── manpage ───────────────────────────────────────────────────────────────────

#[test]
fn manpage_section() {
    let m = Manpage {
        page: "ls".into(),
        section: 1,
        text: "ls(1)".into(),
    };
    assert_eq!(m.section, 1);
    assert_eq!(m.text, "ls(1)");
}
