//! ADR 0006 (`docs/adr/0006-extension-nodes.md`) — `NodeKind::Extension`
//! and its per-`(class_name, builder_format)` visit/depart registry in
//! `docutilsrs::plugins`.
//!
//! Exercises the representation/dispatch decision directly (no
//! `sphinxdocrs`/`Sphinx.add_node` facade involved): register a Python
//! visit/depart pair for a made-up node class under a specific builder
//! format, build a tiny `Doctree` containing an `Extension` node of that
//! class, and confirm each writer either uses the registered pair (for
//! the format it was registered under) or falls back to rendering
//! children only (for every other format/writer) — matching upstream's
//! base-class/`unknown_visit` fallback behavior.

use std::collections::HashMap;

use docutilsrs::doctree::{Doctree, NodeKind};
use docutilsrs::plugins::register_node_visitor;
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Build `<test> -> Extension(class_name) -> Text("body")`.
fn tree_with_extension(class_name: &str) -> Doctree {
    let mut tree = Doctree::new_document("<test>");
    let root = tree.root();
    let ext = tree.append(
        root,
        NodeKind::Extension {
            class_name: class_name.to_string(),
            attrs: HashMap::new(),
        },
    );
    tree.append(ext, NodeKind::Text("body".to_string()));
    tree
}

/// A Python callable (`attrs: dict -> str`) built from a literal.
fn py_lambda(py: Python<'_>, returning: &str) -> Py<PyAny> {
    let globals = PyDict::new(py);
    globals
        .set_item("_RET", returning)
        .unwrap();
    py.eval(
        c"lambda attrs: _RET",
        Some(&globals),
        None,
    )
    .unwrap()
    .unbind()
}

#[test]
fn html5_uses_registered_visit_depart() {
    let class_name = "ExtHtmlOnly";
    Python::attach(|py| {
        register_node_visitor(
            class_name.to_string(),
            "html".to_string(),
            py_lambda(py, "<div class=\"x\">"),
            Some(py_lambda(py, "</div>")),
        );
    });
    let tree = tree_with_extension(class_name);
    let html = docutilsrs::html5(
        &tree,
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert_eq!(html, "<div class=\"x\">body</div>");
}

#[test]
fn html5_falls_back_to_children_only_when_unregistered() {
    let tree = tree_with_extension("ExtNeverRegistered");
    let html = docutilsrs::html5(
        &tree,
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert_eq!(html, "body");
}

#[test]
fn latex_uses_registered_visit_depart_independently_of_html() {
    let class_name = "ExtLatexOnly";
    Python::attach(|py| {
        register_node_visitor(
            class_name.to_string(),
            "latex".to_string(),
            py_lambda(py, "\\begin{todo}"),
            Some(py_lambda(py, "\\end{todo}")),
        );
    });
    let tree = tree_with_extension(class_name);

    let latex = docutilsrs::latex(
        &tree,
        &docutilsrs::cli::LatexOptions::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert!(
        latex.contains("\\begin{todo}body\\end{todo}"),
        "latex output missing registered visit/depart wrapper:\n{latex}"
    );

    // Same class has no "html" registration, so the HTML writer must
    // still gracefully fall back to children-only.
    let html = docutilsrs::html5(
        &tree,
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert_eq!(html, "body");
}

#[test]
fn visit_without_depart_renders_open_text_only() {
    let class_name = "ExtVisitOnly";
    Python::attach(|py| {
        register_node_visitor(
            class_name.to_string(),
            "html".to_string(),
            py_lambda(py, "[[marker]]"),
            None,
        );
    });
    let tree = tree_with_extension(class_name);
    let html = docutilsrs::html5(
        &tree,
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    );
    assert_eq!(html, "[[marker]]body");
}

#[test]
fn pseudo_xml_renders_generic_tag_regardless_of_registry() {
    let tree = tree_with_extension("ExtPseudoXml");
    let out = docutilsrs::pseudo_xml(&tree);
    assert!(out.contains("<ExtPseudoXml>"), "pseudo-xml output:\n{out}");
}

#[test]
fn xml_writer_renders_generic_closed_tag_regardless_of_registry() {
    let tree = tree_with_extension("ExtXml");
    let out = docutilsrs::to_xml(&tree);
    assert!(out.contains("<ExtXml"), "xml output:\n{out}");
    assert!(out.contains("</ExtXml>"), "xml output:\n{out}");
}

#[test]
fn doctree_serialization_round_trips_extension_node() {
    let mut attrs = HashMap::new();
    attrs.insert("foo".to_string(), "bar".to_string());
    let mut tree = Doctree::new_document("<test>");
    let root = tree.root();
    tree.append(
        root,
        NodeKind::Extension {
            class_name: "ExtRoundTrip".to_string(),
            attrs: attrs.clone(),
        },
    );
    let bytes = tree.to_bytes();
    let restored = Doctree::from_bytes(&bytes).unwrap();
    let child_id = restored.node(restored.root()).children[0];
    match &restored.node(child_id).kind {
        NodeKind::Extension { class_name, attrs: a } => {
            assert_eq!(class_name, "ExtRoundTrip");
            assert_eq!(a, &attrs);
        }
        other => panic!("expected Extension node, got {other:?}"),
    }
}
