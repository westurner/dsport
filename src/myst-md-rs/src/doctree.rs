//! MyST Markdown to `docutilsrs` doctree lowering.
//!
//! The HTML renderer is intentionally kept separate from this module. Sphinx
//! needs a docutils-shaped tree for transforms, persistence, references, and
//! writers; converting Markdown to HTML first would bypass all of those
//! stages.

use docutilsrs::doctree::NodeId;
use docutilsrs::{Doctree, NodeKind};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::{frontmatter, preprocess, role};

/// Configuration for the Markdown-to-doctree bridge.
#[derive(Debug, Clone)]
pub struct DoctreeOptions {
    pub enable_tables: bool,
    pub enable_footnotes: bool,
    pub enable_strikethrough: bool,
    pub enable_tasklists: bool,
    pub enable_heading_attributes: bool,
    pub enable_gfm: bool,
    pub enable_definition_lists: bool,
}

impl Default for DoctreeOptions {
    fn default() -> Self {
        Self {
            enable_tables: true,
            enable_footnotes: true,
            enable_strikethrough: true,
            enable_tasklists: true,
            enable_heading_attributes: true,
            enable_gfm: true,
            enable_definition_lists: true,
        }
    }
}

enum Frame {
    Container(NodeId),
    Heading {
        section: NodeId,
        title: NodeId,
        level: usize,
        explicit_id: Option<String>,
        classes: String,
    },
    Code {
        id: NodeId,
        kind: CodeKind,
        body: String,
    },
    Image(NodeId),
    Table {
        tgroup: NodeId,
    },
    DefinitionList {
        id: NodeId,
        last_item: Option<NodeId>,
    },
}

enum CodeKind {
    Literal(String),
    Directive(String),
    Math,
}

impl Frame {
    fn id(&self) -> NodeId {
        match self {
            Self::Container(id) | Self::Image(id) => *id,
            Self::Heading { title, .. } => *title,
            Self::Code { id, .. } => *id,
            Self::Table { tgroup } => *tgroup,
            Self::DefinitionList { id, .. } => *id,
        }
    }
}

/// Parse MyST Markdown into a native [`docutilsrs::Doctree`].
///
/// `source_path` is written to the document's `source` attribute and should
/// be the source file path used by the Sphinx environment. The parser options
/// are explicit so this API does not depend on a global HTML renderer.
pub fn parse_to_doctree(
    source: &str,
    source_path: impl Into<String>,
    options: &DoctreeOptions,
) -> Doctree {
    let split = frontmatter::split(source);
    let body = preprocess::preprocess(split.body);
    let mut parser_options = Options::empty();
    if options.enable_tables {
        parser_options.insert(Options::ENABLE_TABLES);
    }
    if options.enable_footnotes {
        parser_options.insert(Options::ENABLE_FOOTNOTES);
    }
    if options.enable_strikethrough {
        parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    }
    if options.enable_tasklists {
        parser_options.insert(Options::ENABLE_TASKLISTS);
    }
    if options.enable_heading_attributes {
        parser_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    }
    if options.enable_gfm {
        parser_options.insert(Options::ENABLE_GFM);
    }
    if options.enable_definition_lists {
        parser_options.insert(Options::ENABLE_DEFINITION_LIST);
    }

    let mut tree = Doctree::new_document(source_path);
    let root = tree.root();
    let mut frames = Vec::new();
    let mut sections: Vec<(usize, NodeId)> = Vec::new();
    let mut first_h1_title = None;

    for event in Parser::new_ext(&body, parser_options) {
        match event {
            Event::Start(tag) => start_tag(&mut tree, &mut frames, &mut sections, root, tag),
            Event::End(end) => end_tag(
                &mut tree,
                &mut frames,
                &mut sections,
                end,
                &mut first_h1_title,
            ),
            Event::Text(text) => append_text(&mut tree, &mut frames, &sections, root, &text),
            Event::Code(text) => {
                let parent = current_parent(&frames, &sections, root);
                let id = tree.append(parent, NodeKind::Literal);
                tree.append(id, NodeKind::Text(text.into_string()));
            }
            Event::InlineMath(math) => {
                let parent = current_parent(&frames, &sections, root);
                tree.append(
                    parent,
                    NodeKind::Math {
                        latex: math.into_string(),
                    },
                );
            }
            Event::DisplayMath(math) => {
                let parent = current_parent(&frames, &sections, root);
                tree.append(
                    parent,
                    NodeKind::MathBlock {
                        latex: math.into_string(),
                    },
                );
            }
            Event::SoftBreak | Event::HardBreak => {
                append_text(&mut tree, &mut frames, &sections, root, "\n")
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                let parent = current_parent(&frames, &sections, root);
                let raw = tree.append(
                    parent,
                    NodeKind::Raw {
                        format: "html".into(),
                    },
                );
                tree.append(raw, NodeKind::Text(html.into_string()));
            }
            Event::Rule => {
                let parent = current_parent(&frames, &sections, root);
                tree.append(parent, NodeKind::Transition);
            }
            Event::TaskListMarker(checked) => {
                let parent = current_parent(&frames, &sections, root);
                tree.append(
                    parent,
                    NodeKind::Text(if checked { "[x] " } else { "[ ] " }.into()),
                );
            }
            Event::FootnoteReference(label) => {
                let parent = current_parent(&frames, &sections, root);
                tree.append(parent, NodeKind::Text(format!("[^{label}]")));
            }
        }
    }

    if let Some(title) = first_h1_title {
        if let NodeKind::Document {
            source, ids, names, ..
        } = tree.node(root).kind.clone()
        {
            tree.set_kind(
                root,
                NodeKind::Document {
                    source,
                    ids,
                    names,
                    title,
                },
            );
        }
    }

    tree
}

