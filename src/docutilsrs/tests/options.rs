use docutilsrs::cli::{CommonOptions, Html5Options};
use docutilsrs::{html5, parse_rst};

#[test]
fn test_html5_footnote_references_superscript() {
    let rst = r#"
Footnote [1]_.

.. [1] It works!
"#;
    let tree = parse_rst(rst);
    let options = Html5Options {
        footnote_references: Some("superscript".to_string()),
        ..Default::default()
    };
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
    let options = docutilsrs::cli::Html5Options {
        math_output: Some("mathjax".to_string()),
        ..Default::default()
    };
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
    let options = docutilsrs::cli::Html5Options {
        table_style: Some("borderless, align-right".to_string()),
        ..Default::default()
    };
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
    let options = docutilsrs::cli::Html5Options {
        cloak_email_addresses: Some("true".to_string()),
        ..Default::default()
    };
    let common = docutilsrs::cli::CommonOptions::default();
    
    let html = docutilsrs::html5(&tree, &options, &common);
    assert!(!html.contains("test@example.com"), "Should not contain cleartext email: {}", html);
    assert!(html.contains("&#37;&#52;&#48;"), "Should contain cloaked @ in href: {}", html);
    assert!(html.contains("&#64;"), "Should contain cloaked @ in text: {}", html);
}

#[test]
fn test_html5_compact_lists() {
    let rst = "* item 1\n* item 2\n";
    let tree = docutilsrs::parse_rst(rst);
    
    // Default -> simple class added
    let html = docutilsrs::html5(&tree, &docutilsrs::cli::Html5Options::default(), &docutilsrs::cli::CommonOptions::default());
    assert!(html.contains("<ul class=\"simple\">"), "Should compact simple lists by default: {}", html);
    
    // --no-compact-lists
    let options = docutilsrs::cli::Html5Options {
        no_compact_lists: Some("true".to_string()),
        ..Default::default()
    };
    let html_no_compact = docutilsrs::html5(&tree, &options, &docutilsrs::cli::CommonOptions::default());
    assert!(!html_no_compact.contains("<ul class=\"simple\">"), "Should not compact if --no-compact-lists is used");
    assert!(html_no_compact.contains("<ul>"), "Should be plain ul");
}

#[test]
fn test_html5_compact_field_lists() {
    let rst = r#"
:Field: Value
:Another: Value 2
"#;
    let tree = docutilsrs::parse_rst(rst);
    
    // Default -> simple class added
    let html = docutilsrs::html5(&tree, &docutilsrs::cli::Html5Options::default(), &docutilsrs::cli::CommonOptions::default());
    assert!(html.contains("<dl class=\"field-list simple\">"), "Should compact field lists by default: {}", html);
    
    // --no-compact-field-lists
    let options = docutilsrs::cli::Html5Options {
        no_compact_field_lists: Some("true".to_string()),
        ..Default::default()
    };
    let html_no_compact = docutilsrs::html5(&tree, &options, &docutilsrs::cli::CommonOptions::default());
    assert!(!html_no_compact.contains("simple"), "Should not compact field lists if --no-compact-field-lists is used");
    assert!(html_no_compact.contains("<dl class=\"field-list\">"), "Should be normal dl");
}

#[test]
fn test_html5_generator() {
    let rst = "Test\n";
    let tree = docutilsrs::parse_rst(rst);
    
    // No generator by default
    let html = docutilsrs::html5(&tree, &docutilsrs::cli::Html5Options::default(), &docutilsrs::cli::CommonOptions::default());
    assert!(!html.contains("Generated by"), "Should not contain generator string if not requested: {}", html);
    
    // --generator
    let common = docutilsrs::cli::CommonOptions {
        generator: Some("true".to_string()),
        ..Default::default()
    };
    let html_gen = docutilsrs::html5(&tree, &docutilsrs::cli::Html5Options::default(), &common);
    assert!(html_gen.contains("Generated by <a href=\"https://docutils.sourceforge.io/\">Docutils</a> from <a href=\"https://docutils.sourceforge.io/rst.html\">reStructuredText</a> source."), "Should contain generator string");
}

#[test]
fn test_html5_datestamp() {
    let rst = "Test\n";
    let tree = docutilsrs::parse_rst(rst);
    
    // --date
    let common = docutilsrs::cli::CommonOptions {
        date: Some("%Y-%m-%d".to_string()),
        ..Default::default()
    };
    let html_date = docutilsrs::html5(&tree, &docutilsrs::cli::Html5Options::default(), &common);
    assert!(html_date.contains("Generated on: "), "Should contain Generated on: string. output: {}", html_date);
}
