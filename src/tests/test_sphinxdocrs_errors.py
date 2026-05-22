"""Parity tests for ``sphinxdocrs.errors`` mirroring ``sphinx.errors``.

Upstream test counterpart: ``src/sphinx/tests/test_errors.py``.
"""

from __future__ import annotations

import sphinxdocrs
from sphinxdocrs import ExtensionError


def test_extension_error_repr() -> None:
    exc = ExtensionError("foo")
    assert repr(exc) == "ExtensionError('foo')"


def test_extension_error_with_orig_exc_repr() -> None:
    exc = ExtensionError("foo", Exception("bar"))
    # Default tuple repr from Python's BaseException.__repr__.
    assert repr(exc) == "ExtensionError('foo', Exception('bar'))"


def test_error_hierarchy_subclasses_sphinxerror() -> None:
    for name in (
        "SphinxWarning",
        "ApplicationError",
        "ExtensionError",
        "BuildEnvironmentError",
        "ConfigError",
        "DocumentError",
        "ThemeError",
        "VersionRequirementError",
        "SphinxParallelError",
    ):
        cls = getattr(sphinxdocrs, name)
        assert issubclass(cls, sphinxdocrs.SphinxError), name


def test_standalone_exceptions_inherit_from_exception() -> None:
    for name in ("PycodeError", "NoUri", "FiletypeNotFoundError"):
        cls = getattr(sphinxdocrs, name)
        assert issubclass(cls, Exception)
        assert not issubclass(cls, sphinxdocrs.SphinxError), name


def test_supports_errors_feature() -> None:
    assert sphinxdocrs.supports("errors:sphinx_hierarchy") is True
