//! Tests for `docutilsrs::python`'s `PyDoctree`/`PyNode::findall` bridge —
//! the Python-visible `.findall(condition=None, include_self=True,
//! descend=True, siblings=False, ascend=False)` method mirroring
//! `docutils.nodes.Node.findall`.
//!
//! Added to close the "no Python-visible doctree/Node bridge" gap: real
//! `doctree-read` listeners (`sphinx.ext.viewcode`/`linkcode`) call
//! `doctree.findall(addnodes.desc)`, which previously crashed with
//! `AttributeError: 'str' object has no attribute 'findall'` because the
//! native `doctree-read` event only ever passed the docname string. See
//! `sphinxdocrs/tests/events_app.rs` for the end-to-end regression test
//! that exercises this through a real `SphinxApp::build()`.

use docutilsrs::parse_rst_with_source;
use docutilsrs::python::PyDoctree;
use pyo3::prelude::*;
use pyo3::types::PyDict;

fn sample_doctree() -> PyDoctree {
    let tree = parse_rst_with_source(
        "Title\n=====\n\nFirst paragraph.\n\nSecond paragraph.\n",
        "<test>",
    );
    PyDoctree::new(tree)
}

#[test]
fn findall_default_condition_includes_document_and_every_paragraph() {
    Python::attach(|py| {
        let doctree = Py::new(py, sample_doctree()).unwrap();
        let globals = PyDict::new(py);
        globals.set_item("doctree", &doctree).unwrap();

        let tags: Vec<String> = py
            .eval(c"[n.tag for n in doctree.findall()]", Some(&globals), None)
            .unwrap()
            .extract()
            .unwrap();

        assert!(tags.contains(&"document".to_string()), "tags: {tags:?}");
        assert_eq!(
            tags.iter().filter(|t| *t == "paragraph").count(),
            2,
            "tags: {tags:?}"
        );
    });
}

#[test]
fn findall_unmodeled_node_class_returns_empty_not_error() {
    // Mirrors `sphinx.ext.viewcode`/`linkcode`'s
    // `doctree.findall(addnodes.desc)`: this port has no "desc"-tagged
    // node kind at all (domain objects are text-scanned, not represented
    // as real doctree nodes), so a class matching by name/tagname must
    // simply find nothing — not raise.
    Python::attach(|py| {
        let doctree = Py::new(py, sample_doctree()).unwrap();
        let globals = PyDict::new(py);
        globals.set_item("doctree", &doctree).unwrap();
        py.run(c"class desc:\n    pass\n", None, Some(&globals))
            .unwrap();

        let matches: Vec<Py<PyAny>> = py
            .eval(c"list(doctree.findall(desc))", Some(&globals), None)
            .unwrap()
            .extract()
            .unwrap();
        assert!(matches.is_empty());
    });
}

#[test]
fn findall_class_condition_matches_by_tagname_attribute() {
    // Real `docutils.nodes` classes carry a `tagname` class attribute
    // (e.g. `nodes.paragraph.tagname == "paragraph"`); since our nodes are
    // never real instances of the caller's class, `isinstance` can't be
    // used, so matching falls back to comparing `tagname`/`__name__`
    // against this port's own node tag.
    Python::attach(|py| {
        let doctree = Py::new(py, sample_doctree()).unwrap();
        let globals = PyDict::new(py);
        globals.set_item("doctree", &doctree).unwrap();
        py.run(
            c"class FakeParagraph:\n    tagname = 'paragraph'\n",
            None,
            Some(&globals),
        )
        .unwrap();

        let count: usize = py
            .eval(
                c"len(list(doctree.findall(FakeParagraph)))",
                Some(&globals),
                None,
            )
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(count, 2);
    });
}

#[test]
fn findall_supports_tuple_condition_and_callable_predicate() {
    Python::attach(|py| {
        let doctree = Py::new(py, sample_doctree()).unwrap();
        let globals = PyDict::new(py);
        globals.set_item("doctree", &doctree).unwrap();
        py.run(
            c"class FakeTitle:\n    tagname = 'title'\nclass FakeParagraph:\n    tagname = 'paragraph'\n",
            None,
            Some(&globals),
        )
        .unwrap();

        let tuple_count: usize = py
            .eval(
                c"len(list(doctree.findall((FakeTitle, FakeParagraph))))",
                Some(&globals),
                None,
            )
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(tuple_count, 3, "1 title + 2 paragraphs");

        let all_count: usize = py
            .eval(
                c"len(list(doctree.findall(lambda n: True)))",
                Some(&globals),
                None,
            )
            .unwrap()
            .extract()
            .unwrap();
        assert!(
            all_count > tuple_count,
            "a callable predicate matching everything should see more nodes \
             than the title/paragraph-only tuple match"
        );
    });
}

#[test]
fn findall_include_self_false_excludes_the_starting_node() {
    Python::attach(|py| {
        let doctree = Py::new(py, sample_doctree()).unwrap();
        let globals = PyDict::new(py);
        globals.set_item("doctree", &doctree).unwrap();

        let root_included: bool = py
            .eval(
                c"any(n.tag == 'document' for n in doctree.root.findall())",
                Some(&globals),
                None,
            )
            .unwrap()
            .extract()
            .unwrap();
        assert!(root_included);

        let root_excluded: bool = py
            .eval(
                c"any(n.tag == 'document' for n in doctree.root.findall(include_self=False))",
                Some(&globals),
                None,
            )
            .unwrap()
            .extract()
            .unwrap();
        assert!(!root_excluded);
    });
}
