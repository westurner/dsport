mod common;

use docutilsrs::{html5, cli::Html5Options};

#[test]
fn test_html5_coverage() {
    let rst = common::coverage_rst("html");
    let tree = common::build_coverage_tree(&rst);
    let options = Html5Options::default();
    let common_opts = common::coverage_common_options();
    let html = html5(&tree, &options, &common_opts);
    assert!(!html.is_empty());
}
