
# dsport

## Dev loop

The Rust workspace is rooted at the repository root (see `Cargo.toml`). All `cargo` commands can run from the repository root or from `src/`. The Python venv lives at `src/.venv/` and is git-ignored.

```sh
# one-time setup
cd src
python3 -m venv .venv && source .venv/bin/activate
pip install maturin pytest

# install Rust dev tools (once — downloads cargo-nextest, cargo-llvm-cov, and
# the llvm-tools rustup component; subsequent runs are no-ops)
make install-nextest-deps
cargo install cargo-insta

# Rust gates (from repository root or src/)
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# or with cargo-nextest (faster, better output)
cargo nextest run --workspace
# via Makefile — also logs to reports/<ISO-timestamp>/
make nextest
make nextest-coverage   # requires nightly + llvm-tools (install via make install-nextest-deps)
# pin nightly to skip per-run channel-sync check:
# make nextest-coverage NIGHTLY_TOOLCHAIN=nightly-2026-07-02

# Build extensions into the venv (re-run after Rust changes)
cd src
(cd docutilsrs && maturin develop --release)
(cd sphinxdocrs && maturin develop --release)
(cd pygmentsrs && maturin develop --release)

# Python gate
pytest tests/ -q
```

Snapshot tests use `insta`. Review pending snapshots with `cargo insta review`
(install via `cargo install cargo-insta`).

## Optional Feature Flags

Some crates in the workspace support optional features that require system dependencies:

### jinja2rs seccomp feature

To build jinja2rs with syscall sandboxing (Linux only):

```sh
# Install system dependency (choose one)

# Ubuntu/Debian
sudo apt install libseccomp-dev

# Fedora/RHEL
sudo dnf install libseccomp-devel
```

Then build with the seccomp feature (from repository root or src/jinja2rs):

```sh
# From repository root:
cargo test -p jinja2rs --features sandbox,seccomp,resource-limits

# Or from src/jinja2rs:
cd src/jinja2rs
cargo test --features sandbox,seccomp,resource-limits
```

See [docs/LIBSECCOMP_SETUP.md](docs/LIBSECCOMP_SETUP.md) for detailed platform-specific installation instructions and troubleshooting.

### Cargo feature flags: optional components

`docutilsrs` and `sphinxdocrs` support optional features to reduce dependencies for minimal builds:

**`syntax-highlighting` (default)**
- Includes `pygmentsrs` for syntax highlighting support
- Required by `docutils` code-block directive 
- Disable with: `--no-default-features`

Example builds:

```sh
# Full build (default) — includes syntax highlighting
cargo build -p docutilsrs
cargo build -p sphinxdocrs

# Minimal build without pygmentsrs (no syntax highlighting)
cargo build -p docutilsrs --no-default-features
cargo build -p sphinxdocrs --no-default-features

# With only specific features
cargo build -p docutilsrs --no-default-features --features syntax-highlighting
```

### Python extension module builds (maturin)

When building as Python extensions via `maturin`, the `extension-module` feature ensures all dependencies are built correctly for embedding:

```sh
# Full extension (includes syntax highlighting)
cd src/docutilsrs && maturin develop --release
cd src/sphinxdocrs && maturin develop --release

# Minimal extension (no syntax highlighting)
cd src/docutilsrs && maturin develop --release -- --no-default-features --features extension-module
cd src/sphinxdocrs && maturin develop --release -- --no-default-features --features extension-module
```

**Feature details:**
- `extension-module` — build as Python extension (PyO3 no-embed mode) + propagate to dependencies
- `syntax-highlighting` — include `pygmentsrs` support (default)

When `extension-module` is enabled, it automatically propagates to `pygmentsrs/extension-module` so the dependency doesn't try to embed libpython independently.

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
- **Bindings stack**: PyO3 + maturin. Cargo workspace rooted at the repository root (see `Cargo.toml`), with `docutilsrs`, `sphinxdocrs`, `pygmentsrs`, and `jinja2rs` as members under `src/`, so shared code (PyO3 conversion layer, plugin-resolver crate, test helpers) lives as path deps and tooling runs once across the workspace.
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

