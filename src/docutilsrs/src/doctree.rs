//! Minimal owned doctree.
//!
//! Phase 1 + phase 2 slice. Extended incrementally; see `docs/compat.md`.
//!
//! Layout: an arena of [`Node`]s indexed by [`NodeId`]. Parent and child
//! pointers are `NodeId`s, never references, so the tree is cheap to mutate
//! and trivial to traverse without lifetime gymnastics.

use std::collections::BTreeMap;
use std::fmt;

pub type NodeId = usize;

/// A resolved link covering a byte range in an object signature.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SignatureReference {
    pub start: usize,
    pub end: usize,
    pub uri: String,
    pub classes: String,
}

/// A non-executable representation of Python node attributes.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum NodeAttributeValue {
    String(String),
    Bool(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    List(Vec<NodeAttributeValue>),
    Map(BTreeMap<String, NodeAttributeValue>),
    None,
    Opaque { type_name: String, repr: String },
}

impl<'de> serde::Deserialize<'de> for NodeAttributeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        fn convert(value: serde_json::Value) -> Result<NodeAttributeValue, String> {
            match value {
                serde_json::Value::Null => Ok(NodeAttributeValue::None),
                serde_json::Value::Bool(value) => Ok(NodeAttributeValue::Bool(value)),
                serde_json::Value::String(value) => Ok(NodeAttributeValue::String(value)),
                serde_json::Value::Number(value) => {
                    if let Some(value) = value.as_i64() {
                        Ok(NodeAttributeValue::Signed(value))
                    } else if let Some(value) = value.as_u64() {
                        Ok(NodeAttributeValue::Unsigned(value))
                    } else if let Some(value) = value.as_f64()
                        && value.is_finite()
                    {
                        Ok(NodeAttributeValue::Float(value))
                    } else {
                        Err("non-finite numeric attribute".to_string())
                    }
                }
                serde_json::Value::Array(values) => values
                    .into_iter()
                    .map(convert)
                    .collect::<Result<Vec<_>, _>>()
                    .map(NodeAttributeValue::List),
                serde_json::Value::Object(mut values) => {
                    if values.len() == 1 {
                        if let Some(value) = values.remove("String") {
                            return Ok(NodeAttributeValue::String(
                                value
                                    .as_str()
                                    .ok_or_else(|| "String attribute is not a string".to_string())?
                                    .to_string(),
                            ));
                        }
                        if let Some(value) = values.remove("Bool") {
                            return Ok(NodeAttributeValue::Bool(
                                value
                                    .as_bool()
                                    .ok_or_else(|| "Bool attribute is not a bool".to_string())?,
                            ));
                        }
                        if let Some(value) = values.remove("Signed") {
                            return Ok(NodeAttributeValue::Signed(value.as_i64().ok_or_else(
                                || "Signed attribute is not an integer".to_string(),
                            )?));
                        }
                        if let Some(value) = values.remove("Unsigned") {
                            return Ok(NodeAttributeValue::Unsigned(value.as_u64().ok_or_else(
                                || "Unsigned attribute is not an integer".to_string(),
                            )?));
                        }
                        if let Some(value) = values.remove("Float") {
                            let value = value
                                .as_f64()
                                .ok_or_else(|| "Float attribute is not a number".to_string())?;
                            return if value.is_finite() {
                                Ok(NodeAttributeValue::Float(value))
                            } else {
                                Err("non-finite numeric attribute".to_string())
                            };
                        }
                        if let Some(value) = values.remove("List") {
                            return convert(serde_json::Value::Array(
                                value
                                    .as_array()
                                    .ok_or_else(|| "List attribute is not an array".to_string())?
                                    .clone(),
                            ));
                        }
                        if let Some(value) = values.remove("Map") {
                            return convert(value);
                        }
                        if values.contains_key("None") {
                            return Ok(NodeAttributeValue::None);
                        }
                        if let Some(value) = values.remove("Opaque") {
                            let object = value
                                .as_object()
                                .ok_or_else(|| "Opaque attribute is not an object".to_string())?;
                            return Ok(NodeAttributeValue::Opaque {
                                type_name: object
                                    .get("type_name")
                                    .and_then(serde_json::Value::as_str)
                                    .unwrap_or_default()
                                    .to_string(),
                                repr: object
                                    .get("repr")
                                    .and_then(serde_json::Value::as_str)
                                    .unwrap_or_default()
                                    .to_string(),
                            });
                        }
                    }
                    values
                        .into_iter()
                        .map(|(key, value)| convert(value).map(|value| (key, value)))
                        .collect::<Result<BTreeMap<_, _>, _>>()
                        .map(NodeAttributeValue::Map)
                }
            }
        }
        convert(value).map_err(serde::de::Error::custom)
    }
}

impl Eq for NodeAttributeValue {}

