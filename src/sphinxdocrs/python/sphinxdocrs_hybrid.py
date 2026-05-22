"""Hybrid Rust/Python wrapper for Sphinx port (Phase 4).

Mirrors `docutilsrs_hybrid`'s pattern: prefer the Rust port when it
advertises support for the requested subsystem, fall back to vendored
Python ``sphinx`` otherwise. Currently covers:

* :class:`EventManager` — Rust-backed event bus (drop-in for
  ``sphinx.events.EventManager``).
* Exception types — re-exported from the Rust extension; aliased to
  ``sphinx.errors`` types when the Rust extension is unavailable.
"""

from __future__ import annotations

from typing import Any

try:
    import sphinxdocrs as _rs
    _HAS_RUST = True
except Exception:  # pragma: no cover - exercised on systems without the wheel
    _rs = None
    _HAS_RUST = False


def has_rust() -> bool:
    """True iff the Rust ``sphinxdocrs`` extension is importable."""
    return _HAS_RUST


def supports(feature: str) -> bool:
    """Probe the Rust port's capability flag list."""
    if not _HAS_RUST:
        return False
    return _rs.supports(feature)


def features() -> list[str]:
    if not _HAS_RUST:
        return []
    return list(_rs.features())


def event_manager(app: Any, *, prefer: str = "rust"):
    """Return an EventManager instance, preferring the Rust port.

    The returned object exposes the same ``connect`` / ``disconnect`` /
    ``emit`` / ``emit_firstresult`` surface as
    ``sphinx.events.EventManager``.

    When the Phase 5 resolver has a registered equivalent for
    ``sphinx.events:EventManager``, that takes precedence over the
    built-in capability probe.
    """
    if prefer == "rust":
        try:
            import docutilsrs_plugins as _resolver
        except Exception:
            _resolver = None
        if _resolver is not None:
            eq = _resolver.discover().get("sphinx.events:EventManager")
            if eq is not None and _resolver.upstream_compatible(eq):
                try:
                    return eq.factory()(app)
                except Exception:
                    pass
        if _HAS_RUST and supports("events:event_manager"):
            return _rs.EventManager(app)
    from sphinx.events import EventManager as _PyEventManager
    return _PyEventManager(app)


def project(srcdir, source_suffix, *, prefer: str = "rust"):
    """Return a Project instance, preferring the Rust port."""
    if prefer == "rust" and _HAS_RUST and supports("project:path2doc"):
        return _rs.Project(srcdir, source_suffix)
    from sphinx.project import Project as _PyProject
    return _PyProject(srcdir, source_suffix)


def dispatch_plan(*, prefer: str = "rust") -> dict[str, str]:
    """Per-component dispatch summary, mirroring docutilsrs_hybrid."""
    if prefer == "rust" and _HAS_RUST:
        return {
            "events": "rust" if supports("events:event_manager") else "python",
            "project": "rust" if supports("project:path2doc") else "python",
            "errors": "rust" if supports("errors:sphinx_hierarchy") else "python",
        }
    return {"events": "python", "project": "python", "errors": "python"}