Parser:
- sections + transitions (including overlined sections), block quotes (with attributions), literal blocks, definition lists (with classifiers), field lists + docinfo, comments, admonitions, image/figure (with captions + legends), code/code-block/sourcecode, raw, inline roles, substitutions (replace), simple + grid tables, phrase references
- nested lists + multi-paragraph list items (covered by `nested_*`/`multipara_*` parity cases)
- phrase refs with embedded URIs, anonymous refs, footnotes (numeric + autonumber + autosymbol), citations
- unresolved-reference system messages (`<problematic>` + trailing `system-messages` section; line tracking for top-level paragraphs only — nested paragraphs report no line)
- grid tables: column spans, row spans, and multi-paragraph cells (covered by `grid_table_colspan`/`grid_table_rowspan`/`grid_table_rowspan_colspan`/`grid_table_multipara_cell` parity cases)

Transforms:
- applied inline in the parser pipeline (title promotion, docinfo, reference resolution, substitution resolution)
- factored into a standalone `docutilsrs::transforms` module mirroring `docutils.transforms.*` with a composable `Transform`/`Pipeline` API

Writers:
- pseudo-XML — byte-parity-gated against vendored `docutils.publish_string(..., writer="pseudoxml")` (`tests/test_parity_pseudoxml.py`, **113 cases**)
- HTML5 (`docutilsrs.parse_to_html5`) — minimal semantic fragment; accepted-deviation, structurally gated
- LaTeX (`docutilsrs.parse_to_latex`) — minimal, accepted-deviation, structurally gated
- manpage/troff (`docutilsrs.parse_to_manpage`) — minimal, accepted-deviation, structurally gated
- ODT (`docutilsrs.parse_to_odt`):
  - native Rust path (default): valid `.odt` ZIP container (`mimetype` + `META-INF/manifest.xml` + `content.xml` + `styles.xml`); accepted-deviation, structurally gated by `tests/test_writer_odt.py`
  - `compat=True` (with optional `settings_overrides=...`): delegates via PyO3 to vendored `docutils.writers.odf_odt`; **byte-parity-gated** against all 13 upstream `.odt` fixtures (`tests/test_writer_odt_parity.py`) using the same `content.xml`-after-`ET.tostring` normalization upstream's own `test_odt.py` uses

Plugin bridges:
- syntax highlighting for `code-block` (Pygments) via the Python directive plugin bridge — see `src/docutilsrs/python/docutilsrs_pygments.py`
- **native** code-block syntax highlighting via the in-workspace `pygmentsrs` crate (Rust→Rust call for supported languages; PyO3 bridge to `docutils.utils.code_analyzer.Lexer` as fallback): wired in `src/docutilsrs/src/code_block.rs`. Smoke-gated by `tests/test_pygments_native.py`; byte-parity gate covers the always-passthrough `text` language + the unparseable/no-lang shape (`tests/test_parity_pseudoxml.py`). Full byte-parity for the `python` lexer is the remaining pygmentsrs Phase 1 followup.

Open handoffs (next-up work, **in progress** via the dedicated `pygmentsrs` workspace crate — see Phase 2.5 below):
- **Native Pygments syntax highlighting** for `code`/`code-block`/`sourcecode` (replace the opt-in plugin-bridge stub with byte-parity emission of `<literal_block>` + token-classed `<inline>` children). Brief, target output, recommended implementation path, fixtures to add, and gate commands: [docs/handoff/pygments.md](docs/handoff/pygments.md).

### Phase 2.5 — pygmentsrs (Rust port of Pygments)

