"""M1 smoke test: both Rust extensions import and expose version()."""

import re

import docutilsrs
import sphinxdocrs

VERSION_RE = re.compile(r"^\d+\.\d+\.\d+$")


def test_docutilsrs_version():
    v = docutilsrs.version()
    assert isinstance(v, str)
    assert VERSION_RE.match(v), v


def test_sphinxdocrs_version():
    v = sphinxdocrs.version()
    assert isinstance(v, str)
    assert VERSION_RE.match(v), v
