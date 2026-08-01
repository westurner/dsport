//! Minimal ODT (OpenDocument Text) writer.
//!
//! Produces a valid `.odt` ZIP container with the four mandatory
//! members (`mimetype`, `META-INF/manifest.xml`, `content.xml`,
//! `styles.xml`). Renders a small but representative subset of the
//! doctree node types. Not parity-gated against
//! `docutils.writers.odf_odt` — see `docs/compat.md` for the
//! accepted-deviation note and the list of supported node kinds.

use crate::doctree::{Doctree, Node, NodeId, NodeKind};
use crate::zip_writer::ZipBuilder;
use std::fmt::Write as _;

/// A deferred unit of output work, driven by an explicit stack instead of
/// renderer-local recursion (see `docs/sphinxdocrs-port-plan.md`'s H12
/// tier). `Enter` dispatches on the node's kind at the given section
/// depth; `EnterFlattenParagraph` mirrors the old inline "unwrap a
/// paragraph child's own `<text:p>` wrapper" special case (used by
/// `BlockQuote`/`FieldBody`); `Append` is a plain string applied
/// verbatim.
enum Task {
    Enter(NodeId, usize),
    EnterFlattenParagraph(NodeId, usize),
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

/// Render `tree` as a binary ODT (`.odt`) document.
pub fn odt(
    tree: &Doctree,
    _options: &crate::cli::OdtOptions,
    _common: &crate::cli::CommonOptions,
) -> Vec<u8> {
    let content = build_content_xml(tree);
    let styles = build_styles_xml();
    let manifest = build_manifest_xml();
    let mut z = ZipBuilder::new();
    // mimetype MUST be the first entry per ODF 1.2 §3.3.
    z.add_file("mimetype", b"application/vnd.oasis.opendocument.text");
    z.add_file("META-INF/manifest.xml", manifest.as_bytes());
    z.add_file("content.xml", content.as_bytes());
    z.add_file("styles.xml", styles.as_bytes());
    z.finish()
}

fn build_manifest_xml() -> String {
    String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <manifest:manifest xmlns:manifest=\"urn:oasis:names:tc:opendocument:xmlns:manifest:1.0\" manifest:version=\"1.2\">\n\
         <manifest:file-entry manifest:full-path=\"/\" manifest:version=\"1.2\" manifest:media-type=\"application/vnd.oasis.opendocument.text\"/>\n\
         <manifest:file-entry manifest:full-path=\"content.xml\" manifest:media-type=\"text/xml\"/>\n\
         <manifest:file-entry manifest:full-path=\"styles.xml\" manifest:media-type=\"text/xml\"/>\n\
         </manifest:manifest>\n",
    )
}

fn build_styles_xml() -> String {
    // Minimal style document defining the named paragraph/text styles
    // referenced by `content.xml` below. Mirrors a small subset of the
    // docutils odf_odt style sheet.
    String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <office:document-styles \
         xmlns:office=\"urn:oasis:names:tc:opendocument:xmlns:office:1.0\" \
         xmlns:style=\"urn:oasis:names:tc:opendocument:xmlns:style:1.0\" \
         xmlns:fo=\"urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0\" \
         xmlns:text=\"urn:oasis:names:tc:opendocument:xmlns:text:1.0\" \
         xmlns:table=\"urn:oasis:names:tc:opendocument:xmlns:table:1.0\" \
         office:version=\"1.2\">\n\
         <office:styles>\n\
         <style:style style:name=\"Standard\" style:family=\"paragraph\" style:class=\"text\"/>\n\
         <style:style style:name=\"Heading_20_1\" style:display-name=\"Heading 1\" style:family=\"paragraph\" style:parent-style-name=\"Standard\" style:next-style-name=\"Standard\">\n\
         <style:text-properties fo:font-size=\"200%\" fo:font-weight=\"bold\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Heading_20_2\" style:display-name=\"Heading 2\" style:family=\"paragraph\" style:parent-style-name=\"Standard\" style:next-style-name=\"Standard\">\n\
         <style:text-properties fo:font-size=\"175%\" fo:font-weight=\"bold\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Heading_20_3\" style:display-name=\"Heading 3\" style:family=\"paragraph\" style:parent-style-name=\"Standard\" style:next-style-name=\"Standard\">\n\
         <style:text-properties fo:font-size=\"150%\" fo:font-weight=\"bold\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Heading_20_4\" style:display-name=\"Heading 4\" style:family=\"paragraph\" style:parent-style-name=\"Standard\" style:next-style-name=\"Standard\">\n\
         <style:text-properties fo:font-size=\"125%\" fo:font-weight=\"bold\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Heading_20_5\" style:display-name=\"Heading 5\" style:family=\"paragraph\" style:parent-style-name=\"Standard\" style:next-style-name=\"Standard\">\n\
         <style:text-properties fo:font-weight=\"bold\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Title\" style:family=\"paragraph\" style:parent-style-name=\"Standard\" style:next-style-name=\"Standard\">\n\
         <style:text-properties fo:font-size=\"250%\" fo:font-weight=\"bold\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Subtitle\" style:family=\"paragraph\" style:parent-style-name=\"Standard\" style:next-style-name=\"Standard\">\n\
         <style:text-properties fo:font-size=\"200%\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Preformatted_20_Text\" style:display-name=\"Preformatted Text\" style:family=\"paragraph\" style:parent-style-name=\"Standard\">\n\
         <style:text-properties style:font-name=\"Courier\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Quotations\" style:family=\"paragraph\" style:parent-style-name=\"Standard\">\n\
         <style:paragraph-properties fo:margin-left=\"1cm\" fo:margin-right=\"1cm\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Emphasis\" style:family=\"text\">\n\
         <style:text-properties fo:font-style=\"italic\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Strong_20_Emphasis\" style:display-name=\"Strong Emphasis\" style:family=\"text\">\n\
         <style:text-properties fo:font-weight=\"bold\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Source_20_Text\" style:display-name=\"Source Text\" style:family=\"text\">\n\
         <style:text-properties style:font-name=\"Courier\"/>\n\
         </style:style>\n\
         <style:style style:name=\"Internet_20_link\" style:display-name=\"Internet link\" style:family=\"text\">\n\
         <style:text-properties fo:color=\"#0000ff\" style:text-underline-style=\"solid\"/>\n\
         </style:style>\n\
         </office:styles>\n\
         </office:document-styles>\n",
    )
}

