"""Byte-parity tests for ``sphinxdocrs``'s ``util.console`` port.

Compares the Rust functions exposed on the ``sphinxdocrs`` module
(`colourise`, `strip_escape_sequences`, `terminal_safe`,
`disable_colour`, `enable_colour`) against the vendored
``sphinx._cli.util.colour`` + ``sphinx._cli.util.errors`` originals.
"""

from __future__ import annotations

import pytest

sphinx_colour = pytest.importorskip("sphinx._cli.util.colour")
sphinx_errors = pytest.importorskip("sphinx._cli.util.errors")
import sphinxdocrs


COLOUR_NAMES = [
    "reset", "bold", "faint", "standout", "underline", "blink",
    "black", "darkred", "darkgreen", "brown", "darkblue", "purple",
    "turquoise", "lightgray", "darkgray", "red", "green", "yellow",
    "blue", "fuchsia", "teal", "white",
]


def _ensure_enabled() -> None:
    """Both ports keep a module-level _COLOURING_DISABLED flag.

    Reset to enabled before every test so order does not matter.
    """
    sphinx_colour.enable_colour()
    sphinxdocrs.enable_colour()


@pytest.fixture(autouse=True)
def _reset_state() -> None:
    _ensure_enabled()
    yield
    _ensure_enabled()


@pytest.mark.parametrize("name", COLOUR_NAMES)
def test_colourise_byte_parity(name: str) -> None:
    text = f"sample-{name}"
    assert sphinxdocrs.colourise(name, text) == sphinx_colour.colourise(name, text)


def test_colourise_unknown_raises() -> None:
    with pytest.raises(ValueError):
        sphinxdocrs.colourise("notacolour", "x")
    with pytest.raises(ValueError):
        sphinx_colour.colourise("notacolour", "x")


def test_disable_enable_round_trip() -> None:
    sphinxdocrs.disable_colour()
    sphinx_colour.disable_colour()
    assert sphinxdocrs.colourise("red", "x") == "x"
    assert sphinx_colour.colourise("red", "x") == "x"
    sphinxdocrs.enable_colour()
    sphinx_colour.enable_colour()
    assert sphinxdocrs.colourise("red", "x") == sphinx_colour.colourise("red", "x")


STRIP_FIXTURES = [
    "",
    "no escapes here",
    "\x1b[91mred\x1b[39;49;00m",
    "\x1b[01;31mbold-red\x1b[39;49;00m and tail",
    "before \x1b[2K after",
    "\x1b[K\x1b[1K\x1b[2K end-of-lines",
    "\x1b[39;49;00mreset alone",
    "mixed \x1b[33mone\x1b[39;49;00m two \x1b[2K three",
]


@pytest.mark.parametrize("text", STRIP_FIXTURES)
def test_strip_escape_sequences_byte_parity(text: str) -> None:
    assert sphinxdocrs.strip_escape_sequences(text) == sphinx_errors.strip_escape_sequences(text)


TERMSAFE_FIXTURES = [
    "",
    "plain ascii",
    "café",
    "ünîçødé",
    "emoji \U0001f600 done",
    "tab\tand\nnewline",
]


@pytest.mark.parametrize("text", TERMSAFE_FIXTURES)
def test_terminal_safe_byte_parity(text: str) -> None:
    assert sphinxdocrs.terminal_safe(text) == sphinx_errors.terminal_safe(text)


def test_colour_names_match_upstream() -> None:
    # Upstream exposes each colour as a module-level function on
    # sphinx._cli.util.colour. Verify our table matches the public
    # set.
    names = set(sphinxdocrs.colour_names())
    for n in COLOUR_NAMES:
        assert n in names
        assert hasattr(sphinx_colour, n)


def test_supports_util_console_flag() -> None:
    assert sphinxdocrs.supports("util:console")
    assert "util:console" in sphinxdocrs.features()
