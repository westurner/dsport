mod common;

use docutilsrs::{cli::LatexOptions, latex};

#[test]
fn test_latex_coverage() {
    let rst = common::coverage_rst("latex");
    let tree = common::build_coverage_tree(&rst);
    let options = LatexOptions::default();
    let common_opts = common::coverage_common_options();
    let res = latex(&tree, &options, &common_opts);
    assert!(!res.is_empty());
}
