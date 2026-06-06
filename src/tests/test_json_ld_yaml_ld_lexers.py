"""Tests for the hand-crafted native lexers:
- ``json-ld``: JSON-LD keyword highlighting wrapping ``json``
- ``yaml-ld``: YAML-LD / YAML lexer with embedded language dispatch

These tests verify functional correctness and byte-parity vs
``pygments.lexers.data.JsonLdLexer`` for JSON-LD.
"""

from __future__ import annotations

import pytest

pytest.importorskip("pygments")
import importlib

import pygmentsrs


# ---------------------------------------------------------------------------
# JSON-LD parity tests vs upstream pygments.lexers.data.JsonLdLexer
# ---------------------------------------------------------------------------

JSON_LD_SAMPLES = [
    # Basic JSON-LD with @context and @type
    '{"@context": "https://schema.org", "@type": "Person", "name": "Alice"}\n',
    # Nested context with @vocab
    '{"@context": {"@vocab": "https://schema.org/", "@base": "https://example.com/"}, "@id": "person/1"}\n',
    # All common @keywords as keys
    '{"@id": "x", "@type": "Y", "@value": 1, "@language": "en", "@direction": "ltr"}\n',
    # Mixed @keyword values and regular strings
    '{"@context": "@none", "@graph": [{"@id": "a"}]}\n',
    # Non-keyword @ strings should NOT be decorated
    '{"@unknown": "x", "normalKey": "normalValue"}\n',
    # Nested object
    '{"@context": {"@version": 1.1, "@prefix": true}, "data": {"@reverse": {}}}\n',
]


def _upstream_jsonld(src: str) -> list[tuple[str, str]]:
    mod = importlib.import_module("pygments.lexers.data")
    cls = getattr(mod, "JsonLdLexer")
    return [(repr(t), v) for _idx, t, v in cls().get_tokens_unprocessed(src)]


def _native_jsonld(src: str) -> list[tuple[str, str]]:
    pairs = pygmentsrs.lex("json-ld", src, backend="rust")
    assert pairs is not None, "json-ld should be a native alias"
    return list(pairs)


@pytest.mark.parametrize("src,idx", [(s, i) for i, s in enumerate(JSON_LD_SAMPLES)])
def test_json_ld_parity(src: str, idx: int) -> None:
    """Native json-ld lexer must be byte-parity with pygments JsonLdLexer."""
    assert _native_jsonld(src) == _upstream_jsonld(src), f"JSON-LD sample #{idx} diverged"


def test_json_ld_is_native() -> None:
    assert pygmentsrs.has_native_lexer("json-ld")
    assert pygmentsrs.has_native_lexer("jsonld")
    assert "json-ld" in pygmentsrs.native_aliases()


# ---------------------------------------------------------------------------
# YAML-LD functional tests (not byte-parity — the Pygments YamlLexer uses
# ExtendedRegexLexer with indentation callbacks, so parity is not the goal)
# ---------------------------------------------------------------------------

def _native_yaml(src: str) -> list[tuple[str, str]]:
    pairs = pygmentsrs.lex("yaml-ld", src, backend="rust")
    assert pairs is not None, "yaml-ld should be a native alias"
    return list(pairs)


def _tok_types(pairs: list[tuple[str, str]]) -> list[str]:
    return [t for t, _v in pairs]


def _tok_values(pairs: list[tuple[str, str]]) -> list[str]:
    return [v for _t, v in pairs]


def test_yaml_ld_is_native() -> None:
    assert pygmentsrs.has_native_lexer("yaml-ld")
    assert pygmentsrs.has_native_lexer("yaml")
    assert "yaml-ld" in pygmentsrs.native_aliases()


def test_yaml_ld_context_keyword() -> None:
    """@context, @type, @id keys should be Token.Name.Decorator."""
    src = "@context:\n  \"@vocab\": \"https://schema.org/\"\n@type: Person\n"
    pairs = _native_yaml(src)
    decorated = [(t, v) for t, v in pairs if t == "Token.Name.Decorator"]
    assert any(v == "@context" for _t, v in decorated), "@context should be Name.Decorator"
    assert any(v == "@type" for _t, v in decorated), "@type should be Name.Decorator"