Spun up as a separate workspace crate at `src/pygmentsrs/` so the
code-block handoff can land as a native Rust→Rust call rather than a
per-block PyO3 hop. Scope: top-N lexers used in Sphinx/RST docs
(`text`, `python`, `rust`, `c`, `cpp`, `js`/`ts`, `bash`, `json`,
`yaml`, `toml`, `go`, `rst`, `html`, `css`, `sql`, `diff`, `make`,
`dockerfile`). Parity strategy: byte-parity against vendored
`pygments` `HtmlFormatter` and against
`docutils.utils.code_analyzer.Lexer`'s token stream (which is what
`test_parity_pseudoxml.py` compares).

Status:

- **Phase 0 done** — workspace member wired, `pygmentsrs.version()` /
  `features()` exposed via PyO3, `insta` snapshot loop proven
  (5 passing `cargo test` cases), `TextLexer` passthrough +
  `HtmlFormatter` skeleton landed.
- **Phase 1 done** — token hierarchy ported (~70 named
  constants from `pygments.token`), `RegexLexer` engine ported
  (state stack, `bygroups`, `default`, `#pop`/`#push`/named-state
  transitions, adjacent-same-type merging, error/whitespace
  fallback). `PythonLexer` covers **33 byte-parity fixtures** in
  `tests/test_parity_pseudoxml.py` (`code_block_python_*`):
  def/class/decorators, imports (relative + parenthesised + `as`),
  `True`/`False`/`None`, walrus, line-continuations, escape
  sequences, raw / triple / prefixed / f-strings (including nested
  literals inside `{…}`), 69 builtins, pseudo-builtins, stdlib
  exceptions, magic variables, comments, numbers, operators, and
  `in`/`is`/`and`/`or`/`not` as `Operator.Word` inside f-string
  expressions. Accepted deviations tracked in
  `src/pygmentsrs/docs/compat.md` (docstring sub-type, `match`/`case`
  soft keywords, complex-number suffix).
- **Phase 2 widening — in progress.** Bridge layer (`src/pygmentsrs/src/bridge.rs`)
  landed: `pygmentsrs.lex(alias, code, backend="auto"|"rust"|"python")`,
  `has_native_lexer(alias)`, `native_aliases()`, and a `highlight()`
  shortcut. `backend="auto"` tries the native Rust lexer first and
  transparently falls back to upstream `pygments` via PyO3 for any
  alias without a Rust implementation, so the workspace gets full
  upstream coverage today and incremental Rust speedups as lexers
  land. **Native lexers landed**: `text`, `python`, `json`
  (`src/pygmentsrs/src/lexers/json.rs`, hand-written state machine,
  10 byte-parity fixtures in `tests/test_pygments_json_lexer.py`),
  `diff` (`src/pygmentsrs/src/lexers/diff.rs`, RegexLexer-engine
  port, 6 byte-parity fixtures in `tests/test_pygments_diff_lexer.py`).
  **`HtmlFormatter`** now ships the full `STANDARD_TYPES` short-name
  table (`src/pygmentsrs/src/token.rs`), so its default-options
  output is byte-compatible with `pygments.formatters.html.HtmlFormatter`
  for every native lexer above.
- **Phase 3 done** — docutilsrs integration wired
  (`pygmentsrs = { path = "../pygmentsrs" }`): the parser's
  `code`/`code-block`/`sourcecode` arm calls `pygmentsrs::tokenize`
  first and only falls back to the existing
  `docutils.utils.code_analyzer.Lexer` Python bridge when the
  native path declines.
  `src/docutilsrs/src/code_block.rs` hosts the dispatcher;
  `Block::LiteralBlock` carries an optional
  `tokens: Vec<(Option<String>, String)>` field that the pseudo-XML
  emit path renders as `<inline classes="…">` token spans. Gates:
  `tests/test_pygments_native.py` (smoke) and
  `tests/test_parity_pseudoxml.py` now contains **33+ byte-parity
  code-block cases** (text alias, sourcecode alias, and the full
  `code_block_python_*` family).

