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
            "<img alt=\"Diagram\" src=\"diagram.svg\" style=\"width: 320px; height: 180px;\" />"
        ),
        "{html}"
    );
}

#[test]
fn epub_mode_formats_lists_and_code_blocks_with_sphinx_block_spacing() {
    let tree = docutilsrs::parse_rst(
        "Guide\n=====\n\n* first item\n* second item\n\n.. code-block:: python\n\n   value = 1\n",
    );
    let html = html5(
        &tree,
        &Html5Options {
            epub_compat: true,
            ..Html5Options::default()
        },
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert!(html.contains("<ul class=\"simple\">\n<li><p>first item</p></li>\n"));
    assert!(html.contains(
        "<div class=\"highlight-python notranslate\"><div class=\"highlight\"><pre><span></span><span class=\"n\">value</span> <span class=\"o\">=</span> <span class=\"mi\">1</span>\n</pre></div>\n</div>\n"
    ));
}

#[test]
fn epub_mode_formats_versionmodified_like_sphinx() {
    let tree = docutilsrs::parse_rst(
        "Reference\n=========\n\n.. versionadded:: 1.0\n\n   A documented feature.\n",
    );
    let html = html5(
        &tree,
        &Html5Options {
            epub_compat: true,
            ..Html5Options::default()
        },
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert!(html.contains(
        "<div class=\"versionadded\">\n<p><span class=\"versionmodified added\">Added in version 1.0: </span>A documented feature.</p>\n</div>"
    ));
    assert!(html.contains("</div>\n"));
}
