"""Regression tests for the top-level Makefile test tasks."""

from __future__ import annotations

import re
import subprocess
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[2]


def _strip_ansi(text: str) -> str:
    return re.sub(r"\x1B\[[0-9;]*[A-Za-z]", "", text)


def _strip_make_directory_lines(text: str) -> str:
    lines = []
    for line in text.splitlines():
        if re.match(r"^make\[\d+\]: (Entering|Leaving) directory ", line):
            continue
        lines.append(line)
    if not lines:
        return ""
    return "\n".join(lines) + "\n"


def _write_emit_script(tmp_path: Path) -> Path:
    script = tmp_path / "emit.sh"
    script.write_text(
        "#!/usr/bin/env bash\n"
        "set -euo pipefail\n"
        "label=${1:-task}\n"
        "printf '\\033[31m%s color stdout\\033[0m\\n' \"$label\"\n"
        "printf '%s plain stdout\\n' \"$label\"\n"
        "printf '\\033[32m%s color stderr\\033[0m\\n' \"$label\" >&2\n"
        "printf '%s plain stderr\\n' \"$label\" >&2\n",
        encoding="utf-8",
    )
    script.chmod(0o755)
    return script


@pytest.mark.parametrize("target", ["test2", "test3"])
def test_test_tasks_write_plain_build_log_from_colored_stream(target: str, tmp_path: Path) -> None:
    emit_script = _write_emit_script(tmp_path)
    run = subprocess.run(
        [
            "make",
            "-f",
            str(ROOT / "Makefile"),
            target,
            f"TEST2_CARGO_CMD=bash {emit_script} cargo",
            f"TEST2_PYTHON_CMD=bash {emit_script} python",
        ],
        cwd=tmp_path,
        text=True,
        capture_output=True,
        check=True,
    )

    stdout_path = tmp_path / f"{target}.stdout.log"
    stdout_path.write_text(run.stdout, encoding="utf-8")

    build_log = tmp_path / "build.log"
    assert build_log.exists(), f"{target} did not produce build.log"

    build_text = build_log.read_text(encoding="utf-8")
    stripped_stdout = _strip_ansi(run.stdout)
    stripped_path = tmp_path / f"{target}.stdout.stripped.log"
    stripped_path.write_text(stripped_stdout, encoding="utf-8")

    normalized_build = _strip_make_directory_lines(build_text)
    normalized_stdout = _strip_make_directory_lines(stripped_stdout)

    assert "\x1b[" in run.stdout
    assert "\x1b[" not in build_text
    assert normalized_build == normalized_stdout
    assert "build.log.ansi" not in {p.name for p in tmp_path.iterdir()}