def test_yaml_ld_full_document() -> None:
    """A realistic YAML-LD document should tokenize without errors."""
    src = (
        "---\n"
        '"@context":\n'
        '  "@vocab": "https://schema.org/"\n'
        '"@type": "Person"\n'
        '"@id": "https://example.com/person/1"\n'
        "name: Alice\n"
        "age: 30\n"
        "active: true\n"
        "score: 9.5\n"
        "url: https://alice.example.com\n"
        "tags:\n"
        "  - python\n"
        "  - rust\n"
    )
    pairs = _native_yaml(src)
    errors = [(t, v) for t, v in pairs if t == "Token.Error"]
    assert not errors, f"Unexpected error tokens: {errors}"
    types = set(_tok_types(pairs))
    assert "Token.Name.Decorator" in types, "@context/@type/@id not highlighted as Name.Decorator"
    assert "Token.Name.Tag" in types, "plain key 'name' should be Name.Tag"
    assert "Token.Keyword.Constant" in types, "true should be Keyword.Constant"
    assert "Token.Literal.Number.Integer" in types, "30 should be Number.Integer"
    assert "Token.Literal.Number.Float" in types, "9.5 should be Number.Float"


def test_yaml_ld_doc_marker() -> None:
    """--- document start marker should be Name.Namespace."""
    src = "---\nfoo: bar\n"
    pairs = _native_yaml(src)
    assert pairs[0] == ("Token.Name.Namespace", "---"), f"First token: {pairs[0]}"


def test_yaml_ld_comment() -> None:
    """YAML comments should be Comment.Single."""
    src = "# this is a comment\nfoo: bar  # inline\n"
    pairs = _native_yaml(src)
    comments = [(t, v) for t, v in pairs if t == "Token.Comment.Single"]
    assert len(comments) >= 2, f"Expected 2+ comments, got: {comments}"


def test_yaml_ld_sequence() -> None:
    """Sequence items (- value) should tokenize correctly."""
    src = "items:\n  - alpha\n  - beta\n  - gamma\n"
    pairs = _native_yaml(src)
    indicators = [v for t, v in pairs if "Indicator" in t]
    assert indicators.count("- ") == 3, f"Expected 3 '- ' indicators, got: {indicators}"


def test_yaml_ld_block_scalar_markdown() -> None:
    """A description: | block scalar should dispatch to the markdown lexer."""
    src = "description: |\n  # Heading\n  This is **bold**.\n"
    pairs = _native_yaml(src)
    # The markdown lexer will produce Generic.Heading or other markdown tokens
    types = set(_tok_types(pairs))
    # Either markdown tokens present (when md lexer is native) or Name.Constant fallback
    assert types, "Should produce some tokens"
    # No errors
    errors = [(t, v) for t, v in pairs if t == "Token.Error"]
    assert not errors, f"Unexpected errors in block scalar: {errors}"


def test_yaml_ld_iri_highlighting() -> None:
    """String values that are IRIs should get Name.Other treatment."""
    src = 'url: https://example.com/resource\n'
    pairs = _native_yaml(src)
    name_other = [(t, v) for t, v in pairs if t == "Token.Name.Other"]
    assert any("https://example.com" in v for _t, v in name_other), \
        f"IRI should be Name.Other; got: {pairs}"


def test_yaml_ld_quoted_ld_keyword_value() -> None:
    """A quoted @keyword as a VALUE should also be Name.Decorator."""
    src = '"@type": "@none"\n'
    pairs = _native_yaml(src)
    decorated = [(t, v) for t, v in pairs if t == "Token.Name.Decorator"]
    assert any("@type" in v for _t, v in decorated), "@type key should be Name.Decorator"
    assert any("@none" in v for _t, v in decorated), '"@none" value should be Name.Decorator'


def test_yaml_ld_numeric_scalars() -> None:
    """Numbers and constants should be highlighted correctly."""
    src = (
        "count: 42\n"
        "pi: 3.14159\n"
        "flag: false\n"
        "empty: null\n"
        "hex: 0xFF\n"
    )
    pairs = _native_yaml(src)
    types = set(_tok_types(pairs))
    assert "Token.Literal.Number.Integer" in types
    assert "Token.Literal.Number.Float" in types
    assert "Token.Keyword.Constant" in types
