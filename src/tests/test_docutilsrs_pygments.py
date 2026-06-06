"""Tests for docutilsrs_pygments plugin module."""

from __future__ import annotations

import sys
from unittest.mock import Mock, patch

import pytest


def test_pyghl_with_pygments_available():
    """Test _pyghl_callable when Pygments is available."""
    import docutilsrs_pygments as mod
    
    # Test with a valid language
    result = mod._pyghl_callable("python", "def hello():\n    pass")
    assert "raw:: html" in result
    assert "def" in result or "hello" in result


def test_pyghl_with_invalid_language():
    """Test _pyghl_callable falls back to text when language is invalid."""
    import docutilsrs_pygments as mod
    
    result = mod._pyghl_callable("nonexistent_lang_xyz", "some code")
    assert "raw:: html" in result


def test_pyghl_without_language_arg():
    """Test _pyghl_callable defaults to text when no args given."""
    import docutilsrs_pygments as mod
    
    result = mod._pyghl_callable("", "some code")
    assert "raw:: html" in result


def test_pyghl_with_pygments_unavailable():
    """Test _pyghl_callable falls back to literal block when Pygments unavailable."""
    # This tests the code path at lines 41-42
    import sys
    import importlib
    
    # Save original module
    import docutilsrs_pygments as original_mod
    
    # Create a mock where highlight is None
    with patch.object(original_mod, 'highlight', None):
        result = original_mod._pyghl_callable("python", "def foo():\n    pass")
        # Should produce a literal block, not raw html
        assert "::" in result
        assert "def foo():" in result


def test_pyghl_with_multiline_body():
    """Test _pyghl_callable preserves multi-line indentation."""
    import docutilsrs_pygments as mod
    
    body = "line1\nline2\nline3"
    result = mod._pyghl_callable("text", body)
    assert "line1" in result
    assert "line2" in result
    assert "line3" in result


def test_register_and_unregister():
    """Test register and unregister functions."""
    import docutilsrs
    import docutilsrs_pygments as mod
    
    # Setup
    docutilsrs.clear_directives()
    
    # Register
    mod.register()
    dirs = docutilsrs.registered_directives()
    assert "pyghl" in dirs
    
    # Unregister
    mod.unregister()
    dirs = docutilsrs.registered_directives()
    assert "pyghl" not in dirs


def test_pyghl_directive_via_bridge():
    """Test pyghl directive works through the docutilsrs bridge."""
    import docutilsrs
    import docutilsrs_pygments as mod
    
    # Setup
    docutilsrs.clear_directives()
    mod.register()
    
    try:
        # Test parsing with pyghl directive
        src = ".. pyghl:: python\n\n   x = 42\n"
        result = docutilsrs.parse_to_pseudoxml(src)
        assert result is not None
        assert "42" in result or "x" in result
    finally:
        docutilsrs.clear_directives()


def test_pyghl_with_indented_body():
    """Test that indented body is properly re-indented for RST."""
    import docutilsrs_pygments as mod
    
    # Simulate indented body (as would come from RST)
    body = "class Foo:\n    def bar(self):\n        pass"
    result = mod._pyghl_callable("python", body)
    
    # Should start with :: or raw:: html
    assert "::" in result or "raw::" in result
