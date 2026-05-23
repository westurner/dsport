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