fn start_tag(
    tree: &mut Doctree,
    frames: &mut Vec<Frame>,
    sections: &mut Vec<(usize, NodeId)>,
    root: NodeId,
    tag: Tag<'_>,
) {
    match tag {
        Tag::Heading {
            level, id, classes, ..
        } => {
            let level = heading_level(level);
            while sections.last().is_some_and(|(old, _)| *old >= level) {
                sections.pop();
            }
            let parent = sections.last().map(|(_, id)| *id).unwrap_or(root);
            let classes = classes
                .iter()
                .map(|class| class.as_ref())
                .collect::<Vec<_>>()
                .join(" ");
            let explicit_id = id.clone().map(String::from);
            let section = tree.append(
                parent,
                NodeKind::Section {
                    ids: explicit_id.clone().unwrap_or_default(),
                    names: String::new(),
                    classes: classes.clone(),
                },
            );
            let title = tree.append(section, NodeKind::Title);
            sections.push((level, section));
            frames.push(Frame::Heading {
                section,
                title,
                level,
                explicit_id,
                classes,
            });
        }
        Tag::Paragraph => push_container(tree, frames, sections, root, NodeKind::Paragraph),
        Tag::Emphasis => push_container(tree, frames, sections, root, NodeKind::Emphasis),
        Tag::Strong => push_container(tree, frames, sections, root, NodeKind::Strong),
        Tag::Strikethrough => push_container(
            tree,
            frames,
            sections,
            root,
            NodeKind::Inline {
                classes: "strikethrough".into(),
            },
        ),
        Tag::Superscript => push_container(tree, frames, sections, root, NodeKind::Superscript),
        Tag::Subscript => push_container(tree, frames, sections, root, NodeKind::Subscript),
        Tag::BlockQuote(_) => push_container(tree, frames, sections, root, NodeKind::BlockQuote),
        Tag::List(start) => {
            let kind = match start {
                Some(start) => NodeKind::EnumeratedList {
                    enumtype: "arabic",
                    prefix: String::new(),
                    suffix: ".".into(),
                    start: Some(start as u32),
                },
                None => NodeKind::BulletList { bullet: '-' },
            };
            push_container(tree, frames, sections, root, kind);
        }
        Tag::Item => push_container(tree, frames, sections, root, NodeKind::ListItem),
        Tag::Link { dest_url, .. } => {
            let dest = dest_url.into_string();
            let classes = if dest.starts_with('#') {
                "reference internal"
            } else {
                "reference external"
            };
            let parent = current_parent(frames, sections, root);
            let id = tree.append(
                parent,
                NodeKind::Reference {
                    name: String::new(),
                    refuri: dest,
                    anonymous: false,
                    classes: classes.into(),
                },
            );
            frames.push(Frame::Container(id));
        }
        Tag::Image { dest_url, .. } => {
            let parent = current_parent(frames, sections, root);
            let id = tree.append(
                parent,
                NodeKind::Image {
                    uri: dest_url.into_string(),
                    alt: None,
                    width: None,
                    height: None,
                },
            );
            frames.push(Frame::Image(id));
        }
        Tag::CodeBlock(kind) => {
            let code_kind = match kind {
                CodeBlockKind::Fenced(info) => {
                    let info = info.into_string();
                    if info == "math" {
                        CodeKind::Math
                    } else if let Some(name) = directive_name(&info) {
                        CodeKind::Directive(name.to_string())
                    } else {
                        CodeKind::Literal(info)
                    }
                }
                CodeBlockKind::Indented => CodeKind::Literal(String::new()),
            };
            let node_kind = match &code_kind {
                CodeKind::Math => NodeKind::MathBlock {
                    latex: String::new(),
                },
                CodeKind::Directive(name) if is_admonition(name) => NodeKind::Admonition {
                    kind: admonition_kind(name),
                },
                CodeKind::Directive(name) => NodeKind::Container {
                    classes: format!("myst-directive {name}"),
                },
                CodeKind::Literal(info) => NodeKind::LiteralBlock {
                    classes: if info.is_empty() {
                        String::new()
                    } else {
                        format!("code {info}")
                    },
                },
            };
            let parent = current_parent(frames, sections, root);
            let id = tree.append(parent, node_kind);
            frames.push(Frame::Code {
                id,
                kind: code_kind,
                body: String::new(),
            });
        }
        Tag::Table(alignments) => {
            let parent = current_parent(frames, sections, root);
            let table = tree.append(parent, NodeKind::Table);
            let tgroup = tree.append(
                table,
                NodeKind::Tgroup {
                    cols: alignments.len() as u32,
                },
            );
            let width = if alignments.is_empty() {
                0
            } else {
                100 / alignments.len() as u32
            };
            for _ in alignments {
                tree.append(tgroup, NodeKind::Colspec { colwidth: width });
            }
            frames.push(Frame::Table { tgroup });
        }
        Tag::TableHead => {
            let parent = current_parent(frames, sections, root);
            let head = tree.append(parent, NodeKind::Thead);
            frames.push(Frame::Container(head));
            let row = tree.append(head, NodeKind::Row);
            frames.push(Frame::Container(row));
        }
        Tag::TableRow => {
            let parent = current_parent(frames, sections, root);
            let row = tree.append(parent, NodeKind::Row);
            frames.push(Frame::Container(row));
        }
        Tag::TableCell => {
            let parent = current_parent(frames, sections, root);
            let entry = tree.append(
                parent,
                NodeKind::Entry {
                    morecols: 0,
                    morerows: 0,
                },
            );
            let paragraph = tree.append(entry, NodeKind::Paragraph);
            frames.push(Frame::Container(paragraph));
        }
        Tag::DefinitionList => {
            let parent = current_parent(frames, sections, root);
            let id = tree.append(parent, NodeKind::DefinitionList);
            frames.push(Frame::DefinitionList {
                id,
                last_item: None,
            });
        }
        Tag::DefinitionListTitle => {
            let Some(index) = frames
                .iter()
                .rposition(|frame| matches!(frame, Frame::DefinitionList { .. }))
            else {
                return;
            };
            let list = frames[index].id();
            let item = tree.append(list, NodeKind::DefinitionListItem);
            let term = tree.append(item, NodeKind::Term);
            if let Frame::DefinitionList { last_item, .. } = &mut frames[index] {
                *last_item = Some(item);
            }
            frames.push(Frame::Container(term));
        }
        Tag::DefinitionListDefinition => {
            let Some(item) = frames.iter().rev().find_map(|frame| {
                if let Frame::DefinitionList { last_item, .. } = frame {
                    *last_item
                } else {
                    None
                }
            }) else {
                return;
            };
            let definition = tree.append(item, NodeKind::Definition);
            let paragraph = tree.append(definition, NodeKind::Paragraph);
            frames.push(Frame::Container(paragraph));
        }
        Tag::HtmlBlock | Tag::FootnoteDefinition(_) | Tag::MetadataBlock(_) => {}
    }
}

