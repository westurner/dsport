//! Tests for the `:math:` role and `.. math::` directive.
//!
//! Math is rendered through `mathrenderrs`, which defaults to the
//! RaTeX backend and emits inline SVG with a `data-renderer="ratex"`
//! attribute. The pseudo-XML writer round-trips the LaTeX source.

use docutilsrs::{html5, parse_rst, pseudo_xml};

#[test]
fn math_role_pseudoxml() {
    let src = "See :math:`E = mc^2` for details.\n";
    let tree = parse_rst(src);
    let out = pseudo_xml(&tree);
    assert!(out.contains("<math>"), "missing <math>: {out}");
    assert!(out.contains("E = mc^2"), "missing latex: {out}");
}

#[test]
fn math_directive_pseudoxml() {
    let src = ".. math::\n\n   a^2 + b^2 = c^2\n";
    let tree = parse_rst(src);
    let out = pseudo_xml(&tree);
    assert!(out.contains("<math_block"), "missing <math_block>: {out}");
    assert!(out.contains("a^2 + b^2 = c^2"), "missing latex: {out}");
}

#[test]
fn math_role_html5_uses_ratex_default() {
    let src = ":math:`x^2`\n";
    let tree = parse_rst(src);
    let out = html5(&tree);
    assert!(
        out.contains("data-renderer=\"ratex\""),
        "expected RaTeX default backend in: {out}"
    );
    assert!(out.contains("<svg"), "expected inline SVG in: {out}");
}

#[test]
fn math_directive_html5_uses_ratex_default() {
    let src = ".. math::\n\n   \\int_0^1 x\\,dx\n";
    let tree = parse_rst(src);
    let out = html5(&tree);
    assert!(
        out.contains("data-renderer=\"ratex\""),
        "expected RaTeX default backend in: {out}"
    );
    assert!(
        out.contains("math-block"),
        "expected block math class in: {out}"
    );
}
