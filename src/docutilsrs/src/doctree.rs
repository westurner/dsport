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
    Document {
        source: String,
        /// Promoted title (if first block is a lone section). Empty otherwise.
        ids: String,
        names: String,
        title: String,
    },
    Section {
        ids: String,
        names: String,
        classes: String,
    },
    Title,
    Subtitle {
        ids: String,
        names: String,
    },
    Transition,
    Paragraph,
    Text(String),
    Emphasis,
    Strong,
    Literal,
    /// `<title_reference>` element (role `:title:` / `:t:`).
    TitleReference,
    /// `<inline classes="...">` element used by some roles/directives.
    Inline {
        classes: String,
    },
    /// `<math>` inline element produced by the `:math:` role. `latex` is
    /// the raw LaTeX source.
    Math {
        latex: String,
    },
    /// `<math_block>` element produced by the `.. math::` directive.
    /// `latex` is the raw LaTeX source.
    MathBlock {
        latex: String,
    },
    /// Pre-formatted block. `classes` is space-separated (e.g. "code python").
    LiteralBlock {
        classes: String,
    },
    BulletList {
        bullet: char,
    },
    /// Ordered list. `enumtype` is one of `arabic`, `loweralpha`,
    /// `upperalpha`, `lowerroman`, `upperroman`; `prefix`/`suffix` bracket
    /// each enumerator (e.g. `(` / `)` or `` / `.`).
    EnumeratedList {
        enumtype: &'static str,
        prefix: String,
        suffix: String,
        start: Option<u32>,
    },
    ListItem,
    DefinitionList,
    DefinitionListItem,
    Term,
    Classifier,
    Definition,
    FieldList,
    Field,
    FieldName,
    FieldBody,
    /// Bibliographic info container (field list at the top of a document
    /// containing recognized bibliographic fields).
    Docinfo,
    /// A recognized bibliographic field (e.g. `<author>`, `<date>`). `tag`
    /// is the docutils element name (lower-case).
    Bibliographic {
        tag: &'static str,
    },
    BlockQuote,
    /// Admonition with a fixed kind (`note`, `warning`, `tip`, etc.).
    Admonition {
        kind: &'static str,
    },
    /// `.. container:: classes` — generic named container `<div>`.
    Container {
        classes: String,
    },
    /// `.. admonition:: Title` — generic admonition with an arbitrary
    /// title (as opposed to the fixed-kind `Admonition` variant).
    /// `classes` holds the optional `:class:` option value.
    GenericAdmonition {
        title: String,
        classes: String,
    },
    /// `.. epigraph::` — a blockquote rendered with an `epigraph` class,
    /// plus any extra classes propagated from a preceding `.. class::`/
    /// `.. cssclass::` directive.
    Epigraph {
        classes: String,
    },
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
    Image {
        uri: String,
        alt: Option<String>,
        width: Option<String>,
        height: Option<String>,
    },
    /// Raw passthrough (`.. raw:: format`).
    Raw {
        format: String,
    },
    Comment,
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
    /// Explicit hyperlink target. `ids` is the normalized identifier,
    /// `names` is the human-readable name (space-separated).
    Target {
        ids: String,
        names: String,
        refuri: String,
        anonymous: bool,
    },
    SubstitutionDefinition {
        names: String,
    },
    SubstitutionReference {
        refname: String,
    },
    Table,
    Tgroup {
        cols: u32,
    },
    Colspec {
        colwidth: u32,
    },
    Thead,
    Tbody,
    Row,
    /// Table cell. `morecols` is the extra number of columns this entry
    /// spans (0 for a regular single-column cell); colspan support is
    /// limited to grid tables. `morerows` (rowspan) is currently always
    /// 0 — see `docs/compat.md`.
    Entry {
        morecols: u32,
        morerows: u32,
    },
    // ── phase 2 deferrals ───────────────────────────────────────────────
    /// Attribution line within a block_quote (`-- Author`).
    Attribution,
    /// Container for an image + optional caption + legend.
    Figure,
    /// First paragraph of a figure body.
    Caption,
    /// Remaining content of a figure body, after the caption.
    Legend,
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
    /// `<citation>` element. Mirrors `Footnote` but for citations (no `auto`).
    Citation {
        ids: String,
        names: String,
        backrefs: String,
    },
    /// `<citation_reference>` element.
    CitationReference {
        ids: String,
        refid: String,
    },
    /// `<label>` element used inside footnote/citation.
    Label,
    /// `<problematic>` element. Produced as a placeholder for unresolved
    /// references; `refid` points to the matching `<system_message>`.
    Problematic {
        ids: String,
        refid: String,
    },
    /// `<system_message>` element produced by the parser for error and
    /// warning conditions (e.g. unresolved references).
    SystemMessage {
        level: u32,
        line: Option<u32>,
        ty: &'static str,
        ids: String,
        backrefs: String,
    },
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
    /// `<abbreviation explanation="...">` element produced by `:abbr:` / `:abbreviation:`.
    Abbreviation {
        explanation: String,
    },
    /// `<subscript>` element produced by `:sub:` / `:subscript:`.
    Subscript,
    /// `<superscript>` element produced by `:sup:` / `:superscript:`.
    Superscript,
    /// `<keyboard>` element produced by `:kbd:`.
    Keyboard,
    /// `<rubric>` element produced by `.. rubric::`.
    Rubric,
    /// `<versionmodified>` element produced by `.. versionadded::`, `.. versionchanged::`, etc.
    VersionModified {
        kind: &'static str,
        version: String,
    },
    /// Cross-reference node to be resolved during post-read reference resolution.
    PendingXref {
        reftype: String,
        reftarget: String,
        refdoc: String,
        refdomain: String,
        refexplicit: bool,
        warn_missing: bool,
    },
    /// A generic Sphinx domain "object description" block, e.g.
    /// `.. rst:directive::`/`.. rst:role::` (Tier **H3c**), reusable for
    /// other domains' description directives later. Renders as
    /// `<dl class="{classes}"><dt id="{ids}">{sig_text}</dt><dd>
    /// {children}</dd></dl>`; `children` (via the arena) are the `<dd>`
    /// body content blocks.
    ObjectDescription {
        /// Space-separated `<dl>`/`<dt>` classes, e.g. `"rst directive"`.
        classes: String,
        /// The `<dt id="...">` anchor id (empty = no id).
        ids: String,
        /// The rendered signature text, e.g. `".. toctree::"` or `":ref:"`.
        sig_text: String,
    },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
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
}