fn end_tag(
    tree: &mut Doctree,
    frames: &mut Vec<Frame>,
    sections: &mut Vec<(usize, NodeId)>,
    end: TagEnd,
    first_h1_title: &mut Option<String>,
) {
    match end {
        TagEnd::Heading(level) => {
            let Some(Frame::Heading {
                section,
                title,
                level: heading_level,
                explicit_id,
                classes,
            }) = frames.pop()
            else {
                return;
            };
            let heading_text = collect_text(tree, title);
            let id = explicit_id.unwrap_or_else(|| slugify(&heading_text));
            tree.set_kind(
                section,
                NodeKind::Section {
                    ids: id,
                    names: heading_text.clone(),
                    classes,
                },
            );
            if heading_level == 1 && first_h1_title.is_none() {
                *first_h1_title = Some(heading_text);
            }
            let _ = level;
        }
        TagEnd::CodeBlock => {
            let Some(Frame::Code { id, kind, body }) = frames.pop() else {
                return;
            };
            let body = body.strip_suffix('\n').unwrap_or(&body).to_string();
            match kind {
                CodeKind::Math => tree.set_kind(id, NodeKind::MathBlock { latex: body }),
                CodeKind::Directive(_) => {
                    let paragraph = tree.append(id, NodeKind::Paragraph);
                    if !body.is_empty() {
                        tree.append(paragraph, NodeKind::Text(body));
                    }
                }
                CodeKind::Literal(_) => {
                    if !body.is_empty() {
                        tree.append(id, NodeKind::Text(body));
                    }
                }
            }
        }
        TagEnd::Table => {
            if matches!(frames.last(), Some(Frame::Container(_))) {
                frames.pop();
            }
            if matches!(frames.last(), Some(Frame::Table { .. })) {
                frames.pop();
            }
        }
        TagEnd::TableHead => {
            let _ = frames.pop();
            let _ = frames.pop();
            let Some(tgroup) = frames.iter().rev().find_map(|frame| {
                if let Frame::Table { tgroup } = frame {
                    Some(*tgroup)
                } else {
                    None
                }
            }) else {
                return;
            };
            let body = tree.append(tgroup, NodeKind::Tbody);
            frames.push(Frame::Container(body));
        }
        TagEnd::TableRow | TagEnd::TableCell => {
            let _ = frames.pop();
        }
        TagEnd::DefinitionListTitle | TagEnd::DefinitionListDefinition => {
            let _ = frames.pop();
        }
        TagEnd::DefinitionList => {
            if matches!(frames.last(), Some(Frame::DefinitionList { .. })) {
                frames.pop();
            }
        }
        TagEnd::Image
        | TagEnd::Paragraph
        | TagEnd::BlockQuote(_)
        | TagEnd::List(_)
        | TagEnd::Item
        | TagEnd::Emphasis
        | TagEnd::Strong
        | TagEnd::Strikethrough
        | TagEnd::Superscript
        | TagEnd::Subscript
        | TagEnd::Link => {
            let _ = frames.pop();
        }
        TagEnd::HtmlBlock | TagEnd::FootnoteDefinition | TagEnd::MetadataBlock(_) => {}
    }
    let _ = sections;
}

