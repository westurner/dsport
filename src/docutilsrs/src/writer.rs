//! Pseudo-XML writer.
//!
//! Output format matches `docutils.writers.pseudoxml` on the node kinds
//! supported in the phase 1 slice. Parity is asserted by
//! `tests/parity_pseudoxml.rs`.
//!
//! Format rules (derived from upstream output, see compat notes):
//! - 4-space indent per depth level.
//! - Each node emits one line: `<indent><tag[ attr="val" ...]>`.
//! - No closing tags.
//! - Text nodes emit `<indent><raw text>` with the text content unescaped
//!   except for the standard XML entities.
//! - Attributes are written in alphabetical order.
//! - Output ends with a single trailing newline.

use crate::doctree::{Doctree, NodeId, NodeKind};
use std::fmt::Write as _;

pub fn pseudo_xml(tree: &Doctree) -> String {
    let mut out = String::new();
    write_node(tree, tree.root(), 0, &mut out);
    out
}

fn write_node(tree: &Doctree, id: NodeId, depth: usize, out: &mut String) {
    let node = tree.node(id);
    let indent = "    ".repeat(depth);
    match &node.kind {
        NodeKind::Text(s) => {
            // Mirror docutils.nodes.Text.pformat: every source line of the
            // text becomes its own indented line.
            for line in s.split('\n') {
                out.push_str(&indent);
                out.push_str(line);
                out.push('\n');
            }
        }
        NodeKind::Document { source } => {
            // pseudo-XML deliberately does NOT XML-escape attribute values;
            // it mirrors `Node.pformat()` from docutils, which formats raw.
            let _ = writeln!(out, "{indent}<document source=\"{source}\">");
        }
        NodeKind::Paragraph => {
            out.push_str(&indent);
            out.push_str("<paragraph>\n");
        }
        NodeKind::Emphasis => {
            out.push_str(&indent);
            out.push_str("<emphasis>\n");
        }
        NodeKind::Strong => {
            out.push_str(&indent);
            out.push_str("<strong>\n");
        }
        NodeKind::Literal => {
            out.push_str(&indent);
            out.push_str("<literal>\n");
        }
        NodeKind::BulletList { bullet } => {
            let _ = writeln!(out, "{indent}<bullet_list bullet=\"{bullet}\">");
        }
        NodeKind::EnumeratedList {
            enumtype,
            prefix,
            suffix,
            start,
        } => {
            if let Some(s) = start {
                let _ = writeln!(
                    out,
                    "{indent}<enumerated_list enumtype=\"{enumtype}\" prefix=\"{prefix}\" start=\"{s}\" suffix=\"{suffix}\">"
                );
            } else {
                let _ = writeln!(
                    out,
                    "{indent}<enumerated_list enumtype=\"{enumtype}\" prefix=\"{prefix}\" suffix=\"{suffix}\">"
                );
            }
        }
        NodeKind::ListItem => {
            out.push_str(&indent);
            out.push_str("<list_item>\n");
        }
        NodeKind::Reference { name, refuri } => {
            let _ = writeln!(
                out,
                "{indent}<reference name=\"{name}\" refuri=\"{refuri}\">"
            );
        }
        NodeKind::Target { ids, names, refuri } => {
            let _ = writeln!(
                out,
                "{indent}<target ids=\"{ids}\" names=\"{names}\" refuri=\"{refuri}\">"
            );
        }
    }
    for &child in &node.children {
        write_node(tree, child, depth + 1, out);
    }
}
