//! Minimal plain-text writer.
//!
//! There is no upstream `docutils.writers.text`; Sphinx's own `text`
//! builder uses `sphinx.writers.text.TextWriter`, a purpose-built writer
//! with full line-wrapping, table-drawing, and indentation-stack logic.
//! Reproducing that in full is out of scope here — this module is a
//! **deliberately simplified, accepted-deviation** plain-text renderer:
//! no line wrapping (each source line/paragraph becomes one or more
//! output lines verbatim), a fixed 3-column indent step, and no visual
//! table-drawing (table rows are rendered as `cell | cell | cell`).
//!
//! Covered constructs: sections (title + underline), paragraphs, emphasis/
//! strong/literal/title-reference inline markup (rendered with their
//! original RST marker characters), bullet/enumerated lists, definition
//! lists, field lists, block quotes (+ attribution), admonitions, literal
//! blocks, transitions, footnotes/citations (+ their inline references),
//! inline/block math, and simple tables. Images, raw passthrough,
//! comments, and system messages/problematic nodes are intentionally
//! silent (matching how docutils' own non-XML writers treat comments).

use crate::doctree::{Doctree, NodeId, NodeKind};

const INDENT_STEP: usize = 3;

/// Render `tree` as plain text.
pub fn text(tree: &Doctree) -> String {
    let mut out = String::new();
    let mut blocks: Vec<String> = Vec::new();
    // A lone top-level section is promoted into `Document.title` by the
    // parser (`promote_document_title`), which also hoists that section's
    // children up to be the document's direct children. Render the
    // promoted title as the top heading first, then walk the (already
    // flattened) children at depth 0 rather than depth 1.
    if let NodeKind::Document { title, .. } = &tree.node(tree.root()).kind {
        if !title.is_empty() {
            let underline = section_underline_char(0).to_string().repeat(display_width(title));
            blocks.push(format!("{title}\n{underline}"));
        }
    }
    for &child in &tree.node(tree.root()).children {
        render_block(tree, child, 0, &mut blocks);
    }
    for (i, block) in blocks.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(block);
        if !block.ends_with('\n') {
            out.push('\n');
        }
    }
    out
}

