"""Phase 3 end-to-end hybrid pipeline parity.

Demonstrates the README's Phase 3 acceptance criterion: a document whose
*parsing* is Rust, one *transform* is Python (invoked against the
Rust-owned doctree), and whose *writer* is Rust, produces output that is
byte-identical to running the same transform inside pure-Python docutils.

The transform used here is a small but real text mutation
(uppercase every ``#text`` node) so we are exercising the bridge, not a
trivial no-op.
"""

from __future__ import annotations

import docutilsrs
import pytest

from docutils import nodes
from docutils.core import publish_string as py_publish_string
from docutils.transforms import Transform


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def _rust_uppercase_transform(doctree):
    """Plugin callable: walk the Rust doctree, return text edits.

    The bridge applies the returned ``(node_id, new_text)`` pairs to the
    arena in-place. We deliberately use the read-only ``PyDoctree`` view
    plus an edit list, which is the supported mutation surface for
    Phase 3 transform plugins.
    """
    edits = []

    def walk(node):
        if node.tag == "#text" and node.text is not None:
            edits.append((node.id, node.text.upper()))
        for child in node.children:
            walk(child)

    walk(doctree.root)
    return edits


class _PythonUppercaseTransform(Transform):
    """Pure-Python equivalent, applied during ``docutils.core.publish``."""

    default_priority = 900

    def apply(self, **kwargs):
        for text_node in list(self.document.findall(nodes.Text)):
            text_node.parent.replace(
                text_node, nodes.Text(text_node.astext().upper())
            )


class _UppercaseReader:
    """Tiny shim that injects ``_PythonUppercaseTransform`` into docutils."""

    def __init__(self, base_reader):
        self._base = base_reader

    def __getattr__(self, name):
        return getattr(self._base, name)


def _py_publish_with_transform(source: str) -> str:
    """Pure-Python pipeline that also runs the uppercase transform."""
    from docutils.readers.standalone import Reader as StandaloneReader

    class _R(StandaloneReader):
        def get_transforms(self):
            return super().get_transforms() + [_PythonUppercaseTransform]

    return py_publish_string(
        source, reader=_R(), writer="pseudoxml"
    ).decode("utf-8")


@pytest.fixture(autouse=True)
def _clean_registry():
    """Ensure the global transform registry is empty around each test."""
    docutilsrs.clear_transforms()
    yield
    docutilsrs.clear_transforms()


# ---------------------------------------------------------------------------
# Bridge mechanics
# ---------------------------------------------------------------------------


def test_register_and_list_transform():
    docutilsrs.register_transform("upper", _rust_uppercase_transform)
    assert "upper" in docutilsrs.registered_transforms()


def test_unregister_transform_returns_bool():
    docutilsrs.register_transform("upper", _rust_uppercase_transform)
    assert docutilsrs.unregister_transform("upper") is True
    assert docutilsrs.unregister_transform("upper") is False


def test_transform_plugin_feature_advertised():
    assert "plugin:python_transforms" in docutilsrs.features()


def test_transform_runs_against_rust_tree():
    docutilsrs.register_transform("upper", _rust_uppercase_transform)
    out = docutilsrs.parse_to_pseudoxml("hello *world*.")
    assert "HELLO" in out
    assert "WORLD" in out
    # Lowercase input must have been replaced.
    assert "hello" not in out
    assert "world" not in out


def test_no_transform_when_registry_empty():
    out = docutilsrs.parse_to_pseudoxml("hello world.")
    assert "hello world." in out
    assert "HELLO" not in out


# ---------------------------------------------------------------------------
# End-to-end Phase 3 parity
# ---------------------------------------------------------------------------


PHASE3_DOCS = [
    "hello *world*.",
    "Intro\n=====\n\nA paragraph with **bold** text.\n",
    "- alpha\n- beta\n- gamma\n",
    "Term\n  Definition body.\n",
]


@pytest.mark.parametrize("source", PHASE3_DOCS)
def test_hybrid_rust_parse_python_transform_rust_writer_matches_pure_python(source):
    """Rust parser + Python transform + Rust writer == pure-Python parity."""
    docutilsrs.register_transform("upper", _rust_uppercase_transform)
    hybrid = docutilsrs.parse_to_pseudoxml(source)
    pure_python = _py_publish_with_transform(source)
    assert hybrid == pure_python, (
        "Phase 3 hybrid output diverged from pure-Python.\n"
        f"--- hybrid ---\n{hybrid}\n--- python ---\n{pure_python}"
    )
