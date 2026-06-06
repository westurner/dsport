"""Tests for the hybrid Rust/Python publish wrapper."""

import sys
from pathlib import Path

# The hybrid module ships next to the Rust crate; add its directory to path.
HYBRID_DIR = Path(__file__).resolve().parents[1] / "docutilsrs" / "python"
if str(HYBRID_DIR) not in sys.path:
    sys.path.insert(0, str(HYBRID_DIR))

import docutilsrs  # noqa: E402
import docutilsrs_hybrid as hybrid  # noqa: E402
from docutils.core import publish_string as py_publish_string  # noqa: E402


def _py(src, writer):
    return py_publish_string(src, writer=writer).decode()


def test_hybrid_pseudoxml_matches_rust():
    src = "Hello *world*."
    assert hybrid.publish_string(src, writer="pseudoxml") == _py(src, "pseudoxml")


def test_hybrid_force_python():
    src = "Hello *world*."
    assert (
        hybrid.publish_string(src, writer="pseudoxml", prefer="python")
        == _py(src, "pseudoxml")
    )


def test_hybrid_unsupported_writer_falls_back():
    src = "Hello *world*."
    assert hybrid.publish_string(src, writer="xml") == _py(src, "xml")


def test_rust_supports_writer_flags():
    assert hybrid.rust_supports_writer("pseudoxml") is True
    assert hybrid.rust_supports_writer("html5") is True
    assert hybrid.rust_supports_writer("xml") is False
    assert hybrid.rust_supports_writer("latex") is False


def test_features_advertised():
    feats = hybrid.features()
    assert "writer:pseudoxml" in feats
    assert "writer:html5" in feats
    assert "parser:footnote_numbered" in feats
    assert "transform:resolve_references" in feats


def test_supports_predicate():
    assert docutilsrs.supports("writer:pseudoxml") is True
    assert docutilsrs.supports("nonexistent:feature") is False


def test_compare_identical():
    src = "Hello *world*."
    cmp = hybrid.compare(src, writer="pseudoxml")
    assert cmp.rust is not None
    assert cmp.identical is True
    assert cmp.rust == cmp.python


def test_compare_unsupported_writer():
    src = "Hello."
    cmp = hybrid.compare(src, writer="xml")
    assert cmp.rust is None
    assert cmp.identical is False
    assert cmp.python  # non-empty


def test_dispatch_plan_rust_default():
    plan = hybrid.dispatch_plan("pseudoxml")
    assert plan == {"parser": "rust", "transforms": "rust", "writer": "rust"}


def test_dispatch_plan_rust_with_python_transforms():
    plan = hybrid.dispatch_plan("pseudoxml", has_python_transforms=True)
    assert plan["transforms"] == "rust+python"
    assert plan["parser"] == "rust"
    assert plan["writer"] == "rust"


def test_dispatch_plan_falls_back_to_python():
    plan = hybrid.dispatch_plan("xml")
    assert plan == {"parser": "python", "transforms": "python", "writer": "python"}


def test_dispatch_plan_force_python():
    plan = hybrid.dispatch_plan("pseudoxml", prefer="python")
    assert plan["parser"] == "python"
    assert plan["writer"] == "python"


def test_hybrid_html5_dispatch():
    """Test that HTML5 route exists (doesn't need to match Python exactly)."""
    src = "Hello *world*."
    result = hybrid.publish_string(src, writer="html5")
    # Just verify it produces output without crashing
    assert result
    assert "Hello" in result or "world" in result


def test_compare_html5_diverges():
    """Test compare with HTML5 writer - expected to diverge from Python."""
    src = "Hello *world*."
    cmp = hybrid.compare(src, writer="html5")
    assert cmp.rust is not None
    # Rust and Python HTML5 outputs may differ (body vs full doc)
    assert cmp.python  # Python output should exist
    # Don't assert identical=True since implementations differ