Remaining followups: continue widening the lexer registry (next
priorities are `rust`, `c`/`cpp`, `yaml`, `rst`, `toml`, `make`).
`bash`/`sh`/`ksh`/`zsh`/`shell` lexers are **unblocked**: the regex
engine was upgraded from `regex` to `fancy-regex` (ADR 0012) to support
backreferences required by heredoc patterns (upstream's heredoc rule uses
`\2`); see `src/pygmentsrs/src/lexer/engine.rs`.

### Phase 3 — transforms module + hybrid mode

This is the integration safety net, not a stretch goal.

- factor the inlined Phase-2 transforms (title promotion, docinfo, reference resolution, substitution resolution) into a `docutilsrs::transforms` module mirroring `docutils.transforms.*`, each pass independently testable — **done** (`src/docutilsrs/src/transforms.rs`, `Transform` trait + `Pipeline`)
- Python-side `docutilsrs` package routes calls to Rust when implemented, falls back to vendored Python otherwise, on a per-component basis (parser, transform, writer) — **done** (`src/docutilsrs/python/docutilsrs_hybrid.py`: `publish_string(prefer=...)` + `dispatch_plan(writer, prefer, has_python_transforms)` reporting `{parser, transforms, writer}`)
- Rust-side: ability to invoke a Python `Transform` or `Writer` against a Rust-owned doctree (via converter) — **done** (`docutilsrs.register_transform(name, callable)`: callable receives a read-only `PyDoctree` view and returns `[(node_id, new_text), ...]` edits applied to the arena after the default pipeline; see `src/docutilsrs/src/plugins.rs::apply_transforms`)
- end-to-end test: a document whose parsing is Rust, one transform is Python, and writer is Rust, produces byte-identical output to pure-Python docutils — **done** (`tests/test_phase3_hybrid_e2e.py`, parametrized over 4 docs, uppercases all text via the bridge and compares to pure-Python with the equivalent `docutils.transforms.Transform`)

### Phase 4 — sphinxdocrs incremental port

- inventory `src/sphinx/tests/` and tag each test by subsystem (config, environment, builders, extensions, domains) — **done** (`docs/sphinx-port-inventory.md`)
- port fast unit tests first (config, util, project); defer builder integration tests until the relevant builder is ported — **in progress** (P1: `errors`, `events`, `project` (incl. `discover`) landed with mirrored parity tests; P2: `extension.Extension` + `verify_needs_extensions` landed (`tests/test_sphinxdocrs_extension.py`); P2: `util.matching` (`compile_matchers`/`Matcher`/`get_matching_files`) landed (`tests/test_sphinxdocrs_util_matching.py`); P2: `util.console` (port of `sphinx.util.console` + `sphinx._cli.util.colour` + `sphinx._cli.util.errors`: `colourise`, `disable_colour`/`enable_colour`, `strip_escape_sequences`, `terminal_safe`, 22 named colour escape codes) landed (`tests/test_sphinxdocrs_util_console.py`, 40 byte-parity tests); `Project.discover` gated by `tests/test_sphinxdocrs_project_discover.py`)
- prioritize the extension/event system early so existing Sphinx extensions keep working under the Rust core — **done for core EventManager** (`src/sphinxdocrs/src/events.rs`: priority ordering, `allowed_exceptions`, `app.pdb` short-circuit, `ExtensionError` wrapping with `__cause__`)
- expose Python import surface mirroring phase 1's pattern — **done** (`sphinxdocrs.{EventManager,Project,SphinxError,…}` + `sphinxdocrs/python/sphinxdocrs_hybrid.py` with `event_manager` / `project` / `dispatch_plan` / `features` / `supports`)

### Phase 5 — plugin interoperability

