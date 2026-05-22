"""Parity tests for ``sphinxdocrs.Extension`` and ``verify_needs_extensions``.

Mirrors the data shape of ``sphinx.extension.Extension`` and the
behaviour of ``sphinx.extension.verify_needs_extensions``.
"""

from __future__ import annotations

from types import SimpleNamespace

import pytest

from sphinxdocrs import Extension, VersionRequirementError, verify_needs_extensions


class _FakeModule:
    pass


def test_extension_defaults() -> None:
    ext = Extension('myext', _FakeModule())
    assert ext.name == 'myext'
    assert ext.version == 'unknown version'
    assert ext.parallel_read_safe is None
    assert ext.parallel_write_safe is True
    assert ext.metadata == {}


def test_extension_pops_known_keys_from_metadata() -> None:
    metadata = {
        'version': '1.2.3',
        'parallel_read_safe': True,
        'parallel_write_safe': False,
        'env_version': 4,
    }
    ext = Extension('myext', _FakeModule(), **metadata)
    assert ext.version == '1.2.3'
    assert ext.parallel_read_safe is True
    assert ext.parallel_write_safe is False
    # Only the non-well-known keys remain in metadata. (Upstream uses
    # the same dict object, so it ends up with only ``env_version``.)
    assert ext.metadata == {'env_version': 4}


def test_extension_preserves_explicit_none() -> None:
    """``version=None`` is an explicit value and must not be replaced by the
    ``'unknown version'`` default. (Upstream's ``pop`` only uses the
    default when the key is absent.)"""
    ext = Extension('myext', _FakeModule(), version=None)
    assert ext.version is None


def test_verify_needs_extensions_no_requirements() -> None:
    app = SimpleNamespace(extensions={})
    config = SimpleNamespace(needs_extensions=None)
    # Should be a no-op.
    verify_needs_extensions(app, config)


def test_verify_needs_extensions_missing_extension_warns_only(caplog) -> None:
    import logging
    caplog.set_level(logging.WARNING, logger='sphinxdocrs.extension')
    app = SimpleNamespace(extensions={})
    config = SimpleNamespace(needs_extensions={'missing.ext': '1.0'})
    # No raise — upstream only warns.
    verify_needs_extensions(app, config)


def test_verify_needs_extensions_version_too_old() -> None:
    ext = Extension('myext', _FakeModule(), version='0.9')
    app = SimpleNamespace(extensions={'myext': ext})
    config = SimpleNamespace(needs_extensions={'myext': '1.0'})
    with pytest.raises(VersionRequirementError):
        verify_needs_extensions(app, config)


def test_verify_needs_extensions_unknown_version_rejected() -> None:
    ext = Extension('myext', _FakeModule())  # default 'unknown version'
    app = SimpleNamespace(extensions={'myext': ext})
    config = SimpleNamespace(needs_extensions={'myext': '1.0'})
    with pytest.raises(VersionRequirementError):
        verify_needs_extensions(app, config)


def test_verify_needs_extensions_satisfied() -> None:
    ext = Extension('myext', _FakeModule(), version='1.5')
    app = SimpleNamespace(extensions={'myext': ext})
    config = SimpleNamespace(needs_extensions={'myext': '1.0'})
    verify_needs_extensions(app, config)  # no raise