fn append_text(
    tree: &mut Doctree,
    frames: &mut [Frame],
    sections: &[(usize, NodeId)],
    root: NodeId,
    text: &str,
) {
    if let Some(Frame::Code { body, .. }) = frames.last_mut() {
        body.push_str(text);
        return;
    }
    if let Some(Frame::Image(id)) = frames.last() {
        if let NodeKind::Image { alt, .. } = &mut tree.node_mut(*id).kind {
            if alt.is_none() {
                *alt = Some(text.to_string());
            }
        }
        return;
    }
    let parent = current_parent(frames, sections, root);
    for piece in role::split_text(text) {
        match piece {
            role::Piece::Text(value) => {
                if !value.is_empty() {
                    tree.append(parent, NodeKind::Text(value.into()));
                }
            }
            role::Piece::Role { name, content } => {
                let id = tree.append(
                    parent,
                    NodeKind::Inline {
                        classes: format!("myst-role {name}"),
                    },
                );
                tree.append(id, NodeKind::Text(content.into()));
            }
            role::Piece::InlineMath(content) => {
                tree.append(
                    parent,
                    NodeKind::Math {
                        latex: content.into(),
                    },
                );
            }
        }
    }
}

fn push_container(
    tree: &mut Doctree,
    frames: &mut Vec<Frame>,
    sections: &[(usize, NodeId)],
    root: NodeId,
    kind: NodeKind,
) {
    let parent = current_parent(frames, sections, root);
    let id = tree.append(parent, kind);
    frames.push(Frame::Container(id));
}

fn current_parent(frames: &[Frame], sections: &[(usize, NodeId)], root: NodeId) -> NodeId {
    frames
        .last()
        .map(Frame::id)
        .or_else(|| sections.last().map(|(_, id)| *id))
        .unwrap_or(root)
}

