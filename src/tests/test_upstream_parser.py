"""Run a curated subset of upstream rST parser tests against docutilsrs.

These are verbatim ``(input, expected_pformat)`` pairs copied from
``docutils/test/test_parsers/test_rst/test_*.py`` (the ``totest`` dicts in
each module). Each entry is something the phase-1 slice claims to support
exactly. Cases that exercise unported grammar (continuation lines that
remain in the same paragraph, system messages, sections, etc.) are not
included here.
"""

from __future__ import annotations

import pytest

import docutilsrs


def _pf(src: str) -> str:
    """Match upstream's ``document.pformat()`` (no trailing-newline strip)."""
    return docutilsrs.parse_to_pseudoxml(src, "test data")


# ── paragraphs ───────────────────────────────────────────────────────────────
PARAGRAPH_CASES: list[tuple[str, str]] = [
    (
        "A paragraph.\n",
        '<document source="test data">\n'
        "    <paragraph>\n"
        "        A paragraph.\n",
    ),
    (
        "Paragraph 1.\n\nParagraph 2.\n",
        '<document source="test data">\n'
        "    <paragraph>\n"
        "        Paragraph 1.\n"
        "    <paragraph>\n"
        "        Paragraph 2.\n",
    ),
    (
        "Line 1.\nLine 2.\nLine 3.\n",
        '<document source="test data">\n'
        "    <paragraph>\n"
        "        Line 1.\n"
        "        Line 2.\n"
        "        Line 3.\n",
    ),
    (
        "Paragraph 1, Line 1.\nLine 2.\nLine 3.\n\n"
        "Paragraph 2, Line 1.\nLine 2.\nLine 3.\n",
        '<document source="test data">\n'
        "    <paragraph>\n"
        "        Paragraph 1, Line 1.\n"
        "        Line 2.\n"
        "        Line 3.\n"
        "    <paragraph>\n"
        "        Paragraph 2, Line 1.\n"
        "        Line 2.\n"
        "        Line 3.\n",
    ),
]


@pytest.mark.parametrize("src,expected", PARAGRAPH_CASES)
def test_upstream_paragraphs(src: str, expected: str) -> None:
    assert _pf(src) == expected


# ── bullet lists ─────────────────────────────────────────────────────────────
BULLET_CASES: list[tuple[str, str]] = [
    (
        "- item\n",
        '<document source="test data">\n'
        '    <bullet_list bullet="-">\n'
        "        <list_item>\n"
        "            <paragraph>\n"
        "                item\n",
    ),
    (
        "- item 1\n- item 2\n- item 3\n",
        '<document source="test data">\n'
        '    <bullet_list bullet="-">\n'
        "        <list_item>\n"
        "            <paragraph>\n"
        "                item 1\n"
        "        <list_item>\n"
        "            <paragraph>\n"
        "                item 2\n"
        "        <list_item>\n"
        "            <paragraph>\n"
        "                item 3\n",
    ),
    (
        "- item 1, para 1.\n  Line 2.\n\n- item 2\n",
        # Continuation lines join the paragraph; blank-line-separated bullets
        # still belong to the same list.
        '<document source="test data">\n'
        '    <bullet_list bullet="-">\n'
        "        <list_item>\n"
        "            <paragraph>\n"
        "                item 1, para 1.\n"
        "                Line 2.\n"
        "        <list_item>\n"
        "            <paragraph>\n"
        "                item 2\n",
    ),
]


@pytest.mark.parametrize("src,expected", BULLET_CASES)
def test_upstream_bullet_lists(src: str, expected: str) -> None:
    assert _pf(src) == expected


# ── targets / references ─────────────────────────────────────────────────────
TARGET_CASES: list[tuple[str, str]] = [
    (
        "ref_\n\n.. _ref: http://example.com\n",
        '<document source="test data">\n'
        "    <paragraph>\n"
        '        <reference name="ref" refuri="http://example.com">\n'
        "            ref\n"
        '    <target ids="ref" names="ref" refuri="http://example.com">\n',
    ),
]


@pytest.mark.parametrize("src,expected", TARGET_CASES)
def test_upstream_targets(src: str, expected: str) -> None:
    assert _pf(src) == expected
