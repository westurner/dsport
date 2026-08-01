//! Minimal manpage (troff) writer.
//!
//! Produces a small subset of `man(7)` output suitable for downstream
//! rendering. Not parity-gated against `docutils.writers.manpage`; see
//! `docs/compat.md` for the accepted-deviation note.

use crate::doctree::{Doctree, Node, NodeId, NodeKind};
use std::fmt::Write as _;

/// A deferred unit of output work, driven by an explicit stack instead of
/// renderer-local recursion (see `docs/sphinxdocrs-port-plan.md`'s H12
/// tier). `Enter` dispatches on the node's kind at the given section
/// depth; `EnterListItemChild` mirrors the old `emit_li_child` special
/// case (a list item's leading paragraph doesn't get its own `.PP`);
/// `Append` is a plain string applied verbatim.
enum Task {
    Enter(NodeId, usize),
    EnterListItemChild(NodeId, usize),
    Append(String),
}

/// Push `sub` (already in document order) onto `tasks` such that popping
/// `tasks` yields `sub` in the same order.
fn push_all(tasks: &mut Vec<Task>, sub: Vec<Task>) {
    for t in sub.into_iter().rev() {
        tasks.push(t);
    }
}

/// Schedule `node`'s children (at `depth`) followed by `close`.
fn schedule(node: &Node, depth: usize, close: impl Into<String>, tasks: &mut Vec<Task>) {
    tasks.push(Task::Append(close.into()));
    for &c in node.children.iter().rev() {
        tasks.push(Task::Enter(c, depth));
    }
}

/// Schedule `node`'s children (at `depth`) with no closing text.
fn schedule_children(node: &Node, depth: usize, tasks: &mut Vec<Task>) {
    for &c in node.children.iter().rev() {
        tasks.push(Task::Enter(c, depth));
    }
}

pub fn manpage(
    tree: &Doctree,
    _options: &crate::cli::ManOptions,
    _common: &crate::cli::CommonOptions,
) -> String {
    let mut out = String::new();
    let root = tree.root();
    let title = if let NodeKind::Document { title, .. } = &tree.node(root).kind {
        title.clone()
    } else {
        String::new()
    };
    let header_title = if title.is_empty() {
        "UNTITLED".to_string()
    } else {
        title.to_uppercase()
    };
    let _ = writeln!(out, ".TH {} 1", header_title);
    let mut tasks: Vec<Task> = Vec::new();
    for &c in tree.node(root).children.iter().rev() {
        tasks.push(Task::Enter(c, 0));
    }
    while let Some(task) = tasks.pop() {
        match task {
            Task::Append(s) => out.push_str(&s),
            Task::Enter(id, depth) => emit_enter(tree, id, depth, &mut out, &mut tasks),
            Task::EnterListItemChild(id, depth) => {
                // Inside an `.IP`, the first paragraph should not
                // introduce its own `.PP` (which resets indentation).
                if let NodeKind::Paragraph = &tree.node(id).kind {
                    let node = tree.node(id);
                    tasks.push(Task::Append("\n".to_string()));
                    for &c in node.children.iter().rev() {
                        tasks.push(Task::Enter(c, depth));
                    }
                } else {
                    emit_enter(tree, id, depth, &mut out, &mut tasks);
                }
            }
        }
    }
    out
}

