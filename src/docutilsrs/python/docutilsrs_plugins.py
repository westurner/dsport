"""Plugin/equivalent resolver for the Rust ports (Phase 5).

Design
------

A Rust crate may declare itself as the equivalent of a Python plugin
(directive, role, transform, writer, Sphinx extension, …) by exposing an
entry point in the group ``docutilsrs.equivalents``:

.. code-block:: toml

    [project.entry-points."docutilsrs.equivalents"]
    "docutils.writers.html5_polyglot:Writer" = "docutilsrs:writers.html5.HTML5Writer"

The key is the upstream Python dotted reference being shadowed; the
value is loaded via :func:`importlib.metadata.EntryPoint.load` and is
typically a class or callable in an installed Rust extension module.

Version guard
-------------

Each equivalent may carry a ``__upstream_requires__`` attribute (a
PEP 440 specifier string) describing the upstream package versions it is
known to be parity-compatible with. The resolver checks the installed
upstream package's ``__version__``; on a mismatch it emits a
``UserWarning`` and falls back to the Python implementation.

In-memory registration
----------------------

For tests and ad-hoc bootstrapping, :func:`register` adds an equivalent
without installing a package. Registrations take precedence over
``importlib.metadata`` entry points to make test ordering deterministic.
"""

from __future__ import annotations

import importlib
import importlib.metadata
import warnings
from dataclasses import dataclass
from typing import Any, Callable, Dict, Optional

GROUP = "docutilsrs.equivalents"

# In-memory registry; keys are upstream dotted refs (the entry-point name).
_REGISTRY: Dict[str, "Equivalent"] = {}


@dataclass(frozen=True)
class Equivalent:
    """A declared Rust equivalent for a Python plugin."""

    target: str
    """Upstream dotted reference (e.g. ``"sphinx.events:EventManager"``)."""

    factory: Callable[[], Any]
    """Zero-arg callable that returns the implementation object."""

    upstream_requires: Optional[str] = None
    """Optional PEP 440 specifier string for the upstream package."""

    source: str = "rust"
    """Where this equivalent came from (``"rust"`` by default)."""


@dataclass(frozen=True)
class Resolution:
    """Outcome of resolving a plugin reference."""

    impl: Any
    """The resolved implementation object."""

    source: str
    """Either ``"rust"`` or ``"python"``."""

    reason: str
    """Why this branch was chosen (for diagnostics)."""


# ---------------------------------------------------------------------------
# Registration
# ---------------------------------------------------------------------------


def register(
    target: str,
    factory: Callable[[], Any],
    *,
    upstream_requires: Optional[str] = None,
) -> None:
    """Register an in-memory equivalent.

    ``target`` is the upstream Python dotted reference of the form
    ``"package.module:Symbol"`` (Sphinx / setuptools entry-point style).
    """
    _REGISTRY[target] = Equivalent(
        target=target,
        factory=factory,
        upstream_requires=upstream_requires,
    )


def unregister(target: str) -> None:
    """Remove an in-memory registration. No-op if absent."""
    _REGISTRY.pop(target, None)


def clear_registry() -> None:
    """Drop all in-memory registrations. Test helper."""
    _REGISTRY.clear()


# ---------------------------------------------------------------------------
# Discovery
# ---------------------------------------------------------------------------


def _entry_point_equivalents() -> Dict[str, Equivalent]:
    """Discover equivalents from installed ``docutilsrs.equivalents`` entry points."""
    found: Dict[str, Equivalent] = {}
    try:
        eps = importlib.metadata.entry_points(group=GROUP)
    except TypeError:  # pragma: no cover - python <3.10 fallback
        eps = importlib.metadata.entry_points().get(GROUP, [])
    for ep in eps:
        # Defer loading to factory() so a broken third-party entry point
        # cannot poison the registry just by being installed.
        def _factory(_ep=ep):
            return _ep.load()

        # Try to read upstream_requires lazily as well.
        def _upstream_requires(_ep=ep) -> Optional[str]:
            try:
                obj = _ep.load()
            except Exception:
                return None
            return getattr(obj, "__upstream_requires__", None)

        found[ep.name] = Equivalent(
            target=ep.name,
            factory=_factory,
            upstream_requires=_upstream_requires(),
        )
    return found


