use docutilsrs::doctree::NodeKind;
use docutilsrs::parse_rst;
use docutilsrs::parser::Block;
use docutilsrs::plugins::{register_native_directive, register_python_directive};
use pyo3::prelude::*;
use pyo3::types::PyDict;

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

#[test]
fn python_directive_can_nested_parse_content() {
    Python::attach(|py| {
        let globals = PyDict::new(py);
        py.run(
            c"from docutils import nodes\nfrom docutils.parsers.rst import Directive\n\nclass NestedDirective(Directive):\n    has_content = True\n\n    def run(self):\n        container = nodes.container()\n        source, line = self.state_machine.get_source_and_line()\n        container += nodes.paragraph(text=f'{source}:{line}')\n        self.state.nested_parse(self.content, self.content_offset, container)\n        return [container]\n",
            None,
            Some(&globals),
        )
        .unwrap();
        let directive = globals.get_item("NestedDirective").unwrap().unwrap();
        register_python_directive("nested", directive.unbind());
    });

    let tree = parse_rst(".. nested::\n\n   nested *content*\n");
    let texts: Vec<String> = (0..tree.nodes_len())
        .filter_map(|id| match &tree.node(id).kind {
            NodeKind::Text(value) => Some(value.clone()),
            _ => None,
        })
        .collect();
    assert!(texts.iter().any(|text| text == "nested *content*"));
}
