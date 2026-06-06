"""Tests for docutilsrs_plugins resolver module."""

from __future__ import annotations

import importlib
import sys
from unittest.mock import Mock, patch, MagicMock

import pytest

import docutilsrs_plugins as plugins


# Fixtures for cleanup
@pytest.fixture(autouse=True)
def clear_registry():
    """Clear registry before and after each test."""
    plugins.clear_registry()
    yield
    plugins.clear_registry()


# Tests for registration and discovery


def test_register_and_discover():
    """Test basic registration and discovery."""
    def my_factory():
        return "my_impl"
    
    plugins.register("test.target:Symbol", my_factory)
    eq = plugins.discover().get("test.target:Symbol")
    assert eq is not None
    assert eq.factory() == "my_impl"


def test_unregister():
    """Test unregister removes a registration."""
    def factory():
        return "impl"
    
    plugins.register("test:target", factory)
    assert "test:target" in plugins.discover()
    
    plugins.unregister("test:target")
    assert "test:target" not in plugins.discover()


def test_unregister_nonexistent():
    """Test unregister is safe for non-existent targets."""
    plugins.unregister("nonexistent:target")
    # Should not raise


def test_clear_registry():
    """Test clear_registry removes all registrations."""
    plugins.register("t1:s1", lambda: "1")
    plugins.register("t2:s2", lambda: "2")
    assert len(plugins.discover()) >= 2
    
    plugins.clear_registry()
    assert len(plugins.discover()) == 0


def test_register_with_upstream_requires():
    """Test registration with version constraint."""
    plugins.register(
        "test:target",
        lambda: "impl",
        upstream_requires=">=1.0,<2.0"
    )
    eq = plugins.discover().get("test:target")
    assert eq.upstream_requires == ">=1.0,<2.0"


def test_entry_point_equivalents_python310():
    """Test _entry_point_equivalents with Python 3.10+ API."""
    with patch('importlib.metadata.entry_points') as mock_ep:
        # Mock successful entry_points call with group parameter
        mock_ep.return_value = []
        result = plugins._entry_point_equivalents()
        assert result == {}


def test_entry_point_equivalents_python39_fallback():
    """Test _entry_point_equivalents with Python 3.9 fallback."""
    # This tests lines 125-126
    with patch('importlib.metadata.entry_points') as mock_ep:
        # Simulate Python 3.9 by raising TypeError for group parameter
        mock_ep.side_effect = TypeError("group parameter not supported")
        
        # Mock the old API
        mock_ep_dict = {}
        
        def entry_points_old(*args, **kwargs):
            if 'group' in kwargs:
                raise TypeError()
            return {'docutilsrs.equivalents': []}
        
        with patch('importlib.metadata.entry_points', entry_points_old):
            result = plugins._entry_point_equivalents()
            assert isinstance(result, dict)


def test_entry_point_discovery_with_lazy_loading():
    """Test that entry point factories defer loading (covers line 130)."""
    mock_ep = Mock()
    mock_ep.name = "test.module:Class"
    
    # Create a mock that tracks calls separately
    load_call_count = [0]
    def mock_load():
        load_call_count[0] += 1
        return "loaded_impl"
    
    mock_ep.load = mock_load
    
    with patch('importlib.metadata.entry_points') as mock_eps_call:
        try:
            mock_eps_call.return_value = [mock_ep]
        except TypeError:
            # Python 3.9 fallback
            mock_eps_call.return_value = {'docutilsrs.equivalents': [mock_ep]}
        
        load_call_count[0] = 0
        result = plugins._entry_point_equivalents()
        # Should have the entry point (covers line 141 and surrounding)
        assert "test.module:Class" in result
        # load() was called once for _upstream_requires
        assert load_call_count[0] == 1
        
        # Now call the factory - this covers line 130 (_factory closure)
        impl = result["test.module:Class"].factory()
        assert impl == "loaded_impl"
        # Factory should call load() again
        assert load_call_count[0] == 2


def test_upstream_requires_lazy_loading_with_exception():
    """Test upstream_requires loading when entry point fails."""
    mock_ep = Mock()
    mock_ep.name = "test:target"
    mock_ep.load.side_effect = ImportError("Module not found")
    
    with patch('importlib.metadata.entry_points') as mock_eps_call:
        try:
            mock_eps_call.return_value = [mock_ep]
        except TypeError:
            mock_eps_call.return_value = {'docutilsrs.equivalents': [mock_ep]}
        
        result = plugins._entry_point_equivalents()
        # _upstream_requires() should catch the exception
        eq = result.get("test:target")
        assert eq is not None
        # upstream_requires should be None when load fails
        assert eq.upstream_requires is None


def test_upstream_requires_attribute_present():
    """Test retrieving __upstream_requires__ attribute from loaded object."""
    mock_obj = Mock()
    mock_obj.__upstream_requires__ = ">=1.0"
    
    mock_ep = Mock()
    mock_ep.name = "test:target"
    mock_ep.load.return_value = mock_obj
    
    with patch('importlib.metadata.entry_points') as mock_eps_call:
        try:
            mock_eps_call.return_value = [mock_ep]
        except TypeError:
            mock_eps_call.return_value = {'docutilsrs.equivalents': [mock_ep]}
        
        result = plugins._entry_point_equivalents()
        eq = result.get("test:target")
        assert eq.upstream_requires == ">=1.0"


