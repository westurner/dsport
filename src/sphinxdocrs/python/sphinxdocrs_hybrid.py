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


def extension(name, module, *, prefer: str = "rust", **kwargs):
    """Return an Extension wrapper, preferring the Rust port.

    Mirrors ``sphinx.extension.Extension(name, module, **metadata)``.
    """
    if prefer == "rust" and _HAS_RUST and supports("extension:wrapper"):
        return _rs.Extension(name, module, **kwargs)
    from sphinx.extension import Extension as _PyExtension
    return _PyExtension(name, module, **kwargs)


def verify_needs_extensions(app, config, *, prefer: str = "rust") -> None:
    """Mirror of ``sphinx.extension.verify_needs_extensions``."""
    if (
        prefer == "rust"
        and _HAS_RUST
        and supports("extension:verify_needs_extensions")
    ):
        return _rs.verify_needs_extensions(app, config)
    from sphinx.extension import verify_needs_extensions as _py
    return _py(app, config)


def matcher(patterns, *, prefer: str = "rust"):
    """Return a ``Matcher`` instance, preferring the Rust port.

    Mirrors ``sphinx.util.matching.Matcher``.
    """
    if prefer == "rust" and _HAS_RUST and supports("util:matching"):
        return _rs.Matcher(list(patterns))
    from sphinx.util.matching import Matcher as _PyMatcher
    return _PyMatcher(list(patterns))


def compile_matchers(patterns, *, prefer: str = "rust"):
    """Mirror of ``sphinx.util.matching.compile_matchers``."""
    if prefer == "rust" and _HAS_RUST and supports("util:matching"):
        return _rs.compile_matchers(list(patterns))
    from sphinx.util.matching import compile_matchers as _py
    return _py(list(patterns))


def get_matching_files(
    dirname,
    include_patterns=("**",),
    exclude_patterns=(),
    *,
    prefer: str = "rust",
):
    """Mirror of ``sphinx.util.matching.get_matching_files``."""
    if prefer == "rust" and _HAS_RUST and supports("util:matching"):
        return _rs.get_matching_files(
            dirname,
            include_patterns=list(include_patterns),
            exclude_patterns=list(exclude_patterns),
        )
    from sphinx.util.matching import get_matching_files as _py
    return _py(dirname, list(include_patterns), list(exclude_patterns))


def dispatch_plan(*, prefer: str = "rust") -> dict[str, str]:
    """Per-component dispatch summary, mirroring docutilsrs_hybrid."""
    if prefer == "rust" and _HAS_RUST:
        return {
            "events": "rust" if supports("events:event_manager") else "python",
            "project": "rust" if supports("project:path2doc") else "python",
            "errors": "rust" if supports("errors:sphinx_hierarchy") else "python",
            "extension": "rust" if supports("extension:wrapper") else "python",
            "matching": "rust" if supports("util:matching") else "python",
            "project_discover": (
                "rust" if supports("project:discover") else "python"
            ),
        }
    return {
        "events": "python",
        "project": "python",
        "errors": "python",
        "extension": "python",
        "matching": "python",
        "project_discover": "python",
    }
