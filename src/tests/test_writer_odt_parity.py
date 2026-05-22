"""Byte-parity gate for ``docutilsrs.parse_to_odt(..., compat=True)``.

Mirrors the comparison performed by upstream's
``test/test_writers/test_odt.py`` (which is the reference test for
``docutils.writers.odf_odt``):

* extract ``content.xml`` from both the produced ``.odt`` zip and the
  expected ``.odt`` fixture
* parse with ``xml.etree.ElementTree``
* sort attributes per element (Python version independent)
* re-serialize with ``ET.tostring``
* byte-compare the results

The fixtures live under
``src/docutils/docutils/test/functional/{input,expected}/`` in the
vendored upstream source tree; this test simply re-uses them.
"""

from __future__ import annotations

from io import BytesIO
from pathlib import Path
import xml.etree.ElementTree as ET
import zipfile

import pytest

import docutilsrs


REPO_ROOT = Path(__file__).resolve().parents[2]
FUNCTIONAL = REPO_ROOT / "src" / "docutils" / "docutils" / "test" / "functional"
INPUT_DIR = FUNCTIONAL / "input"
EXPECTED_DIR = FUNCTIONAL / "expected"


# (rst fixture, odt fixture, settings_overrides-or-None).
# Mirrors the parametrization of upstream DocutilsOdtTestCase methods.
UPSTREAM_FIXTURES: list[tuple[str, str, dict | None]] = [
    ("odt_basic.rst", "odt_basic.odt", None),
    ("odt_nested_class.rst", "odt_nested_class.odt", None),
    ("odt_unnested_class.rst", "odt_unnested_class.odt", None),
    ("odt_no_class.rst", "odt_no_class.odt", None),
    ("odt_tables1.rst", "odt_tables1.odt", None),
    (
        "odt_custom_headfoot.rst",
        "odt_custom_headfoot.odt",
        {
            "custom_header": "Page %p% of %P%",
            "custom_footer": "Title: %t%  Date: %d3%  Time: %t4%",
            "language_code": "en-US",
        },
    ),
    ("odt_header_footer.rst", "odt_header_footer.odt", None),
    ("odt_literal_block.rst", "odt_literal_block.odt", None),
    ("odt_contents.rst", "odt_contents.odt", None),
    ("odt_classifier.rst", "odt_classifier.odt", None),
    ("odt_footnotes.rst", "odt_footnotes.odt", None),
    ("odt_raw.rst", "odt_raw.odt", None),
    ("odt_image.rst", "odt_image.odt", None),
]


def _reorder_attributes(root: ET.Element) -> None:
    """Sort each element's attributes; matches upstream's helper."""
    for el in root.iter():
        attrib = el.attrib
        if len(attrib) > 1:
            sorted_attribs = sorted(attrib.items())
            attrib.clear()
            attrib.update(sorted_attribs)


def _extract_normalized(payload: bytes, filename: str) -> bytes:
    """Reproduce upstream's ``DocutilsOdtTestCase.extract_file``."""
    zfile = zipfile.ZipFile(BytesIO(payload), "r")
    raw = zfile.read(filename)
    doc = ET.fromstring(raw)
    _reorder_attributes(doc)
    return ET.tostring(doc)


@pytest.fixture(scope="module")
def upstream_available() -> bool:
    pytest.importorskip("docutils")
    pytest.importorskip("docutils.writers.odf_odt")
    if not INPUT_DIR.is_dir() or not EXPECTED_DIR.is_dir():
        pytest.skip("vendored upstream functional fixtures not present")
    return True


@pytest.mark.parametrize(
    "rst_name,odt_name,overrides",
    UPSTREAM_FIXTURES,
    ids=[name for name, *_ in UPSTREAM_FIXTURES],
)
def test_odt_compat_matches_upstream_fixture(
    upstream_available: bool,
    rst_name: str,
    odt_name: str,
    overrides: dict | None,
) -> None:
    """``parse_to_odt(compat=True)`` must match the vendored ``.odt``."""
    del upstream_available  # marker fixture
    rst_path = INPUT_DIR / rst_name
    odt_path = EXPECTED_DIR / odt_name
    if not rst_path.exists() or not odt_path.exists():
        pytest.skip(f"fixture missing: {rst_path} or {odt_path}")
    source = rst_path.read_text(encoding="utf-8")
    expected = odt_path.read_bytes()

    # When upstream's fixture was produced with a settings override
    # (custom_headfoot), we cannot pass that through the current
    # `parse_to_odt(compat=True)` signature. Verify the bridge ran and
    # skip the byte-parity assertion; structural validity is enforced
    # by the zip check below.
    produced = docutilsrs.parse_to_odt(
        source,
        source_path=str(rst_path),
        compat=True,
        settings_overrides=overrides,
    )
    assert isinstance(produced, (bytes, bytearray))
    # The bridge always produces a valid ODT zip.
    z = zipfile.ZipFile(BytesIO(produced))
    assert z.namelist()[0] == "mimetype"

    actual_content = _extract_normalized(produced, "content.xml")
    expected_content = _extract_normalized(expected, "content.xml")
    assert actual_content == expected_content, (
        f"content.xml mismatch for {rst_name}\n"
        f"expected len={len(expected_content)} actual len={len(actual_content)}"
    )


def test_odt_compat_styles_xml_round_trips_as_xml() -> None:
    """``styles.xml`` from the compat path must be well-formed XML."""
    src = "Hello\n=====\n\nWorld.\n"
    data = docutilsrs.parse_to_odt(src, compat=True)
    z = zipfile.ZipFile(BytesIO(data))
    # Both the styles.xml and content.xml must parse.
    ET.fromstring(z.read("styles.xml"))
    ET.fromstring(z.read("content.xml"))


def test_odt_compat_and_native_paths_are_distinct() -> None:
    """Sanity: compat mode delegates to upstream; native is our writer."""
    src = "Hi.\n"
    a = docutilsrs.parse_to_odt(src, compat=True)
    b = docutilsrs.parse_to_odt(src, compat=False)
    # Both are valid zips.
    za = zipfile.ZipFile(BytesIO(a))
    zb = zipfile.ZipFile(BytesIO(b))
    assert za.read("mimetype") == b"application/vnd.oasis.opendocument.text"
    assert zb.read("mimetype") == b"application/vnd.oasis.opendocument.text"
    # Upstream emits significantly more boilerplate styling than ours.
    assert len(a) > len(b)


def test_features_advertises_odt_compat() -> None:
    feats = docutilsrs.features()
    assert "writer:odt" in feats
    assert "writer:odt_compat" in feats
    assert docutilsrs.supports("writer:odt_compat")
