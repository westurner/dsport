# ADR 0001 — Vendored upstream pin

**Status**: accepted (M1)
**Date**: 2026-05-22

## Context
Parity is the project's primary correctness signal. "Parity with docutils" /
"parity with Sphinx" is meaningless without naming the exact source revision
the ports are compared against.

## Decision
The upstream targets are the trees currently vendored at:

- `src/docutils/`
- `src/sphinx/`

Both are treated as immutable for parity purposes. The recorded upstream
versions are whatever those trees report (see their `pyproject.toml` /
`__init__.py`). Bumps to either tree are explicit, isolated PRs that:

1. update the vendored source in a single commit,
2. re-run the full compat matrix,
3. record any new deviations in `docs/compat.md`.

## Consequences
- Test snapshots are pinned to the vendored versions.
- We never compare Rust output against a `pip install`ed docutils/sphinx; the
  in-tree copies are the reference.
- Vendor bumps cannot be casually mixed with feature work.
