
# dsport

## Dev loop

All Rust commands run from `src/` (the workspace root). The Python venv lives
at `src/.venv/` and is git-ignored.

```sh
# one-time setup
cd src
python3 -m venv .venv && source .venv/bin/activate
pip install maturin pytest

# Rust gates
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Build extensions into the venv (re-run after Rust changes)
(cd docutilsrs && maturin develop --release)
(cd sphinxdocrs && maturin develop --release)

# Python gate
pytest tests/ -q
```

Snapshot tests use `insta`. Review pending snapshots with `cargo insta review`
(install via `cargo install cargo-insta`).

## Objectives
- port docutils (src/docutils) to rust as `docutilsrs`
- port sphinxdoc (src/sphinx) to rust as `sphinxdocrs`
- use test parametrization and mocks in rust to keep the tests fast
- full python plugin/extension compatibility
- support automatically using the rust equivalent
  of a python sphinxdoc plugin
  by adding a new metadata attribute to pyproject.toml/setup.py to indicate the name of the equivalent cargo project
- must be able to import `docutilsrs` and `sphinxdocrs` from Python
- must be able to import and use Python plugins from Rust


## Decisions to lock before coding

These choices unblock everything downstream. Capture each in a short ADR under `docs/adr/` when made.

- **Upstream pin**: record the exact upstream version/commit of vendored `src/docutils/` and `src/sphinx/`. Parity is defined against that pin; bumps are explicit events.
- **Names**: on-disk crate dirs, Rust crate names, and Python import names are all `docutilsrs` and `sphinxdocrs`. These do not shadow installed `docutils` / `sphinx`. The compiled PyO3 extension modules are exposed directly under those names (no separate `_`-prefixed inner module).
- **Bindings stack**: PyO3 + maturin. Cargo workspace rooted at `src/Cargo.toml`, with `docutilsrs` and `sphinxdocrs` as members, so shared code (PyO3 conversion layer, plugin-resolver crate, test helpers) lives as path deps and tooling runs once across the workspace.
- **Doctree representation**: owned Rust tree (arena + `NodeId` indices, enum-dispatched node kinds) with FFI converters to/from `docutils.nodes` at the boundary. Wrapper-over-Python-nodes was rejected: per-access PyO3/GIL cost is paid on every traversal, and traversals dominate transforms and writers. Converter parity is guarded by round-trip snapshot tests (Python → Rust → Python, asserted via pseudo-XML).
- **Plugin discovery**: define a Python entry point group (e.g. `docutilsrs.equivalents`) that maps a Python dotted name to a Rust crate + symbol. This is more robust than a freeform `pyproject.toml` key and works for already-installed third-party packages.

## Plan

### Phase 0 — bootstrap

- create `src/docutilsrs/Cargo.toml` and `src/sphinxdocrs/Cargo.toml` with `cdylib` + `rlib` crate types
- minimal `src/lib.rs` for each, exporting a `version()` PyO3 function
- maturin-based build producing importable wheels into a local venv
- dev loop commands documented in this README: `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`, `maturin develop`, `pytest`
- `insta` wired for snapshot tests; one trivial snapshot landed to prove the loop
- CI config (GitHub Actions or equivalent) running fmt/clippy/test/maturin-build on Linux

### Phase 1 — docutilsrs vertical slice

Goal: one full input → doctree → output path working end-to-end on a tiny subset, not broad coverage.

- design and land the Rust doctree data model (nodes, attributes, traversal); snapshot-compare against `docutils.nodes` for a fixed set of inputs converted via the FFI boundary
- port the rST inline parser for a minimal grammar slice (paragraphs, emphasis, strong, literal, references); choose tests from `src/docutils/docutils/test/test_parsers/test_rst/` rather than the whole tree
- port one trivial writer (pseudo-XML or a stripped HTML5) to exercise the full pipeline
- expose `parse_rst(source: str) -> Doctree` to Python; validate parity by comparing pseudo-XML output against vendored docutils on the same inputs

### Phase 2 — docutilsrs widening (landed)

Status: **done**. Per-feature status tracked in `docs/compat.md`.

