//! PyO3 bindings for the doctree.
//!
//! Exposes a `Doctree` and `Node` class so Python tests can walk the tree
//! and assert structure, in addition to the `parse_to_pseudoxml` string
//! shortcut.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::doctree::{Doctree, NodeId, NodeKind};
use crate::parser::parse_rst_with_source;
use crate::writer::pseudo_xml;

#[pyclass(name = "Doctree", module = "docutilsrs", frozen)]
pub struct PyDoctree {
    pub(crate) inner: Doctree,
}

#[pymethods]
impl PyDoctree {
    /// Pseudo-XML pformat of the full tree (byte-for-byte compatible with
    /// `docutils.writers.pseudoxml`).
    fn pformat(&self) -> String {
        pseudo_xml(&self.inner)
    }

    /// Root `<document>` node.
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
}

#[pyclass(name = "Node", module = "docutilsrs")]
pub struct PyNode {
    tree: Py<PyDoctree>,
    id: NodeId,
}

#[pymethods]
impl PyNode {
    /// Element name, matching docutils' node tag (`document`, `paragraph`,
    /// `bullet_list`, …). `#text` for text nodes.
    #[getter]
    fn tag(&self, py: Python<'_>) -> &'static str {
        match &self.tree.borrow(py).inner.node(self.id).kind {
            NodeKind::Document { .. } => "document",
            NodeKind::Paragraph => "paragraph",
            NodeKind::Text(_) => "#text",
            NodeKind::Emphasis => "emphasis",
            NodeKind::Strong => "strong",
            NodeKind::Literal => "literal",
            NodeKind::BulletList { .. } => "bullet_list",
            NodeKind::ListItem => "list_item",
            NodeKind::Reference { .. } => "reference",
            NodeKind::Target { .. } => "target",
        }
    }

    /// String value for text nodes; `None` for element nodes.
    #[getter]
    fn text(&self, py: Python<'_>) -> Option<String> {
        match &self.tree.borrow(py).inner.node(self.id).kind {
            NodeKind::Text(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Attribute dict in docutils' attribute naming.
    #[getter]
    fn attributes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        match &self.tree.borrow(py).inner.node(self.id).kind {
            NodeKind::Document { source } => {
                dict.set_item("source", source)?;
            }
            NodeKind::BulletList { bullet } => {
                dict.set_item("bullet", bullet.to_string())?;
            }
            NodeKind::Reference { name, refuri } => {
                dict.set_item("name", name)?;
                dict.set_item("refuri", refuri)?;
            }
            NodeKind::Target { ids, names, refuri } => {
                dict.set_item("ids", ids)?;
                dict.set_item("names", names)?;
                dict.set_item("refuri", refuri)?;
            }
            _ => {}
        }
        Ok(dict)
    }

    /// Child nodes in document order.
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

/// Parse rST `source` and return a `Doctree`.
#[pyfunction(name = "parse_rst", signature = (source, source_path = "<string>"))]
pub fn py_parse_rst(source: &str, source_path: &str) -> PyDoctree {
    PyDoctree {
        inner: parse_rst_with_source(source, source_path),
    }
}
