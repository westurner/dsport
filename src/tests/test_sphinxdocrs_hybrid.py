"""Smoke tests for ``sphinxdocrs_hybrid``."""

from __future__ import annotations

import sys
from pathlib import Path
from types import SimpleNamespace

import pytest

# Make the hybrid helper importable.
_PYDIR = Path(__file__).resolve().parents[1] / 'sphinxdocrs' / 'python'
if str(_PYDIR) not in sys.path:
    sys.path.insert(0, str(_PYDIR))

import sphinxdocrs_hybrid as hybrid  # noqa: E402


def test_has_rust_and_features() -> None:
    assert hybrid.has_rust() is True
    feats = hybrid.features()
    assert 'events:event_manager' in feats
    assert 'project:path2doc' in feats


def test_supports_rejects_unknown() -> None:
    assert hybrid.supports('nope:nope') is False


def test_event_manager_prefers_rust() -> None:
    app = SimpleNamespace(pdb=False)
    em = hybrid.event_manager(app)
    out = []
    em.connect('builder-inited', lambda app: out.append(1), priority=500)
    em.emit('builder-inited')
    assert out == [1]


def test_dispatch_plan_rust() -> None:
    plan = hybrid.dispatch_plan(prefer='rust')
    assert plan['events'] == 'rust'
    assert plan['project'] == 'rust'
    assert plan['errors'] == 'rust'


def test_dispatch_plan_python_fallback() -> None:
    plan = hybrid.dispatch_plan(prefer='python')
    assert plan == {
        'events': 'python',
        'project': 'python',
        'errors': 'python',
        'extension': 'python',
    }


def test_project_prefers_rust(tmp_path) -> None:
    p = hybrid.project(tmp_path, ['.rst'])
    assert p.path2doc('foo.rst') == 'foo'
