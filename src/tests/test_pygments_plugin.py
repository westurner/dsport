"""End-to-end smoke test for the Pygments plugin-bridge example."""

import sys
from pathlib import Path

EXAMPLE_DIR = Path(__file__).resolve().parents[1] / "docutilsrs" / "python"
if str(EXAMPLE_DIR) not in sys.path:
    sys.path.insert(0, str(EXAMPLE_DIR))

import docutilsrs  # noqa: E402
import docutilsrs_pygments  # noqa: E402


def setup_function(_):
    docutilsrs.clear_directives()


def teardown_function(_):
    docutilsrs.clear_directives()


def test_pygments_directive_emits_raw_html():
    docutilsrs_pygments.register()
    src = ".. pyghl:: python\n\n   x = 1\n"
    out = docutilsrs.parse_to_pseudoxml(src)
    # Pygments wraps tokens in spans; we should see them inside a <raw> node.
    assert "<raw" in out
    assert 'format="html"' in out
    # The tokenized source text should still be present somewhere.
    assert "x" in out


def test_pygments_directive_unregister():
    docutilsrs_pygments.register()
    docutilsrs_pygments.unregister()
    assert "pyghl" not in docutilsrs.registered_directives()