fn emit_enter(
    tree: &Doctree,
    id: NodeId,
    section_depth: usize,
    out: &mut String,
    tasks: &mut Vec<Task>,
) {
    let node = tree.node(id);
    match &node.kind {
        NodeKind::Text(s) => out.push_str(&escape(s)),
        NodeKind::Document { .. } => schedule_children(node, section_depth, tasks),
        NodeKind::Section { .. } => schedule_children(node, section_depth + 1, tasks),
        NodeKind::Title => {
            let macro_name = match section_depth {
                0 | 1 => ".SH",
                _ => ".SS",
            };
            let _ = write!(out, "{} ", macro_name);
            schedule(node, section_depth, "\n", tasks);
        }
        NodeKind::Subtitle { .. } => {
            out.push_str(".SS ");
            schedule(node, section_depth, "\n", tasks);
        }
        NodeKind::Transition => out.push_str(".br\n"),
        NodeKind::Paragraph => {
            out.push_str(".PP\n");
            schedule(node, section_depth, "\n", tasks);
        }
        NodeKind::Emphasis | NodeKind::TitleReference => {
            out.push_str("\\fI");
            schedule(node, section_depth, "\\fR", tasks);
        }
        NodeKind::Strong => {
            out.push_str("\\fB");
            schedule(node, section_depth, "\\fR", tasks);
        }
        NodeKind::Literal => {
            out.push_str("\\fC");
            schedule(node, section_depth, "\\fR", tasks);
        }
        NodeKind::Inline { .. } => schedule_children(node, section_depth, tasks),
        NodeKind::Math { latex } => {
            // Manpages have no math typesetting; render the LaTeX
            // source verbatim wrapped in `$…$` so the original is
            // preserved for downstream tooling.
            out.push('$');
            out.push_str(latex);
            out.push('$');
        }
        NodeKind::MathBlock { latex } => {
            out.push_str(".nf\n");
            out.push_str(latex);
            if !latex.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(".fi\n");
        }
        NodeKind::LiteralBlock { .. } => {
            out.push_str(".nf\n");
            for &c in &node.children {
                if let NodeKind::Text(s) = &tree.node(c).kind {
                    out.push_str(s);
                }
            }
            out.push_str("\n.fi\n");
        }
        NodeKind::BulletList { .. } => {
            let mut sub = Vec::new();
            for &c in &node.children {
                sub.push(Task::Append(".IP \\(bu 2\n".to_string()));
                if let NodeKind::ListItem = &tree.node(c).kind {
                    for &cc in &tree.node(c).children {
                        sub.push(Task::EnterListItemChild(cc, section_depth));
                    }
                }
            }
            push_all(tasks, sub);
        }
        NodeKind::EnumeratedList { .. } => {
            let mut sub = Vec::new();
            for (i, &c) in node.children.iter().enumerate() {
                sub.push(Task::Append(format!(".IP {}. 4\n", i + 1)));
                if let NodeKind::ListItem = &tree.node(c).kind {
                    for &cc in &tree.node(c).children {
                        sub.push(Task::EnterListItemChild(cc, section_depth));
                    }
                }
            }
            push_all(tasks, sub);
        }
        NodeKind::ListItem => schedule_children(node, section_depth, tasks),
        NodeKind::DefinitionList | NodeKind::FieldList | NodeKind::Docinfo => {
            schedule_children(node, section_depth, tasks)
        }
        NodeKind::DefinitionListItem | NodeKind::Field => {
            out.push_str(".TP\n");
            schedule_children(node, section_depth, tasks);
        }
        NodeKind::Term | NodeKind::FieldName => schedule(node, section_depth, "\n", tasks),
        NodeKind::Classifier => {
            out.push_str(" : ");
            schedule(node, section_depth, "\n", tasks);
        }
        NodeKind::Definition | NodeKind::FieldBody => schedule_children(node, section_depth, tasks),
        NodeKind::Bibliographic { tag } => {
            let _ = writeln!(out, ".TP\n{}", tag);
            schedule_children(node, section_depth, tasks);
        }
        NodeKind::BlockQuote => {
            out.push_str(".RS\n");
            schedule(node, section_depth, ".RE\n", tasks);
        }
        NodeKind::Admonition { kind } => {
            let _ = writeln!(out, ".PP\n\\fB{}\\fR", kind);
            schedule_children(node, section_depth, tasks);
        }
        NodeKind::Container { .. } => schedule_children(node, section_depth, tasks),
        NodeKind::GenericAdmonition { title, .. } => {
            let _ = writeln!(out, ".PP\n\\fB{}\\fR", title);
            schedule_children(node, section_depth, tasks);
        }
        NodeKind::Epigraph { .. } => {
            out.push_str(".RS\n");
            schedule(node, section_depth, ".RE\n", tasks);
        }
        NodeKind::Toctree { .. } => {}
        NodeKind::Image { uri, .. } => {
            let _ = writeln!(out, "[image: {}]", uri);
        }
        NodeKind::Raw { format } => {
            if format == "manpage" || format == "troff" {
                for &c in &node.children {
                    if let NodeKind::Text(s) = &tree.node(c).kind {
                        out.push_str(s);
                    }
                }
            }
        }
        NodeKind::Comment => {
            for &c in &node.children {
                if let NodeKind::Text(s) = &tree.node(c).kind {
                    for line in s.split('\n') {
                        let _ = writeln!(out, ".\\\" {}", line);
                    }
                }
            }
        }
        NodeKind::Reference { refuri, .. } => {
            let close = if refuri.is_empty() {
                String::new()
            } else {
                format!(" <{}>", refuri)
            };
            schedule(node, section_depth, close, tasks);
        }
        NodeKind::Target { .. } | NodeKind::SubstitutionDefinition { .. } => {}
        NodeKind::SubstitutionReference { refname } => out.push_str(&escape(refname)),
        NodeKind::Table => schedule_children(node, section_depth, tasks),
        NodeKind::Tgroup { .. } | NodeKind::Thead | NodeKind::Tbody => {
            schedule_children(node, section_depth, tasks)
        }
        NodeKind::Colspec { .. } => {}
        NodeKind::Row => {
            let mut sub = Vec::new();
            for (i, &c) in node.children.iter().enumerate() {
                if i > 0 {
                    sub.push(Task::Append("\t".to_string()));
                }
                sub.push(Task::Enter(c, section_depth));
            }
            sub.push(Task::Append("\n".to_string()));
            push_all(tasks, sub);
        }
        NodeKind::Entry { .. } => {
            let mut sub = Vec::new();
            for &c in &node.children {
                if let NodeKind::Paragraph = &tree.node(c).kind {
                    for &cc in &tree.node(c).children {
                        sub.push(Task::Enter(cc, section_depth));
                    }
                } else {
                    sub.push(Task::Enter(c, section_depth));
                }
            }
            push_all(tasks, sub);
        }
        NodeKind::Attribution => {
            out.push_str(".RS\n-- ");
            schedule(node, section_depth, "\n.RE\n", tasks);
        }
        NodeKind::Figure => schedule_children(node, section_depth, tasks),
        NodeKind::Caption => {
            out.push_str(".PP\n\\fI");
            schedule(node, section_depth, "\\fR\n", tasks);
        }
        NodeKind::Legend => schedule_children(node, section_depth, tasks),
        NodeKind::Label | NodeKind::Footnote { .. } | NodeKind::Citation { .. } => {}
        NodeKind::FootnoteReference { .. } | NodeKind::CitationReference { .. } => {
            out.push_str("[*]");
        }
        NodeKind::Problematic { .. } => schedule_children(node, section_depth, tasks),
        NodeKind::SystemMessage { .. } => {}
        NodeKind::ObjectDescription { sig_text, .. } => {
            out.push_str(sig_text);
            out.push_str("\n");
            schedule_children(node, section_depth, tasks);
        }
        NodeKind::Extension { class_name, attrs } => {
            let visit = crate::plugins::invoke_node_visit(class_name, "man", attrs);
            if let Some(open) = &visit {
                out.push_str(open);
            }
            let close = if visit.is_some() {
                crate::plugins::invoke_node_depart(class_name, "man", attrs)
            } else {
                None
            };
            schedule(node, section_depth, close.unwrap_or_default(), tasks);
        }
        NodeKind::Abbreviation { .. }
        | NodeKind::Subscript
        | NodeKind::Superscript
        | NodeKind::Keyboard
        | NodeKind::Rubric
        | NodeKind::VersionModified { .. }
        | NodeKind::PendingXref { .. } => schedule_children(node, section_depth, tasks),
    }
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '-' => out.push_str("\\-"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doctree::Doctree;

    #[test]
    fn deeply_nested_manpage_does_not_use_call_stack() {
        let mut tree = Doctree::new_document("deep.man");
        let mut parent = tree.root();
        for _ in 0..10_000 {
            parent = tree.append(
                parent,
                NodeKind::Container {
                    classes: String::new(),
                },
            );
        }
        tree.append(parent, NodeKind::Text("deep".to_string()));

        let options = crate::cli::ManOptions::default();
        let common = crate::cli::CommonOptions::default();
        let rendered = manpage(&tree, &options, &common);
        assert!(rendered.contains("deep"));
    }
}