- implement the entry-point–based resolver decided above; resolution order: declared Rust equivalent → Python implementation — **done** (`src/docutilsrs/python/docutilsrs_plugins.py`: entry-point group `docutilsrs.equivalents` + in-memory `register()` for tests; `resolve(target, prefer=...) -> Resolution(impl, source, reason)`)
- version guard: Rust equivalent declares a compatible upstream version range; mismatch falls back to Python with a warning — **done** (`Equivalent.upstream_requires` PEP 440 spec; mismatch emits `UserWarning` and falls through to `_load_python`)
- bidirectional calls: Rust → Python plugins via PyO3; Python → Rust crates via the published extension modules — **done** (Python→Rust via `dispatch()` returning Rust classes such as `sphinxdocrs.EventManager`; Rust→Python via the existing `docutilsrs.register_transform` bridge from Phase 3)
- end-to-end tests covering: pure-Python plugin, pure-Rust equivalent, fallback on version mismatch, mixed pipeline — **done** (`tests/test_phase5_resolver.py`, 9 tests including an e2e route of `sphinx.events:EventManager` through the resolver into `sphinxdocrs.EventManager`)

## Quality gates

- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `pytest` all green on every PR
- snapshot diffs against vendored Python output are the primary parity signal
- each phase exit requires: tests, compatibility-matrix update, and a short note in `docs/changes/`
- perf budget for parser: within 2× of CPython baseline by end of phase 2 (tightened later)
- no upstream-source modifications under `src/docutils/` or `src/sphinx/` outside of an explicit, isolated PR

## Milestone 1 (first iteration)

Status: **done.** All deliverables landed; exit criteria met.

- goal
  - land the bootstrap so a contributor can clone, build both crates, import them from Python, and run the test loop — **done**

- deliverables
  - decisions above recorded as ADRs (upstream pin, module names, bindings stack, doctree model, plugin discovery) — **done** (`docs/adr/0001-upstream-pin.md`, `0002-names.md`, `0003-bindings-and-layout.md`, `0004-doctree-representation.md`, `0005-plugin-discovery.md`)
  - `Cargo.toml` + `src/lib.rs` for `docutilsrs` and `sphinxdocrs`, each exporting a PyO3 `version()` — **done** (both crates plus the in-workspace `pygmentsrs` member also expose `version()` / `features()`)
  - `maturin develop` produces importable `docutilsrs` and `sphinxdocrs` modules — **done** (dev-loop block at the top of this README)
  - one `insta` snapshot test per crate proving the test harness works — **done** (`docutilsrs/tests/snapshot.rs`, `sphinxdocrs/tests/snapshot.rs`, `pygmentsrs/tests/snapshot.rs`)
  - CI workflow running fmt, clippy, cargo test, maturin build, and a pytest smoke test — **done** (`.github/workflows/ci.yml`)
  - `docs/compat.md` skeleton with the matrix columns defined — **done** (plus per-crate `src/pygmentsrs/docs/compat.md` for the Pygments port)

- exit criteria
  - fresh clone → documented commands → green build and green tests — **met** (Phase 5 gate: 18 docutilsrs + 1 sphinxdocrs + 8 pygmentsrs cargo tests; pytest suite green including parity, hybrid, resolver, and sphinxdocrs parity)
  - Python smoke test imports both modules and asserts on `version()` — **met** (`tests/test_smoke.py` for docutilsrs; `tests/test_sphinxdocrs_*` cover the sphinxdocrs import surface)
  - one ADR per locked decision is merged — **met** (5/5)

- explicit non-goals for M1 (kept out of scope for the bootstrap; subsequently delivered by later phases)
  - any real parsing — landed in Phase 1/2
  - any plugin resolution — landed in Phase 5
  - any writer output — landed in Phase 2 (pseudo-XML byte-parity, plus HTML5 / LaTeX / manpage / ODT structural and byte-parity gates)

## Documentation

### Port inventories & compatibility

