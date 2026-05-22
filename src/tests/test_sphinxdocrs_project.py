"""Parity tests for ``sphinxdocrs.Project`` (path2doc / doc2path).

Mirrors a subset of ``src/sphinx/tests/test_project.py``; full
``discover()`` coverage is deferred to Phase 4 P2 (depends on
util/matching).
"""

from __future__ import annotations

from pathlib import Path

import pytest

from sphinxdocrs import Project


@pytest.fixture
def rootdir(tmp_path: Path) -> Path:
    """Lightweight rootdir; the path itself doesn't need to exist on disk
    for ``path2doc`` since it only does string-level matching."""
    return tmp_path


def test_path2doc_basic(rootdir: Path) -> None:
    project = Project(rootdir / 'test-basic', {'.rst': 'restructuredtext'})
    assert project.path2doc('index.rst') == 'index'
    assert project.path2doc('index.foo') is None
    assert project.path2doc('index.foo.rst') == 'index.foo'
    assert project.path2doc('index') is None
    assert project.path2doc('path/to/index.rst') == 'path/to/index'
    assert project.path2doc(rootdir / 'test-basic' / 'to/index.rst') == 'to/index'


def test_path2doc_multiple_suffixes(rootdir: Path) -> None:
    project = Project(rootdir / 'src', ['.rst', '.txt'])
    assert project.path2doc('a.rst') == 'a'
    assert project.path2doc('a.txt') == 'a'
    assert project.path2doc('a.md') is None


def test_doc2path_relative(rootdir: Path) -> None:
    project = Project(rootdir / 'src', ['.rst', '.txt'])
    # First suffix wins for unknown docs.
    assert project.doc2path('index', absolute=False) == 'index.rst'


def test_doc2path_absolute(rootdir: Path) -> None:
    src = rootdir / 'src'
    project = Project(src, ['.rst'])
    expected = str(src / 'index.rst')
    assert project.doc2path('index', absolute=True) == expected


def test_project_getters(rootdir: Path) -> None:
    project = Project(rootdir / 'src', ['.rst', '.txt'])
    assert project.srcdir == str(rootdir / 'src')
    assert list(project.source_suffix) == ['.rst', '.txt']
