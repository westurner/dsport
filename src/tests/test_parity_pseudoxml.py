"""Byte-for-byte parity of `docutilsrs.parse_to_pseudoxml` against
vendored `docutils` pseudo-XML output, on the phase 1 grammar slice.

This is the primary parity gate for phase 1. Inputs are intentionally narrow:
paragraphs + inline emphasis/strong/literal only. Broader grammar is tracked
in `docs/compat.md` and added incrementally with paired test cases here.
"""

from __future__ import annotations

import pytest

import docutilsrs
from docutils.core import publish_string


CASES = {
    # paragraphs
    "single_paragraph": "Hello world.",
    "two_paragraphs": "First.\n\nSecond.",
    "three_paragraphs": "One.\n\nTwo.\n\nThree.",
    "leading_blank_lines": "\n\nText.",
    "trailing_blank_lines": "Text.\n\n",
    # inline markup
    "emphasis_only": "An *italic* word.",
    "strong_only": "A **bold** word.",
    "literal_only": "A ``literal`` word.",
    "all_three_inline": "A *star* and **strong** and ``lit``.",
    "strong_then_emphasis": "**bold** and *italic*",
    "nested_emphasis_in_strong": "**bold and *italic* inside**",
    "literal_protects_markup": "``*not emphasis*``",
    # bullet lists
    "bullet_hyphen": "- one\n- two\n- three",
    "bullet_asterisk": "* alpha\n* beta",
    "bullet_plus": "+ x\n+ y",
    "bullet_then_paragraph": "- one\n- two\n\nNext paragraph.",
    "paragraph_then_bullet_then_paragraph": "Before.\n\n- a\n- b\n\nAfter.",
    "bullet_with_inline": "- single item with *emphasis*",
    # escapes
    "escape_star": "A \\*literal star\\* here.",
    "escape_whitespace_collapses": "a \\ b",
    "escape_blocks_emphasis": "\\*not emphasis* here",
    "escape_backtick": "An escaped \\`backtick\\` here.",
    # multi-line paragraphs
    "two_line_paragraph": "line one\nline two",
    "three_line_paragraph_then_para": "line one\nline two\nline three\n\nsecond para\ncontinued",
    # references + explicit targets
    "ref_alone": "ref_\n\n.. _ref: http://example.com",
    "ref_inline": "See ref_ here.\n\n.. _ref: http://example.com",
    "two_refs_two_targets": (
        "A ref_ then more_.\n\n.. _ref: http://r\n.. _more: http://m"
    ),
    "ref_with_inline_around": "**bold** then ref_ here.\n\n.. _ref: http://r",
}


def _docutils_pseudoxml(source: str) -> str:
    return publish_string(source, writer="pseudoxml").decode()


@pytest.mark.parametrize("name, source", list(CASES.items()), ids=list(CASES))
def test_parity(name: str, source: str) -> None:
    expected = _docutils_pseudoxml(source)
    actual = docutilsrs.parse_to_pseudoxml(source)
    assert actual == expected, (
        f"\ncase: {name}\n"
        f"--- expected (docutils) ---\n{expected!r}\n"
        f"--- actual (docutilsrs) ---\n{actual!r}"
    )
