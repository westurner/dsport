"""Tests for the minimal Rust ODT writer (`docutilsrs.parse_to_odt`).

These tests mirror the structure of the upstream
`test/test_writers/test_odt.py` suite (odt_basic, odt_tables1,
odt_literal_block, odt_contents, odt_footnotes, odt_raw, odt_image,
odt_nested_class). The Rust writer does NOT aim for byte-parity with
`docutils.writers.odf_odt` (see ``docs/compat.md``); these tests
verify that the container is structurally valid and that the expected
ODF element appears for each supported feature.
"""

from __future__ import annotations

import io
import zipfile
import xml.etree.ElementTree as ET

import pytest

import docutilsrs


NS = {
    "office": "urn:oasis:names:tc:opendocument:xmlns:office:1.0",
    "text": "urn:oasis:names:tc:opendocument:xmlns:text:1.0",
    "table": "urn:oasis:names:tc:opendocument:xmlns:table:1.0",
    "xlink": "http://www.w3.org/1999/xlink",
    "draw": "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0",
    "style": "urn:oasis:names:tc:opendocument:xmlns:style:1.0",
    "manifest": "urn:oasis:names:tc:opendocument:xmlns:manifest:1.0",
}


def _odt(src: str) -> zipfile.ZipFile:
    data = docutilsrs.parse_to_odt(src)
    assert isinstance(data, (bytes, bytearray))
    return zipfile.ZipFile(io.BytesIO(data))


def _content(src: str) -> ET.Element:
    return ET.fromstring(_odt(src).read("content.xml"))


# ── Container structure ─────────────────────────────────────────────────


def test_odt_basic_zip_structure() -> None:
    z = _odt("Hello\n\nWorld.\n")
    names = z.namelist()
    assert names[0] == "mimetype", "mimetype must be the first zip entry"
    assert "META-INF/manifest.xml" in names
    assert "content.xml" in names
    assert "styles.xml" in names
    assert z.read("mimetype") == b"application/vnd.oasis.opendocument.text"


def test_odt_manifest_is_well_formed_xml() -> None:
    z = _odt("Hi\n")
    root = ET.fromstring(z.read("META-INF/manifest.xml"))
    entries = root.findall("manifest:file-entry", NS)
    paths = {e.get(f"{{{NS['manifest']}}}full-path") for e in entries}
    assert "/" in paths
    assert "content.xml" in paths
    assert "styles.xml" in paths


def test_odt_styles_is_well_formed_xml() -> None:
    z = _odt("Hi\n")
    root = ET.fromstring(z.read("styles.xml"))
    styles = root.findall(".//style:style", NS)
    names = {s.get(f"{{{NS['style']}}}name") for s in styles}
    # A small set of named styles must be present.
    for required in (
        "Standard",
        "Heading_20_1",
        "Heading_20_2",
        "Title",
        "Emphasis",
        "Strong_20_Emphasis",
        "Source_20_Text",
        "Preformatted_20_Text",
        "Quotations",
        "Internet_20_link",
    ):
        assert required in names, f"missing style {required!r}"


def test_odt_content_is_well_formed_xml() -> None:
    root = _content("paragraph one\n\nparagraph two\n")
    paras = root.findall(".//text:p", NS)
    assert len(paras) == 2


# ── Block-level features (mirrors upstream odt_basic) ───────────────────


def test_odt_basic_title_and_section_heading() -> None:
    root = _content("Hello\n=====\n\nA paragraph.\n")
    titles = [p for p in root.findall(".//text:p", NS)
              if p.get(f"{{{NS['text']}}}style-name") == "Title"]
    assert titles, "expected a Title paragraph"
    assert titles[0].text == "Hello"
    headings = root.findall(".//text:h", NS)
    assert headings, "expected at least one heading"
    assert headings[0].get(f"{{{NS['text']}}}outline-level") == "1"


def test_odt_basic_inline_emphasis_strong_literal() -> None:
    root = _content("This *is* **strong** and ``code``.\n")
    spans = root.findall(".//text:span", NS)
    styles = {s.get(f"{{{NS['text']}}}style-name") for s in spans}
    assert "Emphasis" in styles
    assert "Strong_20_Emphasis" in styles
    assert "Source_20_Text" in styles


def test_odt_bullet_list_renders_items() -> None:
    root = _content("* one\n* two\n* three\n")
    lists = root.findall(".//text:list", NS)
    assert lists, "expected a text:list"
    items = root.findall(".//text:list-item", NS)
    assert len(items) == 3


def test_odt_enumerated_list_renders_items() -> None:
    root = _content("1. one\n2. two\n")
    items = root.findall(".//text:list-item", NS)
    assert len(items) == 2


