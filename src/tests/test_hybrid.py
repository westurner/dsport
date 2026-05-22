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
