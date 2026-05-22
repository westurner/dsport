//! Rust-side parser tests: shape of the doctree on the phase 1 slice.
//!
//! Byte-for-byte parity against vendored docutils' pseudo-XML writer is
//! asserted from Python in `src/tests/test_parity_pseudoxml.py`, which has
//! the upstream available.

use docutilsrs::{parse_rst, pseudo_xml};

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
