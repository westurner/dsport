mod common;

use docutilsrs::{odt, cli::OdtOptions};

#[test]
fn test_odt_coverage() {
    let rst = common::coverage_rst("odt");
    let tree = common::build_coverage_tree(&rst);
    let options = OdtOptions::default();
    let common_opts = common::coverage_common_options();
    let res = odt(&tree, &options, &common_opts);
    assert!(!res.is_empty());
}