fn content_header() -> &'static str {
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
     <office:document-content \
     xmlns:office=\"urn:oasis:names:tc:opendocument:xmlns:office:1.0\" \
     xmlns:style=\"urn:oasis:names:tc:opendocument:xmlns:style:1.0\" \
     xmlns:text=\"urn:oasis:names:tc:opendocument:xmlns:text:1.0\" \
     xmlns:table=\"urn:oasis:names:tc:opendocument:xmlns:table:1.0\" \
     xmlns:fo=\"urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0\" \
     xmlns:xlink=\"http://www.w3.org/1999/xlink\" \
     office:version=\"1.2\">\n\
     <office:body>\n\
     <office:text>\n"
}

fn content_footer() -> &'static str {
    "</office:text>\n</office:body>\n</office:document-content>\n"
}

fn build_content_xml(tree: &Doctree) -> String {
    let mut out = String::new();
    out.push_str(content_header());
    let root = tree.root();
    if let NodeKind::Document { title, .. } = &tree.node(root).kind {
        if !title.is_empty() {
            let _ = writeln!(
                out,
                "<text:p text:style-name=\"Title\">{}</text:p>",
                escape(title)
            );
        }
    }
    let mut tasks: Vec<Task> = Vec::new();
    for &c in tree.node(root).children.iter().rev() {
        tasks.push(Task::Enter(c, 0));
    }
    while let Some(task) = tasks.pop() {
        match task {
            Task::Append(s) => out.push_str(&s),
            Task::Enter(id, depth) => emit_enter(tree, id, depth, &mut out, &mut tasks),
            Task::EnterFlattenParagraph(id, depth) => {
                if let NodeKind::Paragraph = &tree.node(id).kind {
                    let node = tree.node(id);
                    for &c in node.children.iter().rev() {
                        tasks.push(Task::Enter(c, depth));
                    }
                } else {
                    emit_enter(tree, id, depth, &mut out, &mut tasks);
                }
            }
        }
    }
    out.push_str(content_footer());
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
            let level = section_depth.clamp(1, 5);
            let _ = write!(
                out,
                "<text:h text:style-name=\"Heading_20_{level}\" text:outline-level=\"{level}\">"
            );
            schedule(node, section_depth, "</text:h>\n", tasks);
        }
        NodeKind::Subtitle { .. } => {
            out.push_str("<text:p text:style-name=\"Subtitle\">");
            schedule(node, section_depth, "</text:p>\n", tasks);
        }
        NodeKind::Paragraph => {
            out.push_str("<text:p text:style-name=\"Standard\">");
            schedule(node, section_depth, "</text:p>\n", tasks);
        }
        NodeKind::Transition => {
            out.push_str("<text:p text:style-name=\"Standard\">* * *</text:p>\n");
        }
        NodeKind::Emphasis | NodeKind::TitleReference => {
            out.push_str("<text:span text:style-name=\"Emphasis\">");
            schedule(node, section_depth, "</text:span>", tasks);
        }
        NodeKind::Strong => {
            out.push_str("<text:span text:style-name=\"Strong_20_Emphasis\">");
            schedule(node, section_depth, "</text:span>", tasks);
        }
        NodeKind::Literal => {
            out.push_str("<text:span text:style-name=\"Source_20_Text\">");
            schedule(node, section_depth, "</text:span>", tasks);
        }
        NodeKind::Inline { .. } => schedule_children(node, section_depth, tasks),
        NodeKind::Math { latex } => {
            // ODT has no native math container in this minimal writer;
            // emit the LaTeX source inside a styled span so the
            // information round-trips for downstream tooling.
            let _ = write!(
                out,
                "<text:span text:style-name=\"Math\">{}</text:span>",
                escape(latex)
            );
        }
        NodeKind::MathBlock { latex } => {
            let _ = write!(
                out,
                "<text:p text:style-name=\"Math_20_Block\">{}</text:p>",
                escape(latex)
            );
        }
        NodeKind::LiteralBlock { .. } => {
            for &c in &node.children {
                if let NodeKind::Text(s) = &tree.node(c).kind {
                    for (i, line) in s.split('\n').enumerate() {
                        if i > 0 {
                            out.push('\n');
                        }
                        let _ = write!(
                            out,
                            "<text:p text:style-name=\"Preformatted_20_Text\">{}</text:p>",
                            escape(line)
                        );
                    }
                    out.push('\n');
                }
            }
        }
        NodeKind::BlockQuote => {
            out.push_str("<text:p text:style-name=\"Quotations\">");
            // Flatten block-quote children into nested paragraphs.
            let mut sub = Vec::new();
            for (i, &c) in node.children.iter().enumerate() {
                if i > 0 {
                    sub.push(Task::Append("<text:line-break/>".to_string()));
                }
                sub.push(Task::EnterFlattenParagraph(c, section_depth));
            }
            sub.push(Task::Append("</text:p>\n".to_string()));
            push_all(tasks, sub);
        }
        NodeKind::BulletList { .. } | NodeKind::EnumeratedList { .. } => {
            out.push_str("<text:list>\n");
            schedule(node, section_depth, "</text:list>\n", tasks);
        }
        NodeKind::ListItem => {
            out.push_str("<text:list-item>\n");
            schedule(node, section_depth, "</text:list-item>\n", tasks);
        }
        NodeKind::DefinitionList | NodeKind::DefinitionListItem => {
            schedule_children(node, section_depth, tasks)
        }
        NodeKind::Term => {
            out.push_str("<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Strong_20_Emphasis\">");
            schedule(node, section_depth, "</text:span></text:p>\n", tasks);
        }
        NodeKind::Definition => schedule_children(node, section_depth, tasks),
        NodeKind::Classifier => {
            out.push_str(" : ");
            schedule_children(node, section_depth, tasks);
        }
        NodeKind::FieldList | NodeKind::Docinfo => schedule_children(node, section_depth, tasks),
        NodeKind::Field => {
            out.push_str("<text:p text:style-name=\"Standard\">");
            schedule(node, section_depth, "</text:p>\n", tasks);
        }
        NodeKind::FieldName => {
            out.push_str("<text:span text:style-name=\"Strong_20_Emphasis\">");
            schedule(node, section_depth, ":</text:span> ", tasks);
        }
        NodeKind::FieldBody => {
            let mut sub = Vec::new();
            for &c in &node.children {
                sub.push(Task::EnterFlattenParagraph(c, section_depth));
            }
            push_all(tasks, sub);
        }
        NodeKind::Bibliographic { tag } => {
            let _ = write!(
                out,
                "<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Strong_20_Emphasis\">{tag}:</text:span> "
            );
            schedule(node, section_depth, "</text:p>\n", tasks);
        }
        NodeKind::Admonition { kind } => {
            let _ = writeln!(
                out,
                "<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Strong_20_Emphasis\">{}:</text:span></text:p>",
                kind.to_uppercase()
            );
            schedule_children(node, section_depth, tasks);
        }
        NodeKind::Container { .. } => schedule_children(node, section_depth, tasks),
        NodeKind::GenericAdmonition { title, .. } => {
            let _ = writeln!(
                out,
                "<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Strong_20_Emphasis\">{}:</text:span></text:p>",
                escape(title)
            );
            schedule_children(node, section_depth, tasks);
        }
        NodeKind::Epigraph { .. } => schedule_children(node, section_depth, tasks),
        NodeKind::Toctree { .. } => {}
        NodeKind::Image { uri, alt, .. } => {
            let _ = write!(
                out,
                "<text:p text:style-name=\"Standard\"><draw:frame xmlns:draw=\"urn:oasis:names:tc:opendocument:xmlns:drawing:1.0\"><draw:image xlink:href=\"{}\"/>",
                escape_attr(uri)
            );
            if let Some(a) = alt {
                let _ = write!(
                    out,
                    "<svg:desc xmlns:svg=\"urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0\">{}</svg:desc>",
                    escape(a)
                );
            }
            out.push_str("</draw:frame></text:p>\n");
        }
        NodeKind::Figure => schedule_children(node, section_depth, tasks),
        NodeKind::Caption => {
            out.push_str(
                "<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Emphasis\">",
            );
            schedule(node, section_depth, "</text:span></text:p>\n", tasks);
        }
        NodeKind::Legend => schedule_children(node, section_depth, tasks),
        NodeKind::Attribution => {
            out.push_str("<text:p text:style-name=\"Standard\"><text:tab/>\u{2014} ");
            schedule(node, section_depth, "</text:p>\n", tasks);
        }
        NodeKind::Reference { refuri, .. } => {
            let _ = write!(
                out,
                "<text:a xlink:type=\"simple\" xlink:href=\"{}\" text:style-name=\"Internet_20_link\">",
                escape_attr(refuri)
            );
            schedule(node, section_depth, "</text:a>", tasks);
        }
        NodeKind::Target { .. } | NodeKind::SubstitutionDefinition { .. } => {}
        NodeKind::SubstitutionReference { refname } => {
            out.push_str(&escape(&format!("|{refname}|")));
        }
        NodeKind::Comment => {}
        NodeKind::Raw { format } => {
            if format == "odt" {
                for &c in &node.children {
                    if let NodeKind::Text(s) = &tree.node(c).kind {
                        out.push_str(s);
                    }
                }
            }
        }
        NodeKind::Table => {
            out.push_str("<table:table>\n");
            schedule(node, section_depth, "</table:table>\n", tasks);
        }
        NodeKind::Tgroup { .. } => schedule_children(node, section_depth, tasks),
        NodeKind::Colspec { .. } => {
            out.push_str("<table:table-column/>\n");
        }
        NodeKind::Thead | NodeKind::Tbody => schedule_children(node, section_depth, tasks),
        NodeKind::Row => {
            out.push_str("<table:table-row>\n");
            schedule(node, section_depth, "</table:table-row>\n", tasks);
        }
        NodeKind::Entry { morecols, morerows } => {
            out.push_str("<table:table-cell");
            if *morecols > 0 {
                let _ = write!(out, " table:number-columns-spanned=\"{}\"", morecols + 1);
            }
            if *morerows > 0 {
                let _ = write!(out, " table:number-rows-spanned=\"{}\"", morerows + 1);
            }
            out.push_str(">\n");
            schedule(node, section_depth, "</table:table-cell>\n", tasks);
        }
        NodeKind::SystemMessage { .. }
        | NodeKind::Problematic { .. }
        | NodeKind::Footnote { .. }
        | NodeKind::FootnoteReference { .. }
        | NodeKind::Citation { .. }
        | NodeKind::CitationReference { .. }
        | NodeKind::Label => schedule_children(node, section_depth, tasks),
        NodeKind::Extension { class_name, attrs } => {
            let visit = crate::plugins::invoke_node_visit(class_name, "odt", attrs);
            if let Some(open) = &visit {
                out.push_str(open);
            }
            let close = if visit.is_some() {
                crate::plugins::invoke_node_depart(class_name, "odt", attrs)
            } else {
                None
            };
            schedule(node, section_depth, close.unwrap_or_default(), tasks);
        }
        NodeKind::ObjectDescription { sig_text, .. } => {
            out.push_str("<text:p>");
            out.push_str(sig_text);
            out.push_str("</text:p>\n");
            schedule_children(node, section_depth, tasks);
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
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
    out
}

fn escape_attr(s: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_rst_with_source;

    #[test]
    fn odt_basic_zip_structure() {
        let tree = parse_rst_with_source("Hello *world*.\n", "<string>");
        let bytes = odt(
            &tree,
            &crate::cli::OdtOptions::default(),
            &crate::cli::CommonOptions::default(),
        );
        // Mimetype string appears uncompressed near the beginning.
        let mimetype = b"application/vnd.oasis.opendocument.text";
        assert!(bytes.windows(mimetype.len()).any(|w| w == mimetype));
        // ZIP signatures present.
        assert!(bytes.windows(4).any(|w| w == 0x04034b50u32.to_le_bytes()));
        assert!(bytes.windows(4).any(|w| w == 0x06054b50u32.to_le_bytes()));
    }

    #[test]
    fn deeply_nested_odt_does_not_use_call_stack() {
        let mut tree = Doctree::new_document("deep.odt");
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

        let bytes = odt(
            &tree,
            &crate::cli::OdtOptions::default(),
            &crate::cli::CommonOptions::default(),
        );
        assert!(bytes.windows(4).any(|w| w == 0x04034b50u32.to_le_bytes()));
    }
}
