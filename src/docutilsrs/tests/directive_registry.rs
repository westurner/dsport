use docutilsrs::doctree::NodeKind;
use docutilsrs::parse_rst;
use docutilsrs::parser::Block;
use docutilsrs::plugins::register_native_directive;

#[test]
fn registered_native_directive_overrides_builtin_dispatch() {
    register_native_directive("note", |_args, content| {
        let body = content
            .iter()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        Some(vec![Block::Paragraph {
            text: format!("custom: {body}"),
            line: 0,
        }])
    });

    let tree = parse_rst(".. note::\n\n   from registry\n");
    let text = (0..tree.nodes_len()).find_map(|id| match &tree.node(id).kind {
        NodeKind::Text(value) => Some(value.clone()),
        _ => None,
    });
    assert_eq!(text.as_deref(), Some("custom: from registry"));
}
