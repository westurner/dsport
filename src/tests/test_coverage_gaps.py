"""Additional tests to improve coverage of error handling and edge cases."""

import pytest
import sys
from unittest.mock import MagicMock, patch
from pathlib import Path

# Add hybrid modules to path
HYBRID_DIR = Path(__file__).resolve().parents[1] / "docutilsrs" / "python"
SPHINX_HYBRID_DIR = Path(__file__).resolve().parents[1] / "sphinxdocrs" / "python"
for d in [HYBRID_DIR, SPHINX_HYBRID_DIR]:
    if str(d) not in sys.path:
        sys.path.insert(0, str(d))

import docutilsrs  # noqa: E402


class TestDocutilsrsHybridCoverage:
    """Tests for missing coverage in docutilsrs_hybrid.py."""

    def test_hybrid_rust_exception_fallback(self):
        """Test exception handling in publish_string fallback to Python."""
        import docutilsrs_hybrid as hybrid

        # Mock docutilsrs to raise an exception
        src = "Hello *world*."
        with patch("docutilsrs_hybrid.docutilsrs.parse_to_pseudoxml") as mock_rust:
            mock_rust.side_effect = RuntimeError("Rust error")
            # Should fall back to Python
            result = hybrid.publish_string(src, writer="pseudoxml", prefer="rust")
            assert result
            assert "Hello" in result or "world" in result

    def test_compare_rust_exception(self):
        """Test compare when Rust raises exception."""
        import docutilsrs_hybrid as hybrid

        src = "Hello."
        with patch("docutilsrs_hybrid.docutilsrs.parse_to_pseudoxml") as mock_rust:
            mock_rust.side_effect = RuntimeError("Rust error")
            cmp = hybrid.compare(src, writer="pseudoxml")
            # Rust should be None due to exception
            assert cmp.rust is None
            assert cmp.python  # Python should work
            assert cmp.identical is False


class TestSphinxdocrsHybridCoverage:
    """Tests for missing coverage in sphinxdocrs_hybrid.py."""

    def test_without_rust_extension(self):
        """Test behavior when sphinxdocrs is not available."""
        import sphinxdocrs_hybrid as hybrid

        # Check the fallback path when _HAS_RUST is False
        with patch.object(hybrid, "_HAS_RUST", False):
            assert hybrid.has_rust() is False
            assert hybrid.supports("events:event_manager") is False
            assert hybrid.features() == []

    def test_event_manager_without_rust(self):
        """Test event_manager fallback to Python."""
        import sphinxdocrs_hybrid as hybrid

        mock_app = MagicMock()
        with patch.object(hybrid, "_HAS_RUST", False):
            em = hybrid.event_manager(mock_app, prefer="rust")
            # Should return Python EventManager
            from sphinx.events import EventManager
            assert isinstance(em, EventManager)

    def test_project_without_rust(self):
        """Test project fallback to Python."""
        import sphinxdocrs_hybrid as hybrid

        with patch.object(hybrid, "_HAS_RUST", False):
            proj = hybrid.project("/tmp", ".rst", prefer="rust")
            # Should return Python Project
            from sphinx.project import Project
            assert isinstance(proj, Project)

    def test_extension_fallback(self):
        """Test extension factory fallback."""
        import sphinxdocrs_hybrid as hybrid

        with patch.object(hybrid, "_HAS_RUST", False):
            # Should get Python extension
            ext = hybrid.extension("sphinx.ext.autodoc", None, prefer="rust")
            assert ext is not None

    def test_verify_needs_extensions_fallback(self):
        """Test verify_needs_extensions fallback."""
        import sphinxdocrs_hybrid as hybrid

        mock_app = MagicMock()
        mock_config = MagicMock()
        with patch.object(hybrid, "_HAS_RUST", False):
            # Should not raise
            result = hybrid.verify_needs_extensions(mock_app, mock_config, prefer="rust")
            # Should complete without error
