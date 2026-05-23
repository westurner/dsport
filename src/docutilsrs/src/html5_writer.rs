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

pub fn html5(tree: &Doctree, options: &crate::cli::Html5Options, common: &crate::cli::CommonOptions) -> String {
    let mut out = String::new();
    let root = tree.root();
    for &c in &tree.node(root).children {
        emit(tree, c, &mut out, options, common);
    }
    out
}

fn emit(tree: &Doctree, id: NodeId, out: &mut String, options: &crate::cli::Html5Options, common: &crate::cli::CommonOptions) {
    let node = tree.node(id);
    match &node.kind {
        NodeKind::Text(s) => out.push_str(&escape(s)),
        NodeKind::Document { .. } => {
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
        }
        NodeKind::Section { ids, .. } => {
            let _ = write!(out, "<section id=\"{ids}\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</section>");
        }
        NodeKind::Title => {
            // Heading level is determined by ancestor section depth; we
            // approximate as `<h1>` and rely on CSS for visual depth.
            out.push_str("<h1>");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</h1>");
        }
        NodeKind::Subtitle { .. } => {
            out.push_str("<p class=\"subtitle\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</p>");
        }
        NodeKind::Transition => out.push_str("<hr/>"),
        NodeKind::Paragraph => wrap(tree, &node.children, "p", out, options, common),
        NodeKind::Emphasis => wrap(tree, &node.children, "em", out, options, common),
        NodeKind::Strong => wrap(tree, &node.children, "strong", out, options, common),
        NodeKind::Literal => wrap(tree, &node.children, "code", out, options, common),
        NodeKind::TitleReference => {
            out.push_str("<cite>");
            for &c in &node.children {
                emit(tree, c, out, options, common);
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
                emit(tree, c, out, options, common);
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
                emit(tree, c, out, options, common);
            }
            out.push_str("</pre>");
        }
        NodeKind::BulletList { .. } => wrap(tree, &node.children, "ul", out, options, common),
        NodeKind::EnumeratedList { .. } => wrap(tree, &node.children, "ol", out, options, common),
        NodeKind::ListItem => wrap(tree, &node.children, "li", out, options, common),
        NodeKind::DefinitionList => wrap(tree, &node.children, "dl", out, options, common),
        NodeKind::DefinitionListItem => {
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
        }
        NodeKind::Term => wrap(tree, &node.children, "dt", out, options, common),
        NodeKind::Classifier => {
            out.push_str("<span class=\"classifier\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</span>");
        }
        NodeKind::Definition => wrap(tree, &node.children, "dd", out, options, common),
        NodeKind::FieldList => wrap(tree, &node.children, "dl", out, options, common),
        NodeKind::Field => {
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
        }
        NodeKind::FieldName => wrap(tree, &node.children, "dt", out, options, common),
        NodeKind::FieldBody => wrap(tree, &node.children, "dd", out, options, common),
        NodeKind::Docinfo => wrap(tree, &node.children, "dl", out, options, common),
        NodeKind::Bibliographic { tag } => {
            let _ = write!(out, "<dt>{tag}</dt><dd>");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</dd>");
        }
        NodeKind::BlockQuote => wrap(tree, &node.children, "blockquote", out, options, common),
        NodeKind::Admonition { kind } => {
            let _ = write!(out, "<aside class=\"{kind}\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
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
        NodeKind::Math { latex } => {
            let backend = get_math_backend(options);
            out.push_str(&mathrenderrs::render(
                backend,
                mathrenderrs::MathDisplay::Inline,
                latex,
            ));
        }
        NodeKind::MathBlock { latex } => {
            let backend = get_math_backend(options);
            out.push_str(&mathrenderrs::render(
                backend,
                mathrenderrs::MathDisplay::Block,
                latex,
            ));
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
            let is_mailto = refuri.starts_with("mailto:");
            let should_cloak = is_mailto && options.cloak_email_addresses.is_some();
            let mut uri = escape(refuri);
            if should_cloak {
                uri = uri.replace("@", "&#37;&#52;&#48;").replace(".", "&#46;");
            }
            let _ = write!(out, "<a href=\"{}\">", uri);
            if should_cloak {
                for &c in &node.children {
                    if let NodeKind::Text(s) = &tree.node(c).kind {
                        let mut cloaked = escape(s);
                        cloaked = cloaked.replace("@", "&#64;").replace(".", "&#46;");
                        out.push_str(&cloaked);
                    } else {
                        emit(tree, c, out, options, common);
                    }
                }
            } else {
                for &c in &node.children {
                    emit(tree, c, out, options, common);
                }
            }
            out.push_str("</a>");
        }
        NodeKind::Target { .. } => {}
        NodeKind::SubstitutionDefinition { .. } => {}
        NodeKind::SubstitutionReference { refname } => {
            let _ = write!(out, "{}", escape(refname));
        }
        NodeKind::Table => {
            let mut classes = vec![];
            if let Some(style) = &options.table_style {
                classes.extend(style.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()));
            }
            if classes.is_empty() {
                wrap(tree, &node.children, "table", out, options, common)
            } else {
                let _ = write!(out, "<table class=\"{}\">", classes.join(" "));
                for &c in &node.children {
                    emit(tree, c, out, options, common);
                }
                out.push_str("</table>");
            }
        }
        NodeKind::Tgroup { .. } => {
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
        }
        NodeKind::Colspec { .. } => {}
        NodeKind::Thead => wrap(tree, &node.children, "thead", out, options, common),
        NodeKind::Tbody => wrap(tree, &node.children, "tbody", out, options, common),
        NodeKind::Row => wrap(tree, &node.children, "tr", out, options, common),
        NodeKind::Entry { morecols, morerows } => {
            let mut tag = String::from("<td");
            if *morecols > 0 {
                let _ = write!(tag, " colspan=\"{}\"", morecols + 1);
            }
            if *morerows > 0 {
                let _ = write!(tag, " rowspan=\"{}\"", morerows + 1);
            }
            tag.push('>');
            out.push_str(&tag);
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</td>");
        }
        NodeKind::Attribution => {
            out.push_str("<p class=\"attribution\">— ");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</p>");
        }
        NodeKind::Figure => wrap(tree, &node.children, "figure", out, options, common),
        NodeKind::Caption => wrap(tree, &node.children, "figcaption", out, options, common),
        NodeKind::Legend => {
            out.push_str("<div class=\"legend\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</div>");
        }
        NodeKind::Label => {
            out.push_str("<span class=\"label\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</span>");
        }
        NodeKind::Footnote { ids, .. } => {
            let _ = write!(out, "<aside class=\"footnote\" id=\"{ids}\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</aside>");
        }
        NodeKind::FootnoteReference { refid, .. } => {
            let style = options.footnote_references.as_deref().unwrap_or("brackets");
            if style == "brackets" {
                let _ = write!(out, "<a class=\"{style}\" href=\"#{refid}\" id=\"footnote-reference-1\" role=\"doc-noteref\">");
                out.push_str("<span class=\"fn-bracket\">[</span>");
                for &c in &node.children {
                    emit(tree, c, out, options, common);
                }
                out.push_str("<span class=\"fn-bracket\">]</span></a>");
            } else {
                let _ = write!(out, "<a class=\"{style}\" href=\"#{refid}\" id=\"footnote-reference-1\" role=\"doc-noteref\">");
                for &c in &node.children {
                    emit(tree, c, out, options, common);
                }
                out.push_str("</a>");
            }
        }
        NodeKind::Citation { ids, .. } => {
            let _ = write!(out, "<aside class=\"citation\" id=\"{ids}\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</aside>");
        }
        NodeKind::CitationReference { refid, .. } => {
            let _ = write!(out, "<a class=\"citation-reference\" href=\"#{refid}\">");
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</a>");
        }
        NodeKind::Problematic { refid, ids } => {
            let _ = write!(
                out,
                "<a class=\"problematic\" id=\"{ids}\" href=\"#{refid}\">"
            );
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</a>");
        }
        NodeKind::SystemMessage { level, ty, .. } => {
            let _ = write!(
                out,
                "<aside class=\"system-message level-{level} type-{ty}\">"
            );
            for &c in &node.children {
                emit(tree, c, out, options, common);
            }
            out.push_str("</aside>");
        }
    }
}

fn wrap(tree: &Doctree, children: &[NodeId], tag: &str, out: &mut String, options: &crate::cli::Html5Options, common: &crate::cli::CommonOptions) {
    let _ = write!(out, "<{tag}>");
    for &c in children {
        emit(tree, c, out, options, common);
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

fn get_math_backend(options: &crate::cli::Html5Options) -> mathrenderrs::MathBackend {
    if let Some(format) = &options.math_output {
        let fmt = format.split_whitespace().next().unwrap_or("").to_lowercase();
        match fmt.as_str() {
            "mathjax" => mathrenderrs::MathBackend::MathJax,
            "html" | "mathml" => mathrenderrs::MathBackend::Ratex, // Fallback since MathML isn't natively MathML here in our backend yet.
            _ => mathrenderrs::MathBackend::default(),
        }
    } else {
        mathrenderrs::MathBackend::default()
    }
}
