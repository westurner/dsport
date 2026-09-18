# sphinxdocrs Branch Coverage and Console Parity Plan

## Scope and baseline

This plan covers hand-written Rust production code in `src/sphinxdocrs`, with
upstream behavior traced from `src/sphinx/tests/` and native behavior verified
by `src/sphinxdocrs/tests/`.

Measured on 2026-09-18:

- `cargo test -p sphinxdocrs --all-targets`: 821 tests passed in the latest
  clean run.
- Library-only LLVM branch coverage: 62.35% branches, 75.57% lines after the
  utility, console, assets, EventManager, warning-parity, make-mode, parser,
  configuration, environment, and extension tranches.
- The last completed all-target LLVM report measured 68.13% branches and
  86.11% lines; the all-target report is expensive and includes integration
  binaries, so library and all-target measurements must both remain visible.
- Recent focused results: `toctree` 100% branches, `events` 100%,
  `util_display` 100%, `util_uri` 97%+, `util_matching` 83%+,
  `util_console` 75%, `assets` 50%, `make_mode` 59%, `build/args.rs` 91.67%,
  `config.rs` 40%, `environment.rs` 40.65%, and `extension.rs` 83.33%.
- Existing external HTML parity remains a separate contract: 10/14 cases pass,
  with five documented theme/tree residuals.

The 100% target means every reachable branch in hand-written native runtime
code covered by the selected package targets. Generated lexer/template data,
vendored Python sources, and deliberately platform-exclusive branches are not
silently ignored: each must either receive a platform-gated test or be listed
in a narrowly scoped coverage exclusion with a reason and an owner.

## Upstream test review order

Use this order to preserve Sphinx's dependency flow and keep each addition
traceable to an upstream contract:

1. `test_util/test_util_display.py`, `test__cli/test__cli_util_errors.py`,
   `test_util/test_util_logging.py`, and `test_builders/test_build_warnings.py`
   for console/status/warning behavior.
2. `test_command_line.py`, `test_builders/test_build.py`,
   `test_builders/test_build_all.py`, and `test_builders/test_incremental_reading.py`
   for parser, dispatch, exit-code, and lifecycle branches.
3. `test_config/test_config.py`, `test_environment/test_environment.py`,
   `test_environment/test_environment_toctree.py`, and
   `test_util/test_util_matching.py` for configuration and filesystem branches.
4. Builder-specific suites in this order: `test_build_html*`, dirhtml,
   singlehtml, JSON, text/XML/pseudo-XML, linkcheck, LaTeX, manpage, texinfo,
   gettext, and EPUB.
5. Domain and extension suites after core builders: standard/RST/Python/JS
   domains, autodoc/autosummary, intersphinx, math, graphviz, and remaining
   extension hooks.
6. PyO3-facing tests (`test_events.py`, `test_application.py`, extension
   tests) after native pure logic is covered, using Python integration tests
   only where the public boundary requires Python objects or exception types.

Every ported case records one of: exact parity, accepted deviation, or pending.
The upstream test file and the native test file should be named in the test
module documentation or a nearby comment when the mapping is non-obvious.

## Coverage workstreams

### C0: Measurement and inventory

- Keep two repeatable commands:
  - fast loop: `cargo +nightly llvm-cov -p sphinxdocrs --lib --branch`
  - gate loop: `cargo +nightly llvm-cov -p sphinxdocrs --all-targets --branch`
- Export JSON summaries and retain per-file branch counts in
  `reports/<timestamp>/`.
- Add a small script or documented `jq` command that ranks files by missed
  branches, not only percentage; a 0%-covered 14-line adapter is cheaper than
  a 40%-covered 1,000-line subsystem.
- Treat coverage-run failures, timeout-prone integration fixtures, and skipped
  Python/theme tests as separate status categories rather than as covered code.

### C1: Pure utilities and data transforms

Complete the low-risk branch surface first. Targets include `util_console`,
`util_uri`, `util_matching`, `util_osutil`, `util_rst`, `util_strypes`,
`util_lines`, `util_docstrings`, `util_extra`, `toctree`, `genindex`, locale,
versioning, and `stemmer`.

Test dimensions:

- empty/default/invalid values;
- every enum/config alternative;
- path separator and missing-file behavior;
- malformed input and fallback branches;
- ordering, deduplication, recursion limits, and cycle guards;
- Python-compatible escaping, Unicode, and ANSI edge cases.

Exit gate: each module with fewer than 100 branches has either 100% branch
coverage or a documented platform/FFI reason, and utility tests are runnable
without Python subprocesses where the production path is pure Rust.