#[derive(serde::Serialize, serde::Deserialize)]
enum NodeKindData {
    Document {
        source: String,
        ids: String,
        names: String,
        title: String,
    },
    Section {
        ids: String,
        names: String,
        classes: String,
    },
    Title,
    Subtitle {
        ids: String,
        names: String,
    },
    Transition,
    Paragraph,
    Text(String),
    Emphasis,
    Strong,
    Literal,
    TitleReference,
    Inline {
        classes: String,
    },
    Math {
        latex: String,
    },
    MathBlock {
        latex: String,
    },
    LiteralBlock {
        classes: String,
    },
    BulletList {
        bullet: char,
    },
    EnumeratedList {
        enumtype: String,
        prefix: String,
        suffix: String,
        start: Option<u32>,
    },
    ListItem,
    DefinitionList,
    DefinitionListItem,
    Term,
    Classifier,
    Definition,
    FieldList,
    Field,
    FieldName,
    FieldBody,
    Docinfo,
    Bibliographic {
        tag: String,
    },
    BlockQuote,
    Admonition {
        kind: String,
    },
    Container {
        classes: String,
    },
    GenericAdmonition {
        title: String,
        classes: String,
    },
    Epigraph {
        classes: String,
    },
    Toctree {
        caption: Option<String>,
        maxdepth: i32,
        hidden: bool,
        entries: Vec<String>,
    },
    Image {
        uri: String,
        alt: Option<String>,
        width: Option<String>,
        height: Option<String>,
    },
    Raw {
        format: String,
    },
    Comment,
    Reference {
        name: String,
        refuri: String,
        anonymous: bool,
        #[serde(default)]
        classes: String,
    },
    Target {
        ids: String,
        names: String,
        refuri: String,
        anonymous: bool,
    },
    SubstitutionDefinition {
        names: String,
    },
    SubstitutionReference {
        refname: String,
    },
    Table,
    Tgroup {
        cols: u32,
    },
    Colspec {
        colwidth: u32,
    },
    Thead,
    Tbody,
    Row,
    Entry {
        morecols: u32,
        morerows: u32,
    },
    Attribution,
    Figure,
    Caption,
    Legend,
    Footnote {
        ids: String,
        names: String,
        backrefs: String,
        auto: Option<String>,
    },
    FootnoteReference {
        ids: String,
        refid: String,
        auto: Option<String>,
    },
    Citation {
        ids: String,
        names: String,
        backrefs: String,
    },
    CitationReference {
        ids: String,
        refid: String,
    },
    Label,
    Problematic {
        ids: String,
        refid: String,
    },
    SystemMessage {
        level: u32,
        line: Option<u32>,
        ty: String,
        ids: String,
        backrefs: String,
    },
    Extension {
        class_name: String,
        attrs: BTreeMap<String, NodeAttributeValue>,
    },
    Abbreviation {
        explanation: String,
    },
    Subscript,
    Superscript,
    Keyboard,
    Rubric,
    VersionModified {
        kind: String,
        version: String,
    },
    PendingXref {
        reftype: String,
        reftarget: String,
        refdoc: String,
        refdomain: String,
        refexplicit: bool,
        warn_missing: bool,
    },
    ObjectDescription {
        classes: String,
        ids: String,
        sig_text: String,
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
        }
    }
}

