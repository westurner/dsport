//! PyO3 bindings for the doctree.

#![allow(clippy::collapsible_match)]

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple, PyType};

use crate::doctree::{Doctree, NodeId, NodeKind};
use crate::parser::parse_rst_with_source;
use crate::writer::pseudo_xml;

#[pyclass(name = "Doctree", module = "docutilsrs")]
pub struct PyDoctree {
    pub(crate) inner: Doctree,
}

impl PyDoctree {
    /// Wrap an owned [`Doctree`] for exposure to Python. Used by the
    /// transform-plugin bridge to hand a snapshot to a Python callable.
    pub fn new(inner: Doctree) -> Self {
        Self { inner }
    }

    /// Move the inner [`Doctree`] out, replacing it with an empty
    /// placeholder. Used by the transform bridge after a Python callable
    /// returns so the parser can resume ownership.
    pub fn take_inner(&mut self) -> Doctree {
        std::mem::replace(&mut self.inner, Doctree::new_document("<empty>"))
    }
}

#[pymethods]
impl PyDoctree {
    fn pformat(&self) -> String {
        pseudo_xml(&self.inner)
    }

    #[getter]
    fn root(slf: Py<Self>, py: Python<'_>) -> PyNode {
        let root_id = slf.borrow(py).inner.root();
        PyNode {
            tree: slf,
            id: root_id,
        }
    }

    fn __repr__(&self) -> String {
        format!("<Doctree root={}>", self.inner.root())
    }

    fn __str__(&self) -> String {
        self.pformat()
    }

    /// Mirrors `docutils.nodes.Node.findall` (called on the document root),
    /// e.g. `sphinx.ext.viewcode`/`linkcode`'s `doctree.findall(addnodes.desc)`.
    /// See [`PyNode::findall`] for the full semantics and accepted
    /// deviations (this just delegates to the root node with itself as the
    /// traversal start).
    #[pyo3(signature = (condition=None, include_self=true, descend=true, siblings=false, ascend=false))]
    fn findall(
        slf: Py<Self>,
        py: Python<'_>,
        condition: Option<Py<PyAny>>,
        include_self: bool,
        descend: bool,
        siblings: bool,
        ascend: bool,
    ) -> PyResult<Vec<Py<PyNode>>> {
        let root_id = slf.borrow(py).inner.root();
        let root = Py::new(
            py,
            PyNode {
                tree: slf,
                id: root_id,
            },
        )?;
        PyNode::findall(root, py, condition, include_self, descend, siblings, ascend)
    }
}

/// Tag name for a node kind, matching upstream `docutils.nodes.Node.tagname`.
/// Extracted from [`PyNode::tag`] so [`findall`](PyNode::findall)'s
/// class-based `condition` matching can reuse the exact same mapping.
fn node_kind_tag(kind: &NodeKind) -> String {
    match kind {
        NodeKind::Document { .. } => "document".into(),
        NodeKind::Section { .. } => "section".into(),
        NodeKind::Title => "title".into(),
        NodeKind::Subtitle { .. } => "subtitle".into(),
        NodeKind::Transition => "transition".into(),
        NodeKind::Paragraph => "paragraph".into(),
        NodeKind::Text(_) => "#text".into(),
        NodeKind::Emphasis => "emphasis".into(),
        NodeKind::Strong => "strong".into(),
        NodeKind::Literal => "literal".into(),
        NodeKind::TitleReference => "title_reference".into(),
        NodeKind::Inline { .. } => "inline".into(),
        NodeKind::Math { .. } => "math".into(),
        NodeKind::MathBlock { .. } => "math_block".into(),
        NodeKind::LiteralBlock { .. } => "literal_block".into(),
        NodeKind::BulletList { .. } => "bullet_list".into(),
        NodeKind::EnumeratedList { .. } => "enumerated_list".into(),
        NodeKind::ListItem => "list_item".into(),
        NodeKind::DefinitionList => "definition_list".into(),
        NodeKind::DefinitionListItem => "definition_list_item".into(),
        NodeKind::Term => "term".into(),
        NodeKind::Classifier => "classifier".into(),
        NodeKind::Definition => "definition".into(),
        NodeKind::FieldList => "field_list".into(),
        NodeKind::Field => "field".into(),
        NodeKind::FieldName => "field_name".into(),
        NodeKind::FieldBody => "field_body".into(),
        NodeKind::Docinfo => "docinfo".into(),
        NodeKind::Bibliographic { tag } => (*tag).into(),
        NodeKind::BlockQuote => "block_quote".into(),
        NodeKind::Admonition { kind } => (*kind).into(),
        NodeKind::Container { .. } => "container".into(),
        NodeKind::GenericAdmonition { .. } => "admonition".into(),
        NodeKind::Epigraph { .. } => "block_quote".into(),
        NodeKind::Toctree { .. } => "toctree".into(),
        NodeKind::Image { .. } => "image".into(),
        NodeKind::Raw { .. } => "raw".into(),
        NodeKind::Comment => "comment".into(),
        NodeKind::Reference { .. } => "reference".into(),
        NodeKind::Target { .. } => "target".into(),
        NodeKind::SubstitutionDefinition { .. } => "substitution_definition".into(),
        NodeKind::SubstitutionReference { .. } => "substitution_reference".into(),
        NodeKind::Table => "table".into(),
        NodeKind::Tgroup { .. } => "tgroup".into(),
        NodeKind::Colspec { .. } => "colspec".into(),
        NodeKind::Thead => "thead".into(),
        NodeKind::Tbody => "tbody".into(),
        NodeKind::Row => "row".into(),
        NodeKind::Entry { .. } => "entry".into(),
        NodeKind::Attribution => "attribution".into(),
        NodeKind::Figure => "figure".into(),
        NodeKind::Caption => "caption".into(),
        NodeKind::Legend => "legend".into(),
        NodeKind::Label => "label".into(),
        NodeKind::Footnote { .. } => "footnote".into(),
        NodeKind::FootnoteReference { .. } => "footnote_reference".into(),
        NodeKind::Citation { .. } => "citation".into(),
        NodeKind::CitationReference { .. } => "citation_reference".into(),
        NodeKind::Problematic { .. } => "problematic".into(),
        NodeKind::SystemMessage { .. } => "system_message".into(),
        NodeKind::Extension { class_name, .. } => class_name.clone(),
        NodeKind::Abbreviation { .. } => "abbreviation".into(),
        NodeKind::Subscript => "subscript".into(),
        NodeKind::Superscript => "superscript".into(),
        NodeKind::Keyboard => "keyboard".into(),
        NodeKind::Rubric => "rubric".into(),
        NodeKind::VersionModified { .. } => "versionmodified".into(),
        NodeKind::PendingXref { .. } => "pending_xref".into(),
        NodeKind::ObjectDescription { .. } => "object_description".into(),
    }
}

