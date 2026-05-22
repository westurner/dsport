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
    "single_paragraph": "Hello world.",
    "two_paragraphs": "First.\n\nSecond.",
    "three_paragraphs": "One.\n\nTwo.\n\nThree.",
    "leading_blank_lines": "\n\nText.",
    "trailing_blank_lines": "Text.\n\n",
    "emphasis_only": "An *italic* word.",
    "strong_only": "A **bold** word.",
    "literal_only": "A ``literal`` word.",
    "all_three_inline": "A *star* and **strong** and ``lit``.",
    "strong_then_emphasis": "**bold** and *italic*",
    "nested_emphasis_in_strong": "**bold and *italic* inside**",
    "literal_protects_markup": "``*not emphasis*``",
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
