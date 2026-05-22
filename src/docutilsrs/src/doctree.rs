//! Minimal owned doctree.
//!
//! Phase 1 slice: only the node kinds reachable from
//! paragraphs + inline emphasis/strong/literal. Extended in phase 2.
//!
//! Layout: an arena of [`Node`]s indexed by [`NodeId`]. Parent and child
//! pointers are `NodeId`s, never references, so the tree is cheap to mutate
//! and trivial to traverse without lifetime gymnastics.

pub type NodeId = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Document { source: String },
    Paragraph,
    Text(String),
    Emphasis,
    Strong,
    Literal,
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

    /// Append a new node under `parent` and return its id.
    pub fn append(&mut self, parent: NodeId, kind: NodeKind) -> NodeId {
        let id = self.push(kind, Some(parent));
        self.nodes[parent].children.push(id);
        id
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
