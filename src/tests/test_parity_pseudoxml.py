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
    # enumerated lists
    "enum_arabic_period": "1. one\n2. two\n3. three",
    "enum_loweralpha": "a. one\nb. two",
    "enum_upperalpha_paren": "A) first\nB) second",
    "enum_paren_arabic": "(1) first\n(2) second",
    "enum_lowerroman": "i. roman\nii. two",
    "enum_upperroman": "I. one\nII. two\nIII. three",
    "enum_auto_hash": "#. auto\n#. items",
    "enum_arabic_start": "3. three\n4. four",
    "enum_loweralpha_start": "c. just c",
    "enum_continuation": "1. one\n   continued\n2. two",
    "enum_between_paragraphs": "Before.\n\n1. a\n2. b\n\nAfter.",
    # phase 2: sections / transitions
    "section_simple": "Title\n=====\n\nBody.",
    "section_two_levels": "Top\n===\n\nIntro.\n\nSub\n---\n\nMore.",
    "section_three_levels": "A\n=\n\nB\n-\n\nC\n~\n\nbody",
    "doc_title_and_subtitle": "Title\n=====\n\nSubtitle\n--------\n\nBody.",
    "transition_simple": "Para one.\n\n----\n\nPara two.",
    # phase 2: block quote
    "block_quote": "Intro:\n\n   quoted text\n\nAfter.",
    # phase 2: literal blocks
    "literal_block_expanded": "Intro::\n\n   code line\n   code two\n\nAfter.",
    "literal_block_partial": "Intro ::\n\n   code\n",
    "literal_block_quoted": "::\n\n   code line\n",
    # phase 2: definition lists
    "def_list_simple": "term\n    definition.",
    "def_list_two": "term1\n    def 1\nterm2\n    def 2",
    "def_list_classifier": "term : kind\n    def text",
    # phase 2: field list / docinfo
    "field_list_simple": ":Author: A. N. Other\n:Date: 2024-01-01",
    "docinfo_after_title": "Title\n=====\n\n:Author: Me\n:Date: 2024-01-01\n\nBody.",
    # phase 2: explicit blocks
    "comment": ".. this is a comment\n",
    "note": ".. note::\n\n   A note.",
    "warning": ".. warning::\n\n   A warning.",
    "image": ".. image:: foo.png\n",
    "image_with_alt": ".. image:: foo.png\n   :alt: an image\n   :width: 100\n",
    "code_no_lang": ".. code::\n\n   x = 1\n",
    "raw": ".. raw:: html\n\n   <hr/>\n",
    # phase 2: inline roles
    "role_emphasis": ":emphasis:`hi`",
    "role_literal": ":literal:`hi`",
    "role_title_ref": ":title:`hi`",
    # phase 2: substitutions
    "substitution": "Use |x| here.\n\n.. |x| replace:: replacement text",
    # phase 2: tables
    "simple_table": "=== ===\n A   B\n=== ===\n 1   2\n=== ===\n",
    "grid_table": "+---+---+\n| A | B |\n+===+===+\n| 1 | 2 |\n+---+---+\n",
    # phase 2: phrase references
    "phrase_ref": "See `Some Title`_.\n\n.. _Some Title: http://example.com",
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
