"""Byte-parity tests for ``pygmentsrs``'s native ``DiffLexer``.

Asserts that the native Rust lexer's ``(ttype, value)`` stream matches
vendored ``pygments.lexers.diff.DiffLexer().get_tokens_unprocessed``
exactly.
"""

from __future__ import annotations

import pytest

pygments_diff = pytest.importorskip("pygments.lexers.diff")
import pygmentsrs


def _upstream(src: str) -> list[tuple[str, str]]:
    lx = pygments_diff.DiffLexer()
    return [(repr(t), v) for _, t, v in lx.get_tokens_unprocessed(src)]


def _native(src: str) -> list[tuple[str, str]]:
    pairs = pygmentsrs.lex("diff", src, backend="rust")
    assert pairs is not None
    return [(repr_t, val) for repr_t, val in pairs]


DIFF_FIXTURES = {
    "simple_unified": (
        "--- a/foo.txt\n"
        "+++ b/foo.txt\n"
        "@@ -1,3 +1,3 @@\n"
        " context\n"
        "-old line\n"
        "+new line\n"
        " trailing\n"
    ),
    "index_heading": "Index: foo.py\n===================================================================\n--- a/foo.py\n+++ b/foo.py\n",
    "git_diff_header": "diff --git a/x b/x\n--- a/x\n+++ b/x\n",
    "ed_style_hunk": "1,3c1,3\n< old1\n< old2\n---\n> new1\n> new2\n",
    "bang_strong": "!some text\n",
    "trailing_context": " just context\n",
}


@pytest.mark.parametrize("name", sorted(DIFF_FIXTURES))
def test_diff_lexer_byte_parity(name: str) -> None:
    src = DIFF_FIXTURES[name]
    assert _native(src) == _upstream(src), name


def test_diff_aliases() -> None:
    aliases = pygmentsrs.native_aliases()
    assert "diff" in aliases
    assert "udiff" in aliases