impl From<NodeData> for Node {
    fn from(data: NodeData) -> Self {
        Node {
            kind: data.kind.into(),
            parent: data.parent,
            children: data.children,
        }
    }
}

impl From<&NodeKind> for NodeKindData {
    fn from(kind: &NodeKind) -> Self {
        match kind.clone() {
            NodeKind::Document {
                source,
                ids,
                names,
                title,
            } => NodeKindData::Document {
                source,
                ids,
                names,
                title,
            },
            NodeKind::Section {
                ids,
                names,
                classes,
            } => NodeKindData::Section {
                ids,
                names,
                classes,
            },
            NodeKind::Title => NodeKindData::Title,
            NodeKind::Subtitle { ids, names } => NodeKindData::Subtitle { ids, names },
            NodeKind::Transition => NodeKindData::Transition,
            NodeKind::Paragraph => NodeKindData::Paragraph,
            NodeKind::Text(s) => NodeKindData::Text(s),
            NodeKind::Emphasis => NodeKindData::Emphasis,
            NodeKind::Strong => NodeKindData::Strong,
            NodeKind::Literal => NodeKindData::Literal,
            NodeKind::TitleReference => NodeKindData::TitleReference,
            NodeKind::Inline { classes } => NodeKindData::Inline { classes },
            NodeKind::Math { latex } => NodeKindData::Math { latex },
            NodeKind::MathBlock { latex } => NodeKindData::MathBlock { latex },
            NodeKind::LiteralBlock { classes } => NodeKindData::LiteralBlock { classes },
            NodeKind::BulletList { bullet } => NodeKindData::BulletList { bullet },
            NodeKind::EnumeratedList {
                enumtype,
                prefix,
                suffix,
                start,
            } => NodeKindData::EnumeratedList {
                enumtype: enumtype.to_string(),
                prefix,
                suffix,
                start,
            },
            NodeKind::ListItem => NodeKindData::ListItem,
            NodeKind::DefinitionList => NodeKindData::DefinitionList,
            NodeKind::DefinitionListItem => NodeKindData::DefinitionListItem,
            NodeKind::Term => NodeKindData::Term,
            NodeKind::Classifier => NodeKindData::Classifier,
            NodeKind::Definition => NodeKindData::Definition,
            NodeKind::FieldList => NodeKindData::FieldList,
            NodeKind::Field => NodeKindData::Field,
            NodeKind::FieldName => NodeKindData::FieldName,
            NodeKind::FieldBody => NodeKindData::FieldBody,
            NodeKind::Docinfo => NodeKindData::Docinfo,
            NodeKind::Bibliographic { tag } => NodeKindData::Bibliographic {
                tag: tag.to_string(),
            },
            NodeKind::BlockQuote => NodeKindData::BlockQuote,
            NodeKind::Admonition { kind } => NodeKindData::Admonition {
                kind: kind.to_string(),
            },
            NodeKind::Container { classes } => NodeKindData::Container { classes },
            NodeKind::GenericAdmonition { title, classes } => {
                NodeKindData::GenericAdmonition { title, classes }
            }
            NodeKind::Epigraph { classes } => NodeKindData::Epigraph { classes },
            NodeKind::Toctree {
                caption,
                maxdepth,
                hidden,
                entries,
            } => NodeKindData::Toctree {
                caption,
                maxdepth,
                hidden,
                entries,
            },
            NodeKind::Image {
                uri,
                alt,
                width,
                height,
            } => NodeKindData::Image {
                uri,
                alt,
                width,
                height,
            },
            NodeKind::Raw { format } => NodeKindData::Raw { format },
            NodeKind::Comment => NodeKindData::Comment,
            NodeKind::Reference {
                name,
                refuri,
                anonymous,
                classes,
            } => NodeKindData::Reference {
                name,
                refuri,
                anonymous,
                classes,
            },
            NodeKind::Target {
                ids,
                names,
                refuri,
                anonymous,
            } => NodeKindData::Target {
                ids,
                names,
                refuri,
                anonymous,
            },
            NodeKind::SubstitutionDefinition { names } => {
                NodeKindData::SubstitutionDefinition { names }
            }
            NodeKind::SubstitutionReference { refname } => {
                NodeKindData::SubstitutionReference { refname }
            }
            NodeKind::Table => NodeKindData::Table,
            NodeKind::Tgroup { cols } => NodeKindData::Tgroup { cols },
            NodeKind::Colspec { colwidth } => NodeKindData::Colspec { colwidth },
            NodeKind::Thead => NodeKindData::Thead,
            NodeKind::Tbody => NodeKindData::Tbody,
            NodeKind::Row => NodeKindData::Row,
            NodeKind::Entry { morecols, morerows } => NodeKindData::Entry { morecols, morerows },
            NodeKind::Attribution => NodeKindData::Attribution,
            NodeKind::Figure => NodeKindData::Figure,
            NodeKind::Caption => NodeKindData::Caption,
            NodeKind::Legend => NodeKindData::Legend,
            NodeKind::Footnote {
                ids,
                names,
                backrefs,
                auto,
            } => NodeKindData::Footnote {
                ids,
                names,
                backrefs,
                auto: auto.map(str::to_string),
            },
            NodeKind::FootnoteReference { ids, refid, auto } => NodeKindData::FootnoteReference {
                ids,
                refid,
                auto: auto.map(str::to_string),
            },
            NodeKind::Citation {
                ids,
                names,
                backrefs,
            } => NodeKindData::Citation {
                ids,
                names,
                backrefs,
            },
            NodeKind::CitationReference { ids, refid } => {
                NodeKindData::CitationReference { ids, refid }
            }
            NodeKind::Label => NodeKindData::Label,
            NodeKind::Problematic { ids, refid } => NodeKindData::Problematic { ids, refid },
            NodeKind::SystemMessage {
                level,
                line,
                ty,
                ids,
                backrefs,
            } => NodeKindData::SystemMessage {
                level,
                line,
                ty: ty.to_string(),
                ids,
                backrefs,
            },
            NodeKind::Extension { class_name, attrs } => {
                NodeKindData::Extension { class_name, attrs }
            }
            NodeKind::Abbreviation { explanation } => NodeKindData::Abbreviation { explanation },
            NodeKind::Subscript => NodeKindData::Subscript,
            NodeKind::Superscript => NodeKindData::Superscript,
            NodeKind::Keyboard => NodeKindData::Keyboard,
            NodeKind::Rubric => NodeKindData::Rubric,
            NodeKind::VersionModified { kind, version } => NodeKindData::VersionModified {
                kind: kind.to_string(),
                version,
            },
            NodeKind::PendingXref {
                reftype,
                reftarget,
                refdoc,
                refdomain,
                refexplicit,
                warn_missing,
            } => NodeKindData::PendingXref {
                reftype,
                reftarget,
                refdoc,
                refdomain,
                refexplicit,
                warn_missing,
            },
            NodeKind::ObjectDescription {
                classes,
                ids,
                sig_text,
            } => NodeKindData::ObjectDescription {
                classes,
                ids,
                sig_text,
            },
        }
    }
}

