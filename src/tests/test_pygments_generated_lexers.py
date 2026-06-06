"""Byte-parity tests for the transpiled (`tools/gen_lexer.py`) lexers.

Each native Rust lexer in ``pygmentsrs/src/lexers/generated/`` must
produce a `(repr(ttype), value)` stream identical to the vendored
``pygments`` lexer it was generated from. Mirrors the strategy used by
``test_pygments_json_lexer.py`` and the ``code_block_python_*`` fixtures.

If a generated lexer ever drifts from upstream, this test fails and the
fix is to regenerate (or to mark the lexer bridge-only).
"""

from __future__ import annotations

import importlib

import pytest

pytest.importorskip("pygments")
import pygmentsrs


# alias -> (module, ClassName, [sample inputs])
GENERATED = {
    "ini": (
        "pygments.lexers.configs",
        "IniLexer",
        [
            "[section]\nkey = value\n",
            "; comment\n# also comment\n[s]\nk: v\n",
            'name = "quoted value"\n',
            "a = 1 \\\n    continued\n",
            "[empty]\n\nlonely\n",
        ],
    ),
    "properties": (
        "pygments.lexers.configs",
        "PropertiesLexer",
        [
            "a.b.c = value\n",
            "# comment\n! also comment\nkey:val\n",
            "escaped\\=key = v\n",
            "multi = one \\\n   two\n",
            "key value without separator\n",
        ],
    ),
    "toml": (
        "pygments.lexers.configs",
        "TOMLLexer",
        [
            '[table]\nkey = "value"\n',
            "n = 42\nf = 3.14\nb = true\n",
            "# comment\n[a.b.c]\nx = [1, 2, 3]\n",
            'date = 1979-05-27\nname = "x"\n',
            "arr = [\n  1,\n  2,\n]\n",
        ],
    ),
    "pot": (
        "pygments.lexers.textfmts",
        "GettextLexer",
        [
            'msgid "hello"\nmsgstr "bonjour"\n',
            '# translator comment\nmsgid ""\nmsgstr ""\n',
            '#: source.c:42\nmsgid "x"\nmsgstr "y"\n',
        ],
    ),
    "dpatch": (
        "pygments.lexers.diff",
        "DarcsPatchLexer",
        [
            "hunk ./file 1\n+added\n-removed\n",
            "addfile ./newfile\n",
        ],
    ),
    "vctreestatus": (
        "pygments.lexers.console",
        "VCTreeStatusLexer",
        [
            "? untracked\nA added\nM modified\nD deleted\n",
            "  C conflict\n",
        ],
    ),
    "groff": (
        "pygments.lexers.text",
        "GroffLexer",
        [
            ".TH TITLE 1\n.SH NAME\nfoo \\- bar\n",
            ".B bold\n.I italic\n\\fBmanual\\fP\n",
            ".\\\" a comment\nplain text\n",
        ],
    ),
    "bash": (
        "pygments.lexers.shell",
        "BashLexer",
        [
            "echo hi\nx=1\nif true; then echo $x; fi\n",
            # heredoc exercises the `\\2` backreference (fancy-regex).
            "cat <<EOF\nbody $v\nEOF\nrest\n",
            "for i in 1 2 3; do\n  echo $i\ndone\n",
            "func() {\n  local a=$1\n  return 0\n}\n",
            "x=${y:-default}\necho \"$x\"\n",
        ],
    ),
    "cmake": (
        "pygments.lexers.make",
        "CMakeLexer",
        [
            "set(X 1)\n# comment\nproject(Foo)\n",
            # bracket-argument exercises the `(?P=level)` named backref.
            "message([[\nmulti\n]])\n",
            "if(WIN32)\n  add_library(a)\nendif()\n",
        ],
    ),
}


def _upstream(module: str, classname: str, src: str) -> list[tuple[str, str]]:
    mod = importlib.import_module(module)
    cls = getattr(mod, classname)
    return [(repr(t), v) for _idx, t, v in cls().get_tokens_unprocessed(src)]


def _native(alias: str, src: str) -> list[tuple[str, str]]:
    pairs = pygmentsrs.lex(alias, src, backend="rust")
    assert pairs is not None, f"{alias} should be a native pygmentsrs alias"
    return [(repr_t, val) for repr_t, val in pairs]


def _cases() -> list[tuple[str, str, str, str]]:
    out = []
    for alias, (module, classname, samples) in GENERATED.items():
        for i, src in enumerate(samples):
            out.append((alias, module, classname, src, i))
    return out


@pytest.mark.parametrize(
    "alias,module,classname,src,idx",
    [(a, m, c, s, i) for (a, m, c, s, i) in _cases()],
    ids=[f"{a}-{i}" for (a, _m, _c, _s, i) in _cases()],
)
def test_generated_lexer_byte_parity(
    alias: str, module: str, classname: str, src: str, idx: int
) -> None:
    assert _native(alias, src) == _upstream(module, classname, src), (
        f"{alias} sample #{idx} diverged from upstream"
    )


@pytest.mark.parametrize("alias", sorted(GENERATED))
def test_generated_alias_is_native(alias: str) -> None:
    assert pygmentsrs.has_native_lexer(alias)
    assert alias in pygmentsrs.native_aliases()


def test_generated_routes_through_auto_backend() -> None:
    # Auto backend must reach the native lexer (not fall through to the
    # python bridge) for a generated alias.
    src = "[s]\nk = 1\n"
    assert pygmentsrs.lex("ini", src, backend="auto") == pygmentsrs.lex(
        "ini", src, backend="rust"
    )