/// Node ids in the order upstream `docutils.nodes.Node.findall` would
/// visit them (before `condition` filtering), per its documented
/// `include_self`/`descend`/`siblings`/`ascend` semantics.
fn findall_candidate_ids(
    tree: &Doctree,
    start: NodeId,
    include_self: bool,
    descend: bool,
    siblings: bool,
    ascend: bool,
) -> Vec<NodeId> {
    fn collect(
        tree: &Doctree,
        id: NodeId,
        include_self: bool,
        descend: bool,
        out: &mut Vec<NodeId>,
    ) {
        if include_self {
            out.push(id);
        }
        if descend {
            for &child in &tree.node(id).children {
                collect(tree, child, true, descend, out);
            }
        }
    }

    let mut out = Vec::new();
    collect(tree, start, include_self, descend, &mut out);

    if siblings || ascend {
        let mut node = start;
        loop {
            let Some(parent) = tree.node(node).parent else {
                break;
            };
            let idx = tree.node(parent).children.iter().position(|&c| c == node);
            if let Some(idx) = idx {
                for &sibling in &tree.node(parent).children[idx + 1..] {
                    collect(tree, sibling, true, descend, &mut out);
                }
            }
            if !ascend {
                break;
            }
            node = parent;
        }
    }

    out
}

/// Whether `condition` (a `findall` argument) matches a node with the
/// given `tag`. See [`PyNode::findall`]'s doc comment for the accepted
/// class-matching deviation.
fn condition_matches(
    py: Python<'_>,
    condition: &Bound<'_, PyAny>,
    tag: &str,
    node_obj: &Py<PyNode>,
) -> PyResult<bool> {
    if let Ok(tuple) = condition.cast::<PyTuple>() {
        for item in tuple.iter() {
            if class_or_predicate_matches(py, &item, tag, node_obj)? {
                return Ok(true);
            }
        }
        return Ok(false);
    }
    class_or_predicate_matches(py, condition, tag, node_obj)
}

fn class_or_predicate_matches(
    py: Python<'_>,
    condition: &Bound<'_, PyAny>,
    tag: &str,
    node_obj: &Py<PyNode>,
) -> PyResult<bool> {
    if condition.cast::<PyType>().is_ok() {
        if let Ok(tagname) = condition.getattr("tagname") {
            if let Ok(s) = tagname.extract::<String>() {
                return Ok(s == tag);
            }
        }
        if let Ok(name) = condition.getattr("__name__") {
            if let Ok(s) = name.extract::<String>() {
                return Ok(s == tag);
            }
        }
        return Ok(false);
    }
    let result = condition.call1((node_obj.clone_ref(py),))?;
    result.is_truthy()
}

