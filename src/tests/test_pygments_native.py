"""Smoke tests that the native pygmentsrs path is wired into the
docutilsrs `code`/`code-block`/`sourcecode` parser arm.

Byte-parity for the full pygments-Python lexer is gated separately
(pygmentsrs Phase 1 followups); these tests only assert structural
properties of the integration."""

import docutilsrs


def test_text_language_is_byte_passthrough():
    out = docutilsrs.parse_to_pseudoxml(
        ".. code:: text\n\n   hello\n   world\n"
    )
    assert '<literal_block classes="code text"' in out
    assert "hello" in out
    assert "<inline" not in out  # text → no token wrappers


def test_no_language_emits_plain_literal_block():
    out = docutilsrs.parse_to_pseudoxml(".. code::\n\n   x = 1\n")
    assert '<literal_block classes="code"' in out
    assert "<inline" not in out


def test_python_native_path_emits_token_inlines():
    out = docutilsrs.parse_to_pseudoxml(
        ".. code-block:: python\n\n   def f():\n       return 1\n"
    )
    assert '<literal_block classes="code python"' in out
    assert '<inline classes="keyword">' in out
    assert "def" in out
    assert '<inline classes="name function">' in out


def test_sourcecode_alias_routes_through_same_path():
    out = docutilsrs.parse_to_pseudoxml(
        ".. sourcecode:: python\n\n   x = 1\n"
    )
    assert '<literal_block classes="code python"' in out
    assert '<inline' in out


def test_unknown_language_falls_through_to_raw():
    # No pygmentsrs lexer + (likely) no docutils lexer → None → raw text.
    out = docutilsrs.parse_to_pseudoxml(
        ".. code-block:: nosuchlang_xyz\n\n   raw body\n"
    )
    assert '<literal_block classes="code nosuchlang_xyz"' in out
    assert "raw body" in out
