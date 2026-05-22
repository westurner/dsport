"""Byte-parity tests for ``pygmentsrs``'s native ``JsonLexer``.

Asserts that the native Rust lexer's `(ttype, value)` stream matches
vendored ``pygments.lexers.data.JsonLexer().get_tokens_unprocessed``
exactly. Mirrors the strategy used by the ``code_block_python_*``
fixtures in ``tests/test_parity_pseudoxml.py``.
"""

from __future__ import annotations

import pytest

pygments_lex = pytest.importorskip("pygments.lexers.data")
import pygmentsrs


def _upstream(src: str) -> list[tuple[str, str]]:
    lx = pygments_lex.JsonLexer()
    out: list[tuple[str, str]] = []
    for _idx, ttype, val in lx.get_tokens_unprocessed(src):
        out.append((repr(ttype), val))
    return out


def _native(src: str) -> list[tuple[str, str]]:
    pairs = pygmentsrs.lex("json", src, backend="rust")
    assert pairs is not None, "json should be a native pygmentsrs alias"
    return [(repr_t, val) for repr_t, val in pairs]


def _assert_parity(src: str) -> None:
    assert _native(src) == _upstream(src), src


JSON_FIXTURES = {
    "empty_object": "{}",
    "empty_array": "[]",
    "simple_kv": '{"name": "alice"}',
    "nested": '{"a": {"b": [1, 2, 3]}}',
    "constants": "[true, false, null]",
    "numbers": "[1, -2, 3.14, -4.5e10, 6E-7]",
    "string_with_escapes": r'{"k": "line\n\tnext\u00ff end"}',
    "ws_around": "  {\n  \"k\" : 1\n}  ",
    "line_comment": "// hi\n{\"k\": 1}",
    "block_comment": "/* multi\n   line */\n[1]",
}


@pytest.mark.parametrize("name", sorted(JSON_FIXTURES))
def test_json_lexer_byte_parity(name: str) -> None:
    _assert_parity(JSON_FIXTURES[name])


def test_json_aliases_are_advertised() -> None:
    aliases = pygmentsrs.native_aliases()
    assert "json" in aliases
    assert "json-object" in aliases


def test_json_lexer_routes_through_auto_backend() -> None:
    auto = pygmentsrs.lex("json", '{"a": 1}', backend="auto")
    rust = pygmentsrs.lex("json", '{"a": 1}', backend="rust")
    assert auto == rust
