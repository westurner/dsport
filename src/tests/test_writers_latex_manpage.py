"""Smoke tests for the minimal LaTeX and manpage writers.

These writers are not parity-gated against `docutils.writers.latex2e` /
`docutils.writers.manpage`; we only assert that they produce sensible,
predictable output for the supported node kinds. See `docs/compat.md`.
"""

import docutilsrs


SOURCE = """\
Title
=====

Hello *world* with **bold**.

- one
- two

1. first
2. second
"""


def test_latex_writer_smoke() -> None:
    out = docutilsrs.parse_to_latex(SOURCE)
    assert out.startswith("\\documentclass{article}")
    assert "\\begin{document}" in out
    assert "\\end{document}" in out
    assert "\\emph{world}" in out
    assert "\\textbf{bold}" in out
    assert "\\begin{itemize}" in out
    assert "\\begin{enumerate}" in out


def test_latex_writer_escapes_specials() -> None:
    src = "Has $ and % and # and _ and & chars.\n"
    out = docutilsrs.parse_to_latex(src)
    for esc in ("\\$", "\\%", "\\#", "\\_", "\\&"):
        assert esc in out, f"missing escape {esc!r}"


def test_manpage_writer_smoke() -> None:
    out = docutilsrs.parse_to_manpage(SOURCE)
    assert out.startswith(".TH ")
    assert ".SH Title" in out
    assert "\\fIworld\\fR" in out
    assert "\\fBbold\\fR" in out
    assert ".IP \\(bu" in out
    assert ".IP 1." in out


def test_features_advertises_new_writers() -> None:
    feats = docutilsrs.features()
    assert "writer:latex" in feats
    assert "writer:manpage" in feats
    assert docutilsrs.supports("writer:latex")
    assert docutilsrs.supports("writer:manpage")
