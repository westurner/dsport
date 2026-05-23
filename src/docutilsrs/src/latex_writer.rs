//! Minimal LaTeX writer.
//!
//! Produces a self-contained LaTeX document body suitable for
//! downstream consumers that want to render the supported node kinds.
//! Output is not parity-gated against `docutils.writers.latex2e`; see
//! `docs/compat.md` for the accepted-deviation note.

use crate::doctree::{Doctree, NodeId, NodeKind};
use std::fmt::Write as _;

const SECTION_CMDS: &[&str] = &[
    "\\section",
    "\\subsection",
    "\\subsubsection",
    "\\paragraph",
    "\\subparagraph",
];

pub fn latex(tree: &Doctree) -> String {
    let mut out = String::new();
    out.push_str("\\documentclass{article}\n");
    out.push_str("\\usepackage[utf8]{inputenc}\n");
    out.push_str("\\usepackage{hyperref}\n");
    out.push_str("\\begin{document}\n");
    let root = tree.root();
    if let NodeKind::Document { title, .. } = &tree.node(root).kind {
        if !title.is_empty() {
            let _ = writeln!(out, "\\title{{{}}}", escape(title));
            out.push_str("\\maketitle\n");
        }
    }
    for &c in &tree.node(root).children {
        emit(tree, c, 0, &mut out);
    }
    out.push_str("\\end{document}\n");
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
            let cmd = SECTION_CMDS
                .get(section_depth.saturating_sub(1))
                .unwrap_or(&"\\paragraph");
            let _ = write!(out, "{cmd}{{");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("}\n");
        }
        NodeKind::Subtitle { .. } => {
            out.push_str("\\subsection*{");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("}\n");
        }
        NodeKind::Transition => out.push_str("\\hrulefill\n"),
        NodeKind::Paragraph => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\n\n");
        }
        NodeKind::Emphasis => wrap_cmd(tree, &node.children, "\\emph", section_depth, out),
        NodeKind::Strong => wrap_cmd(tree, &node.children, "\\textbf", section_depth, out),
        NodeKind::Literal => wrap_cmd(tree, &node.children, "\\texttt", section_depth, out),
        NodeKind::TitleReference => wrap_cmd(tree, &node.children, "\\textit", section_depth, out),
        NodeKind::Inline { .. } => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Math { latex } => {
            // Inline math in LaTeX output round-trips as `$…$`.
            out.push('$');
            out.push_str(latex);
            out.push('$');
        }
        NodeKind::MathBlock { latex } => {
            // Display math in LaTeX output uses the `equation*`
            // environment (matches docutils' `latex2e` writer for
            // `<math_block>` without a label).
            out.push_str("\\begin{equation*}\n");
            out.push_str(latex);
            if !latex.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("\\end{equation*}\n");
        }
        NodeKind::LiteralBlock { .. } => {
            out.push_str("\\begin{verbatim}\n");
            for &c in &node.children {
                if let NodeKind::Text(s) = &tree.node(c).kind {
                    out.push_str(s);
                }
            }
            out.push_str("\n\\end{verbatim}\n");
        }
        NodeKind::BulletList { .. } => {
            out.push_str("\\begin{itemize}\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\end{itemize}\n");
        }
        NodeKind::EnumeratedList { .. } => {
            out.push_str("\\begin{enumerate}\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\end{enumerate}\n");
        }
        NodeKind::ListItem => {
            out.push_str("\\item ");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::DefinitionList | NodeKind::FieldList | NodeKind::Docinfo => {
            out.push_str("\\begin{description}\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\end{description}\n");
        }
        NodeKind::DefinitionListItem | NodeKind::Field => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Term | NodeKind::FieldName => {
            out.push_str("\\item[");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("] ");
        }
        NodeKind::Classifier => {
            out.push_str(" : ");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Definition | NodeKind::FieldBody => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Bibliographic { tag } => {
            let _ = write!(out, "\\item[{}] ", escape(tag));
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::BlockQuote => {
            out.push_str("\\begin{quote}\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\end{quote}\n");
        }
        NodeKind::Admonition { kind } => {
            let _ = writeln!(out, "\\textbf{{{}}}\\par", escape(kind));
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Image { uri, .. } => {
            let _ = writeln!(out, "\\includegraphics{{{}}}", escape(uri));
        }
        NodeKind::Raw { format } => {
            if format == "latex" {
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
                    out.push_str("% ");
                    out.push_str(s);
                    out.push('\n');
                }
            }
        }
        NodeKind::Reference { refuri, .. } => {
            if refuri.is_empty() {
                for &c in &node.children {
                    emit(tree, c, section_depth, out);
                }
            } else {
                let _ = write!(out, "\\href{{{}}}{{", escape(refuri));
                for &c in &node.children {
                    emit(tree, c, section_depth, out);
                }
                out.push('}');
            }
        }
        NodeKind::Target { .. } => {}
        NodeKind::SubstitutionDefinition { .. } => {}
        NodeKind::SubstitutionReference { refname } => {
            out.push_str(&escape(refname));
        }
        NodeKind::Table => {
            // Minimal: tabular with l columns; counts cols from first row.
            let cols = first_row_len(tree, id);
            if cols == 0 {
                return;
            }
            let spec: String = std::iter::repeat('l').take(cols).collect();
            let _ = writeln!(out, "\\begin{{tabular}}{{{}}}", spec);
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\end{tabular}\n");
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
                    out.push_str(" & ");
                }
                emit(tree, c, section_depth, out);
            }
            out.push_str(" \\\\\n");
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
            out.push_str("\\par\\hfill --- ");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push('\n');
        }
        NodeKind::Figure => {
            out.push_str("\\begin{figure}[h]\n\\centering\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("\\end{figure}\n");
        }
        NodeKind::Caption => {
            out.push_str("\\caption{");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("}\n");
        }
        NodeKind::Legend => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Label | NodeKind::Footnote { .. } | NodeKind::Citation { .. } => {
            // Rendered inline by their references; suppress definitions.
        }
        NodeKind::FootnoteReference { .. } | NodeKind::CitationReference { .. } => {
            out.push_str("\\footnotemark{}");
        }
        NodeKind::Problematic { .. } => {
            out.push_str("\\textcolor{red}{");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push('}');
        }
        NodeKind::SystemMessage { .. } => {
            // Suppressed in body output.
        }
    }
}

fn wrap_cmd(
    tree: &Doctree,
    children: &[NodeId],
    cmd: &str,
    section_depth: usize,
    out: &mut String,
) {
    let _ = write!(out, "{cmd}{{");
    for &c in children {
        emit(tree, c, section_depth, out);
    }
    out.push('}');
}

fn first_row_len(tree: &Doctree, table_id: NodeId) -> usize {
    fn walk(tree: &Doctree, id: NodeId) -> Option<usize> {
        let n = tree.node(id);
        if let NodeKind::Row = &n.kind {
            return Some(n.children.len());
        }
        for &c in &n.children {
            if let Some(x) = walk(tree, c) {
                return Some(x);
            }
        }
        None
    }
    walk(tree, table_id).unwrap_or(0)
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\textbackslash{}"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '$' => out.push_str("\\$"),
            '&' => out.push_str("\\&"),
            '%' => out.push_str("\\%"),
            '#' => out.push_str("\\#"),
            '_' => out.push_str("\\_"),
            '~' => out.push_str("\\textasciitilde{}"),
            '^' => out.push_str("\\textasciicircum{}"),
            _ => out.push(ch),
        }
    }
    out
}
