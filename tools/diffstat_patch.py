#!/usr/bin/env python3
"""Print per-file insertion and deletion counts from a unified diff.

Usage:
    tools/diffstat_patch.py -f patch.diff
    tools/diffstat_patch.py -f - < patch.diff
    tools/diffstat_patch.py -f patch.diff --stat
"""

from __future__ import annotations

import argparse
import re
import shlex
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass
class FileStat:
    filename: str
    additions: int = 0
    deletions: int = 0
    binary: bool = False


_HUNK = re.compile(
    r"^@@\s+-\d+(?:,(?P<old_count>\d+))?\s+"
    r"\+\d+(?:,(?P<new_count>\d+))?\s+@@"
)
_BINARY = re.compile(r"^Binary files (.+) and (.+) differ$")


def _header_path(line: str) -> str:
    path = line[4:]
    if path.startswith('"'):
        try:
            path = shlex.split(path)[0]
        except ValueError:
            pass
    else:
        path = path.split("\t", 1)[0]
    return path


def _git_paths(line: str) -> tuple[str, str] | None:
    try:
        paths = shlex.split(line[len("diff --git ") :])
    except ValueError:
        return None
    if len(paths) != 2:
        return None
    return paths[0], paths[1]


def _display_path(path: str) -> str:
    if path.startswith(("a/", "b/")):
        return path[2:]
    return path


def diffstat(patch: str) -> list[FileStat]:
    files: list[FileStat] = []
    current: FileStat | None = None
    old_path: str | None = None
    in_hunk = False
    old_remaining = 0
    new_remaining = 0
    git_header_pending = False

    for line in patch.splitlines():
        if in_hunk:
            if line.startswith("+"):
                current.additions += 1
                new_remaining -= 1
            elif line.startswith("-"):
                current.deletions += 1
                old_remaining -= 1
            elif line.startswith(" "):
                old_remaining -= 1
                new_remaining -= 1
            elif line.startswith("\\"):
                continue
            else:
                in_hunk = False

            if line.startswith(("+", "-", " ")):
                if old_remaining <= 0 and new_remaining <= 0:
                    in_hunk = False
                continue

        if line.startswith("diff --git "):
            git_paths = _git_paths(line)
            current = FileStat(_display_path(git_paths[1])) if git_paths else None
            if current is not None:
                files.append(current)
            git_header_pending = current is not None
            old_path = None
            in_hunk = False
        elif line.startswith("--- "):
            old_path = _header_path(line)
            in_hunk = False
        elif line.startswith("+++ ") and old_path is not None:
            new_path = _header_path(line)
            filename = old_path if new_path == "/dev/null" else new_path
            if git_header_pending and current is not None:
                current.filename = _display_path(filename)
            else:
                current = FileStat(filename)
                files.append(current)
            git_header_pending = False
            old_path = None
        else:
            binary_match = _BINARY.match(line)
            if binary_match:
                binary_filename = _display_path(binary_match.group(2))
                if git_header_pending and current is not None:
                    current.binary = True
                elif (
                    current is not None
                    and current.filename == binary_filename
                    and current.additions == 0
                    and current.deletions == 0
                ):
                    current.binary = True
                else:
                    current = FileStat(binary_filename, binary=True)
                    files.append(current)
                git_header_pending = False
                in_hunk = False
            elif line == "GIT binary patch" and current is not None:
                current.binary = True
                git_header_pending = False
                in_hunk = False
            elif _HUNK.match(line):
                if current is None and old_path is not None:
                    current = FileStat(old_path)
                    files.append(current)
                hunk_match = _HUNK.match(line)
                if current is not None and hunk_match is not None:
                    old_remaining = int(hunk_match.group("old_count") or 1)
                    new_remaining = int(hunk_match.group("new_count") or 1)
                    in_hunk = old_remaining > 0 or new_remaining > 0

    return files


def read_patch(filename: str) -> str:
    if filename == "-":
        return sys.stdin.read()
    return Path(filename).read_text(encoding="utf-8", errors="replace")


def print_stat(files: list[FileStat]) -> None:
    path_width = max(len(item.filename) for item in files)
    changes = [item.additions + item.deletions for item in files]
    max_changes = max(changes, default=0)
    count_width = max(
        [len("Bin") if item.binary else len(str(count)) for item, count in zip(files, changes)]
        or [1]
    )

    for item, changed in zip(files, changes):
        count = "Bin" if item.binary else str(changed)
        line = f"{item.filename:<{path_width}} | {count:>{count_width}}"
        if changed:
            bar_length = max(1, int(changed * 50 / max_changes + 0.5))
            if item.additions and item.deletions and bar_length < 2:
                bar_length = 2
            additions = int(bar_length * item.additions / changed + 0.5)
            if item.additions and additions < 1:
                additions = 1
            if item.deletions and additions >= bar_length:
                additions = bar_length - 1
            bar = "+" * additions + "-" * (bar_length - additions)
            line += f" {bar}"
        print(line)

    additions = sum(item.additions for item in files)
    deletions = sum(item.deletions for item in files)
    noun = "file" if len(files) == 1 else "files"
    print(
        f"{len(files)} {noun} changed, {additions} insertions(+), "
        f"{deletions} deletions(-)"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "-f",
        "--file",
        required=True,
        metavar="PATCH",
        help="unified diff file to read, or - for stdin",
    )
    parser.add_argument(
        "--stat",
        action="store_true",
        help="print Git-style per-file change bars",
    )
    args = parser.parse_args()

    try:
        files = diffstat(read_patch(args.file))
    except OSError as error:
        parser.error(str(error))

    if not files:
        print("No file changes found.")
        return 0

    if args.stat:
        print_stat(files)
        return 0

    name_width = max(len("File"), *(len(item.filename) for item in files))
    print(f"{'File':<{name_width}}  {'Insertions':>10}  {'Deletions':>9}")
    for item in files:
        additions = "binary" if item.binary else str(item.additions)
        deletions = "binary" if item.binary else str(item.deletions)
        print(f"{item.filename:<{name_width}}  {additions:>10}  {deletions:>9}")

    additions = sum(item.additions for item in files)
    deletions = sum(item.deletions for item in files)
    print(
        f"{len(files)} files changed, {additions} insertions(+), "
        f"{deletions} deletions(-)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())