"""Parity tests for ``sphinxdocrs.EventManager``.

Mirrors ``src/sphinx/tests/test_events.py`` (test_event_priority,
test_event_allowed_exceptions, test_event_pdb), plus a small
emit_firstresult check.
"""

from __future__ import annotations

from types import SimpleNamespace

import pytest

from sphinxdocrs import EventManager, ExtensionError


def test_event_priority() -> None:
    result = []
    app = SimpleNamespace(pdb=False)
    events = EventManager(app)
    events.connect('builder-inited', lambda app: result.append(1), priority=500)
    events.connect('builder-inited', lambda app: result.append(2), priority=500)
    events.connect('builder-inited', lambda app: result.append(3), priority=200)
    events.connect('builder-inited', lambda app: result.append(4), priority=700)
    events.connect('builder-inited', lambda app: result.append(5), priority=500)

    events.emit('builder-inited')
    assert result == [3, 1, 2, 5, 4]


def test_event_allowed_exceptions() -> None:
    def raise_error(app):
        raise RuntimeError

    app = SimpleNamespace(pdb=False)
    events = EventManager(app)
    events.connect('builder-inited', raise_error, priority=500)

    with pytest.raises(ExtensionError):
        events.emit('builder-inited')

    with pytest.raises(RuntimeError):
        events.emit('builder-inited', allowed_exceptions=(RuntimeError,))


def test_event_pdb() -> None:
    def raise_error(app):
        raise RuntimeError

    app = SimpleNamespace(pdb=True)
    events = EventManager(app)
    events.connect('builder-inited', raise_error, priority=500)

    with pytest.raises(RuntimeError):
        events.emit('builder-inited')

    with pytest.raises(RuntimeError):
        events.emit('builder-inited', allowed_exceptions=(RuntimeError,))


def test_emit_returns_results_in_priority_order() -> None:
    app = SimpleNamespace(pdb=False)
    events = EventManager(app)
    events.connect('builder-inited', lambda app: 'a', priority=200)
    events.connect('builder-inited', lambda app: 'b', priority=500)
    events.connect('builder-inited', lambda app: 'c', priority=800)
    assert events.emit('builder-inited') == ['a', 'b', 'c']


def test_emit_firstresult_skips_none() -> None:
    app = SimpleNamespace(pdb=False)
    events = EventManager(app)
    events.connect('builder-inited', lambda app: None, priority=200)
    events.connect('builder-inited', lambda app: 'second', priority=500)
    events.connect('builder-inited', lambda app: 'third', priority=800)
    assert events.emit_firstresult('builder-inited') == 'second'


def test_disconnect_removes_listener() -> None:
    app = SimpleNamespace(pdb=False)
    events = EventManager(app)
    out = []
    lid_a = events.connect('builder-inited', lambda app: out.append('a'), priority=500)
    events.connect('builder-inited', lambda app: out.append('b'), priority=500)
    events.disconnect(lid_a)
    events.emit('builder-inited')
    assert out == ['b']
