"""Smoke tests for the minimal HTML5 writer exposed by docutilsrs.

This writer is intentionally not parity-gated against vendored docutils; see
docs/compat.md (`HTML5 writer (minimal)`). These tests pin the fragment shape
we currently emit so accidental regressions are caught.
"""

from __future__ import annotations

import docutilsrs


def html(src: str) -> str:
    return docutilsrs.parse_to_html5(src)


def test_paragraph():
    assert html("Hello world.") == "<p>Hello world.</p>"


def test_inline_emphasis_strong_literal():
    out = html("A *b* **c** ``d``.")
    assert out == "<p>A <em>b</em> <strong>c</strong> <code>d</code>.</p>"


def test_bullet_list():
    out = html("- one\n- two\n")
    assert out == "<ul><li><p>one</p></li><li><p>two</p></li></ul>"


def test_section_with_title():
    src = "Title\n=====\n\nBody.\n"
    out = html(src)
    assert "<h1>Title</h1>" in out
    assert "<p>Body.</p>" in out


def test_literal_block():
    src = "Intro::\n\n    code line\n"
    out = html(src)
    assert "<pre" in out and "code line" in out


def test_definition_list():
    src = "term\n    definition\n"
    out = html(src)
    assert "<dl>" in out and "<dt>term</dt>" in out and "<dd>" in out


def test_reference():
    src = "See `link`_.\n\n.. _link: https://example.com/\n"
    out = html(src)
    assert '<a href="https://example.com/">link</a>' in out


def test_image():
    src = ".. image:: pic.png\n   :alt: a picture\n"
    out = html(src)
    assert '<img' in out and 'src="pic.png"' in out and 'alt="a picture"' in out


def test_html_escaping():
    out = html("a < b & c > d")
    assert out == "<p>a &lt; b &amp; c &gt; d</p>"


def test_table_basic():
    src = (
        "===  ===\n"
        "A    B\n"
        "===  ===\n"
        "1    2\n"
        "===  ===\n"
    )
    out = html(src)
    assert "<table>" in out and "<tr>" in out and "<td>" in out
