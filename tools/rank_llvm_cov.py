#!/usr/bin/env python3
"""Rank cargo-llvm-cov text rows by missed branches.

Usage:
    cargo +nightly llvm-cov -p sphinxdocrs --lib --branch --summary-only \
        | tools/rank_llvm_cov.py
    tools/rank_llvm_cov.py coverage-summary.txt --limit 30

The parser accepts llvm-cov's whitespace-wrapped summary rows, so it is safe
against narrow terminal output. It intentionally reports only nonzero misses;
TOTAL is omitted because the purpose is file-level triage.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


_ROW = re.compile(
    r"^(?P<file>\S+)\s+"
    r"(?P<regions>\d+)\s+(?P<missed_regions>\d+)\s+\d+\.\d+%\s+"
    r"(?P<functions>\d+)\s+(?P<missed_functions>\d+)\s+\d+\.\d+%\s+"
    r"(?P<lines>\d+)\s+(?P<missed_lines>\d+)\s+\d+\.\d+%\s+"
    r"(?P<branches>\d+)\s+(?P<missed_branches>\d+)\s+\d+\.\d+%",
    re.MULTILINE,
)


def read_report(path: str | None) -> str:
    if path is None or path == "-":
        return sys.stdin.read()
    return Path(path).read_text(encoding="utf-8")


def rank_rows(report: str) -> list[tuple[str, int, int]]:
    rows = []
    for match in _ROW.finditer(report):
        filename = match.group("file")
        if filename == "TOTAL":
            continue
        missed = int(match.group("missed_branches"))
        if missed:
            rows.append((filename, missed, int(match.group("branches"))))
    return sorted(rows, key=lambda row: (-row[1], -row[2], row[0]))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report", nargs="?", help="summary text file, or stdin")
    parser.add_argument(
        "--limit",
        type=int,
        default=20,
        help="maximum rows to print (default: 20; use 0 for all)",
    )
    args = parser.parse_args()

    rows = rank_rows(read_report(args.report))
    if args.limit > 0:
        rows = rows[: args.limit]
    print("missed/total  file")
    for filename, missed, total in rows:
        print(f"{missed:5}/{total:<5}  {filename}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
