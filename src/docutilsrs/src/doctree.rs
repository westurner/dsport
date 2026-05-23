//! Minimal owned doctree.
//!
//! Phase 1 + phase 2 slice. Extended incrementally; see `docs/compat.md`.
//!
//! Layout: an arena of [`Node`]s indexed by [`NodeId`]. Parent and child
//! pointers are `NodeId`s, never references, so the tree is cheap to mutate
//! and trivial to traverse without lifetime gymnastics.

pub type NodeId = usize;

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
}
