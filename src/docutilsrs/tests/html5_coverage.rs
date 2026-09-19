mod common;

use docutilsrs::{cli::Html5Options, html5};

#[test]
fn test_html5_coverage() {
    let rst = common::coverage_rst("html");
    let tree = common::build_coverage_tree(&rst);
    let options = Html5Options::default();
    let common_opts = common::coverage_common_options();
    let html = html5(&tree, &options, &common_opts);
    assert!(!html.is_empty());
}

#[test]
fn image_dimensions_are_preserved_in_html5_output() {
    let tree = docutilsrs::parse_rst(
        ".. image:: diagram.svg\n   :alt: Diagram\n   :width: 320px\n   :height: 180px\n",
    );
    let html = html5(
        &tree,
        &Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert!(
        html.contains(
            "<img src=\"diagram.svg\" alt=\"Diagram\" width=\"320px\" height=\"180px\"/>"
        ),
        "{html}"
    );
}