- [docs/compat.md](docs/compat.md) — feature compatibility matrix
- [docs/docutils-port-inventory.md](docs/docutils-port-inventory.md) — docutils port inventory
- [docs/sphinx-port-inventory.md](docs/sphinx-port-inventory.md) — sphinx port inventory
- [docs/pygments-port-inventory.md](docs/pygments-port-inventory.md) — pygments port inventory
- [docs/handoff/pygments.md](docs/handoff/pygments.md) — pygmentsrs native syntax-highlighting handoff
- [docs/myst-md-port-inventory.md](docs/myst-md-port-inventory.md) — MyST-MD port inventory
- [docs/PYGMENTS_FEATURE_FLAGS.md](docs/PYGMENTS_FEATURE_FLAGS.md) — pygmentsrs feature flags

### Architecture & design

- [docs/BRIDGE_SELECTION_GUIDE.md](docs/BRIDGE_SELECTION_GUIDE.md) — guide to choosing Python/Rust bridge
- [docs/sphinxdocrs-cli-port-plan.md](docs/sphinxdocrs-cli-port-plan.md) — sphinxdocrs CLI port plan
- [docs/RATEX_INTEGRATION_PLAN.md](docs/RATEX_INTEGRATION_PLAN.md) — RaTeX integration plan
- [docs/RASTER_FORMAT_STRATEGY.md](docs/RASTER_FORMAT_STRATEGY.md) — raster image format strategy
- [docs/PYGMENTSRS_CODEBASE_EXPLORATION.md](docs/PYGMENTSRS_CODEBASE_EXPLORATION.md) — pygmentsrs codebase exploration notes
- [docs/E-phase-strategy.md](docs/E-phase-strategy.md) — E-phase strategy
- [docs/E4-analysis.md](docs/E4-analysis.md) — E4 analysis

### Sandbox & security

- jinja2rs sandboxing
  - [docs/SANDBOX_README.md](docs/SANDBOX_README.md) — sandbox overview
  - [docs/SANDBOX_IMPLEMENTATION_GUIDE.md](docs/SANDBOX_IMPLEMENTATION_GUIDE.md) — implementation guide
  - [docs/SANDBOX_SECURITY_ANALYSIS.md](docs/SANDBOX_SECURITY_ANALYSIS.md) — security analysis
  - [docs/SANDBOX_COMPARISON_MATRIX.md](docs/SANDBOX_COMPARISON_MATRIX.md) — comparison matrix
  - [docs/SECURITY_AUDIT_FORMATTERS.md](docs/SECURITY_AUDIT_FORMATTERS.md) — security audit: formatters
  - [docs/LIBSECCOMP_SETUP.md](docs/LIBSECCOMP_SETUP.md) — libseccomp platform setup

### Fuzzing
- [docs/FUZZING.md](docs/FUZZING.md) — fuzzing setup
- [docs/OSS_FUZZ_SETUP.md](docs/OSS_FUZZ_SETUP.md) — OSS-Fuzz setup
- [docs/OSS_FUZZ_ASAN_MSAN_IMPLEMENTATION.md](docs/OSS_FUZZ_ASAN_MSAN_IMPLEMENTATION.md) — OSS-Fuzz ASan/MSan implementation

### Django mode

- jinja2rs Django mode
  - [docs/DJANGO_MODE_DESIGN.md](docs/DJANGO_MODE_DESIGN.md) — design
  - [docs/DJANGO_MODE_IMPLEMENTATION.md](docs/DJANGO_MODE_IMPLEMENTATION.md) — implementation
  - [docs/DJANGO_MODE_SUMMARY.md](docs/DJANGO_MODE_SUMMARY.md) — summary
  - [docs/DJANGO_ARCHITECTURE_REFERENCE.md](docs/DJANGO_ARCHITECTURE_REFERENCE.md) — architecture reference
  - [docs/DJANGO_QUICK_REFERENCE.md](docs/DJANGO_QUICK_REFERENCE.md) — quick reference
  - [docs/DJANGO_USAGE_EXAMPLES.md](docs/DJANGO_USAGE_EXAMPLES.md) — usage examples

### Phase completion summaries

