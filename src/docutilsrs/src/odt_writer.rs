//! Minimal ODT (OpenDocument Text) writer.
//!
//! Produces a valid `.odt` ZIP container with the four mandatory
//! members (`mimetype`, `META-INF/manifest.xml`, `content.xml`,
//! `styles.xml`). Renders a small but representative subset of the
//! doctree node types. Not parity-gated against
//! `docutils.writers.odf_odt` — see `docs/compat.md` for the
//! accepted-deviation note and the list of supported node kinds.

use crate::doctree::{Doctree, NodeId, NodeKind};
use crate::zip_writer::ZipBuilder;
use std::fmt::Write as _;

/// Render `tree` as a binary ODT (`.odt`) document.
pub fn odt(tree: &Doctree, options: &crate::cli::OdtOptions, common: &crate::cli::CommonOptions) -> Vec<u8> {
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
            let _ = write!(
                out,
                "<text:p text:style-name=\"Title\">{}</text:p>\n",
                escape(title)
            );
        }
    }
    for &c in &tree.node(root).children {
        emit(tree, c, 0, &mut out);
    }
    out.push_str(content_footer());
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
            let level = section_depth.clamp(1, 5);
            let _ = write!(
                out,
                "<text:h text:style-name=\"Heading_20_{level}\" text:outline-level=\"{level}\">"
            );
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:h>\n");
        }
        NodeKind::Subtitle { .. } => {
            out.push_str("<text:p text:style-name=\"Subtitle\">");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:p>\n");
        }
        NodeKind::Paragraph => {
            out.push_str("<text:p text:style-name=\"Standard\">");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:p>\n");
        }
        NodeKind::Transition => {
            out.push_str("<text:p text:style-name=\"Standard\">* * *</text:p>\n");
        }
        NodeKind::Emphasis | NodeKind::TitleReference => {
            out.push_str("<text:span text:style-name=\"Emphasis\">");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:span>");
        }
        NodeKind::Strong => {
            out.push_str("<text:span text:style-name=\"Strong_20_Emphasis\">");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:span>");
        }
        NodeKind::Literal => {
            out.push_str("<text:span text:style-name=\"Source_20_Text\">");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:span>");
        }
        NodeKind::Inline { .. } => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
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
            let mut first = true;
            for &c in &node.children {
                if !first {
                    out.push_str("<text:line-break/>");
                }
                first = false;
                let cn = tree.node(c);
                if matches!(cn.kind, NodeKind::Paragraph) {
                    for &cc in &cn.children {
                        emit(tree, cc, section_depth, out);
                    }
                } else {
                    emit(tree, c, section_depth, out);
                }
            }
            out.push_str("</text:p>\n");
        }
        NodeKind::BulletList { .. } => {
            out.push_str("<text:list>\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:list>\n");
        }
        NodeKind::EnumeratedList { .. } => {
            out.push_str("<text:list>\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:list>\n");
        }
        NodeKind::ListItem => {
            out.push_str("<text:list-item>\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:list-item>\n");
        }
        NodeKind::DefinitionList => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::DefinitionListItem => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Term => {
            out.push_str("<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Strong_20_Emphasis\">");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:span></text:p>\n");
        }
        NodeKind::Definition => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Classifier => {
            out.push_str(" : ");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::FieldList | NodeKind::Docinfo => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Field => {
            out.push_str("<text:p text:style-name=\"Standard\">");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:p>\n");
        }
        NodeKind::FieldName => {
            out.push_str("<text:span text:style-name=\"Strong_20_Emphasis\">");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str(":</text:span> ");
        }
        NodeKind::FieldBody => {
            for &c in &node.children {
                let cn = tree.node(c);
                if matches!(cn.kind, NodeKind::Paragraph) {
                    for &cc in &cn.children {
                        emit(tree, cc, section_depth, out);
                    }
                } else {
                    emit(tree, c, section_depth, out);
                }
            }
        }
        NodeKind::Bibliographic { tag } => {
            let _ = write!(
                out,
                "<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Strong_20_Emphasis\">{tag}:</text:span> "
            );
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:p>\n");
        }
        NodeKind::Admonition { kind } => {
            let _ = write!(
                out,
                "<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Strong_20_Emphasis\">{}:</text:span></text:p>\n",
                kind.to_uppercase()
            );
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
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
        NodeKind::Figure => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Caption => {
            out.push_str(
                "<text:p text:style-name=\"Standard\"><text:span text:style-name=\"Emphasis\">",
            );
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:span></text:p>\n");
        }
        NodeKind::Legend => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Attribution => {
            out.push_str("<text:p text:style-name=\"Standard\"><text:tab/>\u{2014} ");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:p>\n");
        }
        NodeKind::Reference { refuri, .. } => {
            let _ = write!(
                out,
                "<text:a xlink:type=\"simple\" xlink:href=\"{}\" text:style-name=\"Internet_20_link\">",
                escape_attr(refuri)
            );
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</text:a>");
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
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</table:table>\n");
        }
        NodeKind::Tgroup { .. } => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Colspec { .. } => {
            out.push_str("<table:table-column/>\n");
        }
        NodeKind::Thead | NodeKind::Tbody => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
        NodeKind::Row => {
            out.push_str("<table:table-row>\n");
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</table:table-row>\n");
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
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
            out.push_str("</table:table-cell>\n");
        }
        NodeKind::SystemMessage { .. }
        | NodeKind::Problematic { .. }
        | NodeKind::Footnote { .. }
        | NodeKind::FootnoteReference { .. }
        | NodeKind::Citation { .. }
        | NodeKind::CitationReference { .. }
        | NodeKind::Label => {
            for &c in &node.children {
                emit(tree, c, section_depth, out);
            }
        }
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
        let bytes = odt(&tree, &crate::cli::OdtOptions::default(), &crate::cli::CommonOptions::default());
        // Mimetype string appears uncompressed near the beginning.
        let mimetype = b"application/vnd.oasis.opendocument.text";
        assert!(bytes.windows(mimetype.len()).any(|w| w == mimetype));
        // ZIP signatures present.
        assert!(bytes.windows(4).any(|w| w == 0x04034b50u32.to_le_bytes()));
        assert!(bytes.windows(4).any(|w| w == 0x06054b50u32.to_le_bytes()));
    }
}