fn heading_level(level: HeadingLevel) -> usize {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn collect_text(tree: &Doctree, id: NodeId) -> String {
    let node = tree.node(id);
    if let NodeKind::Text(text) = &node.kind {
        return text.clone();
    }
    node.children
        .iter()
        .map(|child| collect_text(tree, *child))
        .collect()
}

fn slugify(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() {
            out.extend(ch.to_lowercase());
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
}

fn directive_name(info: &str) -> Option<&str> {
    let first = info.split_whitespace().next().unwrap_or("");
    let inner = first.strip_prefix('{')?.strip_suffix('}')?;
    (!inner.is_empty()).then_some(inner)
}

fn is_admonition(name: &str) -> bool {
    matches!(
        name,
        "attention"
            | "caution"
            | "danger"
            | "error"
            | "hint"
            | "important"
            | "note"
            | "seealso"
            | "tip"
            | "warning"
    )
}

fn admonition_kind(name: &str) -> &'static str {
    match name {
        "attention" => "attention",
        "caution" => "caution",
        "danger" => "danger",
        "error" => "error",
        "hint" => "hint",
        "important" => "important",
        "seealso" => "seealso",
        "tip" => "tip",
        "warning" => "warning",
        _ => "note",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn child_kinds(tree: &Doctree, parent: NodeId) -> Vec<NodeKind> {
        tree.node(parent)
            .children
            .iter()
            .map(|id| tree.node(*id).kind.clone())
            .collect()
    }

    #[test]
    fn lowers_sections_inline_nodes_and_source_metadata() {
        let tree = parse_to_doctree(
            "# Guide\n\nA **bold** *word* with `code` and [link](guide.md).\n",
            "docs/guide.md",
            &DoctreeOptions::default(),
        );

        assert!(matches!(
            tree.node(tree.root()).kind,
            NodeKind::Document { ref source, ref title, .. }
                if source == "docs/guide.md" && title == "Guide"
        ));
        let section = tree.node(tree.root()).children[0];
        assert!(matches!(
            tree.node(section).kind,
            NodeKind::Section { ref ids, ref names, .. }
                if ids == "guide" && names == "Guide"
        ));
        let title = tree.node(section).children[0];
        assert!(matches!(tree.node(title).kind, NodeKind::Title));
        let paragraph = tree.node(section).children[1];
        assert!(matches!(tree.node(paragraph).kind, NodeKind::Paragraph));
        let kinds = child_kinds(&tree, paragraph);
        assert!(kinds.iter().any(|kind| matches!(kind, NodeKind::Strong)));
        assert!(kinds.iter().any(|kind| matches!(kind, NodeKind::Emphasis)));
        assert!(kinds.iter().any(|kind| matches!(kind, NodeKind::Literal)));
        assert!(
            kinds
                .iter()
                .any(|kind| matches!(kind, NodeKind::Reference { .. }))
        );
    }

    #[test]
    fn lowers_directives_math_and_front_matter_without_html_intermediate() {
        let source = "---\ntitle: Demo\n---\n\n# Demo\n\n:::note\nRead this.\n:::\n\nInline $x^2$ and:\n\n$$\ny = mx + b\n$$\n";
        let tree = parse_to_doctree(source, "index.md", &DoctreeOptions::default());

        let xml = docutilsrs::to_xml(&tree);
        assert!(xml.contains("<note>"));
        assert!(xml.contains("<math>"));
        assert!(xml.contains("<math_block"));
        assert!(
            !xml.contains("myst-directive"),
            "HTML wrapper must not be the bridge representation"
        );
    }

    #[test]
    fn doctree_round_trip_preserves_native_html_output() {
        let tree = parse_to_doctree(
            "# Title\n\nText with **emphasis**.\n",
            "guide.md",
            &DoctreeOptions::default(),
        );
        let restored = Doctree::from_bytes(&tree.to_bytes()).expect("doctree should deserialize");
        assert_eq!(
            docutilsrs::html5(&tree, &Default::default(), &Default::default()),
            docutilsrs::html5(&restored, &Default::default(), &Default::default())
        );
    }

    #[test]
    fn lowers_tables_definition_lists_and_heading_attributes() {
        let source =
            "# Title {#custom .lead}\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\nTerm\n: Definition\n";
        let tree = parse_to_doctree(source, "index.md", &DoctreeOptions::default());
        let xml = docutilsrs::to_xml(&tree);

        assert!(xml.contains("<section classes=\"lead\" ids=\"custom\" names=\"Title\">"));
        assert!(xml.contains("<table>"));
        assert!(xml.contains("<tgroup cols=\"2\">"));
        assert!(xml.contains("<thead>"));
        assert!(xml.contains("<tbody>"));
        assert!(xml.contains("<entry>"));
        assert!(xml.contains("<definition_list>"));
        assert!(xml.contains("<definition_list_item>"));
        assert!(xml.contains("<term>"));
        assert!(xml.contains("<definition>"));
        let definition = xml.find("<definition>").expect("definition node");
        assert!(xml[definition..].contains("<paragraph>"));
    }
}