def test_odt_block_quote_renders_quotation_style() -> None:
    root = _content("Intro.\n\n    quoted text\n")
    quotes = [p for p in root.findall(".//text:p", NS)
              if p.get(f"{{{NS['text']}}}style-name") == "Quotations"]
    assert quotes, "expected a Quotations paragraph"


# ── Literal block (mirrors upstream odt_literal_block) ───────────────────


def test_odt_literal_block_uses_preformatted_style() -> None:
    src = "Some intro::\n\n    line one\n    line two\n"
    root = _content(src)
    pres = [p for p in root.findall(".//text:p", NS)
            if p.get(f"{{{NS['text']}}}style-name") == "Preformatted_20_Text"]
    assert pres, "expected at least one preformatted paragraph"


# ── Tables (mirrors upstream odt_tables1) ────────────────────────────────


def test_odt_grid_table_emits_table_cells() -> None:
    src = (
        "+----+----+\n"
        "| A  | B  |\n"
        "+----+----+\n"
        "| 1  | 2  |\n"
        "+----+----+\n"
    )
    root = _content(src)
    tables = root.findall(".//table:table", NS)
    assert tables, "expected a table:table"
    cols = root.findall(".//table:table-column", NS)
    assert len(cols) == 2
    cells = root.findall(".//table:table-cell", NS)
    assert len(cells) == 4


def test_odt_grid_table_colspan_emits_spanned_attribute() -> None:
    src = (
        "+----+----+\n"
        "| a  | b  |\n"
        "+----+----+\n"
        "| c       |\n"
        "+----+----+\n"
    )
    root = _content(src)
    spanned = [c for c in root.findall(".//table:table-cell", NS)
               if c.get(f"{{{NS['table']}}}number-columns-spanned") == "2"]
    assert spanned, "expected a table-cell with number-columns-spanned=2"


def test_odt_grid_table_rowspan_emits_spanned_attribute() -> None:
    src = (
        "+----+----+\n"
        "| a  | b  |\n"
        "+    +----+\n"
        "|    | c  |\n"
        "+----+----+\n"
    )
    root = _content(src)
    spanned = [c for c in root.findall(".//table:table-cell", NS)
               if c.get(f"{{{NS['table']}}}number-rows-spanned") == "2"]
    assert spanned, "expected a table-cell with number-rows-spanned=2"


# ── Image (mirrors upstream odt_image) ───────────────────────────────────


def test_odt_image_directive_emits_draw_image() -> None:
    src = ".. image:: pic.png\n   :alt: A picture\n"
    root = _content(src)
    images = root.findall(".//draw:image", NS)
    assert images, "expected a draw:image element"
    assert images[0].get(f"{{{NS['xlink']}}}href") == "pic.png"


# ── Raw passthrough (mirrors upstream odt_raw) ───────────────────────────


def test_odt_raw_directive_with_odt_format_passes_through() -> None:
    src = (
        ".. raw:: odt\n\n"
        "   <text:p text:style-name=\"Standard\">RAW</text:p>\n"
    )
    root = _content(src)
    paras = [p for p in root.findall(".//text:p", NS) if (p.text or "").strip() == "RAW"]
    assert paras, "expected raw:: odt to pass through into the body"


def test_odt_raw_directive_with_other_format_is_dropped() -> None:
    src = (
        ".. raw:: html\n\n"
        "   <p>SHOULD_NOT_APPEAR</p>\n"
    )
    data = docutilsrs.parse_to_odt(src)
    assert b"SHOULD_NOT_APPEAR" not in data


# ── Reference / Internet link ────────────────────────────────────────────


def test_odt_external_link_emits_text_a() -> None:
    src = "See `the python site <https://www.python.org>`_ for details.\n"
    root = _content(src)
    anchors = root.findall(".//text:a", NS)
    assert anchors, "expected a text:a element"
    assert anchors[0].get(f"{{{NS['xlink']}}}href") == "https://www.python.org"


# ── Definition list / field list ────────────────────────────────────────


def test_odt_definition_list_renders_term_and_definition() -> None:
    src = "term\n  definition body\n"
    root = _content(src)
    # Term renders as a Standard paragraph with a Strong span.
    spans = [s for s in root.findall(".//text:span", NS)
             if s.get(f"{{{NS['text']}}}style-name") == "Strong_20_Emphasis"]
    assert spans, "expected a strong span (term)"


def test_odt_field_list_renders_each_field() -> None:
    src = ":Author: A Name\n:Date: 2020-01-01\n"
    root = _content(src)
    spans = [s for s in root.findall(".//text:span", NS)
             if s.get(f"{{{NS['text']}}}style-name") == "Strong_20_Emphasis"]
    assert len(spans) >= 2


# ── Feature-flag advertises the new writer ──────────────────────────────


def test_features_advertises_odt_writer() -> None:
    feats = docutilsrs.features()
    assert "writer:odt" in feats
    assert docutilsrs.supports("writer:odt")