# Tests for version matching


def test_split_upstream_package():
    """Test package extraction from dotted target."""
    assert plugins._split_upstream_package("sphinx.events:EventManager") == "sphinx"
    assert plugins._split_upstream_package("docutils.writers:Writer") == "docutils"
    assert plugins._split_upstream_package("single:Class") == "single"


def test_installed_upstream_version_via_metadata():
    """Test version lookup via importlib.metadata."""
    with patch('importlib.metadata.version') as mock_version:
        mock_version.return_value = "2.1.0"
        version = plugins._installed_upstream_version("sphinx.events:EventManager")
        assert version == "2.1.0"


def test_installed_upstream_version_not_found():
    """Test version lookup when package not in metadata."""
    # This tests line 171-172
    with patch('importlib.metadata.version') as mock_version:
        mock_version.side_effect = importlib.metadata.PackageNotFoundError()
        
        # Will fallback to import
        with patch('importlib.import_module') as mock_import:
            mock_module = Mock()
            mock_module.__version__ = "1.5.0"
            mock_import.return_value = mock_module
            
            version = plugins._installed_upstream_version("test.module:Class")
            assert version == "1.5.0"


def test_installed_upstream_version_import_fails():
    """Test version lookup when both metadata and import fail."""
    # Tests lines 174-176
    with patch('importlib.metadata.version') as mock_version:
        mock_version.side_effect = importlib.metadata.PackageNotFoundError()
        
        with patch('importlib.import_module') as mock_import:
            mock_import.side_effect = ImportError("Module not found")
            
            version = plugins._installed_upstream_version("nonexistent.module:Class")
            assert version is None


def test_installed_upstream_version_no_version_attr():
    """Test version lookup when module has no __version__."""
    # Tests line 177
    with patch('importlib.metadata.version') as mock_version:
        mock_version.side_effect = importlib.metadata.PackageNotFoundError()
        
        with patch('importlib.import_module') as mock_import:
            mock_module = Mock(spec=[])  # No __version__ attribute
            mock_import.return_value = mock_module
            
            version = plugins._installed_upstream_version("test.module:Class")
            assert version is None


def test_specifier_matches_with_packaging():
    """Test PEP 440 specifier matching when packaging is available."""
    result = plugins._specifier_matches(">=1.0,<2.0", "1.5.0")
    assert result is True
    
    result = plugins._specifier_matches(">=1.0,<2.0", "2.5.0")
    assert result is False


def test_specifier_matches_packaging_import_fails():
    """Test specifier matching when packaging import raises exception."""
    # This tests lines 185-187 - the exception handler when importing packaging
    
    # Directly patch the imports inside _specifier_matches
    import builtins
    original_import = builtins.__import__
    
    def failing_import(name, *args, **kwargs):
        if 'packaging' in name:
            raise ImportError("packaging not installed")
        return original_import(name, *args, **kwargs)
    
    with patch('builtins.__import__', side_effect=failing_import):
        # When packaging import fails, should return True (permissive)
        result = plugins._specifier_matches(">=1.0", "1.5.0")
        assert result is True  # Permissive when packaging unavailable


def test_specifier_matches_version_error_in_parser():
    """Test specifier matching when Version parsing fails."""
    # This tests lines 190-191 - exception in SpecifierSet evaluation
    try:
        from packaging.specifiers import SpecifierSet
        from packaging.version import Version
        
        # Use an invalid spec that will raise when evaluated
        result = plugins._specifier_matches("invalid_spec_!!!!", "1.5.0")
        # Should catch and return False
        assert result is False
    except ImportError:
        # packaging not available, test would be skipped
        pass


def test_upstream_compatible_no_constraint():
    """Test upstream_compatible when no constraint is set."""
    # Tests line 196-197
    eq = plugins.Equivalent(target="test:target", factory=lambda: "impl")
    assert plugins.upstream_compatible(eq) is True


def test_upstream_compatible_unknown_version():
    """Test upstream_compatible when installed version is unknown."""
    # Tests lines 199-201
    eq = plugins.Equivalent(
        target="nonexistent.pkg:Symbol",
        factory=lambda: "impl",
        upstream_requires=">=1.0"
    )
    with patch.object(plugins, '_installed_upstream_version', return_value=None):
        # Unknown version is permissive
        assert plugins.upstream_compatible(eq) is True


def test_upstream_compatible_version_mismatch():
    """Test upstream_compatible when version doesn't match."""
    eq = plugins.Equivalent(
        target="test:target",
        factory=lambda: "impl",
        upstream_requires=">=1.0,<2.0"
    )
    with patch.object(plugins, '_installed_upstream_version', return_value="3.0.0"):
        assert plugins.upstream_compatible(eq) is False


