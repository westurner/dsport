"""Pytest configuration for dsport tests.

Ensures vendored pygments lexers are discoverable by prioritizing
the local vendored pygments package over system-installed versions.
"""

import sys
from pathlib import Path

# Add vendored pygments to path BEFORE any pygments import
# This allows byte-parity tests to access lexers like pygments.lexers.bitbake
vendored_pygments = Path(__file__).parent.parent / "pygments"
if vendored_pygments.exists():
    vendored_path = str(vendored_pygments)
    sys.path.insert(0, vendored_path)
    
    # If pygments is already imported, remove it from cache so it gets reimported
    # from the vendored location
    modules_to_remove = [m for m in sys.modules if m.startswith('pygments')]
    for module in modules_to_remove:
        del sys.modules[module]