impl fmt::Display for NodeAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => f.write_str(value),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Signed(value) => write!(f, "{value}"),
            Self::Unsigned(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::List(values) => write!(f, "{values:?}"),
            Self::Map(values) => write!(f, "{values:?}"),
            Self::None => f.write_str("None"),
            Self::Opaque { repr, .. } => f.write_str(repr),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    /// `<abbreviation explanation="...">` element produced by `:abbr:` / `:abbreviation:`.
    Abbreviation { explanation: String },
    /// Admonition with a fixed kind (`note`, `warning`, `tip`, etc.).
    Admonition { kind: &'static str },
    /// Attribution line within a block_quote (`-- Author`).
    Attribution,
    /// A recognized bibliographic field (e.g. `<author>`, `<date>`). `tag`
    /// is the docutils element name (lower-case).
    Bibliographic { tag: &'static str },
    /// Extended quotation set apart from the surrounding text.
    BlockQuote,
    /// Unordered list; `bullet` records the marker used in the source.
    BulletList { bullet: char },
    /// Caption text associated with a figure.
    Caption,
    /// `<citation>` element. Mirrors `Footnote` but for citations (no `auto`).
    Citation {
        ids: String,
        names: String,
        backrefs: String,
    },
    /// Inline reference to a citation entry.
    /// `<citation_reference>` element.
    CitationReference { ids: String, refid: String },
    /// Qualifier attached to a term in a definition list.
    Classifier,
    /// Specification for a column in a table group.
    Colspec { colwidth: u32 },
    /// Author note retained in the tree but hidden from output.
    Comment,
    /// `.. container:: classes` — generic named container `<div>`.
    Container { classes: String },
    /// Definition body associated with a term in a definition list.
    Definition,
    /// List of terms and their definitions, as in a glossary or dictionary.
    DefinitionList,
    /// Entry containing a term, optional classifiers, and its definition.
    DefinitionListItem,
    /// Bibliographic info container (field list at the top of a document
    /// containing recognized bibliographic fields).
    Docinfo,
    /// Root element for a parsed document.
    Document {
        source: String,
        /// Promoted title (if first block is a lone section). Empty otherwise.
        ids: String,
        names: String,
        title: String,
    },
    /// Inline text marked for emphasis.
    Emphasis,
    /// Table cell. `morecols` is the extra number of columns this entry
    /// spans (0 for a regular single-column cell); colspan support is
    /// limited to grid tables. `morerows` (rowspan) is currently always
    /// 0 — see `docs/compat.md`.
    Entry {
        morecols: u32,
        morerows: u32,
        /// Space-separated table-cell classes such as `text-center`.
        classes: String,
    },
    /// Ordered list of items. `enumtype` is one of `arabic`, `loweralpha`,
    /// `upperalpha`, `lowerroman`, `upperroman`; `prefix`/`suffix` bracket
    /// each enumerator (e.g. `(` / `)` or `` / `.`).
    EnumeratedList {
        enumtype: &'static str,
        prefix: String,
        suffix: String,
        start: Option<u32>,
    },
    /// `.. epigraph::` — a blockquote rendered with an `epigraph` class,
    /// plus any extra classes propagated from a preceding `.. class::`/
    /// `.. cssclass::` directive.
    Epigraph { classes: String },
    /// An extension-defined node registered via upstream `Sphinx.add_node`
    /// (see `docs/adr/0006-extension-nodes.md`). `class_name` is the
    /// registering Python class's `__name__`, used to key the per-
    /// `(class_name, builder_format)` visit/depart callable registry in
    /// [`crate::plugins`]. `attrs` is a deterministic typed representation of
    /// the node's real `docutils.nodes.Element` attribute dict. Unsupported
    /// Python values are retained as informational `Opaque` values and are
    /// never executed.
    /// Every writer falls back to rendering children only (no wrapper
    /// markup) when no visit/depart pair is registered for its own
    /// builder format, mirroring upstream's `unknown_visit` fallback.
    Extension {
        class_name: String,
        attrs: BTreeMap<String, NodeAttributeValue>,
    },
    /// One named field containing a name and a body.
    Field,
    /// Content value of a field.
    FieldBody,
    /// Sequence of named fields, typically rendered as label/data pairs.
    FieldList,
    /// Name or label of a field.
    FieldName,
    /// Formal figure grouping an image, caption, and optional legend.
    Figure { ids: String, classes: String },
    /// `<footnote>` element. `ids` is the auto-assigned identifier,
    /// `names` is the visible label (digit or `*`/`#`), `backrefs` is the
    /// space-separated list of inline-reference ids that resolve here, and
    /// `auto` carries an autonumber/autosymbol marker when applicable
    /// (`"1"` for autonumber, `"*"` for autosymbol).
    Footnote {
        ids: String,
        names: String,
        backrefs: String,
        auto: Option<&'static str>,
    },
    /// `<footnote_reference>` element. `ids` is the inline-reference id,
    /// `refid` points to the matching footnote, `auto` mirrors the
    /// footnote's autonumber/autosymbol marker.
    FootnoteReference {
        ids: String,
        refid: String,
        auto: Option<&'static str>,
    },
    /// `.. admonition:: Title` — generic admonition with an arbitrary
    /// title (as opposed to the fixed-kind `Admonition` variant).
    /// `classes` holds the optional `:class:` option value.
    GenericAdmonition { title: String, classes: String },
    /// Reference to an image resource; may occur inline or as a block.
    Image {
        uri: String,
        alt: Option<String>,
        width: Option<String>,
        height: Option<String>,
        classes: String,
    },
    /// Generic inline element used by roles and directives.
    /// `<inline classes="...">` element used by some roles/directives.
    Inline { classes: String },
    /// Inline keyboard input text produced by `:kbd:`.
    /// `<keyboard>` element produced by `:kbd:`.
    Keyboard,
    /// `<label>` element used inside footnote/citation.
    Label,
    /// Remaining content of a figure body, after the caption.
    Legend,
    /// One item in a bullet or enumerated list.
    ListItem,
    /// Inline literal text, typically rendered in a monospace font.
    Literal,
    /// Pre-formatted block. `classes` is space-separated (e.g. "code python").
    LiteralBlock {
        ids: Option<String>,
        classes: String,
        caption: Option<String>,
        source: Option<String>,
    },
    /// `<math>` inline element produced by the `:math:` role. `latex` is
    /// the raw LaTeX source.
    Math { latex: String },
    /// `<math_block>` element produced by the `.. math::` directive.
    /// `latex` is the raw LaTeX source.
    MathBlock { latex: String },
    /// Sphinx object description containing signatures and shared content.
    /// A generic Sphinx domain "object description" block, e.g.
    /// `.. rst:directive::`/`.. rst:role::` (Tier **H3c**), reusable for
    /// other domains' description directives later. Renders as
    /// `<dl class="{classes}"><dt id="{ids}">{sig_text}</dt><dd>
    /// {children}</dd></dl>`; `children` (via the arena) are the `<dd>`
    /// body content blocks.
    ObjectDescription {
        /// Space-separated `<dl>`/`<dt>` classes, e.g. "rst directive".
        classes: String,
        /// The `<dt id="...">` anchor id (empty = no id).
        ids: String,
        /// The rendered signature text, e.g. ".. toctree::" or ":ref:".
        sig_text: String,
        /// Optional resolved references in `sig_text`, rendered by HTML writers.
        signature_references: Vec<SignatureReference>,
        /// Optional viewcode source-page link rendered beside `sig_text`.
        source_link: Option<String>,
    },
    /// Block of prose containing inline content.
    Paragraph,
    /// Sphinx cross-reference that cannot be resolved until project documents
    /// are known; it is resolved before output is written.
    /// Cross-reference node to be resolved during post-read reference resolution.
    PendingXref {
        reftype: String,
        reftarget: String,
        refdoc: String,
        refdomain: String,
        refexplicit: bool,
        warn_missing: bool,
    },
    /// `<problematic>` element. Produced as a placeholder for unresolved
    /// references; `refid` points to the matching `<system_message>`.
    Problematic { ids: String, refid: String },
    /// Raw passthrough (`.. raw:: format`).
    Raw { format: String },
    /// Hyperlink reference. `refuri` is empty until resolved.
    Reference {
        name: String,
        refuri: String,
        anonymous: bool,
        /// Docutils-style `classes` attribute (space-separated), e.g.
        /// `"reference internal"` / `"reference external"`. Empty for
        /// plain docutils hyperlink references (whose class is left to
        /// the writer); populated by Sphinx xref-role resolution
        /// (`sphinxdocrs::environment::BuildEnvironment::resolve_xref_nodes`).
        classes: String,
    },
    /// Row of table cells.
    Row,
    /// Standalone heading that does not create a section.
    /// `<rubric>` element produced by `.. rubric::`.
    Rubric,
    /// Hierarchical document section with a title and optional subsections.
    Section {
        ids: String,
        names: String,
        classes: String,
    },
    /// Inline text marked as strongly emphasized.
    Strong,
    /// `<subscript>` element produced by `:sub:` / `:subscript:`.
    Subscript,
    /// Definition of reusable substitution content.
    SubstitutionDefinition { names: String },
    /// Inline use of a named substitution.
    SubstitutionReference { refname: String },
    /// Subtitle following a title in a document or section.
    Subtitle { ids: String, names: String },
    /// `<superscript>` element produced by `:sup:` / `:superscript:`.
    Superscript,
    /// `<system_message>` element produced by the parser for error and
    /// warning conditions (e.g. unresolved references).
    SystemMessage {
        level: u32,
        line: Option<u32>,
        ty: &'static str,
        ids: String,
        backrefs: String,
    },
    /// Tabular data organized into groups of columns and rows.
    Table,
    /// Explicit hyperlink target. `ids` is the normalized identifier,
    /// `names` is the human-readable name (space-separated).
    Target {
        ids: String,
        names: String,
        refuri: String,
        anonymous: bool,
    },
    /// Body rows within a table group.
    Tbody,
    /// Term described by a definition-list item.
    Term,
    /// Plain text leaf node in the document tree.
    Text(String),
    /// Table group defining columns, an optional head, and a body.
    Tgroup { cols: u32 },
    /// Header rows within a table group.
    Thead,
    /// Title of a document, section, or titled body element.
    Title,
    /// `<title_reference>` element (role `:title:` / `:t:`).
    TitleReference,
    /// `.. toctree::` — placeholder marker carrying the directive's raw
    /// options/entries, resolved into a real `<div class="toctree-wrapper
    /// compound">` subtree (caption + nested bullet list of
    /// [`NodeKind::Reference`]s) by
    /// `sphinxdocrs::environment::BuildEnvironment::resolve_toctree_nodes`,
    /// which has the multi-document project data (`env.titles`,
    /// `env.toctree_includes`) docutilsrs itself has no notion of.
    Toctree {
        caption: Option<String>,
        maxdepth: i32,
        hidden: bool,
        entries: Vec<String>,
    },
    /// Semantic break between adjacent document elements.
    Transition,
    /// Sphinx version-change entry produced by version-related directives.
    /// `<versionmodified>` element produced by `.. versionadded::`, `.. versionchanged::`, etc.
    VersionModified { kind: &'static str, version: String },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    /// 1-based source line when the parser can provide one.
    pub line: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Doctree {
    nodes: Vec<Node>,
    root: NodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VisitEvent {
    Enter { id: NodeId, depth: usize },
    Exit { id: NodeId, depth: usize },
}

#[derive(Debug, Clone, Copy)]
struct VisitFrame {
    id: NodeId,
    depth: usize,
    next_child: usize,
}

pub(crate) struct DepthFirst<'a> {
    tree: &'a Doctree,
    stack: Vec<VisitFrame>,
    enter_pending: bool,
}

impl<'a> Iterator for DepthFirst<'a> {
    type Item = VisitEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.enter_pending {
            self.enter_pending = false;
            let frame = self.stack.last()?;
            return Some(VisitEvent::Enter {
                id: frame.id,
                depth: frame.depth,
            });
        }
        if self.stack.is_empty() {
            return None;
        }
        let parent_index = self.stack.len() - 1;
        let (parent_id, parent_depth, child_index) = {
            let frame = &mut self.stack[parent_index];
            let child_index = frame.next_child;
            frame.next_child += 1;
            (frame.id, frame.depth, child_index)
        };
        if let Some(&child) = self.tree.node(parent_id).children.get(child_index) {
            self.stack.push(VisitFrame {
                id: child,
                depth: parent_depth + 1,
                next_child: 0,
            });
            self.enter_pending = true;
            return self.next();
        }
        let frame = self.stack.pop()?;
        Some(VisitEvent::Exit {
            id: frame.id,
            depth: frame.depth,
        })
    }
}

impl Doctree {
    /// Create an empty document with the given `source` attribute.
    pub fn new_document(source: impl Into<String>) -> Self {
        let mut tree = Doctree {
            nodes: Vec::new(),
            root: 0,
        };
        tree.root = tree.push(
            NodeKind::Document {
                source: source.into(),
                ids: String::new(),
                names: String::new(),
                title: String::new(),
            },
            None,
        );
        tree
    }

    /// Replace the root document's source attribute.
    pub fn set_source(&mut self, source: impl Into<String>) {
        if let NodeKind::Document {
            source: document_source,
            ..
        } = &mut self.node_mut(self.root).kind
        {
            *document_source = source.into();
        }
    }

    /// Identifier of the root `<document>` node.
    pub fn root(&self) -> NodeId {
        self.root
    }

    pub(crate) fn depth_first(&self, root: NodeId) -> DepthFirst<'_> {
        DepthFirst {
            tree: self,
            stack: vec![VisitFrame {
                id: root,
                depth: 0,
                next_child: 0,
            }],
            enter_pending: true,
        }
    }

    /// Borrow a node by id. Panics on unknown id (ids are arena-local; never
    /// constructed by callers).
    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }

    /// Mutably borrow a node by id.
    pub fn node_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id]
    }

    /// Set the 1-based source line for a node.
    pub fn set_line(&mut self, id: NodeId, line: u32) {
        self.nodes[id].line = Some(line);
    }

    /// Total number of nodes in the arena (max valid id + 1).
    pub fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    /// Append a new node under `parent` and return its id.
    pub fn append(&mut self, parent: NodeId, kind: NodeKind) -> NodeId {
        let id = self.push(kind, Some(parent));
        self.nodes[parent].children.push(id);
        id
    }

    /// Replace the kind of an existing node.
    pub fn set_kind(&mut self, id: NodeId, kind: NodeKind) {
        self.nodes[id].kind = kind;
    }

    /// Detach `id` from its parent (does not remove the node from the arena).
    pub fn detach(&mut self, id: NodeId) {
        if let Some(parent) = self.nodes[id].parent {
            self.nodes[parent].children.retain(|&c| c != id);
            self.nodes[id].parent = None;
        }
    }

    fn push(&mut self, kind: NodeKind, parent: Option<NodeId>) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(Node {
            kind,
            parent,
            children: Vec::new(),
            line: None,
        });
        id
    }

    /// Serialize this doctree to a self-contained JSON byte buffer.
    ///
    /// Used by `sphinxdocrs`'s doctree store to persist the parsed result
    /// of the read phase (`doctreedir/<docname>.doctree`) so the write
    /// phase can render without re-parsing RST.
    pub fn to_bytes(&self) -> Vec<u8> {
        let data: DoctreeData = self.into();
        // Construction from a valid `Doctree` always serializes; the
        // `unwrap` only fails on non-serializable types (floats/maps with
        // non-string keys), none of which appear in `NodeKindData`.
        serde_json::to_vec(&data).expect("Doctree -> JSON serialization cannot fail")
    }

    /// Deserialize a doctree previously produced by [`Doctree::to_bytes`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        let data: DoctreeData = serde_json::from_slice(bytes)?;
        Ok(data.into())
    }
}