- [docs/PHASE_2_COVERAGE_SUMMARY.md](docs/PHASE_2_COVERAGE_SUMMARY.md)
- [docs/PHASE_3_COMPLETION_SUMMARY.md](docs/PHASE_3_COMPLETION_SUMMARY.md)
- [docs/PHASE_3_COVERAGE_ANALYSIS.md](docs/PHASE_3_COVERAGE_ANALYSIS.md)
- [docs/PHASE_4_COMPLETION_SUMMARY.md](docs/PHASE_4_COMPLETION_SUMMARY.md)
- [docs/PHASE_5_BRIDGE_TESTS.md](docs/PHASE_5_BRIDGE_TESTS.md)
- [docs/PHASE_5_COMPLETION_SUMMARY.md](docs/PHASE_5_COMPLETION_SUMMARY.md)
- [docs/PHASE_F_COMPLETION_SUMMARY.md](docs/PHASE_F_COMPLETION_SUMMARY.md)
- [docs/BRANCH_COVERAGE_IMPROVEMENTS.md](docs/BRANCH_COVERAGE_IMPROVEMENTS.md)

### Architecture decision records (ADRs)

- [docs/adr/0001-upstream-pin.md](docs/adr/0001-upstream-pin.md)
- [docs/adr/0002-names.md](docs/adr/0002-names.md)
- [docs/adr/0003-bindings-and-layout.md](docs/adr/0003-bindings-and-layout.md)
- [docs/adr/0004-doctree-representation.md](docs/adr/0004-doctree-representation.md)
- [docs/adr/0005-plugin-discovery.md](docs/adr/0005-plugin-discovery.md)

#### Decisions made but not yet recorded as ADRs

The following design choices were locked during Phases 2–5 and need ADRs
written under `docs/adr/`:

| # | Decision | Where decided | Phase |
|---|----------|---------------|-------|
| 0006 | **Transform / Pipeline API** — composable `Transform` trait + `Pipeline` struct in a standalone `docutilsrs::transforms` module, mirroring `docutils.transforms.*`; inline-in-parser transforms rejected | `src/docutilsrs/src/transforms.rs` | 2 / 3 |
| 0007 | **Hybrid-mode dispatch** — Python-side `docutilsrs_hybrid.py` routes parse / transform / write decisions via `publish_string(prefer=...)` + `dispatch_plan()`; per-component granularity, not all-or-nothing | `src/docutilsrs/python/docutilsrs_hybrid.py` | 3 |
| 0008 | **pygmentsrs scope and `backend="auto"` strategy** — spun as a separate workspace crate; scope is the top-N lexers used in Sphinx/RST docs; `backend="auto"` tries native Rust first and falls back to Python for any alias without a native implementation; byte-parity target is vendored `HtmlFormatter` | `src/pygmentsrs/` | 2.5 |
| 0009 | **Code-block dispatcher** — `docutilsrs::code_block` calls `pygmentsrs::tokenize` first; falls back to the `docutils.utils.code_analyzer.Lexer` PyO3 bridge only when the native path declines | `src/docutilsrs/src/code_block.rs` | 2.5 / 3 |
| 0010 | **ODT writer dual-path** — default native Rust path produces a valid `.odt` ZIP container (accepted-deviation); `compat=True` delegates via PyO3 to `docutils.writers.odf_odt` and is byte-parity-gated against all 13 upstream fixtures | `src/docutilsrs/src/writers/odt.rs` | 2 |
| 0011 | **Version guard for Rust equivalents** — `Equivalent.upstream_requires` field holds a PEP 440 specifier; a version mismatch emits `UserWarning` and falls through to `_load_python` rather than raising | `src/docutilsrs/python/docutilsrs_plugins.py` | 5 |
| 0012 | **Regex engine upgrade to `fancy-regex`** — upgraded from `regex` to `fancy-regex` to support backreferences required by heredoc patterns in `bash`, `sh`, `ksh`, `zsh`, and `shell` lexers; **landed** | `src/pygmentsrs/src/lexer/engine.rs`, `src/pygmentsrs/docs/compat.md` | 2.5 |

