use docutilsrs::cli::{CommonOptions, Html5Options};
use docutilsrs::{html5, parse_rst, Doctree};

#[test]
fn test_html5_footnote_references_superscript() {
    let rst = r#"
Footnote [1]_.

.. [1] It works!
"#;
    let tree = parse_rst(rst);
    let mut options = Html5Options::default();
    options.footnote_references = Some("superscript".to_string());
    let common = CommonOptions::default();
    
    let html = html5(&tree, &options, &common);
    assert!(html.contains("<a class=\"superscript\""), "Should contain superscript class");
    assert!(!html.contains("<span class=\"fn-bracket\">[</span>"), "Should not contain bracket spans");
}

#[test]
fn test_html5_footnote_references_brackets() {
    let rst = r#"
Footnote [1]_.

.. [1] It works!
"#;
    let tree = parse_rst(rst);
    let options = Html5Options::default(); // default should fall back to brackets
    let common = CommonOptions::default();
    
    let html = html5(&tree, &options, &common);
    assert!(html.contains("<a class=\"brackets\""), "Should contain brackets class");
    assert!(html.contains("<span class=\"fn-bracket\">[</span>"), "Should contain bracket spans");
}

#[test]
fn test_html5_math_output_mathjax() {
    let rst = r#"
:math:`x^2`
"#;
    let tree = docutilsrs::parse_rst(rst);
    let mut options = docutilsrs::cli::Html5Options::default();
    options.math_output = Some("mathjax".to_string());
    let common = docutilsrs::cli::CommonOptions::default();
    
    let html = docutilsrs::html5(&tree, &options, &common);
    assert!(!html.contains("<svg"), "Should not contain SVG for MathJax");
    assert!(html.contains("\\(x^2\\)"), "Should contain MathJax delimiter \\(..\\) for inline math output: {}", html);
}

#[test]
fn test_html5_table_style() {
    let rst = r#"
=====  =====
Col 1  Col 2
=====  =====
1      2
=====  =====
"#;
    let tree = docutilsrs::parse_rst(rst);
    let mut options = docutilsrs::cli::Html5Options::default();
    options.table_style = Some("borderless, align-right".to_string());
    let common = docutilsrs::cli::CommonOptions::default();
    
    let html = docutilsrs::html5(&tree, &options, &common);
    assert!(html.contains("<table class=\"borderless align-right\">"), "Should contain table with applied styles: {}", html);
}

#[test]
fn test_html5_cloak_email_addresses() {
    let rst = r#"
`test@example.com <mailto:test@example.com>`_
"#;
    let tree = docutilsrs::parse_rst(rst);
    let mut options = docutilsrs::cli::Html5Options::default();
    options.cloak_email_addresses = Some("true".to_string());
    let common = docutilsrs::cli::CommonOptions::default();
    
    let html = docutilsrs::html5(&tree, &options, &common);
    assert!(!html.contains("test@example.com"), "Should not contain cleartext email: {}", html);
    assert!(html.contains("&#37;&#52;&#48;"), "Should contain cloaked @ in href: {}", html);
    assert!(html.contains("&#64;"), "Should contain cloaked @ in text: {}", html);
}
