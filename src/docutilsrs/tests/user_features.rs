use docutilsrs::{Doctree, NodeKind, TitlePromotion, parse_rst, parse_rst_with_options};

fn find_text(tree: &Doctree, parent: usize) -> Option<String> {
    for &c in &tree.node(parent).children {
        if let NodeKind::Text(s) = &tree.node(c).kind {
            return Some(s.clone());
        }
    }
    None
}

#[test]
fn explicit_target_before_section_gets_distinct_id() {
    let tree = parse_rst_with_options(
        ".. _get-started:\n\nGet started\n===========\n",
        "<test>",
        TitlePromotion::Preserve,
    );
    let children = &tree.node(tree.root()).children;
    assert!(matches!(
        tree.node(children[0]).kind,
        NodeKind::Section { ref ids, .. } if ids == "get-started"
    ));
    let section_children = &tree.node(children[0]).children;
    assert!(matches!(
        tree.node(section_children[0]).kind,
        NodeKind::Target { ref ids, .. } if ids == "id1"
    ));
}

#[test]
fn figure_directive_params_and_options() {
    let src = ".. figure:: picture.png\n   :alt: map\n   :width: 100px\n   :height: 200px\n   :align: right\n   :target: https://example.test/map\n   :class: map-figure\n\n   This is a caption.\n";
    let tree = parse_rst(src);

    // Find Image node under Figure
    let root = tree.root();
    let mut fig_id = None;
    for &c in &tree.node(root).children {
        if matches!(tree.node(c).kind, NodeKind::Figure { .. }) {
            fig_id = Some(c);
            break;
        }
    }
    let fig_id = fig_id.expect("figure node found");

    match &tree.node(fig_id).kind {
        NodeKind::Figure { ids, classes } => {
            assert!(!ids.is_empty());
            assert_eq!(classes, "align-right map-figure");
        }
        _ => panic!("figure node has the wrong kind"),
    }

    let mut image_reference_found = false;
    let mut img_found = false;
    let mut cap_found = false;
    for &c in &tree.node(fig_id).children {
        match &tree.node(c).kind {
            NodeKind::Reference {
                refuri, classes, ..
            } => {
                image_reference_found = true;
                assert_eq!(refuri, "https://example.test/map");
                assert_eq!(classes, "reference external image-reference");
                assert!(tree.node(c).children.iter().any(|&child| {
                    if let NodeKind::Image {
                        uri,
                        alt,
                        width,
                        height,
                        ..
                    } = &tree.node(child).kind
                    {
                        img_found = true;
                        assert_eq!(uri, "picture.png");
                        assert_eq!(alt.as_deref(), Some("map"));
                        assert_eq!(width.as_deref(), Some("100px"));
                        assert_eq!(height.as_deref(), Some("200px"));
                        true
                    } else {
                        false
                    }
                }));
            }
            NodeKind::Image {
                uri,
                alt,
                width,
                height,
                ..
            } => {
                img_found = true;
                assert_eq!(uri, "picture.png");
                assert_eq!(alt.as_deref(), Some("map"));
                assert_eq!(width.as_deref(), Some("100px"));
                assert_eq!(height.as_deref(), Some("200px"));
            }
            NodeKind::Caption => {
                cap_found = true;
                assert_eq!(find_text(&tree, c).as_deref(), Some("This is a caption."));
            }
            _ => {}
        }
    }
    assert!(image_reference_found, "image reference wrapper in Figure");
    assert!(img_found, "Image child in Figure");
    assert!(cap_found, "Caption child in Figure");
}

#[test]
fn rst_class_applies_to_section() {
    let tree = parse_rst_with_options(
        ".. rst-class:: hide-header\n\nTitle\n=====\n",
        "<test>",
        TitlePromotion::Preserve,
    );
    let section = tree.node(tree.root()).children.iter().find(|&&id| {
        matches!(tree.node(id).kind, NodeKind::Section { .. })
    }).copied().expect("section node found");

    assert!(matches!(
        &tree.node(section).kind,
        NodeKind::Section { classes, .. } if classes == "hide-header"
    ));
}

#[test]
fn unknown_target_name_system_message_location() {
    let src = "Paragraph referencing unknown_\n";
    let tree = parse_rst(src);
    let root = tree.root();

    let mut found_sm = false;
    for &c in &tree.node(root).children {
        if matches!(tree.node(c).kind, NodeKind::Section { .. }) {
            for &sc in &tree.node(c).children {
                if let NodeKind::SystemMessage { line, .. } = &tree.node(sc).kind {
                    found_sm = true;
                    assert_eq!(*line, Some(1));
                    let p_id = tree.node(sc).children[0];
                    let text = find_text(&tree, p_id).unwrap();
                    assert_eq!(text, "Unknown target name: \"unknown\" at line 1.");
                }
            }
        }
    }
    assert!(found_sm, "System message with location found");
}

#[test]
fn unknown_directive_logs_file_and_line() {
    use docutilsrs::parse_rst_with_source;
    let src = ".. misspelled:: arg\n";
    let _tree = parse_rst_with_source(src, "docs/index.rst");
}

#[test]
fn unknown_role_logs_file_and_line() {
    use docutilsrs::parse_rst_with_source;
    let src = "Text with :unknownrole:`sample` content.\n";
    let _tree = parse_rst_with_source(src, "docs/index.rst");
}
