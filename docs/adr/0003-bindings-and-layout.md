# ADR 0003 — Bindings stack and Cargo layout

**Status**: accepted (M1)
**Date**: 2026-05-22

## Context
The ports must be importable from Python and need a build tool that produces
wheels. The two crates will share code (doctree types, conversion layer,
plugin resolver, test helpers); they should not drift on PyO3 version.

## Decision
- **Bindings**: PyO3.
- **Build tool**: maturin (per-crate `pyproject.toml`).
- **Layout**: a Cargo workspace rooted at `src/Cargo.toml`, with
  `docutilsrs` and `sphinxdocrs` as members. PyO3 and other shared deps are
  declared in `[workspace.dependencies]` so both crates pin the same version.
- **`extension-module` feature**: declared per-crate and enabled by maturin
  (`[tool.maturin].features = ["extension-module"]`). `cargo test` and
  `cargo build` run without the feature so test binaries can link libpython.
- **`auto-initialize`**: enabled on the `pyo3` dep so Rust-side tests that
  cross into Python (added later) don't need explicit interpreter setup.

## Consequences
- One `Cargo.lock`, one `target/`, one `cargo fmt`/`clippy`/`test` invocation
  for everything.
- maturin builds happen from inside each member crate's directory; this works
  unchanged inside a workspace.
- Adding a shared crate (e.g. `doctree`, `compat`) later is a workspace
  member addition, not a packaging exercise.