#[pyclass(name = "Node", module = "docutilsrs")]
pub struct PyNode {
    tree: Py<PyDoctree>,
    id: NodeId,
}

#[pymethods]
impl PyNode {
    #[getter]
    fn id(&self) -> usize {
        self.id
    }

    #[getter]
    fn tag(&self, py: Python<'_>) -> String {
        node_kind_tag(&self.tree.borrow(py).inner.node(self.id).kind)
    }

    #[getter]
    fn text(&self, py: Python<'_>) -> Option<String> {
        match &self.tree.borrow(py).inner.node(self.id).kind {
            NodeKind::Text(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Mirrors `docutils.nodes.Node.findall(condition=None, include_self=True,
    /// descend=True, siblings=False, ascend=False)`: a pre-order traversal
    /// (self, then children recursively; `siblings`/`ascend` additionally
    /// walk later siblings, and — if `ascend` — repeat up each ancestor)
    /// collecting every node matching `condition`.
    ///
    /// **`condition` matching (accepted deviation):** upstream compares via
    /// `isinstance(node, condition)` against real `docutils.nodes.Element`
    /// subclasses; this port's nodes are always a single `Node` Python
    /// class, never a real subclass of e.g. `sphinx.addnodes.desc`. So a
    /// class (or tuple of classes) is matched by comparing this node's
    /// [`tag`](Self::tag) against the class's `tagname` attribute (or
    /// `__name__` if `tagname` is absent) instead of a real `isinstance`
    /// check. A node kind this port doesn't model at all (e.g. Sphinx's
    /// `desc`/`desc_signature` addnodes, since domain objects are scanned
    /// as text rather than represented as real doctree nodes — see
    /// `sphinxdocrs::domains::scan`) therefore never matches, same as if
    /// upstream's tree simply never contained one — this is what makes
    /// `sphinx.ext.viewcode`/`linkcode`'s `doctree.findall(addnodes.desc)`
    /// return an empty list here rather than crash. A non-class `condition`
    /// is treated as a predicate callable and invoked as `condition(node)`,
    /// matching upstream.
    #[pyo3(signature = (condition=None, include_self=true, descend=true, siblings=false, ascend=false))]
    fn findall(
        slf: Py<Self>,
        py: Python<'_>,
        condition: Option<Py<PyAny>>,
        include_self: bool,
        descend: bool,
        siblings: bool,
        ascend: bool,
    ) -> PyResult<Vec<Py<PyNode>>> {
        let (tree, start_id) = {
            let this = slf.borrow(py);
            (this.tree.clone_ref(py), this.id)
        };
        let ids = {
            let tree_ref = tree.borrow(py);
            findall_candidate_ids(
                &tree_ref.inner,
                start_id,
                include_self,
                descend,
                siblings,
                ascend,
            )
        };

        let mut out = Vec::with_capacity(ids.len());
        for id in ids {
            let node_py = Py::new(
                py,
                PyNode {
                    tree: tree.clone_ref(py),
                    id,
                },
            )?;
            let is_match = match &condition {
                None => true,
                Some(cond) => {
                    let tag = node_kind_tag(&tree.borrow(py).inner.node(id).kind);
                    condition_matches(py, cond.bind(py), &tag, &node_py)?
                }
            };
            if is_match {
                out.push(node_py);
            }
        }
        Ok(out)
    }

    #[getter]
    fn attributes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        match &self.tree.borrow(py).inner.node(self.id).kind {
            NodeKind::Document {
                source,
                ids,
                names,
                title,
            } => {
                dict.set_item("source", source)?;
                if !title.is_empty() {
                    dict.set_item("ids", ids)?;
                    dict.set_item("names", names)?;
                    dict.set_item("title", title)?;
                }
            }
            NodeKind::Section {
                ids,
                names,
                classes,
            } => {
                if !classes.is_empty() {
                    dict.set_item("classes", classes)?;
                }
                dict.set_item("ids", ids)?;
                dict.set_item("names", names)?;
            }
            NodeKind::Subtitle { ids, names } => {
                dict.set_item("ids", ids)?;
                dict.set_item("names", names)?;
            }
            NodeKind::BulletList { bullet } => {
                dict.set_item("bullet", bullet.to_string())?;
            }
            NodeKind::EnumeratedList {
                enumtype,
                prefix,
                suffix,
                start,
            } => {
                dict.set_item("enumtype", *enumtype)?;
                dict.set_item("prefix", prefix)?;
                dict.set_item("suffix", suffix)?;
                if let Some(s) = start {
                    dict.set_item("start", *s)?;
                }
            }
            NodeKind::Inline { classes } => {
                if !classes.is_empty() {
                    dict.set_item("classes", classes)?;
                }
            }
            NodeKind::LiteralBlock { classes } => {
                if !classes.is_empty() {
                    dict.set_item("classes", classes)?;
                }
                dict.set_item("xml:space", "preserve")?;
            }
            NodeKind::Image {
                uri,
                alt,
                width,
                height,
            } => {
                dict.set_item("uri", uri)?;
                if let Some(v) = alt {
                    dict.set_item("alt", v)?;
                }
                if let Some(v) = width {
                    dict.set_item("width", v)?;
                }
                if let Some(v) = height {
                    dict.set_item("height", v)?;
                }
            }
            NodeKind::Raw { format } => {
                dict.set_item("format", format)?;
                dict.set_item("xml:space", "preserve")?;
            }
            NodeKind::Reference {
                name,
                refuri,
                anonymous,
                classes,
            } => {
                if *anonymous {
                    dict.set_item("anonymous", "1")?;
                }
                if !classes.is_empty() {
                    dict.set_item("classes", classes)?;
                }
                dict.set_item("name", name)?;
                dict.set_item("refuri", refuri)?;
            }
            NodeKind::Target {
                ids,
                names,
                refuri,
                anonymous,
            } => {
                if *anonymous {
                    dict.set_item("anonymous", "1")?;
                }
                dict.set_item("ids", ids)?;
                if !names.is_empty() {
                    dict.set_item("names", names)?;
                }
                dict.set_item("refuri", refuri)?;
            }
            NodeKind::SubstitutionDefinition { names } => {
                dict.set_item("names", names)?;
            }
            NodeKind::SubstitutionReference { refname } => {
                dict.set_item("refname", refname)?;
            }
            NodeKind::Tgroup { cols } => {
                dict.set_item("cols", *cols)?;
            }
            NodeKind::Colspec { colwidth } => {
                dict.set_item("colwidth", *colwidth)?;
            }
            NodeKind::Comment => {
                dict.set_item("xml:space", "preserve")?;
            }
            NodeKind::Footnote {
                ids,
                names,
                backrefs,
                auto,
            } => {
                if let Some(a) = auto {
                    dict.set_item("auto", *a)?;
                }
                if !backrefs.is_empty() {
                    dict.set_item("backrefs", backrefs)?;
                }
                dict.set_item("ids", ids)?;
                if !names.is_empty() && !matches!(*auto, Some("*")) {
                    dict.set_item("names", names)?;
                }
            }
            NodeKind::FootnoteReference { ids, refid, auto } => {
                if let Some(a) = auto {
                    dict.set_item("auto", *a)?;
                }
                dict.set_item("ids", ids)?;
                dict.set_item("refid", refid)?;
            }
            NodeKind::Citation {
                ids,
                names,
                backrefs,
            } => {
                if !backrefs.is_empty() {
                    dict.set_item("backrefs", backrefs)?;
                }
                dict.set_item("ids", ids)?;
                dict.set_item("names", names)?;
            }
            NodeKind::CitationReference { ids, refid } => {
                dict.set_item("ids", ids)?;
                dict.set_item("refid", refid)?;
            }
            NodeKind::Problematic { ids, refid } => {
                dict.set_item("ids", ids)?;
                dict.set_item("refid", refid)?;
            }
            NodeKind::SystemMessage {
                level,
                line,
                ty,
                ids,
                backrefs,
            } => {
                if !backrefs.is_empty() {
                    dict.set_item("backrefs", backrefs)?;
                }
                if !ids.is_empty() {
                    dict.set_item("ids", ids)?;
                }
                dict.set_item("level", *level)?;
                if let Some(l) = line {
                    dict.set_item("line", *l)?;
                }
                dict.set_item("source", "<string>")?;
                dict.set_item("type", *ty)?;
            }
            NodeKind::Extension { attrs, .. } => {
                for (k, v) in attrs {
                    dict.set_item(k, v)?;
                }
            }
            NodeKind::ObjectDescription {
                classes,
                ids,
                sig_text,
            } => {
                if !classes.is_empty() {
                    dict.set_item("classes", classes)?;
                }
                if !ids.is_empty() {
                    dict.set_item("ids", ids)?;
                }
                dict.set_item("sig", sig_text)?;
            }
            _ => {}
        }
        Ok(dict)
    }

    #[getter]
    fn children(&self, py: Python<'_>) -> Vec<PyNode> {
        let ids: Vec<NodeId> = self.tree.borrow(py).inner.node(self.id).children.clone();
        ids.into_iter()
            .map(|id| PyNode {
                tree: self.tree.clone_ref(py),
                id,
            })
            .collect()
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!("<Node {} id={}>", self.tag(py), self.id)
    }
}

#[pyfunction(name = "parse_rst", signature = (source, source_path = "<string>"))]
pub fn py_parse_rst(source: &str, source_path: &str) -> PyDoctree {
    PyDoctree {
        inner: parse_rst_with_source(source, source_path),
    }
}