// ── serde codec ───────────────────────────────────────────────────────────────
//
// `NodeKind` carries a handful of `&'static str` / `Option<&'static str>`
// fields (interned constants such as `EnumeratedList::enumtype` or
// `Admonition::kind`). `serde::Deserialize` cannot produce a `&'static str`
// from an owned buffer, so we mirror `NodeKind` with an all-owned variant
// (`NodeKindData`) that derives `Serialize`/`Deserialize` normally, and
// convert the handful of `&'static str` fields back via [`intern`], which
// leaks a small owned `String` into a `&'static str`. This only runs once
// per doctree load (deserialize), and the leaked strings are a handful of
// short tags (e.g. `"note"`, `"arabic"`) — negligible in practice for a
// process that loads a bounded number of doctrees per build.

fn intern(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DoctreeData {
    nodes: Vec<NodeData>,
    root: NodeId,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct NodeData {
    kind: NodeKindData,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    line: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
enum NodeKindData {
    Abbreviation {
        explanation: String,
    },
    Admonition {
        kind: String,
    },
    Attribution,
    Bibliographic {
        tag: String,
    },
    BlockQuote,
    BulletList {
        bullet: char,
    },
    Caption,
    Citation {
        backrefs: String,
        ids: String,
        names: String,
    },
    CitationReference {
        ids: String,
        refid: String,
    },
    Classifier,
    Colspec {
        colwidth: u32,
    },
    Comment,
    Container {
        classes: String,
    },
    Definition,
    DefinitionList,
    DefinitionListItem,
    Docinfo,
    Document {
        ids: String,
        names: String,
        source: String,
        title: String,
    },
    Emphasis,
    Entry {
        #[serde(default)]
        classes: String,
        morecols: u32,
        morerows: u32,
    },
    EnumeratedList {
        enumtype: String,
        prefix: String,
        start: Option<u32>,
        suffix: String,
    },
    Epigraph {
        classes: String,
    },
    Extension {
        attrs: BTreeMap<String, NodeAttributeValue>,
        class_name: String,
    },
    Field,
    FieldBody,
    FieldList,
    FieldName,
    Figure {
        #[serde(default)]
        classes: String,
        #[serde(default)]
        ids: String,
    },
    Footnote {
        auto: Option<String>,
        backrefs: String,
        ids: String,
        names: String,
    },
    FootnoteReference {
        auto: Option<String>,
        ids: String,
        refid: String,
    },
    GenericAdmonition {
        classes: String,
        title: String,
    },
    Image {
        alt: Option<String>,
        #[serde(default)]
        classes: String,
        height: Option<String>,
        uri: String,
        width: Option<String>,
    },
    Inline {
        classes: String,
    },
    Keyboard,
    Label,
    Legend,
    ListItem,
    Literal,
    LiteralBlock {
        caption: Option<String>,
        classes: String,
        ids: Option<String>,
        source: Option<String>,
    },
    Math {
        latex: String,
    },
    MathBlock {
        latex: String,
    },
    ObjectDescription {
        classes: String,
        ids: String,
        sig_text: String,
        #[serde(default)]
        signature_references: Vec<SignatureReference>,
        source_link: Option<String>,
    },
    Paragraph,
    PendingXref {
        refdoc: String,
        refdomain: String,
        refexplicit: bool,
        reftarget: String,
        reftype: String,
        warn_missing: bool,
    },
    Problematic {
        ids: String,
        refid: String,
    },
    Rubric,
    Raw {
        format: String,
    },
    Reference {
        anonymous: bool,
        #[serde(default)]
        classes: String,
        name: String,
        refuri: String,
    },
    Row,
    Section {
        classes: String,
        ids: String,
        names: String,
    },
    Strong,
    Subtitle {
        ids: String,
        names: String,
    },
    Table,
    Tbody,
    Tgroup {
        cols: u32,
    },
    Thead,
    Target {
        anonymous: bool,
        ids: String,
        names: String,
        refuri: String,
    },
    Term,
    Text(String),
    Title,
    TitleReference,
    Toctree {
        caption: Option<String>,
        entries: Vec<String>,
        hidden: bool,
        maxdepth: i32,
    },
    Transition,
    Subscript,
    SubstitutionDefinition {
        names: String,
    },
    SubstitutionReference {
        refname: String,
    },
    Superscript,
    SystemMessage {
        backrefs: String,
        ids: String,
        level: u32,
        line: Option<u32>,
        ty: String,
    },
    VersionModified {
        kind: String,
        version: String,
    },
}

impl From<&Doctree> for DoctreeData {
    fn from(tree: &Doctree) -> Self {
        DoctreeData {
            nodes: tree.nodes.iter().map(NodeData::from).collect(),
            root: tree.root,
        }
    }
}

impl From<DoctreeData> for Doctree {
    fn from(data: DoctreeData) -> Self {
        Doctree {
            nodes: data.nodes.into_iter().map(Node::from).collect(),
            root: data.root,
        }
    }
}

impl From<&Node> for NodeData {
    fn from(node: &Node) -> Self {
        NodeData {
            kind: NodeKindData::from(&node.kind),
            parent: node.parent,
            children: node.children.clone(),
            line: node.line,
        }
    }
}

impl From<NodeData> for Node {
    fn from(data: NodeData) -> Self {
        Node {
            kind: data.kind.into(),
            parent: data.parent,
            children: data.children,
            line: data.line,
        }
    }
}

impl From<&NodeKind> for NodeKindData {
    fn from(kind: &NodeKind) -> Self {
        match kind.clone() {
            NodeKind::Abbreviation { explanation } => NodeKindData::Abbreviation { explanation },
            NodeKind::Admonition { kind } => NodeKindData::Admonition {
                kind: kind.to_string(),
            },
            NodeKind::Attribution => NodeKindData::Attribution,
            NodeKind::Bibliographic { tag } => NodeKindData::Bibliographic {
                tag: tag.to_string(),
            },
            NodeKind::BlockQuote => NodeKindData::BlockQuote,
            NodeKind::BulletList { bullet } => NodeKindData::BulletList { bullet },
            NodeKind::Caption => NodeKindData::Caption,
            NodeKind::Citation {
                backrefs,
                ids,
                names,
            } => NodeKindData::Citation {
                backrefs,
                ids,
                names,
            },
            NodeKind::CitationReference { ids, refid } => {
                NodeKindData::CitationReference { ids, refid }
            }
            NodeKind::Classifier => NodeKindData::Classifier,
            NodeKind::Colspec { colwidth } => NodeKindData::Colspec { colwidth },
            NodeKind::Comment => NodeKindData::Comment,
            NodeKind::Container { classes } => NodeKindData::Container { classes },
            NodeKind::Definition => NodeKindData::Definition,
            NodeKind::DefinitionList => NodeKindData::DefinitionList,
            NodeKind::DefinitionListItem => NodeKindData::DefinitionListItem,
            NodeKind::Docinfo => NodeKindData::Docinfo,
            NodeKind::Document {
                ids,
                names,
                source,
                title,
            } => NodeKindData::Document {
                ids,
                names,
                source,
                title,
            },
            NodeKind::Emphasis => NodeKindData::Emphasis,
            NodeKind::Entry {
                classes,
                morecols,
                morerows,
            } => NodeKindData::Entry {
                classes,
                morecols,
                morerows,
            },
            NodeKind::EnumeratedList {
                enumtype,
                prefix,
                start,
                suffix,
            } => NodeKindData::EnumeratedList {
                enumtype: enumtype.to_string(),
                prefix,
                start,
                suffix,
            },
            NodeKind::Epigraph { classes } => NodeKindData::Epigraph { classes },
            NodeKind::Extension { class_name, attrs } => {
                NodeKindData::Extension { class_name, attrs }
            }
            NodeKind::Field => NodeKindData::Field,
            NodeKind::FieldBody => NodeKindData::FieldBody,
            NodeKind::FieldList => NodeKindData::FieldList,
            NodeKind::FieldName => NodeKindData::FieldName,
            NodeKind::Figure { ids, classes } => NodeKindData::Figure { ids, classes },
            NodeKind::Footnote {
                auto,
                backrefs,
                ids,
                names,
            } => NodeKindData::Footnote {
                auto: auto.map(str::to_string),
                backrefs,
                ids,
                names,
            },
            NodeKind::FootnoteReference { ids, refid, auto } => NodeKindData::FootnoteReference {
                auto: auto.map(str::to_string),
                ids,
                refid,
            },
            NodeKind::GenericAdmonition { title, classes } => {
                NodeKindData::GenericAdmonition { title, classes }
            }
            NodeKind::Image {
                alt,
                classes,
                height,
                uri,
                width,
            } => NodeKindData::Image {
                alt,
                classes,
                height,
                uri,
                width,
            },
            NodeKind::Inline { classes } => NodeKindData::Inline { classes },
            NodeKind::Keyboard => NodeKindData::Keyboard,
            NodeKind::Label => NodeKindData::Label,
            NodeKind::Legend => NodeKindData::Legend,
            NodeKind::ListItem => NodeKindData::ListItem,
            NodeKind::Literal => NodeKindData::Literal,
            NodeKind::LiteralBlock {
                caption,
                classes,
                ids,
                source,
            } => NodeKindData::LiteralBlock {
                caption,
                classes,
                ids,
                source,
            },
            NodeKind::Math { latex } => NodeKindData::Math { latex },
            NodeKind::MathBlock { latex } => NodeKindData::MathBlock { latex },
            NodeKind::ObjectDescription {
                classes,
                ids,
                sig_text,
                signature_references,
                source_link,
            } => NodeKindData::ObjectDescription {
                classes,
                ids,
                sig_text,
                signature_references,
                source_link,
            },
            NodeKind::Paragraph => NodeKindData::Paragraph,
            NodeKind::PendingXref {
                refdoc,
                refdomain,
                refexplicit,
                reftarget,
                reftype,
                warn_missing,
            } => NodeKindData::PendingXref {
                refdoc,
                refdomain,
                refexplicit,
                reftarget,
                reftype,
                warn_missing,
            },
            NodeKind::Problematic { ids, refid } => NodeKindData::Problematic { ids, refid },
            NodeKind::Raw { format } => NodeKindData::Raw { format },
            NodeKind::Row => NodeKindData::Row,
            NodeKind::Reference {
                anonymous,
                classes,
                name,
                refuri,
            } => NodeKindData::Reference {
                anonymous,
                classes,
                name,
                refuri,
            },
            NodeKind::Rubric => NodeKindData::Rubric,
            NodeKind::Section {
                classes,
                ids,
                names,
            } => NodeKindData::Section {
                classes,
                ids,
                names,
            },
            NodeKind::Strong => NodeKindData::Strong,
            NodeKind::Subscript => NodeKindData::Subscript,
            NodeKind::Subtitle { ids, names } => NodeKindData::Subtitle { ids, names },
            NodeKind::Superscript => NodeKindData::Superscript,
            NodeKind::Table => NodeKindData::Table,
            NodeKind::Tbody => NodeKindData::Tbody,
            NodeKind::Tgroup { cols } => NodeKindData::Tgroup { cols },
            NodeKind::Thead => NodeKindData::Thead,
            NodeKind::Target {
                anonymous,
                ids,
                names,
                refuri,
            } => NodeKindData::Target {
                anonymous,
                ids,
                names,
                refuri,
            },
            NodeKind::Term => NodeKindData::Term,
            NodeKind::Text(s) => NodeKindData::Text(s),
            NodeKind::Title => NodeKindData::Title,
            NodeKind::TitleReference => NodeKindData::TitleReference,
            NodeKind::Toctree {
                caption,
                entries,
                hidden,
                maxdepth,
            } => NodeKindData::Toctree {
                caption,
                entries,
                hidden,
                maxdepth,
            },
            NodeKind::Transition => NodeKindData::Transition,
            NodeKind::SubstitutionDefinition { names } => {
                NodeKindData::SubstitutionDefinition { names }
            }
            NodeKind::SubstitutionReference { refname } => {
                NodeKindData::SubstitutionReference { refname }
            }
            NodeKind::SystemMessage {
                backrefs,
                ids,
                level,
                line,
                ty,
            } => NodeKindData::SystemMessage {
                backrefs,
                ids,
                level,
                line,
                ty: ty.to_string(),
            },
            NodeKind::VersionModified { kind, version } => NodeKindData::VersionModified {
                kind: kind.to_string(),
                version,
            },
        }
    }
}

impl From<NodeKindData> for NodeKind {
    fn from(data: NodeKindData) -> Self {
        match data {
            NodeKindData::Abbreviation { explanation } => NodeKind::Abbreviation { explanation },
            NodeKindData::Admonition { kind } => NodeKind::Admonition { kind: intern(kind) },
            NodeKindData::Attribution => NodeKind::Attribution,
            NodeKindData::Bibliographic { tag } => NodeKind::Bibliographic { tag: intern(tag) },
            NodeKindData::BlockQuote => NodeKind::BlockQuote,
            NodeKindData::BulletList { bullet } => NodeKind::BulletList { bullet },
            NodeKindData::Caption => NodeKind::Caption,
            NodeKindData::Citation {
                backrefs,
                ids,
                names,
            } => NodeKind::Citation {
                backrefs,
                ids,
                names,
            },
            NodeKindData::CitationReference { ids, refid } => {
                NodeKind::CitationReference { ids, refid }
            }
            NodeKindData::Classifier => NodeKind::Classifier,
            NodeKindData::Colspec { colwidth } => NodeKind::Colspec { colwidth },
            NodeKindData::Comment => NodeKind::Comment,
            NodeKindData::Container { classes } => NodeKind::Container { classes },
            NodeKindData::Definition => NodeKind::Definition,
            NodeKindData::DefinitionList => NodeKind::DefinitionList,
            NodeKindData::DefinitionListItem => NodeKind::DefinitionListItem,
            NodeKindData::Docinfo => NodeKind::Docinfo,
            NodeKindData::Document {
                ids,
                names,
                source,
                title,
            } => NodeKind::Document {
                ids,
                names,
                source,
                title,
            },
            NodeKindData::Emphasis => NodeKind::Emphasis,
            NodeKindData::Entry {
                classes,
                morecols,
                morerows,
            } => NodeKind::Entry {
                classes,
                morecols,
                morerows,
            },
            NodeKindData::EnumeratedList {
                enumtype,
                prefix,
                start,
                suffix,
            } => NodeKind::EnumeratedList {
                enumtype: intern(enumtype),
                prefix,
                start,
                suffix,
            },
            NodeKindData::Figure { ids, classes } => NodeKind::Figure { ids, classes },
            NodeKindData::Footnote {
                auto,
                backrefs,
                ids,
                names,
            } => NodeKind::Footnote {
                auto: auto.map(intern),
                backrefs,
                ids,
                names,
            },
            NodeKindData::FootnoteReference { ids, refid, auto } => NodeKind::FootnoteReference {
                auto: auto.map(intern),
                ids,
                refid,
            },
            NodeKindData::Epigraph { classes } => NodeKind::Epigraph { classes },
            NodeKindData::Extension { class_name, attrs } => {
                NodeKind::Extension { class_name, attrs }
            }
            NodeKindData::Field => NodeKind::Field,
            NodeKindData::FieldBody => NodeKind::FieldBody,
            NodeKindData::FieldList => NodeKind::FieldList,
            NodeKindData::FieldName => NodeKind::FieldName,
            NodeKindData::GenericAdmonition { title, classes } => {
                NodeKind::GenericAdmonition { title, classes }
            }
            NodeKindData::Image {
                alt,
                classes,
                height,
                uri,
                width,
            } => NodeKind::Image {
                alt,
                classes,
                height,
                uri,
                width,
            },
            NodeKindData::Inline { classes } => NodeKind::Inline { classes },
            NodeKindData::Keyboard => NodeKind::Keyboard,
            NodeKindData::Label => NodeKind::Label,
            NodeKindData::Legend => NodeKind::Legend,
            NodeKindData::ListItem => NodeKind::ListItem,
            NodeKindData::Literal => NodeKind::Literal,
            NodeKindData::LiteralBlock {
                caption,
                classes,
                ids,
                source,
            } => NodeKind::LiteralBlock {
                caption,
                classes,
                ids,
                source,
            },
            NodeKindData::Math { latex } => NodeKind::Math { latex },
            NodeKindData::MathBlock { latex } => NodeKind::MathBlock { latex },
            NodeKindData::ObjectDescription {
                classes,
                ids,
                sig_text,
                signature_references,
                source_link,
            } => NodeKind::ObjectDescription {
                classes,
                ids,
                sig_text,
                signature_references,
                source_link,
            },
            NodeKindData::Paragraph => NodeKind::Paragraph,
            NodeKindData::PendingXref {
                refdoc,
                refdomain,
                refexplicit,
                reftarget,
                reftype,
                warn_missing,
            } => NodeKind::PendingXref {
                refdoc,
                refdomain,
                refexplicit,
                reftarget,
                reftype,
                warn_missing,
            },
            NodeKindData::Problematic { ids, refid } => NodeKind::Problematic { ids, refid },
            NodeKindData::Raw { format } => NodeKind::Raw { format },
            NodeKindData::Reference {
                anonymous,
                classes,
                name,
                refuri,
            } => NodeKind::Reference {
                anonymous,
                classes,
                name,
                refuri,
            },
            NodeKindData::Row => NodeKind::Row,
            NodeKindData::Rubric => NodeKind::Rubric,
            NodeKindData::Section {
                classes,
                ids,
                names,
            } => NodeKind::Section {
                classes,
                ids,
                names,
            },
            NodeKindData::Strong => NodeKind::Strong,
            NodeKindData::Subscript => NodeKind::Subscript,
            NodeKindData::Subtitle { ids, names } => NodeKind::Subtitle { ids, names },
            NodeKindData::Superscript => NodeKind::Superscript,
            NodeKindData::Table => NodeKind::Table,
            NodeKindData::Tbody => NodeKind::Tbody,
            NodeKindData::Tgroup { cols } => NodeKind::Tgroup { cols },
            NodeKindData::Thead => NodeKind::Thead,
            NodeKindData::Target {
                anonymous,
                ids,
                names,
                refuri,
            } => NodeKind::Target {
                anonymous,
                ids,
                names,
                refuri,
            },
            NodeKindData::Term => NodeKind::Term,
            NodeKindData::Text(s) => NodeKind::Text(s),
            NodeKindData::Title => NodeKind::Title,
            NodeKindData::TitleReference => NodeKind::TitleReference,
            NodeKindData::Transition => NodeKind::Transition,
            NodeKindData::Toctree {
                caption,
                entries,
                hidden,
                maxdepth,
            } => NodeKind::Toctree {
                caption,
                entries,
                hidden,
                maxdepth,
            },
            NodeKindData::SubstitutionDefinition { names } => {
                NodeKind::SubstitutionDefinition { names }
            }
            NodeKindData::SubstitutionReference { refname } => {
                NodeKind::SubstitutionReference { refname }
            }
            NodeKindData::SystemMessage {
                backrefs,
                ids,
                level,
                line,
                ty,
            } => NodeKind::SystemMessage {
                backrefs,
                ids,
                level,
                line,
                ty: intern(ty),
            },
            NodeKindData::VersionModified { kind, version } => NodeKind::VersionModified {
                kind: intern(kind),
                version,
            },
        }
    }
}

#[cfg(test)]
mod doctree_serde_tests {
    use super::*;

    #[test]
    fn round_trip_preserves_structure() {
        let mut tree = Doctree::new_document("test.rst");
        let root = tree.root();
        let sec = tree.append(
            root,
            NodeKind::Section {
                ids: "s1".into(),
                names: "s1".into(),
                classes: String::new(),
            },
        );
        tree.append(sec, NodeKind::Title);
        tree.append(sec, NodeKind::Text("hello".into()));
        tree.append(
            sec,
            NodeKind::EnumeratedList {
                enumtype: "arabic",
                prefix: String::new(),
                suffix: ".".into(),
                start: None,
            },
        );
        tree.append(sec, NodeKind::Admonition { kind: "note" });

        let bytes = tree.to_bytes();
        let restored = Doctree::from_bytes(&bytes).expect("round trip");

        assert_eq!(restored.nodes_len(), tree.nodes_len());
        assert_eq!(restored.root(), tree.root());
        match &restored.node(sec).kind {
            NodeKind::Section { ids, .. } => assert_eq!(ids, "s1"),
            other => panic!("unexpected kind: {other:?}"),
        }
        assert_eq!(restored.node(sec).children.len(), 4);
    }

    #[test]
    fn literal_block_source_round_trips_and_defaults_for_older_data() {
        let mut tree = Doctree::new_document("test.md");
        let literal = tree.append(
            tree.root(),
            NodeKind::LiteralBlock {
                ids: None,
                classes: String::new(),
                caption: None,
                source: Some("docs/example.md".into()),
            },
        );

        let restored = Doctree::from_bytes(&tree.to_bytes()).unwrap();
        let NodeKind::LiteralBlock { source, .. } = &restored.node(literal).kind else {
            panic!("expected literal block");
        };
        assert_eq!(source.as_deref(), Some("docs/example.md"));

        let mut old_data: serde_json::Value = serde_json::from_slice(&tree.to_bytes()).unwrap();
        old_data["nodes"][literal]["kind"]["LiteralBlock"]
            .as_object_mut()
            .unwrap()
            .remove("source");
        let restored_old = Doctree::from_bytes(&serde_json::to_vec(&old_data).unwrap()).unwrap();
        let NodeKind::LiteralBlock { source, .. } = &restored_old.node(literal).kind else {
            panic!("expected literal block");
        };
        assert_eq!(*source, None);
    }

    #[test]
    fn round_trip_enumtype_and_admonition_kind_are_static_strs() {
        let mut tree = Doctree::new_document("test.rst");
        let root = tree.root();
        let list = tree.append(
            root,
            NodeKind::EnumeratedList {
                enumtype: "upperroman",
                prefix: "(".into(),
                suffix: ")".into(),
                start: Some(3),
            },
        );
        let bytes = tree.to_bytes();
        let restored = Doctree::from_bytes(&bytes).unwrap();
        match &restored.node(list).kind {
            NodeKind::EnumeratedList {
                enumtype, start, ..
            } => {
                assert_eq!(*enumtype, "upperroman");
                assert_eq!(*start, Some(3));
            }
            other => panic!("unexpected kind: {other:?}"),
        }
    }

    #[test]
    fn empty_document_round_trips() {
        let tree = Doctree::new_document("empty.rst");
        let bytes = tree.to_bytes();
        let restored = Doctree::from_bytes(&bytes).unwrap();
        assert_eq!(restored.nodes_len(), tree.nodes_len());
    }

    #[test]
    fn depth_first_traversal_is_stack_safe_and_ordered() {
        let mut tree = Doctree::new_document("deep.rst");
        let mut parent = tree.root();
        for _ in 0..10_000 {
            parent = tree.append(
                parent,
                NodeKind::Container {
                    classes: String::new(),
                },
            );
        }

        let events: Vec<_> = tree.depth_first(tree.root()).collect();
        assert_eq!(events.len(), 20_002);
        assert!(matches!(
            events.first(),
            Some(VisitEvent::Enter { depth: 0, .. })
        ));
        assert!(matches!(
            events.last(),
            Some(VisitEvent::Exit { depth: 0, .. })
        ));
        assert!(events.iter().enumerate().all(|(index, event)| match event {
            VisitEvent::Enter { depth, .. } => *depth == index,
            VisitEvent::Exit { depth, .. } => *depth == 20_001 - index,
        }));
    }
}