def test_upstream_compatible_version_match():
    """Test upstream_compatible when version matches."""
    eq = plugins.Equivalent(
        target="test:target",
        factory=lambda: "impl",
        upstream_requires=">=1.0,<3.0"
    )
    with patch.object(plugins, '_installed_upstream_version', return_value="2.0.0"):
        assert plugins.upstream_compatible(eq) is True


# Tests for _load_python


def test_load_python_with_colon_syntax():
    """Test _load_python with module:attribute syntax."""
    # This tests the happy path at lines 212-213
    result = plugins._load_python("docutils.parsers.rst:Parser")
    assert result is not None
    # Should be the Parser class
    assert hasattr(result, '__name__')


def test_load_python_with_dot_syntax():
    """Test _load_python with dotted attribute syntax."""
    # This tests lines 214-215
    result = plugins._load_python("docutils.parsers.rst.Parser")
    assert result is not None


def test_load_python_with_nested_attributes():
    """Test _load_python with deeply nested attributes."""
    # Test that it properly traverses nested attributes
    with patch('importlib.import_module') as mock_import:
        mock_module = Mock()
        mock_module.level1 = Mock()
        mock_module.level1.level2 = Mock()
        mock_module.level1.level2.Symbol = "target_impl"
        mock_import.return_value = mock_module
        
        result = plugins._load_python("test.module:level1.level2.Symbol")
        assert result == "target_impl"


def test_load_python_invalid_syntax():
    """Test _load_python with invalid target syntax."""
    # This tests lines 216-217
    with pytest.raises(ValueError):
        plugins._load_python("invalid")


# Tests for resolve


def test_resolve_prefer_rust_found():
    """Test resolve finds Rust equivalent with prefer='rust'."""
    def factory():
        return "rust_impl"
    
    plugins.register("test:target", factory)
    result = plugins.resolve("test:target", prefer="rust")
    
    assert result.impl == "rust_impl"
    assert result.source == "rust"
    assert result.reason == "rust-equivalent"


def test_resolve_rust_factory_fails():
    """Test resolve fallback when Rust factory raises exception."""
    def failing_factory():
        raise RuntimeError("Factory error")
    
    plugins.register("test:target", failing_factory)
    
    # Mock the Python implementation
    with patch.object(plugins, '_load_python') as mock_load:
        mock_load.return_value = "python_impl"
        
        with patch('warnings.warn') as mock_warn:
            result = plugins.resolve("test:target", prefer="rust")
        
        assert result.impl == "python_impl"
        assert result.source == "python"
        # Should have warned
        mock_warn.assert_called_once()


def test_resolve_rust_version_guard_fails():
    """Test resolve when version guard fails."""
    # This tests lines 254-263
    def factory():
        return "rust_impl"
    
    plugins.register("test:target", factory, upstream_requires=">=1.0,<2.0")
    
    with patch.object(plugins, '_installed_upstream_version', return_value="3.0.0"):
        with patch.object(plugins, '_load_python') as mock_load:
            mock_load.return_value = "python_impl"
            
            with patch('warnings.warn') as mock_warn:
                result = plugins.resolve("test:target", prefer="rust")
            
            # Should fallback to Python
            assert result.impl == "python_impl"
            assert result.source == "python"
            mock_warn.assert_called_once()


def test_resolve_prefer_python():
    """Test resolve with prefer='python' ignores Rust."""
    def factory():
        return "rust_impl"
    
    plugins.register("test:target", factory)
    
    with patch.object(plugins, '_load_python') as mock_load:
        mock_load.return_value = "python_impl"
        
        result = plugins.resolve("test:target", prefer="python")
    
    assert result.impl == "python_impl"
    assert result.source == "python"
    assert "python-preferred" in result.reason


def test_dispatch_convenience_wrapper():
    """Test dispatch returns just the implementation."""
    def factory():
        return "impl"
    
    plugins.register("test:target", factory)
    impl = plugins.dispatch("test:target", prefer="rust")
    assert impl == "impl"


def test_dispatch_plan_rust_preference():
    """Test dispatch_plan with prefer='rust'."""
    def factory():
        return "impl"
    
    plugins.register("t1:target", factory)
    
    plan = plugins.dispatch_plan(["t1:target", "t2:target"], prefer="rust")
    
    assert plan["t1:target"] == "rust"
    assert plan["t2:target"] == "python"


def test_dispatch_plan_python_preference():
    """Test dispatch_plan with prefer='python'."""
    def factory():
        return "impl"
    
    plugins.register("t1:target", factory)
    
    plan = plugins.dispatch_plan(["t1:target", "t2:target"], prefer="python")
    
    # All should be python when prefer='python'
    assert plan["t1:target"] == "python"
    assert plan["t2:target"] == "python"


def test_dispatch_plan_version_guard():
    """Test dispatch_plan respects version guards."""
    def factory():
        return "impl"
    
    plugins.register("test:target", factory, upstream_requires=">=1.0,<2.0")
    
    # Simulate version mismatch
    with patch.object(plugins, '_installed_upstream_version', return_value="3.0.0"):
        plan = plugins.dispatch_plan(["test:target"], prefer="rust")
        assert plan["test:target"] == "python"