def test_hybrid_html5_force_python():
    """Test HTML5 with force python."""
    src = "Hello *world*."
    result = hybrid.publish_string(src, writer="html5", prefer="python")
    py_result = _py(src, "html5")
    assert result == py_result


def test_rust_render_html5_directly():
    """Test _rust_render directly with html5 to cover line 36."""
    import docutilsrs_hybrid as hybrid
    
    src = "Hello *world*."
    # Directly call _rust_render with html5 writer to exercise line 36
    result = hybrid._rust_render(src, "html5", "<string>")
    assert result is not None
    assert len(result) > 0


def test_rust_render_unsupported_writer():
    """Test _rust_render raises ValueError for unsupported writers"""
    import docutilsrs_hybrid as hybrid
    import pytest
    
    src = "Hello *world*."
    # Call _rust_render with an unsupported writer to trigger exception
    with pytest.raises(ValueError, match="Rust path does not support writer"):
        hybrid._rust_render(src, "unsupported_writer", "<string>")


def test_publish_string_rust_exception_fallback():
    """Test publish_string falls back to Python when _rust_render raises (covers lines 65-66)."""
    import docutilsrs_hybrid as hybrid
    from unittest.mock import patch
    
    src = "Hello *world*."
    
    # Mock _rust_render to raise an exception
    with patch.object(hybrid, "_rust_render", side_effect=RuntimeError("Rust error")):
        result = hybrid.publish_string(src, writer="pseudoxml", prefer="rust")
        # Should fall back to Python and produce output
        assert result
        assert "Hello" in result or "world" in result
        # Verify it matches Python output
        expected = _py(src, "pseudoxml")
        assert result == expected


def test_publish_string_rust_exception_fallback_html5():
    """Test publish_string with HTML5 falls back when _rust_render raises (covers lines 65-66)."""
    import docutilsrs_hybrid as hybrid
    from unittest.mock import patch
    
    src = "Test *emphasis* content."
    
    # Mock _rust_render to raise an exception
    with patch.object(hybrid, "_rust_render", side_effect=RuntimeError("Rust crashed")):
        result = hybrid.publish_string(src, writer="html5", prefer="rust")
        # Should fall back to Python and produce output
        assert result
        expected = _py(src, "html5")
        assert result == expected


def test_compare_rust_exception_fallback():
    """Test compare falls back when _rust_render raises (covers lines 95-96)."""
    import docutilsrs_hybrid as hybrid
    from unittest.mock import patch
    
    src = "Hello *world*."
    
    # Mock _rust_render to raise an exception
    with patch.object(hybrid, "_rust_render", side_effect=RuntimeError("Rust error")):
        cmp = hybrid.compare(src, writer="pseudoxml")
        # When Rust raises, rust should be None
        assert cmp.rust is None
        assert cmp.python is not None
        assert cmp.identical is False


def test_compare_rust_exception_fallback_html5():
    """Test compare with HTML5 falls back when _rust_render raises (covers lines 95-96)."""
    import docutilsrs_hybrid as hybrid
    from unittest.mock import patch
    
    src = "Test *content*."
    
    # Mock _rust_render to raise an exception with various exception types
    with patch.object(hybrid, "_rust_render", side_effect=ValueError("Invalid rust")):
        cmp = hybrid.compare(src, writer="html5")
        # When Rust raises, rust should be None
        assert cmp.rust is None
        assert cmp.python is not None
        assert cmp.identical is False


def test_compare_rust_exception_type_error():
    """Test compare with various exception types from Rust (covers lines 95-96)."""
    import docutilsrs_hybrid as hybrid
    from unittest.mock import patch
    
    src = "Exception test."
    
    # Test with TypeError
    with patch.object(hybrid, "_rust_render", side_effect=TypeError("Type mismatch")):
        cmp = hybrid.compare(src, writer="pseudoxml")
        assert cmp.rust is None
        assert cmp.identical is False