### C2: CLI parser, make mode, and logging lifecycle

Cover `build/parser.rs`, `build/args.rs`, `build/make_mode.rs`,
`build/native_runner.rs`, `build/logging.rs`, `cli/io.rs`, and the binaries.
Use upstream `test_command_line.py` as the contract for option placement,
missing arguments, make-mode rewriting, native-builder dispatch, Python
fallback, quiet modes, warning files, and exit codes.

Required cases:

- direct mode and `-M` mode with options before, between, and after
  positional arguments;
- parse failures return code 2 and do not start a build;
- native, unsupported, and explicit Python-fallback builders;
- `-q`, `-Q`, `-w`, `-W`, `--keep-going`, `--exception-on-warning`, and
  combinations of those options;
- successful build, build error, warning-as-error, and warning-file write
  failure paths;
- stdout stays empty for status output when the selected contract says stderr,
  and all path/ANSI normalization is deterministic.

### C3: Config, application, environment, and events

Cover `config.rs`, `application.rs`, `environment.rs`, `events.rs`,
`app_events.rs`, `extension.rs`, `app_facade.rs`, and `registry.rs`.
Port upstream `test_application.py`, `test_events.py`, config tests, and the
environment/toctree tests in dependency order.

Required cases:

- default values, overrides, invalid values, type conversion, and config
  status changes;
- extension registration, duplicate registration, version guards, missing
  dependencies, event ordering, listener removal, first-result behavior,
  allowed exception passthrough, and wrapped exceptions;
- source-read/doctree-read/build-finished lifecycle and warning accumulation;
- persisted doctrees, incremental read decisions, missing sources, encoding
  failures, and environment consistency warnings;
- Python object conversion through the PyO3 boundary, including `None`, bad
  types, exception classes, and `app.pdb` behavior.

Use Rust unit tests for state machines and Python integration tests only for
actual PyO3 contracts. This avoids inflating coverage by testing implementation
details through a slower bridge.

### C4: Builders and external integrations

Cover each branch of the builders after their shared environment contract is
stable. Prioritize the currently weak modules: JSON, linkcheck, manpage,
texinfo, LaTeX, EPUB, HTML fallback paths, and native runner error paths.

For every builder, test:

- empty input and one representative document;
- nested paths and invalid/sanitized docnames;
- success, missing dependency, malformed input, and write failure;
- synthetic pages and asset/index generation where applicable;
- builder-specific warning and exit behavior.

Use `wiremock`/temporary directories for HTTP and filesystem branches. Do not
use broad external documentation builds as the only coverage mechanism: they
are parity tests, not deterministic branch tests.

### C5: Coverage completion and policy

- Re-run library and all-target reports after each workstream.
- Add `--fail-under` gates only after the report is stable; ratchet in small
  increments (for example 70%, 80%, 90%, 95%, then 100% branch coverage).
- For each remaining branch, classify it as tested, unreachable by design,
  platform-only, FFI-only, or a real missing test. No blanket source-directory
  exclusion is permitted.
- Update this plan and the repository port plan with the measured percentage,
  test count, and remaining exceptions after every tranche.

## Console-output parity plan

### P0: Establish a stream contract

Create a single native output contract around status, warning, and error
streams. The contract must make these fields explicit:

- stream: stdout, stderr, warning file, or error database;
- severity: debug, verbose, info, warning, critical, error;
- verbosity threshold and quiet/really-quiet behavior;
- newline/non-newline behavior;
- source location and warning type/subtype;
- ANSI policy: auto, forced, disabled, and file stripping.

Keep `eprintln!` out of new build paths once the contract is in place. Existing
Rust-specific prefixes such as `sphinxdocrs:` remain allowed only where the
parity harness classifies them as native metadata.

### P1: Match pure formatting behavior

Port and test the contracts from `test_util_display.py` and
`test__cli/test__cli_util_errors.py`:

- `display_chunk` for strings, one-item sequences, and multi-item sequences;
- status iterator output for zero/known lengths and verbosity 0/1;
- progress success, skipped, failed, decorator, and non-newline logging;
- ANSI color names, unknown names, disabled color, forced color, short forms
  (`ESC[m`, `ESC[0m`, `ESC[K`), cursor controls, and Unicode-safe stripping;
- terminal-safe conversion for `\\x`, `\\u`, and `\\U` escapes.

These should be deterministic unit tests with captured `Vec<u8>` sinks.

### P2: Match warning semantics

