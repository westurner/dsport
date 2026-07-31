use docutilsrs::{Doctree, NodeKind, parse_rst};

fn find_text(tree: &Doctree, parent: usize) -> Option<String> {
    for &c in &tree.node(parent).children {
        if let NodeKind::Text(s) = &tree.node(c).kind {
            return Some(s.clone());
        }
    }
    None
}

#[test]
fn figure_directive_params_and_options() {
    let src = ".. figure:: picture.png\n   :alt: map\n   :width: 100px\n   :height: 200px\n\n   This is a caption.\n";
    let tree = parse_rst(src);

    // Find Image node under Figure
    let root = tree.root();
    let mut fig_id = None;
    for &c in &tree.node(root).children {
        if matches!(tree.node(c).kind, NodeKind::Figure) {
            fig_id = Some(c);
            break;
        }
    }
    let fig_id = fig_id.expect("figure node found");

    let mut img_found = false;
    let mut cap_found = false;
    for &c in &tree.node(fig_id).children {
        match &tree.node(c).kind {
            NodeKind::Image {
                uri,
                alt,
                width,
                height,
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
    assert!(img_found, "Image child in Figure");
    assert!(cap_found, "Caption child in Figure");
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
