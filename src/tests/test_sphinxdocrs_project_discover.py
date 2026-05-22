"""Parity tests for ``sphinxdocrs.Project.discover``.

Mirrors a subset of ``src/sphinx/tests/test_project.py`` using a small
synthetic source tree under ``tmp_path``.
"""

from __future__ import annotations

from pathlib import Path

import sphinxdocrs
from sphinxdocrs import Project


def _make_tree(root: Path) -> None:
    (root / 'index.rst').write_text('')
    (root / 'guide.rst').write_text('')
    (root / 'note.txt').write_text('')  # ignored when only .rst is a suffix
    sub = root / 'sub'
    sub.mkdir()
    (sub / 'inner.rst').write_text('')
    (sub / 'inner.txt').write_text('')
    excluded = root / 'excluded'
    excluded.mkdir()
    (excluded / 'leaf.rst').write_text('')


def test_discover_basic(tmp_path: Path) -> None:
    _make_tree(tmp_path)
    p = Project(tmp_path, ['.rst'])
    docs = p.discover()
    assert docs == {'index', 'guide', 'sub/inner', 'excluded/leaf'}
    assert p.docnames == docs


def test_discover_exclude_patterns(tmp_path: Path) -> None:
    _make_tree(tmp_path)
    p = Project(tmp_path, ['.rst'])
    docs = p.discover(['excluded/*'])
    assert docs == {'index', 'guide', 'sub/inner'}


def test_discover_multiple_suffixes(tmp_path: Path) -> None:
    _make_tree(tmp_path)
    p = Project(tmp_path, ['.rst', '.txt'])
    docs = p.discover()
    assert docs == {
        'index',
        'guide',
        'note',
        'sub/inner',  # only the .rst variant wins (.rst is first suffix)
        'excluded/leaf',
    }


def test_discover_records_paths_for_doc2path(tmp_path: Path) -> None:
    _make_tree(tmp_path)
    p = Project(tmp_path, ['.rst'])
    p.discover()
    # Known doc: returns the recorded relative path.
    assert p.doc2path('index', absolute=False) == 'index.rst'
    assert p.doc2path('index', absolute=True) == str(tmp_path / 'index.rst')
    # Unknown doc: falls back to docname + first_suffix.
    assert p.doc2path('missing', absolute=False) == 'missing.rst'


def test_discover_applies_default_exclude_paths(tmp_path: Path) -> None:
    """``Project.discover`` must always exclude ``**/_sources`` (which is
    upstream's ``EXCLUDE_PATHS``). The ``**/`` prefix means it applies to
    nested ``_sources`` directories only — matching upstream behaviour
    where root-level ``_sources`` is NOT excluded by default."""
    (tmp_path / 'index.rst').write_text('')
    inner = tmp_path / 'guide'
    inner.mkdir()
    (inner / 'page.rst').write_text('')
    nested_sources = inner / '_sources'
    nested_sources.mkdir()
    (nested_sources / 'leaked.rst').write_text('')

    p = Project(tmp_path, ['.rst'])
    docs = p.discover()
    assert docs == {'index', 'guide/page'}


def test_supports_discover_flag() -> None:
    assert sphinxdocrs.supports('project:discover')
