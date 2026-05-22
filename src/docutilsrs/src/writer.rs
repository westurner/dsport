//! Pseudo-XML writer.
//!
//! Output format matches `docutils.writers.pseudoxml` on the supported
//! node kinds. Parity is asserted from
//! `src/tests/test_parity_pseudoxml.py`.
//!
//! Format rules (derived from upstream output):
//! - 4-space indent per depth level.
//! - Each node emits one line: `<indent><tag[ attr="val" ...]>`.
//! - No closing tags.
//! - Text nodes emit one line per source line.
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
            // Match Python `str.splitlines()` semantics (used by
            // upstream `docutils.nodes.Text.pformat`): "\n" yields
            // one empty line, not two. Without this, a trailing `\n`
            // in a text node produces a spurious blank line.
            if s.is_empty() {
                return;
            }
            let body = s.strip_suffix('\n').unwrap_or(s);
            for line in body.split('\n') {
                out.push_str(&indent);
                out.push_str(line);
                out.push('\n');
            }
        }
        NodeKind::Document {
            source,
            ids,
            names,
            title,
        } => {
            if title.is_empty() {
                let _ = writeln!(out, "{indent}<document source=\"{source}\">");
            } else {
                let _ = writeln!(
                    out,
                    "{indent}<document ids=\"{ids}\" names=\"{names}\" source=\"{source}\" title=\"{title}\">"
                );
            }
        }
        NodeKind::Section {
            ids,
            names,
            classes,
        } => {
            let mut s = format!("{indent}<section");
            if !classes.is_empty() {
                let _ = write!(s, " classes=\"{classes}\"");
            }
            if !ids.is_empty() {
                let _ = write!(s, " ids=\"{ids}\"");
            }
            if !names.is_empty() {
                let _ = write!(s, " names=\"{names}\"");
            }
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
        NodeKind::Title => {
            let _ = writeln!(out, "{indent}<title>");
        }
        NodeKind::Subtitle { ids, names } => {
            let _ = writeln!(out, "{indent}<subtitle ids=\"{ids}\" names=\"{names}\">");
        }
        NodeKind::Transition => {
            let _ = writeln!(out, "{indent}<transition>");
        }
        NodeKind::Paragraph => {
            let _ = writeln!(out, "{indent}<paragraph>");
        }
        NodeKind::Emphasis => {
            let _ = writeln!(out, "{indent}<emphasis>");
        }
        NodeKind::Strong => {
            let _ = writeln!(out, "{indent}<strong>");
        }
        NodeKind::Literal => {
            let _ = writeln!(out, "{indent}<literal>");
        }
        NodeKind::TitleReference => {
            let _ = writeln!(out, "{indent}<title_reference>");
        }
        NodeKind::Inline { classes } => {
            if classes.is_empty() {
                let _ = writeln!(out, "{indent}<inline>");
            } else {
                let _ = writeln!(out, "{indent}<inline classes=\"{classes}\">");
            }
        }
        NodeKind::LiteralBlock { classes } => {
            if classes.is_empty() {
                let _ = writeln!(out, "{indent}<literal_block xml:space=\"preserve\">");
            } else {
                let _ = writeln!(
                    out,
                    "{indent}<literal_block classes=\"{classes}\" xml:space=\"preserve\">"
                );
            }
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
            let _ = writeln!(out, "{indent}<list_item>");
        }
        NodeKind::DefinitionList => {
            let _ = writeln!(out, "{indent}<definition_list>");
        }
        NodeKind::DefinitionListItem => {
            let _ = writeln!(out, "{indent}<definition_list_item>");
        }
        NodeKind::Term => {
            let _ = writeln!(out, "{indent}<term>");
        }
        NodeKind::Classifier => {
            let _ = writeln!(out, "{indent}<classifier>");
        }
        NodeKind::Definition => {
            let _ = writeln!(out, "{indent}<definition>");
        }
        NodeKind::FieldList => {
            let _ = writeln!(out, "{indent}<field_list>");
        }
        NodeKind::Field => {
            let _ = writeln!(out, "{indent}<field>");
        }
        NodeKind::FieldName => {
            let _ = writeln!(out, "{indent}<field_name>");
        }
        NodeKind::FieldBody => {
            let _ = writeln!(out, "{indent}<field_body>");
        }
        NodeKind::Docinfo => {
            let _ = writeln!(out, "{indent}<docinfo>");
        }
        NodeKind::Bibliographic { tag } => {
            let _ = writeln!(out, "{indent}<{tag}>");
        }
        NodeKind::BlockQuote => {
            let _ = writeln!(out, "{indent}<block_quote>");
        }
        NodeKind::Admonition { kind } => {
            let _ = writeln!(out, "{indent}<{kind}>");
        }
        NodeKind::Image {
            uri,
            alt,
            width,
            height,
        } => {
            let mut s = format!("{indent}<image");
            if let Some(v) = alt {
                let _ = write!(s, " alt=\"{v}\"");
            }
            if let Some(v) = height {
                let _ = write!(s, " height=\"{v}\"");
            }
            let _ = write!(s, " uri=\"{uri}\"");
            if let Some(v) = width {
                let _ = write!(s, " width=\"{v}\"");
            }
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
        NodeKind::Raw { format } => {
            let _ = writeln!(
                out,
                "{indent}<raw format=\"{format}\" xml:space=\"preserve\">"
            );
        }
        NodeKind::Comment => {
            let _ = writeln!(out, "{indent}<comment xml:space=\"preserve\">");
        }
        NodeKind::Reference {
            name,
            refuri,
            anonymous,
        } => {
            let mut s = format!("{indent}<reference");
            if *anonymous {
                let _ = write!(s, " anonymous=\"1\"");
            }
            let _ = write!(s, " name=\"{name}\" refuri=\"{refuri}\"");
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
        NodeKind::Target {
            ids,
            names,
            refuri,
            anonymous,
        } => {
            let mut s = format!("{indent}<target");
            if *anonymous {
                let _ = write!(s, " anonymous=\"1\"");
            }
            let _ = write!(s, " ids=\"{ids}\"");
            if !names.is_empty() {
                let _ = write!(s, " names=\"{names}\"");
            }
            let _ = write!(s, " refuri=\"{refuri}\"");
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
        NodeKind::SubstitutionDefinition { names } => {
            let _ = writeln!(out, "{indent}<substitution_definition names=\"{names}\">");
        }
        NodeKind::SubstitutionReference { refname } => {
            let _ = writeln!(
                out,
                "{indent}<substitution_reference refname=\"{refname}\">"
            );
        }
        NodeKind::Table => {
            let _ = writeln!(out, "{indent}<table>");
        }
        NodeKind::Tgroup { cols } => {
            let _ = writeln!(out, "{indent}<tgroup cols=\"{cols}\">");
        }
        NodeKind::Colspec { colwidth } => {
            let _ = writeln!(out, "{indent}<colspec colwidth=\"{colwidth}\">");
        }
        NodeKind::Thead => {
            let _ = writeln!(out, "{indent}<thead>");
        }
        NodeKind::Tbody => {
            let _ = writeln!(out, "{indent}<tbody>");
        }
        NodeKind::Row => {
            let _ = writeln!(out, "{indent}<row>");
        }
        NodeKind::Entry { morecols, morerows } => {
            let mut s = format!("{indent}<entry");
            if *morecols > 0 {
                let _ = write!(s, " morecols=\"{morecols}\"");
            }
            if *morerows > 0 {
                let _ = write!(s, " morerows=\"{morerows}\"");
            }
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
        NodeKind::Attribution => {
            let _ = writeln!(out, "{indent}<attribution>");
        }
        NodeKind::Figure => {
            let _ = writeln!(out, "{indent}<figure>");
        }
        NodeKind::Caption => {
            let _ = writeln!(out, "{indent}<caption>");
        }
        NodeKind::Legend => {
            let _ = writeln!(out, "{indent}<legend>");
        }
        NodeKind::Label => {
            let _ = writeln!(out, "{indent}<label>");
        }
        NodeKind::Footnote {
            ids,
            names,
            backrefs,
            auto,
        } => {
            // Attribute order (alphabetical): auto, backrefs, ids, names.
            let mut s = format!("{indent}<footnote");
            if let Some(a) = auto {
                let _ = write!(s, " auto=\"{a}\"");
            }
            if !backrefs.is_empty() {
                let _ = write!(s, " backrefs=\"{backrefs}\"");
            }
            let _ = write!(s, " ids=\"{ids}\"");
            // Autosymbol footnotes have no `names` attribute (label is `*`).
            if !names.is_empty() && !matches!(*auto, Some("*")) {
                let _ = write!(s, " names=\"{names}\"");
            }
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
        NodeKind::FootnoteReference { ids, refid, auto } => {
            let mut s = format!("{indent}<footnote_reference");
            if let Some(a) = auto {
                let _ = write!(s, " auto=\"{a}\"");
            }
            let _ = write!(s, " ids=\"{ids}\" refid=\"{refid}\"");
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
        NodeKind::Citation {
            ids,
            names,
            backrefs,
        } => {
            let mut s = format!("{indent}<citation");
            if !backrefs.is_empty() {
                let _ = write!(s, " backrefs=\"{backrefs}\"");
            }
            let _ = write!(s, " ids=\"{ids}\" names=\"{names}\"");
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
        NodeKind::CitationReference { ids, refid } => {
            let _ = writeln!(
                out,
                "{indent}<citation_reference ids=\"{ids}\" refid=\"{refid}\">"
            );
        }
        NodeKind::Problematic { ids, refid } => {
            let _ = writeln!(
                out,
                "{indent}<problematic ids=\"{ids}\" refid=\"{refid}\">"
            );
        }
        NodeKind::SystemMessage {
            level,
            line,
            ty,
            ids,
            backrefs,
        } => {
            let mut s = format!("{indent}<system_message");
            if !backrefs.is_empty() {
                let _ = write!(s, " backrefs=\"{backrefs}\"");
            }
            if !ids.is_empty() {
                let _ = write!(s, " ids=\"{ids}\"");
            }
            let _ = write!(s, " level=\"{level}\"");
            if let Some(l) = line {
                let _ = write!(s, " line=\"{l}\"");
            }
            // SystemMessage always carries source="..." in upstream output;
            // we omit it because we don't track per-message source paths.
            let _ = write!(s, " source=\"<string>\"");
            let _ = write!(s, " type=\"{ty}\"");
            s.push('>');
            s.push('\n');
            out.push_str(&s);
        }
    }
    for &child in &node.children {
        write_node(tree, child, depth + 1, out);
    }
}
