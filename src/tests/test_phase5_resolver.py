"""Phase 5 — plugin interoperability tests.

Covers the four acceptance cases from README §Phase 5:

* pure-Python plugin (no Rust equivalent registered)
* pure-Rust equivalent (registered + version guard passes)
* fallback on version mismatch (guard rejects, warning emitted, Python wins)
* mixed pipeline (some targets resolve to Rust, others to Python)
"""

from __future__ import annotations

import sys
import warnings
from pathlib import Path

import pytest

HYBRID_DIR = Path(__file__).resolve().parents[1] / "docutilsrs" / "python"
if str(HYBRID_DIR) not in sys.path:
    sys.path.insert(0, str(HYBRID_DIR))

import docutilsrs_plugins as plugins  # noqa: E402


@pytest.fixture(autouse=True)
def _clean_registry():
    plugins.clear_registry()
    yield
    plugins.clear_registry()


# ---------------------------------------------------------------------------
# 1. Pure-Python plugin: no Rust equivalent registered → fall back.
# ---------------------------------------------------------------------------


def test_pure_python_plugin_resolves_to_python():
    res = plugins.resolve("docutils.transforms.frontmatter:DocTitle")
    assert res.source == "python"
    assert res.reason == "python-fallback"
    # And it's actually the upstream class.
    from docutils.transforms.frontmatter import DocTitle as Upstream
    assert res.impl is Upstream


# ---------------------------------------------------------------------------
# 2. Pure-Rust equivalent: registered + (no) version guard → rust wins.
# ---------------------------------------------------------------------------


class _RustReplacement:
    """Sentinel standing in for a Rust-backed implementation."""


def test_rust_equivalent_wins_when_registered():
    plugins.register(
        "docutils.transforms.frontmatter:DocTitle",
        factory=lambda: _RustReplacement,
    )
    res = plugins.resolve("docutils.transforms.frontmatter:DocTitle")
    assert res.source == "rust"
    assert res.impl is _RustReplacement


def test_rust_equivalent_passes_version_guard():
    import docutils  # noqa: F401 — confirm import path exists

    plugins.register(
        "docutils.transforms.frontmatter:DocTitle",
        factory=lambda: _RustReplacement,
        upstream_requires=">=0.0",  # always satisfied
    )
    res = plugins.resolve("docutils.transforms.frontmatter:DocTitle")
    assert res.source == "rust"


# ---------------------------------------------------------------------------
# 3. Version mismatch: guard rejects, warning emitted, Python wins.
# ---------------------------------------------------------------------------


def test_version_mismatch_falls_back_with_warning():
    plugins.register(
        "docutils.transforms.frontmatter:DocTitle",
        factory=lambda: _RustReplacement,
        upstream_requires="<0.0.1",  # impossible
    )
    with warnings.catch_warnings(record=True) as caught:
        warnings.simplefilter("always")
        res = plugins.resolve("docutils.transforms.frontmatter:DocTitle")
    assert res.source == "python"
    assert any(
        "upstream_requires" in str(w.message) and issubclass(w.category, UserWarning)
        for w in caught
    )


def test_rust_factory_failure_warns_and_falls_back():
    def _explode():
        raise RuntimeError("simulated load failure")

    plugins.register(
        "docutils.transforms.frontmatter:DocTitle",
        factory=_explode,
    )
    with warnings.catch_warnings(record=True) as caught:
        warnings.simplefilter("always")
        res = plugins.resolve("docutils.transforms.frontmatter:DocTitle")
    assert res.source == "python"
    assert any("failed to load" in str(w.message) for w in caught)


# ---------------------------------------------------------------------------
# 4. Mixed pipeline: some targets resolve to Rust, others to Python.
# ---------------------------------------------------------------------------


def test_mixed_dispatch_plan():
    plugins.register(
        "docutils.writers.html5_polyglot:Writer",
        factory=lambda: _RustReplacement,
    )
    plan = plugins.dispatch_plan(
        [
            "docutils.writers.html5_polyglot:Writer",  # rust
            "docutils.transforms.frontmatter:DocTitle",  # python
            "sphinx.events:EventManager",  # python
        ]
    )
    assert plan == {
        "docutils.writers.html5_polyglot:Writer": "rust",
        "docutils.transforms.frontmatter:DocTitle": "python",
        "sphinx.events:EventManager": "python",
    }


def test_prefer_python_bypasses_rust_registration():
    plugins.register(
        "docutils.transforms.frontmatter:DocTitle",
        factory=lambda: _RustReplacement,
    )
    res = plugins.resolve(
        "docutils.transforms.frontmatter:DocTitle", prefer="python"
    )
    assert res.source == "python"
    assert res.reason == "python-preferred"


# ---------------------------------------------------------------------------
# 5. End-to-end with real Rust class: route sphinx EventManager via resolver.
# ---------------------------------------------------------------------------


def test_e2e_resolves_sphinx_eventmanager_to_rust():
    from types import SimpleNamespace

    import sphinxdocrs

    plugins.register(
        "sphinx.events:EventManager",
        factory=lambda: sphinxdocrs.EventManager,
        upstream_requires=">=0",
    )
    res = plugins.resolve("sphinx.events:EventManager")
    assert res.source == "rust"
    EM = res.impl
    em = EM(SimpleNamespace(pdb=False))
    out: list[int] = []
    em.connect("builder-inited", lambda app: out.append(1), priority=500)
    em.emit("builder-inited")
    assert out == [1]


def test_dispatch_returns_callable_implementation():
    plugins.register(
        "docutils.transforms.frontmatter:DocTitle",
        factory=lambda: _RustReplacement,
    )
    impl = plugins.dispatch("docutils.transforms.frontmatter:DocTitle")
    assert impl is _RustReplacement
