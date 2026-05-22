"""Hybrid Rust/Python publish wrapper.

Phase 3 hybrid surface: prefer the Rust port when its feature set covers
the requested writer, otherwise fall back to the vendored Python
``docutils`` implementation. Also exposes a ``compare`` helper used by
parity tooling.
"""

from __future__ import annotations

from typing import NamedTuple

import docutilsrs
from docutils.core import publish_string as _py_publish_string


# Map writer name -> Rust feature flag that must be present to dispatch
# through the native pipeline.
_RUST_WRITER_FEATURE = {
    "pseudoxml": "writer:pseudoxml",
    "html5": "writer:html5",
}


def rust_supports_writer(writer: str) -> bool:
    """True iff the Rust extension advertises support for ``writer``."""
    feature = _RUST_WRITER_FEATURE.get(writer)
    return bool(feature) and docutilsrs.supports(feature)


def _rust_render(source: str, writer: str, source_path: str) -> str:
    if writer == "pseudoxml":
        return docutilsrs.parse_to_pseudoxml(source, source_path)
    if writer == "html5":
        return docutilsrs.parse_to_html5(source, source_path)
    raise ValueError(f"Rust path does not support writer {writer!r}")


def _python_render(source: str, writer: str) -> str:
    out = _py_publish_string(source, writer=writer)
    if isinstance(out, bytes):
        out = out.decode("utf-8")
    return out


def publish_string(
    source: str,
    *,
    writer: str = "pseudoxml",
    source_path: str = "<string>",
    prefer: str = "rust",
) -> str:
    """Render ``source`` to a string using the best available backend.

    Dispatch policy:

    1. If ``prefer == "rust"`` and the Rust extension advertises support
       for the writer, render with Rust.
    2. If the Rust path raises any exception, fall back to Python.
    3. Otherwise, render with Python.
    """
    if prefer == "rust" and rust_supports_writer(writer):
        try:
            return _rust_render(source, writer, source_path)
        except Exception:
            pass
    return _python_render(source, writer)


class Comparison(NamedTuple):
    """Result of running both backends against the same input."""

    rust: str | None
    python: str
    identical: bool


def compare(
    source: str,
    *,
    writer: str = "pseudoxml",
    source_path: str = "<string>",
) -> Comparison:
    """Render ``source`` with both backends and report whether they agree.

    ``rust`` is ``None`` when the Rust path doesn't claim support for the
    requested writer or raises. Useful for parity probes and regression
    triage.
    """
    python_out = _python_render(source, writer)
    if not rust_supports_writer(writer):
        return Comparison(rust=None, python=python_out, identical=False)
    try:
        rust_out = _rust_render(source, writer, source_path)
    except Exception:
        return Comparison(rust=None, python=python_out, identical=False)
    return Comparison(
        rust=rust_out,
        python=python_out,
        identical=(rust_out == python_out),
    )


def features() -> list[str]:
    """Return the Rust port's advertised feature list."""
    return list(docutilsrs.features())


# ---------------------------------------------------------------------------
# Per-component dispatch (Phase 3, item 2)
# ---------------------------------------------------------------------------


def dispatch_plan(
    writer: str = "pseudoxml",
    *,
    prefer: str = "rust",
    has_python_transforms: bool = False,
) -> dict[str, str]:
    """Describe which backend will run each pipeline component.

    Returned dict has keys ``parser``, ``transforms``, ``writer`` whose
    values are ``"rust"`` or ``"python"``. The Rust pipeline always runs
    parser+default-transforms+writer as a single unit; when
    ``has_python_transforms`` is true and the Rust writer is selected,
    the registered Python transforms run between Rust's default
    transforms and the Rust writer (via the
    ``docutilsrs.register_transform`` bridge).
    """
    if prefer == "rust" and rust_supports_writer(writer):
        return {
            "parser": "rust",
            "transforms": "rust+python" if has_python_transforms else "rust",
            "writer": "rust",
        }
    return {"parser": "python", "transforms": "python", "writer": "python"}
