//! Rust-side parser tests: shape of the doctree on the phase 1 slice.
//!
//! Byte-for-byte parity against vendored docutils' pseudo-XML writer is
//! asserted from Python in `src/tests/test_parity_pseudoxml.py`, which has
//! the upstream available.

use docutilsrs::{html5, parse_rst, pseudo_xml};

#[test]
fn empty_input_yields_only_document() {
    let tree = parse_rst("");
    insta::assert_snapshot!("empty", pseudo_xml(&tree));
}

#[test]
fn single_paragraph() {
    insta::assert_snapshot!("single_paragraph", pseudo_xml(&parse_rst("Hello world.")));
}

#[test]
fn two_paragraphs() {
    insta::assert_snapshot!(
        "two_paragraphs",
        pseudo_xml(&parse_rst("First.\n\nSecond."))
    );
}

#[test]
fn inline_emphasis_strong_literal() {
    let src = "A *star* and **strong** and ``lit``.";
    insta::assert_snapshot!("inline_mix", pseudo_xml(&parse_rst(src)));
}

#[test]
fn inline_markup_does_not_nest() {
    // Per rST: `*italic*` inside `**...**` stays as plain text.
    let src = "**bold and *italic* inside**";
    insta::assert_snapshot!("no_nesting", pseudo_xml(&parse_rst(src)));
}

#[test]
fn literal_does_not_nest() {
    // The * inside a literal must remain plain text, not become emphasis.
    let src = "``*not emphasis*``";
    insta::assert_snapshot!("literal_no_nest", pseudo_xml(&parse_rst(src)));
}

#[test]
fn bullet_list_hyphen() {
    let src = "- one\n- two\n- three";
    insta::assert_snapshot!("bullet_list_hyphen", pseudo_xml(&parse_rst(src)));
}

#[test]
fn paragraph_surrounding_bullet_list() {
    let src = "Before.\n\n- a\n- b\n\nAfter.";
    insta::assert_snapshot!("around_bullet_list", pseudo_xml(&parse_rst(src)));
}

#[test]
fn escape_blocks_emphasis() {
    let src = "\\*not emphasis* here";
    insta::assert_snapshot!("escape_blocks_emphasis", pseudo_xml(&parse_rst(src)));
}

#[test]
fn escape_whitespace_collapses() {
    // `\ ` is consumed entirely.
    let src = "a \\ b";
    insta::assert_snapshot!("escape_ws_collapses", pseudo_xml(&parse_rst(src)));
}

#[test]
fn multi_line_paragraph() {
    let src = "line one\nline two\nline three";
    insta::assert_snapshot!("multi_line_paragraph", pseudo_xml(&parse_rst(src)));
}

#[test]
fn bullet_continuation_line() {
    let src = "- one\n  continued\n- two";
    insta::assert_snapshot!("bullet_continuation", pseudo_xml(&parse_rst(src)));
}

#[test]
fn reference_resolves_to_target() {
    let src = "See ref_.\n\n.. _ref: http://example.com";
    insta::assert_snapshot!("reference_resolved", pseudo_xml(&parse_rst(src)));
}

// ── document title promotion (with leading preamble nodes) ─────────────────

fn document_title(src: &str) -> String {
    use docutilsrs::NodeKind;
    let tree = parse_rst(src);
    match &tree.node(tree.root()).kind {
        NodeKind::Document { title, .. } => title.clone(),
        _ => String::new(),
    }
}

#[test]
fn title_promoted_for_underline_only_section() {
    assert_eq!(
        document_title("Getting started\n===============\n\nBody.\n"),
        "Getting started"
    );
}

#[test]
fn title_promoted_for_overline_and_underline_section() {
    assert_eq!(
        document_title("============\nUsing Sphinx\n============\n\nBody.\n"),
        "Using Sphinx"
    );
}

#[test]
fn title_promoted_past_leading_hyperlink_target() {
    // A common Sphinx convention: a `.. _label:` cross-reference target
    // immediately before the title. Mirrors upstream docutils'
    // `DocTitle` transform, which treats hyperlink targets (and comments,
    // substitution definitions) as invisible "preamble" for title
    // promotion purposes.
    let src = ".. _my-label:\n\nreStructuredText Primer\n=======================\n\nBody.\n";
    assert_eq!(document_title(src), "reStructuredText Primer");
}

#[test]
fn title_promoted_past_leading_comment_and_target() {
    let src = ".. highlight:: rst\n\n.. _rst-primer:\n\n=======================\nreStructuredText Primer\n=======================\n\nBody.\n";
    assert_eq!(document_title(src), "reStructuredText Primer");
}

#[test]
fn title_not_promoted_when_multiple_top_level_sections() {
    // Real docutils only promotes when the (post-preamble) section is the
    // *sole* top-level child; two top-level sections must not promote.
    let src = "One\n===\n\nBody.\n\nTwo\n===\n\nBody.\n";
    assert_eq!(document_title(src), "");
}

