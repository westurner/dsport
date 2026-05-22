//! Minimal HTML5 writer.
//!
//! Produces a small, predictable subset of `docutils.writers.html5_polyglot`
//! output suitable for downstream consumers that just need semantic HTML
//! for the supported node kinds. This is intentionally not a parity
//! gate; see `docs/compat.md` for the accepted-deviation note.
//!
//! Output is a fragment (no `<html>`/`<head>`/`<body>` envelope).

use crate::doctree::{Doctree, NodeId, NodeKind};
use std::fmt::Write as _;

pub fn html5(tree: &Doctree) -> String {
    let mut out = String::new();
    let root = tree.root();
    for &c in &tree.node(root).children {
        emit(tree, c, &mut out);
    }
    out
}

fn emit(tree: &Doctree, id: NodeId, out: &mut String) {
    let node = tree.node(id);
    match &node.kind {
        NodeKind::Text(s) => out.push_str(&escape(s)),
        NodeKind::Document { .. } => {
            for &c in &node.children {
                emit(tree, c, out);
            }
        }
        NodeKind::Section { ids, .. } => {
            let _ = write!(out, "<section id=\"{ids}\">");
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</section>");
        }
        NodeKind::Title => {
            // Heading level is determined by ancestor section depth; we
            // approximate as `<h1>` and rely on CSS for visual depth.
            out.push_str("<h1>");
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</h1>");
        }
        NodeKind::Subtitle { .. } => {
            out.push_str("<p class=\"subtitle\">");
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</p>");
        }
        NodeKind::Transition => out.push_str("<hr/>"),
        NodeKind::Paragraph => wrap(tree, &node.children, "p", out),
        NodeKind::Emphasis => wrap(tree, &node.children, "em", out),
        NodeKind::Strong => wrap(tree, &node.children, "strong", out),
        NodeKind::Literal => wrap(tree, &node.children, "code", out),
        NodeKind::TitleReference => {
            out.push_str("<cite>");
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</cite>");
        }
        NodeKind::Inline { classes } => {
            if classes.is_empty() {
                out.push_str("<span>");
            } else {
                let _ = write!(out, "<span class=\"{classes}\">");
            }
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</span>");
        }
        NodeKind::LiteralBlock { classes } => {
            if classes.is_empty() {
                out.push_str("<pre>");
            } else {
                let _ = write!(out, "<pre class=\"{classes}\">");
            }
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</pre>");
        }
        NodeKind::BulletList { .. } => wrap(tree, &node.children, "ul", out),
        NodeKind::EnumeratedList { .. } => wrap(tree, &node.children, "ol", out),
        NodeKind::ListItem => wrap(tree, &node.children, "li", out),
        NodeKind::DefinitionList => wrap(tree, &node.children, "dl", out),
        NodeKind::DefinitionListItem => {
            for &c in &node.children {
                emit(tree, c, out);
            }
        }
        NodeKind::Term => wrap(tree, &node.children, "dt", out),
        NodeKind::Classifier => {
            out.push_str("<span class=\"classifier\">");
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</span>");
        }
        NodeKind::Definition => wrap(tree, &node.children, "dd", out),
        NodeKind::FieldList => wrap(tree, &node.children, "dl", out),
        NodeKind::Field => {
            for &c in &node.children {
                emit(tree, c, out);
            }
        }
        NodeKind::FieldName => wrap(tree, &node.children, "dt", out),
        NodeKind::FieldBody => wrap(tree, &node.children, "dd", out),
        NodeKind::Docinfo => wrap(tree, &node.children, "dl", out),
        NodeKind::Bibliographic { tag } => {
            let _ = write!(out, "<dt>{tag}</dt><dd>");
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</dd>");
        }
        NodeKind::BlockQuote => wrap(tree, &node.children, "blockquote", out),
        NodeKind::Admonition { kind } => {
            let _ = write!(out, "<aside class=\"{kind}\">");
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</aside>");
        }
        NodeKind::Image { uri, alt, .. } => {
            let _ = write!(out, "<img src=\"{}\"", escape(uri));
            if let Some(a) = alt {
                let _ = write!(out, " alt=\"{}\"", escape(a));
            }
            out.push_str("/>");
        }
        NodeKind::Raw { format } => {
            if format == "html" {
                // Raw HTML passes through.
                for &c in &node.children {
                    if let NodeKind::Text(s) = &tree.node(c).kind {
                        out.push_str(s);
                    }
                }
            }
        }
        NodeKind::Comment => {
            out.push_str("<!-- ");
            for &c in &node.children {
                if let NodeKind::Text(s) = &tree.node(c).kind {
                    out.push_str(&escape(s));
                }
            }
            out.push_str(" -->");
        }
        NodeKind::Reference { refuri, .. } => {
            let _ = write!(out, "<a href=\"{}\">", escape(refuri));
            for &c in &node.children {
                emit(tree, c, out);
            }
            out.push_str("</a>");
        }
        NodeKind::Target { .. } => {}
        NodeKind::SubstitutionDefinition { .. } => {}
        NodeKind::SubstitutionReference { refname } => {
            let _ = write!(out, "{}", escape(refname));
        }
        NodeKind::Table => wrap(tree, &node.children, "table", out),
        NodeKind::Tgroup { .. } => {
            for &c in &node.children {
                emit(tree, c, out);
            }
        }
        NodeKind::Colspec { .. } => {}
        NodeKind::Thead => wrap(tree, &node.children, "thead", out),
        NodeKind::Tbody => wrap(tree, &node.children, "tbody", out),
        NodeKind::Row => wrap(tree, &node.children, "tr", out),
        NodeKind::Entry => wrap(tree, &node.children, "td", out),
    }
}

fn wrap(tree: &Doctree, children: &[NodeId], tag: &str, out: &mut String) {
    let _ = write!(out, "<{tag}>");
    for &c in children {
        emit(tree, c, out);
    }
    let _ = write!(out, "</{tag}>");
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}
