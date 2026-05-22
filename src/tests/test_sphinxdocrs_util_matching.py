"""Parity tests for ``sphinxdocrs.util.matching`` port.

Mirrors ``src/sphinx/tests/test_util/test_util_matching.py`` for the
``compile_matchers`` / ``Matcher`` / ``get_matching_files`` surface.
"""

from __future__ import annotations

import os
from pathlib import Path

import pytest

import sphinxdocrs
from sphinxdocrs import Matcher, compile_matchers, get_matching_files


def test_compile_matchers_exact() -> None:
    pat = compile_matchers(['hello.py']).pop()
    assert pat('hello.py')
    assert not pat('hello-py')
    assert not pat('subdir/hello.py')


def test_compile_matchers_single_star() -> None:
    pat = compile_matchers(['*.py']).pop()
    assert pat('hello.py')
    assert pat('world.py')
    assert not pat('subdir/hello.py')


def test_compile_matchers_double_star() -> None:
    pat = compile_matchers(['**.py']).pop()
    assert pat('hello.py')
    assert pat('subdir/hello.py')

    pat = compile_matchers(['**/hello.py']).pop()
    assert not pat('hello.py')
    assert pat('subdir/hello.py')
    assert pat('subdir/subdir/hello.py')


def test_compile_matchers_question() -> None:
    pat = compile_matchers(['hello.?']).pop()
    assert pat('hello.c')
    assert not pat('hello.py')


def test_compile_matchers_class() -> None:
    pat = compile_matchers(['hello[12].py']).pop()
    assert pat('hello1.py')
    assert pat('hello2.py')
    assert not pat('hello3.py')

    pat = compile_matchers(['hello[^12].py']).pop()  # "^" is literal
    assert pat('hello1.py')
    assert pat('hello^.py')
    assert not pat('hello3.py')


def test_compile_matchers_class_negate() -> None:
    pat = compile_matchers(['hello[!12].py']).pop()
    assert not pat('hello1.py')
    assert not pat('hello/.py')  # negative pattern doesn't match slashes
    assert pat('hello3.py')


def test_compile_matchers_non_pattern() -> None:
    # Unterminated bracket → literal [.
    pat = compile_matchers(['hello[.py']).pop()
    assert pat('hello[.py')
    assert not pat('hello.py')


def test_matcher_expands_doublestar_prefix() -> None:
    matcher = Matcher(['hello.py', '**/world.py'])
    assert matcher('hello.py')
    assert not matcher('subdir/hello.py')
    assert matcher('world.py')
    assert matcher('subdir/world.py')


def test_get_matching_files_includes_recursively(tmp_path: Path) -> None:
    (tmp_path / 'a.txt').write_text('')
    (tmp_path / 'b.py').write_text('')
    sub = tmp_path / 'sub'
    sub.mkdir()
    (sub / 'c.txt').write_text('')

    files = sorted(get_matching_files(tmp_path))
    assert files == ['a.txt', 'b.py', 'sub/c.txt']


def test_get_matching_files_exclude(tmp_path: Path) -> None:
    (tmp_path / 'a.txt').write_text('')
    (tmp_path / 'b.html').write_text('')
    sub = tmp_path / 'sub'
    sub.mkdir()
    (sub / 'c.html').write_text('')
    (sub / 'd.txt').write_text('')

    files = sorted(get_matching_files(tmp_path, exclude_patterns=['**.html']))
    assert files == ['a.txt', 'sub/d.txt']


def test_get_matching_files_excludes_directory(tmp_path: Path) -> None:
    """Excluding a directory pattern prunes the whole subtree."""
    (tmp_path / 'keep.txt').write_text('')
    excluded = tmp_path / '_sources'
    excluded.mkdir()
    (excluded / 'drop.txt').write_text('')

    files = sorted(
        get_matching_files(tmp_path, exclude_patterns=['**/_sources', '_sources'])
    )
    assert files == ['keep.txt']


def test_supports_util_matching_flag() -> None:
    assert sphinxdocrs.supports('util:matching')