#[test]
fn overlined_title_and_underline_only_subsection_share_punctuation_but_differ_in_style() {
    // A very common Sphinx convention: the document title uses an
    // overline+underline `=` adornment, while its immediate child
    // sections use an *underline-only* `=` adornment. Real docutils keys
    // section nesting level on the (character, overline-present) *style*,
    // not the character alone — so these are different levels (title
    // level 0, subsection level 1) even though both use `=`. Getting this
    // wrong flattens every subsection into a document-level sibling of
    // the title, which also breaks title promotion outright (multiple
    // top-level sections).
    let src = "\
=============
Configuration
=============

Intro paragraph.

Builder Options
================

Builder body.

Project Information
====================

Project body.
";
    insta::assert_snapshot!(
        "overlined_title_with_underline_only_subsections",
        pseudo_xml(&parse_rst(src))
    );
    assert_eq!(document_title(src), "Configuration");
}

#[test]
fn enumerated_list_arabic_period() {
    let src = "1. one\n2. two\n3. three";
    insta::assert_snapshot!("enum_arabic_period", pseudo_xml(&parse_rst(src)));
}

#[test]
fn enumerated_list_auto_hash() {
    let src = "#. auto\n#. items";
    insta::assert_snapshot!("enum_auto_hash", pseudo_xml(&parse_rst(src)));
}

#[test]
fn enumerated_list_loweralpha_start() {
    let src = "c. just c";
    insta::assert_snapshot!("enum_loweralpha_start", pseudo_xml(&parse_rst(src)));
}

#[test]
fn enumerated_list_lowerroman_ambiguous() {
    let src = "i. roman\nii. two";
    insta::assert_snapshot!("enum_lowerroman_ambiguous", pseudo_xml(&parse_rst(src)));
}

#[test]
fn rst_directive_bare_name_renders_as_object_description() {
    let src = ".. rst:directive:: toctree\n\n   Insert a toc tree.\n";
    insta::assert_snapshot!("rst_directive_bare_name", pseudo_xml(&parse_rst(src)));
}

#[test]
fn rst_directive_full_signature_renders_dotted_form() {
    let src = ".. rst:directive:: .. attention::\n\n   An attention admonition.\n";
    insta::assert_snapshot!("rst_directive_full_signature", pseudo_xml(&parse_rst(src)));
}

#[test]
fn rst_role_renders_as_object_description() {
    let src = ".. rst:role:: ref\n\n   Cross-reference an arbitrary location.\n";
    insta::assert_snapshot!("rst_role", pseudo_xml(&parse_rst(src)));
}

#[test]
fn rst_directive_html5_renders_dl_dt_dd_with_anchor() {
    let src = ".. rst:directive:: toctree\n\n   Insert a toc tree.\n";
    let html = html5(
        &parse_rst(src),
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert!(
        html.contains(r#"<dl class="rst directive">"#),
        "got:\n{html}"
    );
    assert!(
        html.contains(r#"<dt id="directive-toctree">.. toctree::</dt>"#),
        "got:\n{html}"
    );
    assert!(html.contains("<dd>"), "got:\n{html}");
    assert!(html.contains("Insert a toc tree."), "got:\n{html}");
}

#[test]
fn domain_prefixed_role_renders_as_inline_with_full_role_name() {
    // `:rst:dir:`toctree`` must be recognized as a single role named
    // `rst:dir`, not fail to parse or truncate at the first colon.
    let src = "See :rst:dir:`toctree` for details.";
    let html = html5(
        &parse_rst(src),
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert!(
        html.contains(r#"<span class="xref rst rst-dir">toctree</span>"#),
        "expected full rst:dir role classes in output, got:\n{html}"
    );
    assert!(
        !html.contains(":rst:dir:`toctree`"),
        "role was left unparsed, got:\n{html}"
    );
}

#[test]
fn plain_role_without_domain_prefix_still_works() {
    let src = ":emphasis:`text`";
    insta::assert_snapshot!("plain_role_no_domain", pseudo_xml(&parse_rst(src)));
}

#[test]
fn anonymous_target_does_not_drop_trailing_siblings() {
    // Regression test: `emit_block`'s stack-based rewrite (see H12 in
    // docs/sphinxdocrs-port-plan.md) used `return` inside the anonymous
    // `Block::Target` arm to mean "skip the named-target code below and
    // move on to the next queued block" -- which was correct when
    // `emit_block` recursed per-block, but became "abandon every other
    // block still queued on the shared stack" once the function switched
    // to an explicit `Vec` stack. Any block that followed an anonymous
    // target anywhere in a nested block list (list items, block quotes,
    // sections, admonitions, ...) was silently dropped from the doctree.
    let src = "* Intro.\n\n  __ http://example.com/\n\n  After the anonymous target.\n";
    let xml = pseudo_xml(&parse_rst(src));
    assert!(xml.contains("Intro"), "got:\n{xml}");
    assert!(
        xml.contains("After the anonymous target"),
        "sibling block after an anonymous target was dropped:\n{xml}"
    );
}