def discover() -> Dict[str, Equivalent]:
    """Return all known equivalents (in-memory registrations win)."""
    merged = _entry_point_equivalents()
    merged.update(_REGISTRY)
    return merged


# ---------------------------------------------------------------------------
# Version guard
# ---------------------------------------------------------------------------


def _split_upstream_package(target: str) -> str:
    """Pick the distribution/import-package name from a dotted target."""
    head = target.split(":", 1)[0]
    return head.split(".", 1)[0]


def _installed_upstream_version(target: str) -> Optional[str]:
    """Best-effort lookup of the upstream package version."""
    pkg = _split_upstream_package(target)
    try:
        return importlib.metadata.version(pkg)
    except importlib.metadata.PackageNotFoundError:
        pass
    try:
        module = importlib.import_module(pkg)
    except Exception:
        return None
    return getattr(module, "__version__", None)


def _specifier_matches(spec: str, version: str) -> bool:
    """Evaluate a PEP 440 specifier; ``packaging`` is optional."""
    try:
        from packaging.specifiers import SpecifierSet
        from packaging.version import Version
    except Exception:
        # If packaging isn't available, be permissive (no guard).
        return True
    try:
        return Version(version) in SpecifierSet(spec)
    except Exception:
        return False


def upstream_compatible(eq: Equivalent) -> bool:
    """True if the equivalent's upstream version guard is satisfied."""
    if not eq.upstream_requires:
        return True
    installed = _installed_upstream_version(eq.target)
    if installed is None:
        # Unknown installed version: be permissive, surface via warning.
        return True
    return _specifier_matches(eq.upstream_requires, installed)


# ---------------------------------------------------------------------------
# Resolution
# ---------------------------------------------------------------------------


def _load_python(target: str) -> Any:
    """Import the upstream Python implementation from a dotted target."""
    if ":" in target:
        module_path, attr = target.split(":", 1)
    elif "." in target:
        module_path, attr = target.rsplit(".", 1)
    else:
        raise ValueError(f"Cannot parse upstream target reference: {target!r}")
    module = importlib.import_module(module_path)
    obj = module
    for part in attr.split("."):
        obj = getattr(obj, part)
    return obj


def resolve(target: str, *, prefer: str = "rust") -> Resolution:
    """Resolve ``target`` to a concrete implementation.

    Resolution order when ``prefer == "rust"``:

    1. If an equivalent is registered/declared and its version guard
       passes, return its factory output as ``source="rust"``.
    2. If an equivalent exists but the version guard fails, emit a
       ``UserWarning`` and fall through to Python.
    3. Otherwise import and return the upstream Python implementation as
       ``source="python"``.
    """
    if prefer == "rust":
        eq = discover().get(target)
        if eq is not None:
            if upstream_compatible(eq):
                try:
                    impl = eq.factory()
                except Exception as err:
                    warnings.warn(
                        f"Rust equivalent for {target!r} failed to load "
                        f"({err!r}); falling back to Python.",
                        UserWarning,
                        stacklevel=2,
                    )
                else:
                    return Resolution(
                        impl=impl, source="rust", reason="rust-equivalent"
                    )
            else:
                warnings.warn(
                    f"Rust equivalent for {target!r} declares "
                    f"upstream_requires={eq.upstream_requires!r} which does "
                    f"not match the installed version "
                    f"{_installed_upstream_version(target)!r}; falling back "
                    "to Python.",
                    UserWarning,
                    stacklevel=2,
                )
    return Resolution(
        impl=_load_python(target),
        source="python",
        reason="python-fallback" if prefer == "rust" else "python-preferred",
    )


def dispatch(target: str, *, prefer: str = "rust") -> Any:
    """Convenience wrapper returning just the resolved implementation."""
    return resolve(target, prefer=prefer).impl


def dispatch_plan(targets: list[str], *, prefer: str = "rust") -> Dict[str, str]:
    """Report the resolved source (``"rust"`` / ``"python"``) per target.

    Does not actually load Python fallbacks: if no Rust equivalent is
    available or the guard fails, the entry is ``"python"`` without an
    import. Useful for diagnostics dashboards.
    """
    eqs = discover() if prefer == "rust" else {}
    plan: Dict[str, str] = {}
    for t in targets:
        eq = eqs.get(t)
        if eq is not None and upstream_compatible(eq):
            plan[t] = "rust"
        else:
            plan[t] = "python"
    return plan
