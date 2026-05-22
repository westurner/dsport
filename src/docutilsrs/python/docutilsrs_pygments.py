"""Example: register a Pygments-based ``pyghl`` directive via the
docutilsrs Python plugin bridge.

The Rust core does not bundle Pygments. This module demonstrates how
to extend the parser at runtime by registering a Python callable that
receives ``(args, body)`` (the directive's argument string and indented
body text) and returns replacement rST. The Rust parser re-parses the
returned string in place of the original directive.

Usage::

    import docutilsrs
    import docutilsrs_pygments
    docutilsrs_pygments.register()

    src = '''
    .. pyghl:: python

       def hello():
           return 1
    '''
    print(docutilsrs.parse_to_pseudoxml(src))
"""

from __future__ import annotations

import docutilsrs

try:
    from pygments import highlight
    from pygments.formatters import HtmlFormatter
    from pygments.lexers import get_lexer_by_name
except ImportError:  # pragma: no cover - optional dep
    highlight = None


def _pyghl_callable(args: str, body: str) -> str:
    """Return rST that wraps Pygments-produced HTML in a ``raw:: html`` block."""
    if highlight is None:
        # Fall back to a plain literal block if Pygments is unavailable.
        indented = "\n".join("   " + line for line in body.splitlines())
        return "::\n\n" + indented + "\n"
    lang = (args.strip() or "text").split()[0]
    try:
        lexer = get_lexer_by_name(lang)
    except Exception:
        lexer = get_lexer_by_name("text")
    formatter = HtmlFormatter(nowrap=True)
    rendered = highlight(body, lexer, formatter)
    indented = "\n".join("   " + line for line in rendered.splitlines())
    return ".. raw:: html\n\n" + indented + "\n"


def register() -> None:
    """Install the ``pyghl`` directive into the docutilsrs plugin registry."""
    docutilsrs.register_directive("pyghl", _pyghl_callable)


def unregister() -> None:
    """Remove the ``pyghl`` directive from the plugin registry."""
    docutilsrs.unregister_directive("pyghl")