fn indent_lines(s: &str, columns: usize) -> String {
    let pad = " ".repeat(columns);
    s.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{pad}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render a single block-level node (and everything nested under it,
/// recursively) into zero or more strings appended to `blocks`.
fn render_block(tree: &Doctree, id: NodeId, depth: usize, blocks: &mut Vec<String>) {
    let node = tree.node(id);
    match &node.kind {
        NodeKind::Section { .. } => {
            let mut title_text = String::new();
            let mut body_blocks: Vec<String> = Vec::new();
            for &child in &node.children {
                if matches!(tree.node(child).kind, NodeKind::Title) {
                    title_text = inline_text(tree, child);
                } else {
                    render_block(tree, child, depth + 1, &mut body_blocks);
                }
            }
            if !title_text.is_empty() {
                let underline_char = section_underline_char(depth);
                let underline: String =
                    underline_char.to_string().repeat(display_width(&title_text));
                blocks.push(format!("{title_text}\n{underline}"));
            }
            blocks.extend(body_blocks);
        }
        NodeKind::Title => {
            // Only reached for a stray/unparented Title; render as a plain line.
            blocks.push(inline_text(tree, id));
        }
        NodeKind::Subtitle { .. } => {
            // Promoted by `promote_document_title` when a document has a
            // single top-level section containing a single nested section;
            // render one level deeper than the document title itself.
            let t = inline_text(tree, id);
            let underline = section_underline_char(1).to_string().repeat(display_width(&t));
            blocks.push(format!("{t}\n{underline}"));
        }
        NodeKind::Transition => blocks.push("-".repeat(20)),
        NodeKind::Paragraph => blocks.push(inline_text(tree, id)),
        NodeKind::LiteralBlock { .. } => {
            blocks.push(indent_lines(&inline_text(tree, id), INDENT_STEP));
        }
        NodeKind::MathBlock { latex } => blocks.push(indent_lines(latex, INDENT_STEP)),
        NodeKind::BulletList { bullet } => {
            for &item in &node.children {
                let mut item_blocks = Vec::new();
                for &c in &tree.node(item).children {
                    render_block(tree, c, depth, &mut item_blocks);
                }
                blocks.push(bullet_item(*bullet, &item_blocks));
            }
        }
        NodeKind::EnumeratedList { prefix, suffix, .. } => {
            for (i, &item) in node.children.iter().enumerate() {
                let mut item_blocks = Vec::new();
                for &c in &tree.node(item).children {
                    render_block(tree, c, depth, &mut item_blocks);
                }
                let marker = format!("{prefix}{}{suffix} ", i + 1);
                blocks.push(numbered_item(&marker, &item_blocks));
            }
        }
        NodeKind::DefinitionList => {
            for &item in &node.children {
                render_block(tree, item, depth, blocks);
            }
        }
        NodeKind::DefinitionListItem => {
            let mut term_line = String::new();
            let mut def_blocks = Vec::new();
            for &c in &node.children {
                match &tree.node(c).kind {
                    NodeKind::Term => term_line.push_str(&inline_text(tree, c)),
                    NodeKind::Classifier => {
                        term_line.push_str(" : ");
                        term_line.push_str(&inline_text(tree, c));
                    }
                    NodeKind::Definition => {
                        for &dc in &tree.node(c).children {
                            render_block(tree, dc, depth, &mut def_blocks);
                        }
                    }
                    _ => {}
                }
            }
            blocks.push(term_line);
            blocks.extend(def_blocks.into_iter().map(|b| indent_lines(&b, INDENT_STEP)));
        }
        NodeKind::FieldList => {
            for &field in &node.children {
                render_block(tree, field, depth, blocks);
            }
        }
        NodeKind::Field => {
            let mut name = String::new();
            let mut body_blocks = Vec::new();
            for &c in &node.children {
                match &tree.node(c).kind {
                    NodeKind::FieldName => name = inline_text(tree, c),
                    NodeKind::FieldBody => {
                        for &bc in &tree.node(c).children {
                            render_block(tree, bc, depth, &mut body_blocks);
                        }
                    }
                    _ => {}
                }
            }
            let body = body_blocks.join(" ");
            if body.contains('\n') || body.is_empty() {
                blocks.push(format!(":{name}:"));
                blocks.extend(body_blocks.into_iter().map(|b| indent_lines(&b, INDENT_STEP)));
            } else {
                blocks.push(format!(":{name}: {body}"));
            }
        }
        NodeKind::Docinfo => {
            for &c in &node.children {
                render_block(tree, c, depth, blocks);
            }
        }
        NodeKind::Bibliographic { tag } => blocks.push(format!(":{tag}: {}", inline_text(tree, id))),
        NodeKind::BlockQuote => {
            let mut inner = Vec::new();
            for &c in &node.children {
                if matches!(tree.node(c).kind, NodeKind::Attribution) {
                    inner.push(format!("-- {}", inline_text(tree, c)));
                } else {
                    render_block(tree, c, depth, &mut inner);
                }
            }
            blocks.extend(inner.into_iter().map(|b| indent_lines(&b, INDENT_STEP)));
        }
        NodeKind::Admonition { kind } => {
            let mut inner = Vec::new();
            for &c in &node.children {
                render_block(tree, c, depth, &mut inner);
            }
            let title = format!("{}{}:", &kind[..1].to_uppercase(), &kind[1..]);
            blocks.push(title);
            blocks.extend(inner.into_iter().map(|b| indent_lines(&b, INDENT_STEP)));
        }
        NodeKind::Figure => {
            let mut inner = Vec::new();
            for &c in &node.children {
                match &tree.node(c).kind {
                    NodeKind::Image { .. } => {} // no textual content
                    NodeKind::Caption => inner.push(inline_text(tree, c)),
                    NodeKind::Legend => {
                        for &lc in &tree.node(c).children {
                            render_block(tree, lc, depth, &mut inner);
                        }
                    }
                    _ => render_block(tree, c, depth, &mut inner),
                }
            }
            blocks.extend(inner);
        }
        NodeKind::Footnote {
            ids: _,
            names,
            backrefs: _,
            auto: _,
        } => {
            let mut inner = Vec::new();
            for &c in &node.children {
                if !matches!(tree.node(c).kind, NodeKind::Label) {
                    render_block(tree, c, depth, &mut inner);
                }
            }
            let label = if names.is_empty() { "*".to_string() } else { names.clone() };
            if let Some(first) = inner.first() {
                blocks.push(format!("[{label}] {first}"));
                blocks.extend(inner.into_iter().skip(1));
            } else {
                blocks.push(format!("[{label}]"));
            }
        }
        NodeKind::Citation { ids: _, names, backrefs: _ } => {
            let mut inner = Vec::new();
            for &c in &node.children {
                if !matches!(tree.node(c).kind, NodeKind::Label) {
                    render_block(tree, c, depth, &mut inner);
                }
            }
            if let Some(first) = inner.first() {
                blocks.push(format!("[{names}] {first}"));
                blocks.extend(inner.into_iter().skip(1));
            } else {
                blocks.push(format!("[{names}]"));
            }
        }
        NodeKind::Table => {
            let mut rows: Vec<String> = Vec::new();
            collect_table_rows(tree, id, &mut rows);
            blocks.push(rows.join("\n"));
        }
        // Image / Raw / Comment / SystemMessage / Problematic: silent.
        NodeKind::Image { .. }
        | NodeKind::Raw { .. }
        | NodeKind::Comment
        | NodeKind::SystemMessage { .. }
        | NodeKind::Problematic { .. } => {}
        NodeKind::Extension { class_name, attrs } => {
            let mut inner = Vec::new();
            for &c in &node.children {
                render_block(tree, c, depth, &mut inner);
            }
            let visit = crate::plugins::invoke_node_visit(class_name, "text", attrs);
            let depart = crate::plugins::invoke_node_depart(class_name, "text", attrs);
            if visit.is_some() || depart.is_some() {
                if let Some(open) = visit {
                    if let Some(first) = inner.first_mut() {
                        *first = format!("{open}{first}");
                    } else {
                        inner.push(open);
                    }
                }
                if let Some(close) = depart {
                    if let Some(last) = inner.last_mut() {
                        last.push_str(&close);
                    } else {
                        inner.push(close);
                    }
                }
            }
            blocks.extend(inner);
        }
        // Anything else that reaches block position (targets, substitution
        // defs, labels, etc.) has no standalone textual form.
        _ => {
            for &c in &node.children {
                render_block(tree, c, depth, blocks);
            }
        }
    }
}

fn bullet_item(bullet: char, item_blocks: &[String]) -> String {
    prefix_item(&format!("{bullet} "), item_blocks)
}

fn numbered_item(marker: &str, item_blocks: &[String]) -> String {
    prefix_item(marker, item_blocks)
}

fn prefix_item(marker: &str, item_blocks: &[String]) -> String {
    let pad = " ".repeat(marker.chars().count());
    let joined = item_blocks.join("\n\n");
    let mut out = String::new();
    for (i, line) in joined.lines().enumerate() {
        if i == 0 {
            out.push_str(marker);
        } else {
            out.push_str(&pad);
        }
        out.push_str(line);
        out.push('\n');
    }
    out.trim_end_matches('\n').to_string()
}

fn collect_table_rows(tree: &Doctree, id: NodeId, rows: &mut Vec<String>) {
    let node = tree.node(id);
    match &node.kind {
        NodeKind::Row => {
            let cells: Vec<String> = node
                .children
                .iter()
                .map(|&c| inline_text(tree, c).replace('\n', " "))
                .collect();
            rows.push(cells.join(" | "));
        }
        _ => {
            for &c in &node.children {
                collect_table_rows(tree, c, rows);
            }
        }
    }
}

/// `SECTIONING_CHARS`-style rotation, independent of `sphinxdocrs`'s own
/// constant of the same shape (this crate has no dependency on that one).
const SECTION_UNDERLINES: [char; 4] = ['=', '-', '~', '"'];

fn section_underline_char(depth: usize) -> char {
    SECTION_UNDERLINES[depth.min(SECTION_UNDERLINES.len() - 1)]
}

/// Character-count width (not true terminal display width) used to size
/// the underline under a heading. Good enough for ASCII titles; wide CJK
/// characters will under-count, an accepted deviation.
fn display_width(s: &str) -> usize {
    s.chars().count().max(1)
}

/// Flatten the inline content of `id` into a single-line (or, for a
/// literal block, multi-line) string, applying simple RST-style markers
/// for emphasis/strong/literal/title-reference.
fn inline_text(tree: &Doctree, id: NodeId) -> String {
    let node = tree.node(id);
    match &node.kind {
        NodeKind::Text(s) => s.clone(),
        NodeKind::Emphasis => wrap_children(tree, id, "*", "*"),
        NodeKind::Strong => wrap_children(tree, id, "**", "**"),
        NodeKind::Literal => wrap_children(tree, id, "`", "`"),
        NodeKind::TitleReference => wrap_children(tree, id, "\u{2018}", "\u{2019}"),
        NodeKind::Math { latex } => format!(":math:`{latex}`"),
        NodeKind::FootnoteReference { ids: _, refid: _, auto: _ } => "[?]".to_string(),
        NodeKind::CitationReference { ids: _, refid: _ } => "[?]".to_string(),
        NodeKind::Reference { name, .. } => {
            let inner = children_inline(tree, id);
            if inner.is_empty() {
                name.clone()
            } else {
                inner
            }
        }
        NodeKind::Image { .. } | NodeKind::Comment | NodeKind::Raw { .. } => String::new(),
        _ => children_inline(tree, id),
    }
}

fn children_inline(tree: &Doctree, id: NodeId) -> String {
    tree.node(id)
        .children
        .iter()
        .map(|&c| inline_text(tree, c))
        .collect::<Vec<_>>()
        .join("")
}

fn wrap_children(tree: &Doctree, id: NodeId, open: &str, close: &str) -> String {
    format!("{open}{}{close}", children_inline(tree, id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_rst_with_source;

    #[test]
    fn renders_a_section_title_with_underline() {
        let tree = parse_rst_with_source("Title\n=====\n\nBody text.\n", "<string>");
        let out = text(&tree);
        assert!(out.starts_with("Title\n=====\n"));
        assert!(out.contains("Body text."));
    }

    #[test]
    fn nested_sections_use_rotating_underline_chars() {
        let tree = parse_rst_with_source(
            "Top\n===\n\nSub\n---\n\nbody\n",
            "<string>",
        );
        let out = text(&tree);
        assert!(out.contains("Top\n===\n"));
        assert!(out.contains("Sub\n---\n"));
    }

    #[test]
    fn emphasis_and_strong_use_rst_markers() {
        let tree = parse_rst_with_source("A *word* and **another**.\n", "<string>");
        let out = text(&tree);
        assert!(out.contains("*word*"));
        assert!(out.contains("**another**"));
    }

    #[test]
    fn bullet_list_items_get_bullet_prefix() {
        let tree = parse_rst_with_source("- one\n- two\n", "<string>");
        let out = text(&tree);
        assert!(out.contains("- one"));
        assert!(out.contains("- two"));
    }

    #[test]
    fn literal_block_is_indented_and_unwrapped() {
        let tree = parse_rst_with_source("::\n\n    code here\n", "<string>");
        let out = text(&tree);
        assert!(out.contains("   code here"));
    }

    #[test]
    fn comments_produce_no_output() {
        let tree = parse_rst_with_source(".. This is a comment.\n\nVisible.\n", "<string>");
        let out = text(&tree);
        assert!(!out.contains("This is a comment"));
        assert!(out.contains("Visible."));
    }

    #[test]
    fn transition_renders_as_a_dash_rule() {
        let tree = parse_rst_with_source("one\n\n----\n\ntwo\n", "<string>");
        let out = text(&tree);
        assert!(out.contains(&"-".repeat(20)));
    }

    #[test]
    fn empty_document_renders_empty_string() {
        let tree = parse_rst_with_source("", "<string>");
        assert_eq!(text(&tree), "");
    }
}
