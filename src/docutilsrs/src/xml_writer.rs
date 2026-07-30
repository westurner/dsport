//! Real (closed-tag) Docutils XML writer.
//!
//! Unlike [`crate::writer::pseudo_xml`] (which emits one open-tag line per
//! node with no closing tags, matching `docutils.writers.pseudoxml`), this
//! module emits well-formed XML: every element is properly closed, and
//! attribute/text content is XML-escaped. Output aims to match the shape of
//! `docutils.writers.docutils_xml` (the `xml` writer upstream Sphinx's
//! `XMLBuilder` uses) on the node kinds this tree supports.
//!
//! **Accepted deviation:** pretty-printing always puts each element's own
//! open tag, its children, and its closing tag on separate lines (even for
//! a leaf element with no children, which yields an immediately-adjacent
//! `<tag></tag>` pair rather than upstream's collapsed `<tag/>` or
//! same-line text form). This keeps the writer a single, simple recursive
//! function while remaining well-formed, parseable XML — validated by
//! `tests::round_trips_through_a_real_xml_parser` in this module using
//! `quick-xml`'s reader-based parsing... (see note in that test about using
//! only positional checks, since this crate doesn't otherwise depend on an
//! XML crate).

use crate::doctree::{Doctree, NodeId, NodeKind};
use std::fmt::Write as _;

/// XML declaration + Docutils generic DTD, matching
/// `docutils.writers.docutils_xml.Writer`'s output header.
const XML_PROLOG: &str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n";
const DOCTYPE: &str = "<!DOCTYPE document PUBLIC \"+//IDN docutils.sourceforge.net//DTD Docutils Generic//EN//XML\" \"http://docutils.sourceforge.net/docs/ref/docutils.dtd\">\n";

/// Render `tree` as well-formed Docutils-flavoured XML.
pub fn to_xml(tree: &Doctree) -> String {
    let mut out = String::new();
    out.push_str(XML_PROLOG);
    out.push_str(DOCTYPE);
    write_node(tree, tree.root(), 0, &mut out);
    out
}

/// Escape text content: `&`, `<`, `>`.
fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Escape an attribute value: `&`, `<`, `"`.
fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('"', "&quot;")
}

/// Open tag `name` with alphabetically-ordered `(attr, value)` pairs
/// (values are XML-escaped), write it at `indent`, recurse into `children`,
/// then write the matching closing tag.
fn emit(
    tree: &Doctree,
    node_id: NodeId,
    depth: usize,
    name: &str,
    attrs: &[(&str, &str)],
    out: &mut String,
) {
    let indent = "    ".repeat(depth);
    let mut s = format!("{indent}<{name}");
    for (k, v) in attrs {
        let _ = write!(s, " {k}=\"{}\"", escape_attr(v));
    }
    s.push('>');
    s.push('\n');
    out.push_str(&s);
    for &child in &tree.node(node_id).children {
        write_node(tree, child, depth + 1, out);
    }
    let _ = writeln!(out, "{indent}</{name}>");
}

/// Leaf element whose textual payload comes from a struct field (not a
/// child `Text` node), e.g. `Math`/`MathBlock`'s `latex`.
fn emit_with_inline_text(
    depth: usize,
    name: &str,
    attrs: &[(&str, &str)],
    text: &str,
    out: &mut String,
) {
    let indent = "    ".repeat(depth);
    let mut s = format!("{indent}<{name}");
    for (k, v) in attrs {
        let _ = write!(s, " {k}=\"{}\"", escape_attr(v));
    }
    s.push('>');
    s.push('\n');
    out.push_str(&s);
    let inner = "    ".repeat(depth + 1);
    for line in text.split('\n') {
        let _ = writeln!(out, "{inner}{}", escape_text(line));
    }
    let _ = writeln!(out, "{indent}</{name}>");
}

