mod common;

use docutilsrs::{manpage, cli::ManOptions};

#[test]
fn test_manpage_coverage() {
    let rst = common::coverage_rst("manpage");
    let tree = common::build_coverage_tree(&rst);
    let options = ManOptions::default();
    let common_opts = common::coverage_common_options();
    let res = manpage(&tree, &options, &common_opts);
    assert!(!res.is_empty());
}