Landed:
- parser widened across: sections + transitions, block quotes, literal blocks, definition lists, field lists + docinfo, comments, admonitions, image/figure, code/code-block/sourcecode, raw, inline roles, substitutions (replace), simple + grid tables, phrase references
- transforms applied inline in the parser pipeline (title promotion, docinfo, reference resolution, substitution resolution)
- minimal HTML5 writer (`docutilsrs.parse_to_html5`) producing a semantic fragment; not parity-gated
- pseudo-XML parity gate at 82 cases, byte-for-byte vs vendored `docutils.publish_string(..., writer="pseudoxml")`

Deferred to later phases (tracked as `accepted-deviation` in `docs/compat.md`):
- overlined sections, block-quote attributions (landed)
- nested lists + multi-paragraph list items (landed; covered by `nested_*`/`multipara_*` parity cases)
- phrase refs with embedded URIs, anonymous refs, footnotes (numeric + autonumber + autosymbol), citations (landed)
- unresolved-reference system messages (still deferred — needs source-line plumbing through the block/inline parser)
- table row/column spans and multi-paragraph cells
- figure captions/legends (landed)
- syntax highlighting for `code-block` (Pygments) — available via the Python directive plugin bridge; see `src/docutilsrs/python/docutilsrs_pygments.py`
- transforms factored into a standalone module mirroring `docutils.transforms.*` (landed as `docutilsrs::transforms` with a composable `Transform`/`Pipeline` API)
- LaTeX / ODT / manpage writers

### Phase 3 — transforms module + hybrid mode

This is the integration safety net, not a stretch goal.

- factor the inlined Phase-2 transforms (title promotion, docinfo, reference resolution, substitution resolution) into a `docutilsrs::transforms` module mirroring `docutils.transforms.*`, each pass independently testable
- Python-side `docutilsrs` package routes calls to Rust when implemented, falls back to vendored Python otherwise, on a per-component basis (parser, transform, writer)
- Rust-side: ability to invoke a Python `Transform` or `Writer` against a Rust-owned doctree (via converter)
- end-to-end test: a document whose parsing is Rust, one transform is Python, and writer is Rust, produces byte-identical output to pure-Python docutils

### Phase 4 — sphinxdocrs incremental port

- inventory `src/sphinx/tests/` and tag each test by subsystem (config, environment, builders, extensions, domains)
- port fast unit tests first (config, util, project); defer builder integration tests until the relevant builder is ported
- prioritize the extension/event system early so existing Sphinx extensions keep working under the Rust core
- expose Python import surface mirroring phase 1's pattern

### Phase 5 — plugin interoperability

- implement the entry-point–based resolver decided above; resolution order: declared Rust equivalent → Python implementation
- version guard: Rust equivalent declares a compatible upstream version range; mismatch falls back to Python with a warning
- bidirectional calls: Rust → Python plugins via PyO3; Python → Rust crates via the published extension modules
- end-to-end tests covering: pure-Python plugin, pure-Rust equivalent, fallback on version mismatch, mixed pipeline

## Quality gates

- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `pytest` all green on every PR
- snapshot diffs against vendored Python output are the primary parity signal
- each phase exit requires: tests, compatibility-matrix update, and a short note in `docs/changes/`
- perf budget for parser: within 2× of CPython baseline by end of phase 2 (tightened later)
- no upstream-source modifications under `src/docutils/` or `src/sphinx/` outside of an explicit, isolated PR

## Milestone 1 (first iteration)

- goal
  - land the bootstrap so a contributor can clone, build both crates, import them from Python, and run the test loop

- deliverables
  - decisions above recorded as ADRs (upstream pin, module names, bindings stack, doctree model, plugin discovery)
  - `Cargo.toml` + `src/lib.rs` for `docutilsrs` and `sphinxdocrs`, each exporting a PyO3 `version()`
  - `maturin develop` produces importable `docutilsrs` and `sphinxdocrs` modules
  - one `insta` snapshot test per crate proving the test harness works
  - CI workflow running fmt, clippy, cargo test, maturin build, and a pytest smoke test
  - `docs/compat.md` skeleton with the matrix columns defined

- exit criteria
  - fresh clone → documented commands → green build and green tests
  - Python smoke test imports both modules and asserts on `version()`
  - one ADR per locked decision is merged

- explicit non-goals for M1
  - any real parsing
  - any plugin resolution
  - any writer output

