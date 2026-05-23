//! Minimal manpage (troff) writer.
//!
//! Produces a small subset of `man(7)` output suitable for downstream
//! rendering. Not parity-gated against `docutils.writers.manpage`; see
//! `docs/compat.md` for the accepted-deviation note.

use crate::doctree::{Doctree, NodeId, NodeKind};
use std::fmt::Write as _;

pub fn manpage(tree: &Doctree) -> String {
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
    for &c in &tree.node(root).children {
        emit(tree, c, 0, &mut out);
    }
    out
}

fn emit(tree: &Doctree, id: NodeId, section_depth: usize, out: &mut String) {
    let node = tree.node(id);
    match &node.kind {
        NodeKind::Text(s) => out.push_str(&escape(s)),
        NodeKind::Document { .. } => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Section { .. } => {
            for &c in &node.children {
                emit(tree, c, section_depth + 1, out);
            }
        }
        NodeKind::Title => {
            let macro_name = match section_depth {
                0 | 1 => ".SH",
                _ => ".SS",
            };
            let _ = write!(out, "{} ", macro_name);
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push('\n');
        }
        NodeKind::Subtitle { .. } => {
            out.push_str(".SS ");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push('\n');
        }
        NodeKind::Transition => out.push_str(".br\n"),
        NodeKind::Paragraph => {
            out.push_str(".PP\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push('\n');
        }
        NodeKind::Emphasis | NodeKind::TitleReference => {
            out.push_str("\\fI");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\fR");
        }
        NodeKind::Strong => {
            out.push_str("\\fB");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\fR");
        }
        NodeKind::Literal => {
            out.push_str("\\fC");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\fR");
        }
        NodeKind::Inline { .. } => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
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
            for &c in &node.children {
                out.push_str(".IP \\(bu 2\n");
                if let NodeKind::ListItem = &tree.node(c).kind {
                    for &cc in &tree.node(c).children {
                        emit_li_child(tree, cc, section_depth, out);
                    }
                }
            }
        }
        NodeKind::EnumeratedList { .. } => {
            for (i, &c) in node.children.iter().enumerate() {
                let _ = writeln!(out, ".IP {}. 4", i + 1);
                if let NodeKind::ListItem = &tree.node(c).kind {
                    for &cc in &tree.node(c).children {
                        emit_li_child(tree, cc, section_depth, out);
                    }
                }
            }
        }
        NodeKind::ListItem => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::DefinitionList | NodeKind::FieldList | NodeKind::Docinfo => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::DefinitionListItem | NodeKind::Field => {
            out.push_str(".TP\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Term | NodeKind::FieldName => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push('\n');
        }
        NodeKind::Classifier => {
            out.push_str(" : ");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push('\n');
        }
        NodeKind::Definition | NodeKind::FieldBody => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Bibliographic { tag } => {
            let _ = writeln!(out, ".TP\n{}", tag);
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::BlockQuote => {
            out.push_str(".RS\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str(".RE\n");
        }
        NodeKind::Admonition { kind } => {
            let _ = writeln!(out, ".PP\n\\fB{}\\fR", kind);
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
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
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            if !refuri.is_empty() {
                let _ = write!(out, " <{}>", refuri);
            }
        }
        NodeKind::Target { .. } | NodeKind::SubstitutionDefinition { .. } => {}
        NodeKind::SubstitutionReference { refname } => out.push_str(&escape(refname)),
        NodeKind::Table => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Tgroup { .. } | NodeKind::Thead | NodeKind::Tbody => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Colspec { .. } => {}
        NodeKind::Row => {
            let kids = &node.children;
            for (i, &c) in kids.iter().enumerate() {
                if i > 0 {
                    out.push_str("\t");
                }
                emit(tree, c, section_depth, out);
            }
            out.push('\n');
        }
        NodeKind::Entry { .. } => {
            for &c in &node.children {
                if let NodeKind::Paragraph = &tree.node(c).kind {
                    for &cc in &tree.node(c).children {
                        emit(tree, cc, section_depth, out);
                    }
                } else {
                    emit(tree, c, section_depth, out);
                }
            }
        }
        NodeKind::Attribution => {
            out.push_str(".RS\n-- ");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\n.RE\n");
        }
        NodeKind::Figure => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Caption => {
            out.push_str(".PP\n\\fI");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\fR\n");
        }
        NodeKind::Legend => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Label | NodeKind::Footnote { .. } | NodeKind::Citation { .. } => {}
        NodeKind::FootnoteReference { .. } | NodeKind::CitationReference { .. } => {
            out.push_str("[*]");
        }
        NodeKind::Problematic { .. } => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::SystemMessage { .. } => {}
    }
}

fn emit_li_child(tree: &Doctree, id: NodeId, section_depth: usize, out: &mut String) {
    // Inside an `.IP`, the first paragraph should not introduce its own
    // `.PP` (which resets indentation).
    if let NodeKind::Paragraph = &tree.node(id).kind {
        for &c in &tree.node(id).children {
            emit(tree, c, section_depth, out);
        }
        out.push('\n');
    } else {
        emit(tree, id, section_depth, out);
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
