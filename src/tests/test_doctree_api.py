"""Tests for the PyO3 `Doctree` / `Node` surface."""

from __future__ import annotations

import docutilsrs


def test_root_node_shape() -> None:
    d = docutilsrs.parse_rst("Hello *world*.")
    assert d.root.tag == "document"
    assert d.root.attributes == {"source": "<string>"}
    [para] = d.root.children
    assert para.tag == "paragraph"
    assert para.attributes == {}
    tags = [c.tag for c in para.children]
    assert tags == ["#text", "emphasis", "#text"]
    assert para.children[0].text == "Hello "
    assert para.children[1].children[0].text == "world"
    assert para.children[2].text == "."


def test_source_path_overrides_document_source() -> None:
    d = docutilsrs.parse_rst("x", "my/path.rst")
    assert d.root.attributes == {"source": "my/path.rst"}


def test_bullet_list_attributes() -> None:
    d = docutilsrs.parse_rst("- a\n- b")
    [bl] = d.root.children
    assert bl.tag == "bullet_list"
    assert bl.attributes == {"bullet": "-"}
    assert [li.tag for li in bl.children] == ["list_item", "list_item"]


def test_reference_resolves_via_target() -> None:
    d = docutilsrs.parse_rst("See ref_.\n\n.. _ref: http://example.com")
    [para, tgt] = d.root.children
    ref = para.children[1]
    assert ref.tag == "reference"
    assert ref.attributes == {"name": "ref", "refuri": "http://example.com"}
    assert tgt.tag == "target"
    assert tgt.attributes == {
        "ids": "ref",
        "names": "ref",
        "refuri": "http://example.com",
    }


def test_pformat_matches_parse_to_pseudoxml() -> None:
    src = "Hello *world*.\n\n- one\n- two"
    assert docutilsrs.parse_rst(src).pformat() == docutilsrs.parse_to_pseudoxml(src)