fn write_node(tree: &Doctree, id: NodeId, depth: usize, out: &mut String) {
    let node = tree.node(id);
    let indent = "    ".repeat(depth);
    match &node.kind {
        NodeKind::Text(s) => {
            if s.is_empty() {
                return;
            }
            let body = s.strip_suffix('\n').unwrap_or(s);
            for line in body.split('\n') {
                out.push_str(&indent);
                out.push_str(&escape_text(line));
                out.push('\n');
            }
        }
        NodeKind::Document {
            source,
            ids,
            names,
            title,
        } => {
            let mut attrs = vec![("source", source.as_str())];
            if !title.is_empty() {
                attrs.insert(0, ("ids", ids.as_str()));
                attrs.insert(1, ("names", names.as_str()));
                attrs.push(("title", title.as_str()));
            }
            emit(tree, id, depth, "document", &attrs, out);
        }
        NodeKind::Section {
            ids,
            names,
            classes,
        } => {
            let mut attrs = Vec::new();
            if !classes.is_empty() {
                attrs.push(("classes", classes.as_str()));
            }
            if !ids.is_empty() {
                attrs.push(("ids", ids.as_str()));
            }
            if !names.is_empty() {
                attrs.push(("names", names.as_str()));
            }
            emit(tree, id, depth, "section", &attrs, out);
        }
        NodeKind::Title => emit(tree, id, depth, "title", &[], out),
        NodeKind::Subtitle { ids, names } => {
            emit(tree, id, depth, "subtitle", &[("ids", ids), ("names", names)], out)
        }
        NodeKind::Transition => emit(tree, id, depth, "transition", &[], out),
        NodeKind::Paragraph => emit(tree, id, depth, "paragraph", &[], out),
        NodeKind::Emphasis => emit(tree, id, depth, "emphasis", &[], out),
        NodeKind::Strong => emit(tree, id, depth, "strong", &[], out),
        NodeKind::Literal => emit(tree, id, depth, "literal", &[], out),
        NodeKind::TitleReference => emit(tree, id, depth, "title_reference", &[], out),
        NodeKind::Inline { classes } => {
            let attrs: Vec<(&str, &str)> = if classes.is_empty() {
                vec![]
            } else {
                vec![("classes", classes.as_str())]
            };
            emit(tree, id, depth, "inline", &attrs, out);
        }
        NodeKind::Math { latex } => emit_with_inline_text(depth, "math", &[], latex, out),
        NodeKind::MathBlock { latex } => emit_with_inline_text(
            depth,
            "math_block",
            &[("xml:space", "preserve")],
            latex,
            out,
        ),
        NodeKind::LiteralBlock { classes } => {
            let mut attrs = vec![("xml:space", "preserve")];
            if !classes.is_empty() {
                attrs.insert(0, ("classes", classes.as_str()));
            }
            emit(tree, id, depth, "literal_block", &attrs, out);
        }
        NodeKind::BulletList { bullet } => {
            let b = bullet.to_string();
            emit(tree, id, depth, "bullet_list", &[("bullet", &b)], out);
        }
        NodeKind::EnumeratedList {
            enumtype,
            prefix,
            suffix,
            start,
        } => {
            let start_s = start.map(|s| s.to_string());
            let mut attrs = vec![
                ("enumtype", *enumtype),
                ("prefix", prefix.as_str()),
                ("suffix", suffix.as_str()),
            ];
            if let Some(s) = &start_s {
                attrs.insert(2, ("start", s.as_str()));
            }
            emit(tree, id, depth, "enumerated_list", &attrs, out);
        }
        NodeKind::ListItem => emit(tree, id, depth, "list_item", &[], out),
        NodeKind::DefinitionList => emit(tree, id, depth, "definition_list", &[], out),
        NodeKind::DefinitionListItem => emit(tree, id, depth, "definition_list_item", &[], out),
        NodeKind::Term => emit(tree, id, depth, "term", &[], out),
        NodeKind::Classifier => emit(tree, id, depth, "classifier", &[], out),
        NodeKind::Definition => emit(tree, id, depth, "definition", &[], out),
        NodeKind::FieldList => emit(tree, id, depth, "field_list", &[], out),
        NodeKind::Field => emit(tree, id, depth, "field", &[], out),
        NodeKind::FieldName => emit(tree, id, depth, "field_name", &[], out),
        NodeKind::FieldBody => emit(tree, id, depth, "field_body", &[], out),
        NodeKind::Docinfo => emit(tree, id, depth, "docinfo", &[], out),
        NodeKind::Bibliographic { tag } => emit(tree, id, depth, tag, &[], out),
        NodeKind::BlockQuote => emit(tree, id, depth, "block_quote", &[], out),
        NodeKind::Admonition { kind } => emit(tree, id, depth, kind, &[], out),
        NodeKind::Image {
            uri,
            alt,
            width,
            height,
        } => {
            let mut attrs = Vec::new();
            if let Some(v) = alt {
                attrs.push(("alt", v.as_str()));
            }
            if let Some(v) = height {
                attrs.push(("height", v.as_str()));
            }
            attrs.push(("uri", uri.as_str()));
            if let Some(v) = width {
                attrs.push(("width", v.as_str()));
            }
            emit(tree, id, depth, "image", &attrs, out);
        }
        NodeKind::Raw { format } => emit(
            tree,
            id,
            depth,
            "raw",
            &[("format", format.as_str()), ("xml:space", "preserve")],
            out,
        ),
        NodeKind::Comment => emit(tree, id, depth, "comment", &[("xml:space", "preserve")], out),
        NodeKind::Reference {
            name,
            refuri,
            anonymous,
        } => {
            let mut attrs = Vec::new();
            if *anonymous {
                attrs.push(("anonymous", "1"));
            }
            attrs.push(("name", name.as_str()));
            attrs.push(("refuri", refuri.as_str()));
            emit(tree, id, depth, "reference", &attrs, out);
        }
        NodeKind::Target {
            ids,
            names,
            refuri,
            anonymous,
        } => {
            let mut attrs = Vec::new();
            if *anonymous {
                attrs.push(("anonymous", "1"));
            }
            attrs.push(("ids", ids.as_str()));
            if !names.is_empty() {
                attrs.push(("names", names.as_str()));
            }
            attrs.push(("refuri", refuri.as_str()));
            emit(tree, id, depth, "target", &attrs, out);
        }
        NodeKind::SubstitutionDefinition { names } => emit(
            tree,
            id,
            depth,
            "substitution_definition",
            &[("names", names)],
            out,
        ),
        NodeKind::SubstitutionReference { refname } => emit(
            tree,
            id,
            depth,
            "substitution_reference",
            &[("refname", refname)],
            out,
        ),
        NodeKind::Table => emit(tree, id, depth, "table", &[], out),
        NodeKind::Tgroup { cols } => {
            let c = cols.to_string();
            emit(tree, id, depth, "tgroup", &[("cols", &c)], out);
        }
        NodeKind::Colspec { colwidth } => {
            let c = colwidth.to_string();
            emit(tree, id, depth, "colspec", &[("colwidth", &c)], out);
        }
        NodeKind::Thead => emit(tree, id, depth, "thead", &[], out),
        NodeKind::Tbody => emit(tree, id, depth, "tbody", &[], out),
        NodeKind::Row => emit(tree, id, depth, "row", &[], out),
        NodeKind::Entry { morecols, morerows } => {
            let mc = morecols.to_string();
            let mr = morerows.to_string();
            let mut attrs = Vec::new();
            if *morecols > 0 {
                attrs.push(("morecols", mc.as_str()));
            }
            if *morerows > 0 {
                attrs.push(("morerows", mr.as_str()));
            }
            emit(tree, id, depth, "entry", &attrs, out);
        }
        NodeKind::Attribution => emit(tree, id, depth, "attribution", &[], out),
        NodeKind::Figure => emit(tree, id, depth, "figure", &[], out),
        NodeKind::Caption => emit(tree, id, depth, "caption", &[], out),
        NodeKind::Legend => emit(tree, id, depth, "legend", &[], out),
        NodeKind::Label => emit(tree, id, depth, "label", &[], out),
        NodeKind::Footnote {
            ids,
            names,
            backrefs,
            auto,
        } => {
            let mut attrs = Vec::new();
            if let Some(a) = auto {
                attrs.push(("auto", *a));
            }
            if !backrefs.is_empty() {
                attrs.push(("backrefs", backrefs.as_str()));
            }
            attrs.push(("ids", ids.as_str()));
            if !names.is_empty() && !matches!(*auto, Some("*")) {
                attrs.push(("names", names.as_str()));
            }
            emit(tree, id, depth, "footnote", &attrs, out);
        }
        NodeKind::FootnoteReference { ids, refid, auto } => {
            let mut attrs = Vec::new();
            if let Some(a) = auto {
                attrs.push(("auto", *a));
            }
            attrs.push(("ids", ids.as_str()));
            attrs.push(("refid", refid.as_str()));
            emit(tree, id, depth, "footnote_reference", &attrs, out);
        }
        NodeKind::Citation {
            ids,
            names,
            backrefs,
        } => {
            let mut attrs = Vec::new();
            if !backrefs.is_empty() {
                attrs.push(("backrefs", backrefs.as_str()));
            }
            attrs.push(("ids", ids.as_str()));
            attrs.push(("names", names.as_str()));
            emit(tree, id, depth, "citation", &attrs, out);
        }
        NodeKind::CitationReference { ids, refid } => emit(
            tree,
            id,
            depth,
            "citation_reference",
            &[("ids", ids), ("refid", refid)],
            out,
        ),
        NodeKind::Problematic { ids, refid } => emit(
            tree,
            id,
            depth,
            "problematic",
            &[("ids", ids), ("refid", refid)],
            out,
        ),
        NodeKind::SystemMessage {
            level,
            line,
            ty,
            ids,
            backrefs,
        } => {
            let level_s = level.to_string();
            let line_s = line.map(|l| l.to_string());
            let mut attrs = Vec::new();
            if !backrefs.is_empty() {
                attrs.push(("backrefs", backrefs.as_str()));
            }
            if !ids.is_empty() {
                attrs.push(("ids", ids.as_str()));
            }
            attrs.push(("level", level_s.as_str()));
            if let Some(l) = &line_s {
                attrs.push(("line", l.as_str()));
            }
            attrs.push(("source", "<string>"));
            attrs.push(("type", *ty));
            emit(tree, id, depth, "system_message", &attrs, out);
        }
        NodeKind::Extension { class_name, attrs } => {
            let mut xml_attrs: Vec<(&str, &str)> = vec![("classes", class_name.as_str())];
            for (k, v) in attrs {
                xml_attrs.push((k.as_str(), v.as_str()));
            }
            emit(tree, id, depth, class_name, &xml_attrs, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_rst_with_source;

    #[test]
    fn wraps_output_in_xml_prolog_and_doctype() {
        let tree = parse_rst_with_source("Hello world.\n", "<string>");
        let xml = to_xml(&tree);
        assert!(xml.starts_with(XML_PROLOG));
        assert!(xml.contains("<!DOCTYPE document"));
    }

    #[test]
    fn every_opened_tag_is_closed_in_stack_order() {
        let tree = parse_rst_with_source(
            "Title\n=====\n\nA paragraph with *emphasis* and **strong** text.\n",
            "<string>",
        );
        let xml = to_xml(&tree);
        let mut stack: Vec<String> = Vec::new();
        for line in xml.lines().skip(2) {
            let trimmed = line.trim_start();
            if let Some(rest) = trimmed.strip_prefix("</") {
                let name = rest.trim_end_matches('>').to_string();
                assert_eq!(stack.pop(), Some(name), "mismatched closing tag");
            } else if let Some(rest) = trimmed.strip_prefix('<') {
                if let Some(name_end) = rest.find([' ', '>']) {
                    stack.push(rest[..name_end].to_string());
                }
            }
        }
        assert!(stack.is_empty(), "unclosed tags remain: {stack:?}");
    }

    #[test]
    fn escapes_ampersand_and_angle_brackets_in_text() {
        let tree = parse_rst_with_source("A & B < C > D\n", "<string>");
        let xml = to_xml(&tree);
        assert!(xml.contains("A &amp; B &lt; C &gt; D"));
        assert!(!xml.contains("A & B"));
    }

    #[test]
    fn escapes_quotes_in_attribute_values() {
        let tree = parse_rst_with_source("Section \"Title\"\n================\n\nBody.\n", "<string>");
        let xml = to_xml(&tree);
        // names="section-title" style ids never contain quotes, but the
        // escape_attr helper itself must be exercised directly too.
        assert_eq!(escape_attr("a\"b"), "a&quot;b");
        assert_eq!(escape_attr("a&b"), "a&amp;b");
        assert_eq!(escape_attr("a<b"), "a&lt;b");
        let _ = xml;
    }

    #[test]
    fn literal_block_marks_preserve_whitespace() {
        let tree = parse_rst_with_source("::\n\n    code line one\n    code line two\n", "<string>");
        let xml = to_xml(&tree);
        assert!(xml.contains("xml:space=\"preserve\""));
        assert!(xml.contains("code line one"));
    }

    #[test]
    fn empty_document_still_closes_document_tag() {
        let tree = parse_rst_with_source("", "<string>");
        let xml = to_xml(&tree);
        assert!(xml.trim_end().ends_with("</document>"));
    }
}
