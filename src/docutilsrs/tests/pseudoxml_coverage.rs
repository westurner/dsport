mod common;

use docutilsrs::pseudo_xml;

#[test]
fn test_pseudoxml_coverage() {
    let rst = common::coverage_rst("html");
    let tree = common::build_coverage_tree(&rst);
    let out = pseudo_xml(&tree);
    assert!(!out.is_empty());
}
