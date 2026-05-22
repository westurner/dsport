
# dsport

## Objectives
- port docutils (src/docutils) to rust as docutils.rs
- port sphinxdoc (src/sphinx) to rust as sphinxdoc.rs
- use test parametrization and mocks in rust to keep the tests fast
- full python plugin/extension compatibility
- support automatically using the rust equivalent
  of a python sphinxdoc plugin
  by adding a new metadata attribute to pyproject.toml/setup.py to indicate the name of the equivalent cargo project
- must be able to import docutils.rs and sphinxdoc.rs from Python
- must be able to import and use Python plugins from Rust


## Plan
- bootstrap rust workspaces
  - create `src/docutils.rs/Cargo.toml` and `src/sphinxdoc.rs/Cargo.toml`
  - define crate layout (`src/lib.rs`, feature flags, test modules)
  - add CI-ready commands for `cargo fmt`, `cargo clippy`, and `cargo test`
  - install and use `cargo insta` for snapshot-style parser/render tests

- phase 1: docutils.rs parser-first parity
  - inventory parser-related tests in `src/docutils/docutils/test/`
  - port parser tests first, preserving fixture semantics and edge cases
  - implement parser core and AST/data structures to satisfy the test port
  - add a Python import surface (PyO3/maturin) for early integration checks

- phase 2: docutils.rs pipeline coverage
  - port transforms, readers, and writer-path tests by priority
  - add compatibility shims where Python behavior is relied on by callers
  - publish a small compatibility matrix: implemented, partial, not started

- phase 3: sphinxdoc.rs incremental port
  - inventory tests in `src/sphinx/tests/` and map dependencies on docutils
  - port fast unit tests first, then integration tests that exercise builders
  - prioritize extension loading and event hooks to protect plugin workflows
  - expose import/use from Python and validate mixed Python/Rust execution

- phase 4: plugin interoperability
  - define metadata key in `pyproject.toml`/`setup.py` for Rust equivalent crate
  - implement plugin resolution order: native Rust equivalent, then Python plugin
  - support Rust calling Python plugins and Python calling Rust crates
  - add end-to-end tests that prove fallback and mixed-plugin scenarios

- quality gates
  - keep test runtime fast using parametrization, fixtures, and mocks
  - treat upstream docutils/sphinx behavior as the compatibility source of truth
  - require each milestone to land with tests and a short compatibility note

## Milestone 1 (Week 1)

- goal
  - bootstrap both Rust crates and land first parser-parity test coverage in `docutils.rs`

- deliverables
  - create `src/docutils.rs/Cargo.toml` and `src/sphinxdoc.rs/Cargo.toml`
  - create minimal crate entrypoints (`src/lib.rs`) for both projects
  - add baseline tooling commands and config for `cargo fmt`, `cargo clippy`, and `cargo test`
  - install `insta` and add one snapshot-based parser test harness in `docutils.rs`
  - port an initial small parser test subset from `src/docutils/docutils/test/`
  - expose a minimal Python import check for `docutils.rs` (smoke test only)

- exit criteria
  - crates build cleanly on CI/dev machine
  - formatting/lint/test commands pass on baseline code
  - at least one ported parser test passes in Rust
  - one failing-not-yet-ported parser case is documented in a compatibility note

- stretch (if time remains)
  - set up maturin scaffolding for wheel build experiments
  - add first interop test proving Rust path fallback to Python plugin path

