# ADR 0002 — Names and module surface

**Status**: accepted (M1)
**Date**: 2026-05-22

## Context
The Rust ports need names that work as:

- on-disk directories,
- Rust crate names,
- Python import names,

without shadowing the already-installed `docutils` / `sphinx` Python
distributions, and without forcing a separate inner `_`-prefixed extension
module plus a Python re-export shim.

## Decision
A single name per port, used everywhere:

| Role                       | docutils port | sphinx port  |
|---------------------------|---------------|--------------|
| Directory under `src/`    | `docutilsrs/` | `sphinxdocrs/` |
| Cargo `[package].name`    | `docutilsrs`  | `sphinxdocrs`  |
| Rust `[lib].name`         | `docutilsrs`  | `sphinxdocrs`  |
| PyO3 `#[pymodule]` ident  | `docutilsrs`  | `sphinxdocrs`  |
| Python import             | `docutilsrs`  | `sphinxdocrs`  |
| Distribution (PyPI) name  | `docutilsrs`  | `sphinxdocrs`  |

There is no `_`-prefixed inner module and no Python-side re-export package.
The compiled extension is the importable module.

## Consequences
- Zero shadowing risk against installed `docutils`/`sphinx`.
- `import docutilsrs; docutilsrs.version()` is the single, direct surface.
- If we later need a pure-Python wrapper layer (e.g. for the hybrid-mode
  router in phase 3), it must use a different name (proposal at that time:
  `docutilsrs_compat`).
