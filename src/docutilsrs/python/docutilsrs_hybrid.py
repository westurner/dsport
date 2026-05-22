"""Minimal hybrid Rust/Python publish wrapper.

Demonstrates the Phase 3 hybrid mode: prefer the Rust port when it can
produce byte-identical output for the requested writer, otherwise fall
back to the vendored Python ``docutils`` implementation.

This module is intentionally tiny — it is a reference shape for the
production wrapper, not a feature-complete dispatcher.
"""

from __future__ import annotations

import docutilsrs
from docutils.core import publish_string as _py_publish_string


_RUST_WRITERS = {"pseudoxml", "html5"}


def publish_string(
    source: str,
    *,
    writer: str = "pseudoxml",
    source_path: str = "<string>",
    prefer: str = "rust",
) -> str:
    """Return the rendered output for ``source``.

    Parameters
    ----------
    source:
        rST input.
    writer:
        ``"pseudoxml"`` or ``"html5"`` route through the Rust port when
        ``prefer="rust"``; any other writer always falls back to Python.
    prefer:
        ``"rust"`` (default) tries the Rust path first, ``"python"`` always
        uses docutils.

    On any exception from the Rust path, this falls back to the Python
    implementation so callers get the strongest available behaviour.
    """
    if prefer == "rust" and writer in _RUST_WRITERS:
        try:
            if writer == "pseudoxml":
                return docutilsrs.parse_to_pseudoxml(source, source_path)
            if writer == "html5":
                return docutilsrs.parse_to_html5(source, source_path)
        except Exception:
            pass
    out = _py_publish_string(source, writer=writer)
    if isinstance(out, bytes):
        out = out.decode("utf-8")
    return out
