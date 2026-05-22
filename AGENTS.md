# AGENTS

Purpose: define how coding agents and contributors should navigate this repository and execute work safely.

## Repository map (from tree)

.
|- README.md
|- AGENTS.md
`- src/
   |- docutils/
   |  |- docutils/
   |  |- sandbox/
   |  `- web/
   |- docutilsrs/   (target Rust port, currently empty)
   |- sphinx/
   |  |- sphinx/
   |  |- tests/
   |  |- doc/
   |  `- utils/
   `- sphinxdocrs/  (target Rust port, currently empty)

## Mission

- Port Python docutils into `src/docutilsrs`.
- Port Python sphinx into `src/sphinxdocrs`.
- Keep behavior compatible with upstream Python projects.
- Preserve plugin interoperability across Python and Rust implementations.

## Working rules

- Use test-first or test-near development for all ports.
- Start with parser and core data model parity before feature expansion.
- Prefer small, reviewable commits with explicit compatibility notes.
- Do not modify upstream vendored sources under `src/docutils/` or `src/sphinx/` unless the task explicitly requires it.
- Keep Rust code idiomatic, but match Python-visible behavior where compatibility matters.

## Priority order

1. Bootstrap `docutilsrs` and `sphinxdocrs` Cargo crates.
2. Port docutils parser tests and parser implementation.
3. Port remaining docutils subsystems in dependency order.
4. Port sphinx tests/features incrementally, prioritizing extension hooks.
5. Implement metadata-based plugin mapping and fallback.

## Test strategy

- Reuse upstream tests wherever possible.
- Mark each ported test as one of: exact parity, accepted deviation, or pending.
- Keep fast local loops:
  - unit tests for parser and transforms
  - snapshot tests for structured outputs (insta)
  - focused integration tests for Python<->Rust boundaries

## Python/Rust interop requirements

- Python must be able to import and call `docutilsrs` and `sphinxdocrs` modules.
- Rust must be able to load and execute compatible Python plugins.
- Plugin resolution should prefer Rust equivalent when declared; otherwise fall back to Python implementation.

## Suggested workflow for agents

1. Identify target behavior and locate corresponding upstream tests.
2. Port tests into Rust crate test modules.
3. Implement minimal code to pass tests.
4. Add interop checks when Python boundary is touched.
5. Update compatibility notes in README or milestone docs.

## Out of scope by default

- Large-scale refactors of vendored Python source trees.
- Formatting-only sweeps unrelated to active work.
- Feature additions that do not improve parity or interop.
