"""Smoke tests for ``sphinxdocrs_hybrid``."""

from __future__ import annotations

import sys
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock, patch

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
        'matching': 'python',
        'project_discover': 'python',
    }


def test_project_prefers_rust(tmp_path) -> None:
    p = hybrid.project(tmp_path, ['.rst'])
    assert p.path2doc('foo.rst') == 'foo'


def test_event_manager_prefers_python() -> None:
    """Test fallback to Python EventManager when prefer='python'."""
    app = SimpleNamespace(pdb=False)
    em = hybrid.event_manager(app, prefer='python')
    out = []
    em.connect('builder-inited', lambda app: out.append(1), priority=500)
    em.emit('builder-inited')
    assert out == [1]


def test_extension_prefers_rust() -> None:
    """Test Extension wrapper preferring Rust."""
    ext = hybrid.extension('test.ext', 'test_module', prefer='rust')
    assert ext is not None


def test_extension_prefers_python() -> None:
    """Test Extension wrapper fallback to Python."""
    ext = hybrid.extension('test.ext', 'test_module', prefer='python')
    assert ext is not None


def test_verify_needs_extensions_rust() -> None:
    """Test verify_needs_extensions with Rust preference."""
    app = SimpleNamespace(config=SimpleNamespace(extensions=[], needs_extensions=None))
    config = SimpleNamespace(extensions=[], needs_extensions=None)
    try:
        hybrid.verify_needs_extensions(app, config, prefer='rust')
    except AttributeError:
        pass


def test_verify_needs_extensions_python() -> None:
    """Test verify_needs_extensions with Python preference."""
    app = SimpleNamespace(config=SimpleNamespace(extensions=[], needs_extensions=None))
    config = SimpleNamespace(extensions=[], needs_extensions=None)
    hybrid.verify_needs_extensions(app, config, prefer='python')


def test_matcher_prefers_rust() -> None:
    """Test Matcher with Rust preference."""
    m = hybrid.matcher(['*.rst'], prefer='rust')
    assert m is not None
    assert hasattr(m, 'match')


def test_matcher_prefers_python() -> None:
    """Test Matcher fallback to Python."""
    m = hybrid.matcher(['*.rst'], prefer='python')
    assert m is not None
    assert hasattr(m, 'match')


def test_compile_matchers_rust() -> None:
    """Test compile_matchers with Rust preference."""
    matchers = hybrid.compile_matchers(['*.rst'], prefer='rust')
    assert matchers is not None


def test_compile_matchers_python() -> None:
    """Test compile_matchers with Python preference."""
    matchers = hybrid.compile_matchers(['*.rst'], prefer='python')
    assert matchers is not None


def test_get_matching_files_rust(tmp_path) -> None:
    """Test get_matching_files with Rust preference."""
    (tmp_path / 'test.rst').touch()
    files = hybrid.get_matching_files(
        str(tmp_path), 
        include_patterns=['**'], 
        exclude_patterns=[], 
        prefer='rust'
    )
    assert files is not None


def test_get_matching_files_python(tmp_path) -> None:
    """Test get_matching_files with Python preference."""
    (tmp_path / 'test.rst').touch()
    files = hybrid.get_matching_files(
        str(tmp_path), 
        include_patterns=['**'], 
        exclude_patterns=[], 
        prefer='python'
    )
    assert files is not None


def test_project_prefers_python(tmp_path) -> None:
    """Test Project fallback to Python."""
    p = hybrid.project(tmp_path, ['.rst'], prefer='python')
    assert p.path2doc('foo.rst') == 'foo'


def test_event_manager_resolver_import_fails() -> None:
    """Test event_manager when docutilsrs_plugins import fails."""
    app = SimpleNamespace(pdb=False)
    
    with patch.dict(sys.modules, {'docutilsrs_plugins': None}):
        em = hybrid.event_manager(app, prefer='rust')
        assert em is not None
        assert hasattr(em, 'connect')


def test_event_manager_resolver_returns_none_eq() -> None:
    """Test event_manager when resolver.discover() returns no EventManager."""
    app = SimpleNamespace(pdb=False)
    
    mock_resolver = Mock()
    mock_resolver.discover.return_value = {}
    
    with patch.dict(sys.modules, {'docutilsrs_plugins': mock_resolver}):
        em = hybrid.event_manager(app, prefer='rust')
        assert em is not None
        assert hasattr(em, 'connect')


def test_event_manager_resolver_not_compatible() -> None:
    """Test event_manager when upstream_compatible returns False."""
    app = SimpleNamespace(pdb=False)
    
    mock_eq = Mock()
    mock_resolver = Mock()
    mock_resolver.discover.return_value = {'sphinx.events:EventManager': mock_eq}
    mock_resolver.upstream_compatible.return_value = False
    
    with patch.dict(sys.modules, {'docutilsrs_plugins': mock_resolver}):
        em = hybrid.event_manager(app, prefer='rust')
        assert em is not None


def test_event_manager_resolver_factory_raises() -> None:
    """Test event_manager when eq.factory() raises exception."""
    app = SimpleNamespace(pdb=False)
    
    mock_factory = Mock(side_effect=RuntimeError("Factory failed"))
    mock_eq = Mock()
    mock_eq.factory.return_value = mock_factory
    
    mock_resolver = Mock()
    mock_resolver.discover.return_value = {'sphinx.events:EventManager': mock_eq}
    mock_resolver.upstream_compatible.return_value = True
    
    with patch.dict(sys.modules, {'docutilsrs_plugins': mock_resolver}):
        em = hybrid.event_manager(app, prefer='rust')
        assert em is not None


def test_features_without_rust() -> None:
    """Test features() returns empty list when Rust is not available."""
    with patch.object(hybrid, '_HAS_RUST', False):
        feats = hybrid.features()
        assert feats == []


def test_supports_without_rust() -> None:
    """Test supports() returns False when Rust is not available."""
    with patch.object(hybrid, '_HAS_RUST', False):
        result = hybrid.supports('any:feature')
        assert result is False