impl From<NodeKindData> for NodeKind {
    fn from(data: NodeKindData) -> Self {
        match data {
            NodeKindData::Document {
                source,
                ids,
                names,
                title,
            } => NodeKind::Document {
                source,
                ids,
                names,
                title,
            },
            NodeKindData::Section {
                ids,
                names,
                classes,
            } => NodeKind::Section {
                ids,
                names,
                classes,
            },
            NodeKindData::Title => NodeKind::Title,
            NodeKindData::Subtitle { ids, names } => NodeKind::Subtitle { ids, names },
            NodeKindData::Transition => NodeKind::Transition,
            NodeKindData::Paragraph => NodeKind::Paragraph,
            NodeKindData::Text(s) => NodeKind::Text(s),
            NodeKindData::Emphasis => NodeKind::Emphasis,
            NodeKindData::Strong => NodeKind::Strong,
            NodeKindData::Literal => NodeKind::Literal,
            NodeKindData::TitleReference => NodeKind::TitleReference,
            NodeKindData::Inline { classes } => NodeKind::Inline { classes },
            NodeKindData::Math { latex } => NodeKind::Math { latex },
            NodeKindData::MathBlock { latex } => NodeKind::MathBlock { latex },
            NodeKindData::LiteralBlock { classes } => NodeKind::LiteralBlock { classes },
            NodeKindData::BulletList { bullet } => NodeKind::BulletList { bullet },
            NodeKindData::EnumeratedList {
                enumtype,
                prefix,
                suffix,
                start,
            } => NodeKind::EnumeratedList {
                enumtype: intern(enumtype),
                prefix,
                suffix,
                start,
            },
            NodeKindData::ListItem => NodeKind::ListItem,
            NodeKindData::DefinitionList => NodeKind::DefinitionList,
            NodeKindData::DefinitionListItem => NodeKind::DefinitionListItem,
            NodeKindData::Term => NodeKind::Term,
            NodeKindData::Classifier => NodeKind::Classifier,
            NodeKindData::Definition => NodeKind::Definition,
            NodeKindData::FieldList => NodeKind::FieldList,
            NodeKindData::Field => NodeKind::Field,
            NodeKindData::FieldName => NodeKind::FieldName,
            NodeKindData::FieldBody => NodeKind::FieldBody,
            NodeKindData::Docinfo => NodeKind::Docinfo,
            NodeKindData::Bibliographic { tag } => NodeKind::Bibliographic { tag: intern(tag) },
            NodeKindData::BlockQuote => NodeKind::BlockQuote,
            NodeKindData::Admonition { kind } => NodeKind::Admonition { kind: intern(kind) },
            NodeKindData::Container { classes } => NodeKind::Container { classes },
            NodeKindData::GenericAdmonition { title, classes } => {
                NodeKind::GenericAdmonition { title, classes }
            }
            NodeKindData::Epigraph { classes } => NodeKind::Epigraph { classes },
            NodeKindData::Toctree {
                caption,
                maxdepth,
                hidden,
                entries,
            } => NodeKind::Toctree {
                caption,
                maxdepth,
                hidden,
                entries,
            },
            NodeKindData::Image {
                uri,
                alt,
                width,
                height,
            } => NodeKind::Image {
                uri,
                alt,
                width,
                height,
            },
            NodeKindData::Raw { format } => NodeKind::Raw { format },
            NodeKindData::Comment => NodeKind::Comment,
            NodeKindData::Reference {
                name,
                refuri,
                anonymous,
                classes,
            } => NodeKind::Reference {
                name,
                refuri,
                anonymous,
                classes,
            },
            NodeKindData::Target {
                ids,
                names,
                refuri,
                anonymous,
            } => NodeKind::Target {
                ids,
                names,
                refuri,
                anonymous,
            },
            NodeKindData::SubstitutionDefinition { names } => {
                NodeKind::SubstitutionDefinition { names }
            }
            NodeKindData::SubstitutionReference { refname } => {
                NodeKind::SubstitutionReference { refname }
            }
            NodeKindData::Table => NodeKind::Table,
            NodeKindData::Tgroup { cols } => NodeKind::Tgroup { cols },
            NodeKindData::Colspec { colwidth } => NodeKind::Colspec { colwidth },
            NodeKindData::Thead => NodeKind::Thead,
            NodeKindData::Tbody => NodeKind::Tbody,
            NodeKindData::Row => NodeKind::Row,
            NodeKindData::Entry { morecols, morerows } => NodeKind::Entry { morecols, morerows },
            NodeKindData::Attribution => NodeKind::Attribution,
            NodeKindData::Figure => NodeKind::Figure,
            NodeKindData::Caption => NodeKind::Caption,
            NodeKindData::Legend => NodeKind::Legend,
            NodeKindData::Footnote {
                ids,
                names,
                backrefs,
                auto,
            } => NodeKind::Footnote {
                ids,
                names,
                backrefs,
                auto: auto.map(intern),
            },
            NodeKindData::FootnoteReference { ids, refid, auto } => NodeKind::FootnoteReference {
                ids,
                refid,
                auto: auto.map(intern),
            },
            NodeKindData::Citation {
                ids,
                names,
                backrefs,
            } => NodeKind::Citation {
                ids,
                names,
                backrefs,
            },
            NodeKindData::CitationReference { ids, refid } => {
                NodeKind::CitationReference { ids, refid }
            }
            NodeKindData::Label => NodeKind::Label,
            NodeKindData::Problematic { ids, refid } => NodeKind::Problematic { ids, refid },
            NodeKindData::SystemMessage {
                level,
                line,
                ty,
                ids,
                backrefs,
            } => NodeKind::SystemMessage {
                level,
                line,
                ty: intern(ty),
                ids,
                backrefs,
            },
            NodeKindData::Extension { class_name, attrs } => {
                NodeKind::Extension { class_name, attrs }
            }
            NodeKindData::Abbreviation { explanation } => NodeKind::Abbreviation { explanation },
            NodeKindData::Subscript => NodeKind::Subscript,
            NodeKindData::Superscript => NodeKind::Superscript,
            NodeKindData::Keyboard => NodeKind::Keyboard,
            NodeKindData::Rubric => NodeKind::Rubric,
            NodeKindData::VersionModified { kind, version } => NodeKind::VersionModified {
                kind: intern(kind),
                version,
            },
            NodeKindData::PendingXref {
                reftype,
                reftarget,
                refdoc,
                refdomain,
                refexplicit,
                warn_missing,
            } => NodeKind::PendingXref {
                reftype,
                reftarget,
                refdoc,
                refdomain,
                refexplicit,
                warn_missing,
            },
            NodeKindData::ObjectDescription {
                classes,
                ids,
                sig_text,
            } => NodeKind::ObjectDescription {
                classes,
                ids,
                sig_text,
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
