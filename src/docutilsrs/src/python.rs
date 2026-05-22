//! PyO3 bindings for the doctree.

#![allow(clippy::collapsible_match)]

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
}

#[pyclass(name = "Node", module = "docutilsrs")]
pub struct PyNode {
    tree: Py<PyDoctree>,
    id: NodeId,
}

#[pymethods]
impl PyNode {
    #[getter]
    fn tag(&self, py: Python<'_>) -> String {
        match &self.tree.borrow(py).inner.node(self.id).kind {
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
        }
    }

    #[getter]
    fn text(&self, py: Python<'_>) -> Option<String> {
        match &self.tree.borrow(py).inner.node(self.id).kind {
            NodeKind::Text(s) => Some(s.clone()),
            _ => None,
        }
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
            } => {
                if *anonymous {
                    dict.set_item("anonymous", "1")?;
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
