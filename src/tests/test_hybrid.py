"""Smoke test for the hybrid Rust/Python publish wrapper."""

import sys
from pathlib import Path

# The hybrid module ships next to the Rust crate; add its directory to path.
HYBRID_DIR = Path(__file__).resolve().parents[1] / "docutilsrs" / "python"
if str(HYBRID_DIR) not in sys.path:
    sys.path.insert(0, str(HYBRID_DIR))

import docutilsrs_hybrid as hybrid  # noqa: E402
from docutils.core import publish_string as py_publish_string  # noqa: E402


def test_hybrid_pseudoxml_matches_rust():
    src = "Hello *world*."
    out = hybrid.publish_string(src, writer="pseudoxml")
    expected = py_publish_string(src, writer="pseudoxml").decode()
    assert out == expected


def test_hybrid_force_python():
    src = "Hello *world*."
    out = hybrid.publish_string(src, writer="pseudoxml", prefer="python")
    expected = py_publish_string(src, writer="pseudoxml").decode()
    assert out == expected


def test_hybrid_unsupported_writer_falls_back():
    src = "Hello *world*."
    # `xml` is not in the Rust-supported set; must fall back to Python.
    out = hybrid.publish_string(src, writer="xml")
    expected = py_publish_string(src, writer="xml").decode()
    assert out == expected