Use `test_util_logging.py` and `test_build_warnings.py` as the behavioral
matrix:

- severity routing at verbosity 0/1/2;
- `nonl`, once-only warnings, suppression contexts, pending warning buffers,
  prefixed warnings, warning type/subtype display, and source locations;
- `-q` suppresses status only; `-Q` suppresses warnings; `-w` writes ANSI-free
  warnings; `-W` controls exit status; `--keep-going` controls continuation;
- warning-file and stderr output preserve ordering and final newlines;
- build warnings and native errors normalize to the same `WARNING:`/`ERROR:`
  prefixes and location shape as Python.

Progress: native logging now carries parsed verbosity/color modes, formats
ANSI-aware warning lines, writes ANSI-free full warning lines, shares a build
success formatter between direct and make mode, and formats orphan warnings as
`<source>: WARNING: ... [toc.not_included]`. The make-mode startup banner was
removed as native-only noise. Remaining C2 work is status-iterator wiring into
real build progress, warning type/location coverage beyond orphan warnings,
and subprocess assertions for `-q`, `-Q`, `-w`, and `-W`.

Make-mode coverage now exercises clean safety errors, `PAPER` injection,
runner I/O failures, and the `latexpdf` build/make dispatch using injected
runners. Remaining C2 work is the direct parser/error matrix and subprocess
coverage for the quiet/warning options.

Integrated `BuildArgs` coverage now exercises builder, jobs, paths, isolated
mode, `-D`/`-A`, tags, verbosity, quiet/silent/color, warning files,
warning-as-error, fallback, scan-requirements, filename validation, and clap
errors. The remaining parser work is binary-level stdout/stderr/exit-code
assertion coverage rather than helper-level argument parsing.

Environment edge coverage now exercises longest source-suffix selection,
unknown parser rejection, doctree path traversal rejection, and corrupt
doctree detection. The remaining environment gap is concentrated in domain
resolution, notebook/YAML registration, incremental persistence, and eventful
read/write lifecycle branches.

Extension coverage now exercises metadata defaults and mutation, missing
required extensions, valid and too-old versions, unknown-version failures, and
the no-op `needs_extensions = None` path. The remaining C3 gap is concentrated
in application/facade/config lifecycle branches and domain/event integration.

### P3: CLI and make-mode parity

Port the option-placement and failure cases from `test_command_line.py` into
native parser/make-mode tests. Add subprocess tests that compare:

- exit code;
- stdout exactness after path normalization;
- stderr after ANSI stripping and volatile path/version normalization;
- warning-file bytes;
- whether Python fallback was announced.

The existing parity snapshots should become structured assertions first, then
snapshots for stable residual output. A native-only line is either removed,
normalized as named metadata, or documented as an accepted deviation.

### P4: Build lifecycle output

Align `sphinx_build.rs`, `NativeMakeRunner`, `build/logging.rs`, and
`error_log.rs` around one formatter. Cover startup, success, warning, error,
scan-requirements, fallback, and make-mode messages. Add a small fixture matrix
for HTML, dirhtml, and a delegated builder.

The success contract should include the builder, file count, and final newline;
the warning contract should include deterministic ordering and no ANSI in
`-w` files; the error contract should preserve exit code and avoid duplicate
messages.

### P5: Near-parity gate

Define near parity as:

- exact exit codes for supported command paths;
- exact stdout/stderr after only named volatile normalization;
- exact warning-file content after ANSI stripping;
- identical severity/location/type ordering for shared warning fixtures;
- explicit deviations for native-only prefixes, WebMCP metadata, and Python
  fallback notices.

Run the gate against the upstream fixtures used by `parity.rs`, then update the
normalized log snapshots only after structured assertions pass.

Current console progress: the normalized HTML stderr parity test passes after
the orphan-warning and startup-banner fixes. Native output still intentionally
retains the `Build succeeded: <N> file(s) written.` completion line, which is
covered by a native assertion and remains the next decision point for exact
Python stderr parity.

## Execution order and commits

1. Commit this plan.
2. Implement P0/P1 console sinks and pure display/ANSI tests.
3. Implement P2 warning routing and warning-file semantics.
4. Implement C2 parser/make-mode coverage and P3 subprocess parity.
5. Implement C3 event/config/environment coverage.
6. Implement C4 builder-specific coverage in missed-branch order.
7. Ratchet coverage gates and update both plans with measured results.

Keep each tranche independently testable and committed. Do not stage unrelated
submodule, generated, or devcontainer changes already present in the worktree.
