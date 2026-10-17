//! MyST Markdown to `docutilsrs` doctree lowering.
//!
//! The HTML renderer is intentionally kept separate from this module. Sphinx
//! needs a docutils-shaped tree for transforms, persistence, references, and
//! writers; converting Markdown to HTML first would bypass all of those
//! stages.

use docutilsrs::doctree::NodeId;
use docutilsrs::{Doctree, NodeKind};
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Path, PathBuf};

use crate::{directives, frontmatter, preprocess, role};

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
    pub enable_table_spans: bool,
    pub substitutions: BTreeMap<String, String>,
    /// Root directory that MyST include directives may read from.
    pub source_root: Option<PathBuf>,
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
            enable_table_spans: true,
            substitutions: BTreeMap::new(),
            source_root: None,
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
        parent: NodeId,
        kind: CodeKind,
        body: String,
    },
    Image(NodeId),
    Table {
        tgroup: NodeId,
        alignments: Vec<Alignment>,
        next_cell: usize,
    },
    DefinitionList {
        id: NodeId,
        last_item: Option<NodeId>,
    },
}

enum CodeKind {
    Literal(String),
    Directive { name: String, argument: String },
    Math,
}

impl Frame {
    fn id(&self) -> NodeId {
        match self {
            Self::Container(id) | Self::Image(id) => *id,
            Self::Heading { title, .. } => *title,
            Self::Code { id, .. } => *id,
            Self::Table { tgroup, .. } => *tgroup,
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
    parse_to_doctree_with_registered_roles(source, source_path, options, &[])
}

/// Parse MyST with additional roles registered by the hosting application.
pub fn parse_to_doctree_with_registered_roles(
    source: &str,
    source_path: impl Into<String>,
    options: &DoctreeOptions,
    registered_roles: &[String],
) -> Doctree {
    let source_path = source_path.into();
    let source_path_buf = PathBuf::from(&source_path);
    let source_root = options
        .source_root
        .as_deref()
        .or_else(|| source_path_buf.parent())
        .unwrap_or_else(|| Path::new("."));
    let source_root = canonical_or_absolute(source_root);
    let mut include_stack = vec![canonical_or_absolute(&source_path_buf)];
    parse_to_doctree_inner(
        source,
        &source_path,
        options,
        &source_root,
        &mut include_stack,
        registered_roles,
    )
}

fn parse_to_doctree_inner(
    source: &str,
    source_path: &str,
    options: &DoctreeOptions,
    source_root: &Path,
    include_stack: &mut Vec<PathBuf>,
    registered_roles: &[String],
) -> Doctree {
    let split = frontmatter::split(source);
    let front_matter_diagnostics = front_matter_diagnostics(&split);
    let substitutions = collect_substitutions(split.front_matter.as_ref(), &options.substitutions);
    let body_start = source.len() - split.body.len();
    let body_line_offset = source[..body_start]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count() as u32;
    let (substituted, substitution_diagnostics) = apply_substitutions(split.body, &substitutions);
    let duplicate_reference_diagnostics = duplicate_reference_definitions(split.body);
    let body = preprocess::preprocess(&substituted);
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
    if options.enable_table_spans {
        parser_options.insert(Options::ENABLE_TABLE_SPANS);
    }

    let mut tree = Doctree::new_document(source_path.to_owned());
    let root = tree.root();
    let mut frames = Vec::new();
    let mut sections: Vec<(usize, NodeId)> = Vec::new();
    let mut pending_role: Option<(String, u32)> = None;
    let mut previous_heading_level = None;
    let mut heading_warnings = Vec::new();
    for (event, offset) in Parser::new_ext(&body, parser_options).into_offset_iter() {
        let mut line = body[..offset.start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count() as u32
            + body_line_offset
            + 1;
        if let Event::Start(Tag::Heading { level, .. }) = &event {
            let current_level = heading_level(*level);
            match previous_heading_level {
                None if current_level > 1 => heading_warnings.push((
                    line,
                    format!(
                        "Document headings start at H{current_level}, not H1 [myst.header]"
                    ),
                )),
                Some(previous) if current_level > previous + 1 => heading_warnings.push((
                    line,
                    format!(
                        "Non-consecutive header level increase; H{previous} to H{current_level} [myst.header]"
                    ),
                )),
                _ => {}
            }
            previous_heading_level = Some(current_level);
        }
        if !matches!(&event, Event::Code(_)) {
            if let Some((name, role_line)) = pending_role.take() {
                let start = tree.nodes_len();
                append_text(
                    &mut tree,
                    &mut frames,
                    &sections,
                    root,
                    &format!("{{{name}}}"),
                    role_line,
                    source_path,
                    registered_roles,
                );
                for id in start..tree.nodes_len() {
                    tree.set_line(id, role_line);
                }
            }
        }
        let node_start = tree.nodes_len();
        match event {
            Event::Start(tag) => start_tag(&mut tree, &mut frames, &mut sections, root, tag),
            Event::End(end) => end_tag(
                &mut tree,
                &mut frames,
                &mut sections,
                end,
                options,
                source_root,
                include_stack,
                registered_roles,
            ),
            Event::Text(text) => {
                let text = text.into_string();
                if let Some((prefix, name)) = role::role_opener_suffix(&text) {
                    append_text(
                        &mut tree,
                        &mut frames,
                        &sections,
                        root,
                        prefix,
                        line,
                        source_path,
                        registered_roles,
                    );
                    pending_role = Some((name.to_owned(), line));
                } else {
                    append_text(
                        &mut tree,
                        &mut frames,
                        &sections,
                        root,
                        &text,
                        line,
                        source_path,
                        registered_roles,
                    );
                }
            }
            Event::Code(text) => {
                let parent = content_parent(&mut tree, &frames, &sections, root);
                if let Some((name, role_line)) = pending_role.take() {
                    line = role_line;
                    append_role(
                        &mut tree,
                        parent,
                        &name,
                        &text,
                        role_line,
                        source_path,
                        registered_roles,
                    );
                } else {
                    let id = tree.append(parent, NodeKind::Literal);
                    tree.append(id, NodeKind::Text(text.into_string()));
                }
            }
            Event::InlineMath(math) => {
                let parent = content_parent(&mut tree, &frames, &sections, root);
                tree.append(
                    parent,
                    NodeKind::Math {
                        latex: math.into_string(),
                    },
                );
            }
            Event::DisplayMath(math) => {
                let parent = content_parent(&mut tree, &frames, &sections, root);
                tree.append(
                    parent,
                    NodeKind::MathBlock {
                        latex: math.into_string(),
                    },
                );
            }
            Event::SoftBreak | Event::HardBreak => append_text(
                &mut tree,
                &mut frames,
                &sections,
                root,
                "\n",
                line,
                source_path,
                registered_roles,
            ),
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
                let parent = content_parent(&mut tree, &frames, &sections, root);
                tree.append(parent, NodeKind::Transition);
            }
            Event::TaskListMarker(checked) => {
                let parent = content_parent(&mut tree, &frames, &sections, root);
                tree.append(
                    parent,
                    NodeKind::Text(if checked { "[x] " } else { "[ ] " }.into()),
                );
            }
            Event::FootnoteReference(label) => {
                let parent = content_parent(&mut tree, &frames, &sections, root);
                tree.append(parent, NodeKind::Text(format!("[^{label}]")));
            }
        }
        for id in node_start..tree.nodes_len() {
            if tree.node(id).line.is_none() {
                tree.set_line(id, line);
            }
            if let NodeKind::SystemMessage {
                level,
                line: message_line,
                ty,
                ids,
                backrefs,
                ..
            } = tree.node(id).kind.clone()
            {
                tree.set_kind(
                    id,
                    NodeKind::SystemMessage {
                        level,
                        line: Some(message_line.unwrap_or(line)),
                        ty,
                        ids,
                        backrefs,
                    },
                );
            }
        }
    }

    if let Some((name, role_line)) = pending_role {
        let start = tree.nodes_len();
        append_text(
            &mut tree,
            &mut frames,
            &sections,
            root,
            &format!("{{{name}}}"),
            role_line,
            source_path,
            registered_roles,
        );
        for id in start..tree.nodes_len() {
            tree.set_line(id, role_line);
        }
    }

    for (line, message) in substitution_diagnostics {
        let source_line = line + body_line_offset;
        let id = tree.append(
            root,
            NodeKind::SystemMessage {
                level: 2,
                line: Some(source_line),
                ty: "WARNING",
                ids: String::new(),
                backrefs: String::new(),
            },
        );
        tree.set_line(id, source_line);
        tree.append(id, NodeKind::Text(message));
    }
    for (line, message) in duplicate_reference_diagnostics {
        append_include_warning(&mut tree, root, line + body_line_offset, message);
    }
    for (level, ty, message) in front_matter_diagnostics {
        append_include_message(&mut tree, root, 1, level, ty, message);
    }
    for (line, message) in heading_warnings {
        append_include_warning(&mut tree, root, line, message);
    }
    for (line, target) in unresolved_local_links(&tree, source_path, source_root) {
        append_include_message(
            &mut tree,
            root,
            line,
            3,
            "ERROR",
            format!("Unknown target name: \"{target}\"."),
        );
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
        Tag::ContainerBlock(_, name) => push_container(
            tree,
            frames,
            sections,
            root,
            NodeKind::Container {
                classes: format!("myst-container {name}"),
            },
        ),
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
            let parent = content_parent(tree, frames, sections, root);
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
            let parent = content_parent(tree, frames, sections, root);
            let id = tree.append(
                parent,
                NodeKind::Image {
                    uri: dest_url.into_string(),
                    alt: None,
                    width: None,
                    height: None,
                    classes: String::new(),
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
                    } else if let Some((name, argument)) = directive_info(&info) {
                        CodeKind::Directive { name, argument }
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
                CodeKind::Directive { name, .. } if is_admonition(name) => NodeKind::Admonition {
                    kind: admonition_kind(name),
                },
                CodeKind::Directive { name, argument } if name == "admonition" => {
                    NodeKind::GenericAdmonition {
                        title: argument.clone(),
                        classes: String::new(),
                    }
                }
                CodeKind::Directive { name, .. } if known_directive(name) => NodeKind::Container {
                    classes: format!("myst-directive {name}"),
                },
                CodeKind::Directive { .. } => NodeKind::SystemMessage {
                    level: 2,
                    line: None,
                    ty: "WARNING",
                    ids: String::new(),
                    backrefs: String::new(),
                },
                CodeKind::Literal(info) => NodeKind::LiteralBlock {
                    classes: if info.is_empty() {
                        String::new()
                    } else {
                        format!("code {info}")
                    },
                    caption: None,
                },
            };
            let parent = current_parent(frames, sections, root);
            let id = tree.append(parent, node_kind);
            frames.push(Frame::Code {
                id,
                parent,
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
            for _ in &alignments {
                tree.append(tgroup, NodeKind::Colspec { colwidth: width });
            }
            frames.push(Frame::Table {
                tgroup,
                alignments,
                next_cell: 0,
            });
        }
        Tag::TableHead => {
            reset_table_cell_index(frames);
            let parent = current_parent(frames, sections, root);
            let head = tree.append(parent, NodeKind::Thead);
            frames.push(Frame::Container(head));
            let row = tree.append(head, NodeKind::Row);
            frames.push(Frame::Container(row));
        }
        Tag::TableRow => {
            reset_table_cell_index(frames);
            let parent = current_parent(frames, sections, root);
            let row = tree.append(parent, NodeKind::Row);
            frames.push(Frame::Container(row));
        }
        Tag::TableCell { colspan, rowspan } => {
            let parent = current_parent(frames, sections, root);
            let classes = table_cell_class(frames);
            let entry = tree.append(
                parent,
                NodeKind::Entry {
                    morecols: colspan.saturating_sub(1) as u32,
                    morerows: rowspan.saturating_sub(1) as u32,
                    classes,
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
    options: &DoctreeOptions,
    source_root: &Path,
    include_stack: &mut Vec<PathBuf>,
    registered_roles: &[String],
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
            let _ = heading_level;
            let _ = level;
        }
        TagEnd::CodeBlock => {
            let Some(Frame::Code {
                id,
                parent,
                kind,
                body,
            }) = frames.pop()
            else {
                return;
            };
            let body = body.strip_suffix('\n').unwrap_or(&body).to_string();
            match kind {
                CodeKind::Math => tree.set_kind(id, NodeKind::MathBlock { latex: body }),
                CodeKind::Directive { name, argument } if name == "include" => handle_include(
                    tree,
                    id,
                    parent,
                    &argument,
                    &body,
                    options,
                    source_root,
                    include_stack,
                    registered_roles,
                ),
                CodeKind::Directive { name, .. } if name == "eval-rst" => {
                    let source_line = tree.node(id).line.unwrap_or(0);
                    tree.detach(id);
                    let included = docutilsrs::parse_rst_with_source(&body, "<eval-rst>");
                    clone_children_with_line_offset(
                        &included,
                        included.root(),
                        tree,
                        parent,
                        source_line,
                    );
                }
                CodeKind::Directive { name, .. } if !known_directive(&name) => {
                    let line = tree.node(id).line.unwrap_or(1);
                    tree.append(
                        id,
                        NodeKind::Text(format!(
                            "Unknown directive type: '{name}' [myst.directive_unknown]"
                        )),
                    );
                    if !body.is_empty() {
                        let paragraph = tree.append(id, NodeKind::Paragraph);
                        tree.append(paragraph, NodeKind::Text(body));
                    }
                    tree.set_line(id, line);
                }
                CodeKind::Directive { name, argument } => {
                    let body_lines = body.lines().collect::<Vec<_>>();
                    let spec = directive_spec(&name);
                    let parsed = directives::parse_directive_text(&spec, &argument, &body_lines);
                    let (directive_body, class, warnings) = match parsed {
                        Ok(parsed) => (
                            parsed.body.join("\n"),
                            parsed.coerced_options.get("class").and_then(|value| {
                                if let directives::OptionValue::Classes(classes) = value {
                                    Some(classes.join(" "))
                                } else {
                                    None
                                }
                            }),
                            parsed.warnings,
                        ),
                        Err(_) => {
                            tree.set_kind(
                                id,
                                NodeKind::SystemMessage {
                                    level: 2,
                                    line: None,
                                    ty: "ERROR",
                                    ids: String::new(),
                                    backrefs: String::new(),
                                },
                            );
                            (body, None, Vec::new())
                        }
                    };
                    let line = tree.node(id).line.unwrap_or(1);
                    for warning in warnings {
                        append_include_warning(
                            tree,
                            parent,
                            line,
                            format!("'{name}': {warning} [myst.directive_option]"),
                        );
                    }
                    if spec.has_content && directive_body.trim().is_empty() {
                        append_include_message(
                            tree,
                            parent,
                            line,
                            3,
                            "ERROR",
                            format!(
                                "Content block expected for the \"{name}\" directive; none found."
                            ),
                        );
                    }
                    if let Some(class) = class {
                        match &mut tree.node_mut(id).kind {
                            NodeKind::Container { classes } => {
                                classes.push(' ');
                                classes.push_str(&class);
                            }
                            NodeKind::GenericAdmonition { classes, .. } => *classes = class,
                            _ => {}
                        }
                    }
                    let paragraph = tree.append(id, NodeKind::Paragraph);
                    if !directive_body.is_empty() {
                        tree.append(paragraph, NodeKind::Text(directive_body));
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
                if let Frame::Table { tgroup, .. } = frame {
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
        TagEnd::ContainerBlock(_) => {
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
    line: u32,
    source_path: &str,
    registered_roles: &[String],
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
    let parent = content_parent(tree, frames, sections, root);
    for piece in role::split_text(text) {
        match piece {
            role::Piece::Text(value) => {
                if !value.is_empty() {
                    tree.append(parent, NodeKind::Text(value.into()));
                }
            }
            role::Piece::Role { name, content } => {
                append_role(
                    tree,
                    parent,
                    name,
                    content,
                    line,
                    source_path,
                    registered_roles,
                );
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

fn append_role(
    tree: &mut Doctree,
    parent: NodeId,
    name: &str,
    content: &str,
    line: u32,
    source_path: &str,
    registered_roles: &[String],
) {
    let id = tree.append(
        parent,
        NodeKind::Inline {
            classes: format!("myst-role {name}"),
        },
    );
    tree.append(id, NodeKind::Text(content.to_owned()));
    if !role::is_known_role(name, registered_roles) {
        append_include_warning(
            tree,
            parent,
            line,
            format!("Unknown interpreted text role {name:?} in {source_path}. [myst.role_unknown]"),
        );
    }
}

fn push_container(
    tree: &mut Doctree,
    frames: &mut Vec<Frame>,
    sections: &[(usize, NodeId)],
    root: NodeId,
    kind: NodeKind,
) {
    let parent = if matches!(
        kind,
        NodeKind::Emphasis
            | NodeKind::Strong
            | NodeKind::Literal
            | NodeKind::Inline { .. }
            | NodeKind::Reference { .. }
            | NodeKind::Math { .. }
    ) {
        content_parent(tree, frames, sections, root)
    } else {
        current_parent(frames, sections, root)
    };
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

fn content_parent(
    tree: &mut Doctree,
    frames: &[Frame],
    sections: &[(usize, NodeId)],
    root: NodeId,
) -> NodeId {
    let parent = current_parent(frames, sections, root);
    if matches!(tree.node(parent).kind, NodeKind::ListItem)
        && !tree
            .node(parent)
            .children
            .iter()
            .any(|child| matches!(tree.node(*child).kind, NodeKind::Paragraph))
    {
        tree.append(parent, NodeKind::Paragraph)
    } else {
        parent
    }
}

fn reset_table_cell_index(frames: &mut [Frame]) {
    if let Some(Frame::Table { next_cell, .. }) = frames
        .iter_mut()
        .rev()
        .find(|frame| matches!(frame, Frame::Table { .. }))
    {
        *next_cell = 0;
    }
}

fn table_cell_class(frames: &mut [Frame]) -> String {
    let Some(Frame::Table {
        alignments,
        next_cell,
        ..
    }) = frames
        .iter_mut()
        .rev()
        .find(|frame| matches!(frame, Frame::Table { .. }))
    else {
        return String::new();
    };
    let alignment = alignments
        .get(*next_cell)
        .copied()
        .unwrap_or(Alignment::None);
    *next_cell += 1;
    match alignment {
        Alignment::Left => "text-left",
        Alignment::Center => "text-center",
        Alignment::Right => "text-right",
        Alignment::None => "",
    }
    .to_string()
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

fn directive_info(info: &str) -> Option<(String, String)> {
    let name = directive_name(info)?.to_string();
    let first = info.split_whitespace().next().unwrap_or("");
    let argument = info[first.len()..].trim().to_string();
    Some((name, argument))
}

fn canonical_or_absolute(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(path)
        }
    })
}

fn resolve_include_path(
    argument: &str,
    tree: &Doctree,
    source_root: &Path,
    include_stack: &[PathBuf],
) -> Result<PathBuf, String> {
    let source = match &tree.node(tree.root()).kind {
        NodeKind::Document { source, .. } => PathBuf::from(source),
        _ => PathBuf::new(),
    };
    let requested = PathBuf::from(argument);
    let candidate = if requested.is_absolute() {
        requested
    } else {
        source.parent().unwrap_or(source_root).join(requested)
    };
    let path = candidate.canonicalize().map_err(|_| {
        let missing = candidate.strip_prefix(source_root).unwrap_or(&candidate);
        format!("file not found: '{}'", missing.display())
    })?;
    if !path.starts_with(source_root) {
        return Err(format!(
            "MyST include {:?} escapes source root {}",
            argument,
            source_root.display()
        ));
    }
    if let Some(position) = include_stack.iter().position(|included| included == &path) {
        let mut cycle = include_stack[position..]
            .iter()
            .map(|included| included.display().to_string())
            .collect::<Vec<_>>();
        cycle.push(path.display().to_string());
        return Err(format!("recursive MyST include: {}", cycle.join(" -> ")));
    }
    Ok(path)
}

fn include_directive_spec() -> directives::DirectiveSpec {
    let mut option_spec = BTreeMap::new();
    option_spec.insert("literal".into(), directives::OptionKind::Flag);
    option_spec.insert("code".into(), directives::OptionKind::String);
    option_spec.insert("start-line".into(), directives::OptionKind::Integer);
    option_spec.insert("end-line".into(), directives::OptionKind::Integer);
    option_spec.insert("start-after".into(), directives::OptionKind::String);
    option_spec.insert("end-before".into(), directives::OptionKind::String);
    directives::DirectiveSpec {
        required_arguments: 1,
        has_content: false,
        final_argument_whitespace: true,
        option_spec,
        ..directives::DirectiveSpec::default()
    }
}

fn include_integer_option(
    parsed: &directives::ParsedDirective,
    name: &str,
) -> Result<Option<usize>, String> {
    match parsed.coerced_options.get(name) {
        Some(directives::OptionValue::Integer(value)) if *value >= 0 => Ok(Some(*value as usize)),
        Some(directives::OptionValue::Integer(_)) => {
            Err(format!("MyST include option {name:?} must not be negative"))
        }
        Some(_) => Err(format!("MyST include option {name:?} must be an integer")),
        None => Ok(None),
    }
}

fn include_string_option<'a>(
    parsed: &'a directives::ParsedDirective,
    name: &str,
) -> Option<&'a str> {
    match parsed.coerced_options.get(name) {
        Some(directives::OptionValue::String(value)) => Some(value),
        _ => None,
    }
}

fn select_include_lines(
    source: &str,
    parsed: &directives::ParsedDirective,
) -> Result<String, String> {
    let lines = source.lines().collect::<Vec<_>>();
    let start_line = include_integer_option(parsed, "start-line")?;
    let end_line = include_integer_option(parsed, "end-line")?;
    let start_after = include_string_option(parsed, "start-after");
    let end_before = include_string_option(parsed, "end-before");
    if start_line.is_some() && start_after.is_some() {
        return Err("MyST include start-line and start-after are mutually exclusive".into());
    }
    if end_line.is_some() && end_before.is_some() {
        return Err("MyST include end-line and end-before are mutually exclusive".into());
    }

    let start = if let Some(marker) = start_after {
        lines
            .iter()
            .position(|line| line.contains(marker))
            .map(|index| index + 1)
            .ok_or_else(|| format!("MyST include start-after marker {marker:?} was not found"))?
    } else {
        start_line.unwrap_or(0)
    };
    let end = if let Some(marker) = end_before {
        lines
            .iter()
            .enumerate()
            .skip(start.min(lines.len()))
            .find(|(_, line)| line.contains(marker))
            .map(|(index, _)| index)
            .unwrap_or(lines.len())
    } else {
        end_line.unwrap_or(lines.len()).min(lines.len())
    };
    if start > end {
        return Err("MyST include start position is after its end position".into());
    }

    let mut selected = lines[start.min(lines.len())..end].join("\n");
    if !selected.is_empty() && end == lines.len() && source.ends_with('\n') {
        selected.push('\n');
    }
    Ok(selected)
}

fn handle_include(
    tree: &mut Doctree,
    id: NodeId,
    parent: NodeId,
    argument: &str,
    body: &str,
    options: &DoctreeOptions,
    source_root: &Path,
    include_stack: &mut Vec<PathBuf>,
    registered_roles: &[String],
) {
    let line = tree.node(id).line.unwrap_or(1);
    tree.detach(id);
    let body_lines = body.lines().collect::<Vec<_>>();
    let parsed =
        match directives::parse_directive_text(&include_directive_spec(), argument, &body_lines) {
            Ok(parsed) => parsed,
            Err(error) => {
                let message = match error {
                    directives::DirectiveError::Syntax(message) => message,
                    directives::DirectiveError::Options(error) => format!("{error:?}"),
                };
                append_include_message(
                    tree,
                    parent,
                    line,
                    3,
                    "ERROR",
                    format!("Directive 'include': {message}"),
                );
                return;
            }
        };
    for warning in &parsed.warnings {
        append_include_warning(tree, parent, line, warning.clone());
    }

    let literal = parsed.coerced_options.contains_key("literal");
    let code = include_string_option(&parsed, "code");
    if literal && code.is_some() {
        append_include_warning(
            tree,
            parent,
            line,
            "MyST include literal and code options are mutually exclusive".into(),
        );
        return;
    }

    let path = match resolve_include_path(argument, tree, source_root, include_stack) {
        Ok(path) => path,
        Err(message) => {
            append_include_message(
                tree,
                parent,
                line,
                4,
                "SEVERE",
                format!("Directive \"include\": {message}"),
            );
            return;
        }
    };
    let included_source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            append_include_message(
                tree,
                parent,
                line,
                4,
                "SEVERE",
                format!("Directive \"include\": cannot read {argument:?}: {error}"),
            );
            return;
        }
    };
    let selected = match select_include_lines(&included_source, &parsed) {
        Ok(selected) => selected,
        Err(error) => {
            append_include_warning(tree, parent, line, error);
            return;
        }
    };
    if literal || code.is_some() {
        let classes = code.map_or_else(String::new, |language| {
            if language.is_empty() {
                "code".into()
            } else {
                format!("code {language}")
            }
        });
        let block = tree.append(
            parent,
            NodeKind::LiteralBlock {
                classes,
                caption: None,
            },
        );
        tree.set_line(block, line);
        tree.append(block, NodeKind::Text(selected));
        return;
    }

    let included_path = path.to_string_lossy().into_owned();
    include_stack.push(path);
    let included = parse_to_doctree_inner(
        &selected,
        &included_path,
        options,
        source_root,
        include_stack,
        registered_roles,
    );
    include_stack.pop();
    clone_children(&included, included.root(), tree, parent);
}

fn append_include_warning(tree: &mut Doctree, parent: NodeId, line: u32, message: String) {
    append_include_message(tree, parent, line, 2, "WARNING", message);
}

fn append_include_message(
    tree: &mut Doctree,
    parent: NodeId,
    line: u32,
    level: u32,
    ty: &'static str,
    message: String,
) {
    let id = tree.append(
        parent,
        NodeKind::SystemMessage {
            level,
            line: Some(line),
            ty,
            ids: String::new(),
            backrefs: String::new(),
        },
    );
    tree.set_line(id, line);
    tree.append(id, NodeKind::Text(message));
}

fn clone_children(
    source: &Doctree,
    source_parent: NodeId,
    destination: &mut Doctree,
    parent: NodeId,
) {
    clone_children_with_line_offset(source, source_parent, destination, parent, 0);
}

fn clone_children_with_line_offset(
    source: &Doctree,
    source_parent: NodeId,
    destination: &mut Doctree,
    parent: NodeId,
    line_offset: u32,
) {
    for child in source.node(source_parent).children.clone() {
        clone_node_with_line_offset(source, child, destination, parent, line_offset);
    }
}

fn clone_node_with_line_offset(
    source: &Doctree,
    source_id: NodeId,
    destination: &mut Doctree,
    parent: NodeId,
    line_offset: u32,
) {
    let source_node = source.node(source_id);
    let kind = match &source_node.kind {
        NodeKind::SystemMessage {
            level,
            line,
            ty,
            ids,
            backrefs,
        } => NodeKind::SystemMessage {
            level: *level,
            line: line.map(|line| line.saturating_add(line_offset)),
            ty,
            ids: ids.clone(),
            backrefs: backrefs.clone(),
        },
        kind => kind.clone(),
    };
    let id = destination.append(parent, kind);
    if let Some(line) = source_node.line {
        destination.set_line(id, line.saturating_add(line_offset));
    }
    for child in source_node.children.clone() {
        clone_node_with_line_offset(source, child, destination, id, line_offset);
    }
}

fn collect_substitutions(
    front_matter: Option<&serde_yaml::Value>,
    configured: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut substitutions = configured.clone();
    if let Some(serde_yaml::Value::Mapping(mapping)) = front_matter {
        if let Some(serde_yaml::Value::Mapping(values)) =
            mapping.get(serde_yaml::Value::String("myst".into()))
        {
            if let Some(serde_yaml::Value::Mapping(values)) =
                values.get(serde_yaml::Value::String("substitutions".into()))
            {
                for (key, value) in values {
                    if let (Some(key), Some(value)) = (key.as_str(), value.as_str()) {
                        substitutions.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
    }
    substitutions
}

fn front_matter_diagnostics(split: &frontmatter::Split<'_>) -> Vec<(u32, &'static str, String)> {
    const KNOWN_MYST_FIELDS: &[&str] = &[
        "all_links_external",
        "commonmark_only",
        "disable_syntax",
        "enable_extensions",
        "fence_as_directive",
        "footnote_sort",
        "footnote_transition",
        "gfm_only",
        "heading_anchors",
        "heading_slug_func",
        "html_meta",
        "links_external_new_tab",
        "number_code_blocks",
        "ref_domains",
        "sub_delimiters",
        "substitutions",
        "title_to_header",
        "url_schemes",
        "words_per_minute",
    ];

    let Some(raw_front_matter) = split.raw_front_matter else {
        return Vec::new();
    };
    let Some(front_matter) = split.front_matter.as_ref() else {
        let _ = raw_front_matter;
        return vec![(2, "WARNING", "Malformed YAML [myst.topmatter]".into())];
    };
    let serde_yaml::Value::Mapping(top_level) = front_matter else {
        return vec![(
            2,
            "WARNING",
            "YAML front matter must be a mapping [myst.topmatter]".into(),
        )];
    };
    let myst_key = serde_yaml::Value::String("myst".into());
    let Some(myst_config) = top_level.get(&myst_key) else {
        return Vec::new();
    };
    let serde_yaml::Value::Mapping(myst_config) = myst_config else {
        return vec![(
            2,
            "WARNING",
            "'myst' key must be a mapping [myst.topmatter]".into(),
        )];
    };

    let mut diagnostics = Vec::new();
    for (key, value) in myst_config {
        let Some(key) = key.as_str() else {
            diagnostics.push((
                2,
                "WARNING",
                format!("Unknown field: {key:?} [myst.topmatter]"),
            ));
            continue;
        };
        if !KNOWN_MYST_FIELDS.contains(&key) {
            diagnostics.push((
                2,
                "WARNING",
                format!("Unknown field: {key} [myst.topmatter]"),
            ));
            continue;
        }
        if key == "title_to_header" && !matches!(value, serde_yaml::Value::Bool(_)) {
            let actual_type = yaml_type_name(value);
            let rendered_value = serde_yaml::to_string(value)
                .unwrap_or_default()
                .trim()
                .to_owned();
            diagnostics.push((
                2,
                "WARNING",
                format!(
                    "'title_to_header' must be of type <class 'bool'> (got {rendered_value} that is a <class '{actual_type}'>). [myst.topmatter]"
                ),
            ));
        }
        if key == "url_schemes" {
            let valid_list = matches!(value, serde_yaml::Value::Sequence(values) if values.iter().all(|value| matches!(value, serde_yaml::Value::String(_))));
            let valid_mapping = matches!(value, serde_yaml::Value::Mapping(_));
            if !valid_list && !valid_mapping {
                let rendered_value = serde_yaml::to_string(value)
                    .unwrap_or_default()
                    .trim()
                    .to_owned();
                diagnostics.push((
                    2,
                    "WARNING",
                    format!(
                        "'url_schemes' is not a list of strings: {rendered_value} [myst.topmatter]"
                    ),
                ));
            }
        }
        if key == "html_meta" {
            if let serde_yaml::Value::Mapping(tags) = value {
                for (attribute, content) in tags {
                    let Some(attribute) = attribute.as_str() else {
                        continue;
                    };
                    if let Some(error) = html_meta_attribute_error(attribute, content) {
                        diagnostics.push((
                            3,
                            "ERROR",
                            format!("Error parsing meta tag attribute \"{attribute}\": {error}."),
                        ));
                    }
                }
            }
        }
    }
    diagnostics
}

fn html_meta_attribute_error(attribute: &str, content: &serde_yaml::Value) -> Option<String> {
    let content = match content {
        serde_yaml::Value::Null => String::new(),
        serde_yaml::Value::String(value) => value.clone(),
        value => serde_yaml::to_string(value)
            .unwrap_or_default()
            .trim()
            .to_owned(),
    };
    if content.is_empty() {
        return Some("No content".into());
    }
    for (index, part) in attribute.split_whitespace().enumerate() {
        if !part.contains('=') && index == 0 {
            continue;
        }
        if !part.contains('=') {
            return Some(format!("no '=' in {part}"));
        }
        let (name, value) = part.split_once('=').unwrap_or_default();
        if name.is_empty() || value.is_empty() {
            return Some(format!("malformed {part}"));
        }
    }
    None
}

fn yaml_type_name(value: &serde_yaml::Value) -> &'static str {
    match value {
        serde_yaml::Value::Null => "NoneType",
        serde_yaml::Value::Bool(_) => "bool",
        serde_yaml::Value::Number(number) if number.is_i64() || number.is_u64() => "int",
        serde_yaml::Value::Number(_) => "float",
        serde_yaml::Value::String(_) => "str",
        serde_yaml::Value::Sequence(_) => "list",
        serde_yaml::Value::Mapping(_) => "dict",
        serde_yaml::Value::Tagged(_) => "tagged value",
    }
}

fn duplicate_reference_definitions(source: &str) -> Vec<(u32, String)> {
    let mut seen = BTreeSet::new();
    let mut diagnostics = Vec::new();
    let mut fenced_code: Option<(u8, usize)> = None;

    for (line_number, line) in source.lines().enumerate() {
        if let Some((marker, minimum_length)) = fenced_code {
            if let Some((closing_marker, length)) = code_fence(line) {
                let suffix = line.trim_start().get(length..).unwrap_or_default();
                if closing_marker == marker && length >= minimum_length && suffix.trim().is_empty()
                {
                    fenced_code = None;
                }
            }
            continue;
        }
        if let Some(fence) = code_fence(line) {
            fenced_code = Some(fence);
            continue;
        }

        let indentation = line.len() - line.trim_start().len();
        if indentation >= 4 {
            continue;
        }
        let content = &line[indentation..];
        let Some(reference) = content.strip_prefix('[') else {
            continue;
        };
        if reference.starts_with('^') {
            continue;
        }
        let Some(label_end) = reference.find("]:") else {
            continue;
        };
        let label = reference[..label_end].trim();
        if label.is_empty() {
            continue;
        }
        let normalized = label
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        if !seen.insert(normalized) {
            diagnostics.push((
                line_number as u32 + 1,
                format!(
                    "Duplicate reference definition: {} [myst.duplicate_def]",
                    label.to_uppercase()
                ),
            ));
        }
    }
    diagnostics
}

fn code_fence(line: &str) -> Option<(u8, usize)> {
    let indentation = line.len() - line.trim_start().len();
    if indentation > 3 {
        return None;
    }
    let content = &line[indentation..];
    let marker = *content.as_bytes().first()?;
    if !matches!(marker, b'`' | b'~') {
        return None;
    }
    let length = content.bytes().take_while(|byte| *byte == marker).count();
    (length >= 3).then_some((marker, length))
}

fn unresolved_local_links(
    tree: &Doctree,
    source_path: &str,
    source_root: &Path,
) -> Vec<(u32, String)> {
    (0..tree.nodes_len())
        .filter_map(|id| {
            let node = tree.node(id);
            let NodeKind::Reference { refuri, .. } = &node.kind else {
                return None;
            };
            let target_path = refuri.split(['#', '?']).next().unwrap_or_default();
            if target_path.is_empty() || target_path.starts_with('/') || has_uri_scheme(target_path)
            {
                return None;
            }
            let parent = Path::new(source_path).parent().unwrap_or(source_root);
            if local_target_exists(&parent.join(target_path), source_root) {
                return None;
            }
            Some((node.line.unwrap_or(1), refuri.clone()))
        })
        .collect()
}

fn has_uri_scheme(value: &str) -> bool {
    let Some((scheme, _)) = value.split_once(':') else {
        return false;
    };
    !scheme.is_empty()
        && scheme.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
        })
}

fn local_target_exists(path: &Path, source_root: &Path) -> bool {
    let mut candidates = vec![path.to_path_buf()];
    if path.extension().is_none() {
        for extension in ["md", "rst", "txt"] {
            candidates.push(path.with_extension(extension));
            candidates.push(path.join(format!("index.{extension}")));
        }
    }
    candidates.into_iter().any(|candidate| {
        candidate
            .canonicalize()
            .is_ok_and(|resolved| resolved.starts_with(source_root))
    })
}

#[derive(Debug, Clone)]
struct ResolvedSubstitution {
    value: String,
    issues: Vec<String>,
}

fn apply_substitutions(
    source: &str,
    substitutions: &BTreeMap<String, String>,
) -> (String, Vec<(u32, String)>) {
    let (resolved, cycles) = resolve_substitutions(substitutions);
    let (output, issues) = expand_substitution_template(source, &resolved, &cycles);
    let diagnostics = issues
        .into_iter()
        .map(|(offset, message)| {
            let line = source[..offset]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count() as u32
                + 1;
            (line, message)
        })
        .collect();
    (output, diagnostics)
}

fn resolve_substitutions(
    substitutions: &BTreeMap<String, String>,
) -> (
    BTreeMap<String, ResolvedSubstitution>,
    BTreeMap<String, String>,
) {
    let dependencies = substitutions
        .iter()
        .map(|(name, value)| {
            let dependencies = substitution_markers(value)
                .into_iter()
                .map(|(_, _, dependency)| dependency)
                .filter(|dependency| substitutions.contains_key(dependency))
                .collect::<BTreeSet<_>>();
            (name.clone(), dependencies)
        })
        .collect::<BTreeMap<_, _>>();
    let mut dependents = substitutions
        .keys()
        .map(|name| (name.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    let mut pending = BTreeMap::new();
    for (name, names) in &dependencies {
        pending.insert(name.clone(), names.len());
        for dependency in names {
            dependents
                .get_mut(dependency)
                .expect("dependency comes from the substitution map")
                .insert(name.clone());
        }
    }

    let mut ready = pending
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(name, _)| name.clone())
        .collect::<VecDeque<_>>();
    let mut resolved = BTreeMap::new();
    let no_cycles = BTreeMap::new();
    while let Some(name) = ready.pop_front() {
        let (value, issues) =
            expand_substitution_template(&substitutions[&name], &resolved, &no_cycles);
        let issues = issues
            .into_iter()
            .map(|(_, issue)| issue)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        resolved.insert(name.clone(), ResolvedSubstitution { value, issues });

        for dependent in &dependents[&name] {
            let count = pending
                .get_mut(dependent)
                .expect("dependent comes from the substitution map");
            *count -= 1;
            if *count == 0 {
                ready.push_back(dependent.clone());
            }
        }
    }

    let unresolved = substitutions
        .keys()
        .filter(|name| !resolved.contains_key(*name))
        .cloned()
        .collect::<BTreeSet<_>>();
    let cycles = unresolved
        .iter()
        .filter_map(|name| {
            substitution_cycle_path(name, &dependencies, &unresolved)
                .map(|path| (name.clone(), path))
        })
        .collect();
    (resolved, cycles)
}

fn substitution_cycle_path(
    start: &str,
    dependencies: &BTreeMap<String, BTreeSet<String>>,
    unresolved: &BTreeSet<String>,
) -> Option<String> {
    let mut positions = BTreeMap::new();
    let mut path = Vec::new();
    let mut current = start.to_string();
    loop {
        if let Some(position) = positions.get(&current) {
            let mut cycle = path[*position..].to_vec();
            cycle.push(current);
            return Some(cycle.join(" -> "));
        }
        positions.insert(current.clone(), path.len());
        path.push(current.clone());
        current = dependencies
            .get(&current)?
            .iter()
            .find(|dependency| unresolved.contains(*dependency))?
            .clone();
    }
}

fn expand_substitution_template(
    template: &str,
    resolved: &BTreeMap<String, ResolvedSubstitution>,
    cycles: &BTreeMap<String, String>,
) -> (String, Vec<(usize, String)>) {
    let markers = substitution_markers(template);
    let mut output = String::with_capacity(template.len());
    let mut issues = Vec::new();
    let mut cursor = 0;
    for (start, end, name) in markers {
        output.push_str(&template[cursor..start]);
        if let Some(substitution) = resolved.get(&name) {
            output.push_str(&substitution.value);
            issues.extend(
                substitution
                    .issues
                    .iter()
                    .map(|issue| (start, issue.clone())),
            );
        } else if let Some(cycle) = cycles.get(&name) {
            output.push_str(&template[start..end]);
            issues.push((start, format!("substitution cycle: {cycle}")));
        } else {
            output.push_str(&template[start..end]);
            issues.push((start, format!("undefined substitution '{name}'")));
        }
        cursor = end;
    }
    output.push_str(&template[cursor..]);
    (output, issues)
}

fn substitution_markers(template: &str) -> Vec<(usize, usize, String)> {
    let mut markers = Vec::new();
    let mut cursor = 0;
    while let Some(start_offset) = template[cursor..].find("{{") {
        let start = cursor + start_offset;
        let name_start = start + 2;
        let Some(end_offset) = template[name_start..].find("}}") else {
            break;
        };
        let name_end = name_start + end_offset;
        let end = name_end + 2;
        let name = template[name_start..name_end].trim();
        if !name.is_empty() {
            markers.push((start, end, name.to_string()));
        }
        cursor = end;
    }
    markers
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

fn known_directive(name: &str) -> bool {
    matches!(
        name,
        "admonition" | "container" | "code-block" | "raw" | "math" | "include" | "eval-rst"
    ) || is_admonition(name)
}

fn directive_spec(name: &str) -> directives::DirectiveSpec {
    let mut spec = directives::DirectiveSpec {
        has_content: true,
        ..directives::DirectiveSpec::default()
    };
    if name == "admonition" {
        spec.required_arguments = 1;
        spec.optional_arguments = 0;
        spec.final_argument_whitespace = true;
    }
    spec.option_spec
        .insert("class".into(), directives::OptionKind::Classes);
    spec
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
                if source == "docs/guide.md" && title.is_empty()
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
        assert_eq!(tree.node(paragraph).line, Some(3));
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
    fn lowers_directive_options_and_generic_admonition_title() {
        let tree = parse_to_doctree(
            "```{note}\n:class: important\n\nRead this.\n```\n\n```{admonition} A heading\n:class: custom\n\nBody.\n```\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let xml = docutilsrs::to_xml(&tree);
        assert!(xml.contains("<note>"));
        assert!(xml.contains("Read this."));
        assert!(
            xml.contains("<admonition title=\"A heading\" classes=\"custom\">")
                || xml.contains("<admonition classes=\"custom\" title=\"A heading\">")
        );
        assert!(xml.contains("Body."));
        assert!(!xml.contains(":class:"));
    }

    #[test]
    fn malformed_generic_admonition_preserves_a_diagnostic_node() {
        let tree = parse_to_doctree(
            "```{admonition}\nBody.\n```\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        assert!(
            tree.node(tree.root())
                .children
                .iter()
                .any(|id| matches!(tree.node(*id).kind, NodeKind::SystemMessage { .. }))
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
            tree.node(tree.root()).line,
            restored.node(restored.root()).line
        );
        assert_eq!(
            docutilsrs::html5(&tree, &Default::default(), &Default::default()),
            docutilsrs::html5(&restored, &Default::default(), &Default::default())
        );
    }

    #[test]
    fn lowers_tables_definition_lists_and_heading_attributes() {
        let source = "# Title {#custom .lead}\n\na | b | c\n:- | :-: | -:\n1 | 2 | 3\n\nTerm\n: Definition\n";
        let tree = parse_to_doctree(source, "index.md", &DoctreeOptions::default());
        let xml = docutilsrs::to_xml(&tree);

        assert!(xml.contains("<section classes=\"lead\" ids=\"custom\" names=\"Title\">"));
        assert!(xml.contains("<table>"));
        assert!(xml.contains("<tgroup cols=\"3\">"));
        assert!(xml.contains("<thead>"));
        assert!(xml.contains("<tbody>"));
        assert!(xml.contains("<entry"));
        assert!(xml.contains("classes=\"text-left\""));
        assert!(xml.contains("classes=\"text-center\""));
        assert!(xml.contains("classes=\"text-right\""));
        assert!(xml.contains("<definition_list>"));
        assert!(xml.contains("<definition_list_item>"));
        assert!(xml.contains("<term>"));
        assert!(xml.contains("<definition>"));
        let definition = xml.find("<definition>").expect("definition node");
        assert!(xml[definition..].contains("<paragraph>"));
    }

    #[test]
    fn expands_front_matter_substitutions() {
        let tree = parse_to_doctree(
            "---\nmyst:\n  substitutions:\n    product: Rust\n---\n\n# {{ product }}\n\nWelcome to {{product}}.\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let xml = docutilsrs::to_xml(&tree);
        assert!(xml.contains("names=\"Rust\""));
        assert!(xml.contains("Welcome to Rust."));
    }

    #[test]
    fn malformed_front_matter_reports_a_source_warning() {
        let tree = parse_to_doctree(
            "---\na: {\n---\nBody.\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let xml = docutilsrs::to_xml(&tree);
        assert!(xml.contains("Malformed YAML [myst.topmatter]"));
        assert!(matches!(
            tree.node(tree.root())
                .children
                .iter()
                .find_map(|id| {
                    matches!(tree.node(*id).kind, NodeKind::SystemMessage { .. }).then_some(*id)
                })
                .map(|id| &tree.node(id).kind),
            Some(NodeKind::SystemMessage {
                line: Some(1),
                ty: "WARNING",
                ..
            })
        ));
    }

    #[test]
    fn unknown_myst_front_matter_field_reports_a_warning() {
        let tree = parse_to_doctree(
            "---\nmyst:\n  unknown: true\n---\nBody.\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let xml = docutilsrs::to_xml(&tree);
        assert!(xml.contains("Unknown field: unknown [myst.topmatter]"));
    }

    #[test]
    fn invalid_myst_front_matter_types_report_warnings() {
        let tree = parse_to_doctree(
            "---\nmyst:\n  title_to_header: 1\n  url_schemes: [1]\n  substitutions:\n    key: []\n---\nBody.\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let xml = docutilsrs::to_xml(&tree);
        assert!(xml.contains("title_to_header"));
        assert!(xml.contains("url_schemes"));
        assert!(!xml.contains("substitutions"));
    }

    #[test]
    fn resolves_transitive_substitutions_without_a_pass_limit() {
        let mut options = DoctreeOptions::default();
        for index in 0..12 {
            let value = if index == 0 {
                "complete".to_string()
            } else {
                format!("{{{{ key{:02} }}}}", index - 1)
            };
            options
                .substitutions
                .insert(format!("key{index:02}"), value);
        }

        let tree = parse_to_doctree("Use {{ key11 }}.\n", "index.md", &options);
        let xml = docutilsrs::to_xml(&tree);

        assert!(xml.contains("Use complete."));
    }

    #[test]
    fn undefined_substitution_reports_source_line() {
        let tree = parse_to_doctree(
            "---\ntitle: Example\n---\nFirst paragraph.\n\nUse {{ missing }} here.\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let diagnostics = tree
            .node(tree.root())
            .children
            .iter()
            .filter(|id| matches!(tree.node(**id).kind, NodeKind::SystemMessage { .. }))
            .copied()
            .collect::<Vec<_>>();

        assert_eq!(diagnostics.len(), 1);
        assert!(matches!(
            tree.node(diagnostics[0]).kind,
            NodeKind::SystemMessage {
                level: 2,
                line: Some(6),
                ty: "WARNING",
                ..
            }
        ));
        assert!(docutilsrs::to_xml(&tree).contains("undefined substitution 'missing'"));
    }

    #[test]
    fn cyclic_substitution_reports_dependency_path() {
        let mut options = DoctreeOptions::default();
        options
            .substitutions
            .insert("alpha".into(), "{{ beta }}".into());
        options
            .substitutions
            .insert("beta".into(), "{{ alpha }}".into());

        let tree = parse_to_doctree("Use {{ alpha }}.\n", "index.md", &options);
        let xml = docutilsrs::to_xml(&tree);

        assert!(xml.contains("substitution cycle: alpha -&gt; beta -&gt; alpha"));
    }

    #[test]
    fn unknown_role_warning_uses_the_text_event_source_line() {
        let tree = parse_to_doctree(
            "Before.\n\n{a}`b`\n",
            "included.md",
            &DoctreeOptions::default(),
        );
        let warning = (0..tree.nodes_len())
            .find(|id| matches!(tree.node(*id).kind, NodeKind::SystemMessage { .. }))
            .expect("unknown-role warning");
        assert!(matches!(
            tree.node(warning).kind,
            NodeKind::SystemMessage { line: Some(3), .. }
        ));
    }

    #[test]
    fn lowers_eval_rst_into_native_nodes() {
        let tree = parse_to_doctree(
            "```{eval-rst}\n**bold**\n```\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let xml = docutilsrs::to_xml(&tree);
        assert!(xml.contains("<strong>"));
        assert!(!xml.contains("eval-rst"));
    }

    #[test]
    fn reports_unknown_directive_with_source_line() {
        let tree = parse_to_doctree(
            "before\n\n```{unknown}\nbody\n```\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let message = tree
            .node(tree.root())
            .children
            .iter()
            .find(|id| matches!(tree.node(**id).kind, NodeKind::SystemMessage { .. }))
            .copied()
            .expect("diagnostic node");
        assert!(matches!(
            tree.node(message).kind,
            NodeKind::SystemMessage { line: Some(3), .. }
        ));
        let message_text = tree
            .node(message)
            .children
            .iter()
            .filter_map(|child| match &tree.node(*child).kind {
                NodeKind::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<String>();
        assert_eq!(
            message_text,
            "Unknown directive type: 'unknown' [myst.directive_unknown]"
        );
    }

    #[test]
    fn heading_starting_below_h1_reports_source_warning() {
        let tree = parse_to_doctree("## A heading\n", "index.md", &DoctreeOptions::default());
        let warning = (0..tree.nodes_len())
            .find(|id| matches!(tree.node(*id).kind, NodeKind::SystemMessage { .. }))
            .expect("heading-level warning");
        let message =
            tree.node(warning)
                .children
                .iter()
                .find_map(|child| match &tree.node(*child).kind {
                    NodeKind::Text(text) => Some(text.as_str()),
                    _ => None,
                });
        assert!(matches!(
            tree.node(warning).kind,
            NodeKind::SystemMessage {
                line: Some(1),
                ty: "WARNING",
                ..
            }
        ));
        assert!(
            message
                .unwrap()
                .contains("Document headings start at H2, not H1")
        );
    }

    #[test]
    fn nonconsecutive_heading_reports_level_transition() {
        let tree = parse_to_doctree(
            "# First\n\n### Skipped level\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let warning = (0..tree.nodes_len())
            .find(|id| {
                matches!(
                    &tree.node(*id).kind,
                    NodeKind::SystemMessage { line: Some(3), .. }
                )
            })
            .expect("non-consecutive heading warning");
        let message =
            tree.node(warning)
                .children
                .iter()
                .find_map(|child| match &tree.node(*child).kind {
                    NodeKind::Text(text) => Some(text.as_str()),
                    _ => None,
                });
        assert!(message.unwrap().contains("H1 to H3"));
    }

    #[test]
    fn duplicate_reference_definition_reports_the_second_line() {
        let tree = parse_to_doctree(
            "[a]: one\n[a]: two\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let warning = (0..tree.nodes_len())
            .find(|id| matches!(tree.node(*id).kind, NodeKind::SystemMessage { .. }))
            .expect("duplicate-reference warning");
        let message =
            tree.node(warning)
                .children
                .iter()
                .find_map(|child| match &tree.node(*child).kind {
                    NodeKind::Text(text) => Some(text.as_str()),
                    _ => None,
                });
        assert!(matches!(
            tree.node(warning).kind,
            NodeKind::SystemMessage { line: Some(2), .. }
        ));
        assert_eq!(
            message,
            Some("Duplicate reference definition: A [myst.duplicate_def]")
        );
    }

    #[test]
    fn duplicate_reference_definition_syntax_in_fenced_code_is_ignored() {
        let tree = parse_to_doctree(
            "```md\n[a]: one\n[a]: two\n```\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        assert!(
            !(0..tree.nodes_len())
                .any(|id| matches!(tree.node(id).kind, NodeKind::SystemMessage { .. }))
        );
    }

    #[test]
    fn lowers_table_spans_into_docutils_entry_spans() {
        let tree = parse_to_doctree(
            "| a {colspan=2 rowspan=2} | b |\n|---|---|\n| c | d |\n",
            "index.md",
            &DoctreeOptions::default(),
        );
        let xml = docutilsrs::to_xml(&tree);
        assert!(xml.contains("morecols=\"1\""));
        assert!(xml.contains("morerows=\"1\""));
    }
}
