# sphinxdocrs Branch Coverage and Console Parity Plan

## Scope and baseline

This plan covers hand-written Rust production code in `src/sphinxdocrs`, with
upstream behavior traced from `src/sphinx/tests/` and native behavior verified
by `src/sphinxdocrs/tests/`.

- Measured on 2026-09-18:
  - `cargo test -p sphinxdocrs --all-targets`: 829 tests passed in the latest
    clean run.
  - Library-only LLVM branch coverage: 63.25% branches, 76.35% lines after the
    utility, console, assets, EventManager, warning-parity, make-mode, parser,
    configuration, environment, extension, Project, and docindex tranches.

  - The last completed all-target LLVM report measured 68.13% branches and
    86.11% lines; the all-target report is expensive and includes integration
    binaries, so library and all-target measurements must both remain visible.
  - Recent focused results: `toctree` 100% branches, `events` 100%,
    `util_display` 100%, `util_uri` 97%+, `util_matching` 83%+,
    `util_console` 75%, `assets` 50%, `make_mode` 59%, `build/args.rs` 91.67%,
    `config.rs` 40%, `environment.rs` 40.65%, `extension.rs` 83.33%,
    `project.rs` 78.57%, and `extensions/docindex.rs` 91.67%.
  - Existing external HTML parity remains a separate contract: 10/14 cases pass,
    with five documented theme/tree residuals.
- Measured on 2026-09-19:
  - after the `app_facade.rs`/`application.rs`/`config.rs`/
    `environment.rs` tranches described in the branch map below:
    library-only branch coverage is 68.19% (788/2477 missed),
    with 906 lib tests passing (`cargo test -p sphinxdocrs --lib`).
- Measured on 2026-09-18 (C4 builder sweep):
  - Fresh library-only coverage is 71.46% branches (707/2477 missed) and
    83.96% lines (4173/26024 missed).
  - Fresh all-target coverage is 75.20% branches (700/2823 missed) and
    91.54% lines (2235/26418 missed).
  - `builders/json.rs` is 77.38% branches in both scopes; `builders/html.rs`
    is 71.43% library-only / 73.81% all-targets; `builders/linkcheck.rs` is
    48.39% library-only / 77.42% all-targets.
  - The focused linkcheck validation passes 14 library tests and 11 wiremock
    integration tests.
- Measured on 2026-09-19 (C0-C3 follow-up sweep):
  - Library-only branch coverage is 75.76% (602/2483 missed) and lines are
    86.43% (3655/26937 missed).
  - `build/logging.rs` is now 100% branches; `util_strypes.rs` is 90.79%,
    `config.rs` is 78.79%, `environment.rs` is 67.50%, `build/make_mode.rs`
    is 73.17%, `util_matching.rs` is 85.19%, and `domains/scan.rs` is 79.05%.
  - The focused additions cover C0 ranking, C1 matching/string edges, C2
    make-mode/argument/logging branches, and C3 config/environment/docindex
    branches. The next large gaps remain environment lifecycle, theme/domain,
    autodoc, HTML fallback, and application paths.
- Measured on 2026-09-19 (final all-target gate for this tranche):
  - All-target branch coverage is 77.20% (645/2829 missed) and lines are
    92.73% (1987/27331 missed); functions are 90.81% (282/3069 missed).
  - Integration-test scope changes the key rows: `environment.rs` is
    library 90/280 (67.86%) versus all-targets 55/286 (80.77%); `config.rs`
    is 42/198 (78.79%), `domains/scan.rs` is 31/148 (79.05%),
    `util_console.rs` is 6/28 (78.57%), and the tiny writer rows are 1/4
    (75%) except `manpage.rs` at 4/12 (66.67%).
- Measured on 2026-09-19 (priority-gap continuation):
  - Library-only branch coverage is 77.89% (565/2555 missed) and lines are
    89.00% (3011/27374 missed).
  - `apidoc/generate.rs` is now 13/52 missed branches, `environment.rs` is
    74/296, `config.rs` is 28/198, `autodoc.rs` is 28/122,
    `builders/html.rs` is 34/126, and `theme_render.rs` is 29/138.
  - Added direct resolver coverage for xrefs/toctrees, raw config conversion
    and `read_conf_py`, autodoc selector/signature matrices, and HTML/theme
    URI, asset, wrapper, and soft-failure helpers.
- Measured on 2026-09-19 (priority-gap all-target gate):
  - All-target branch coverage is 78.87% (600/2839 missed) and lines are
    93.51% (1801/27768 missed); functions are 91.56% (261/3092 missed).
  - The corresponding all-target rows are `environment.rs` 57/296 (80.74%),
    `builders/html.rs` 32/126 (74.60%), `theme_render.rs` 22/138 (84.06%),
    `config.rs` 28/198 (85.86%), `autodoc.rs` 28/122 (77.05%), and
    `apidoc/generate.rs` 29/78 (62.82%).

Coverage scope note: `cargo +nightly llvm-cov ... --lib` does not execute
`src/sphinxdocrs/tests/*.rs` integration-test binaries. Use the all-targets
measurement for builder files whose HTTP or end-to-end paths are covered by
those binaries; otherwise the library-only percentage can materially
understate actual branch coverage. A clean all-targets run is intentionally
slow because it rebuilds every instrumented target and executes every test
binary; use `cargo +nightly llvm-cov clean --workspace -p sphinxdocrs` only
for an authoritative fresh baseline, and capture the unfiltered command
output before filtering summary rows so a failed run cannot be mistaken for
a hang or an `rg` exit code 1.

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
- `tools/rank_llvm_cov.py` implements the ranking loop for text summaries:
  `cargo +nightly llvm-cov -p sphinxdocrs --lib --branch --summary-only |
  python tools/rank_llvm_cov.py --limit 20`. It accepts a saved report as
  well, handles whitespace-wrapped rows, and omits `TOTAL`.
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

Project coverage now exercises PyO3 constructor iterables, getters, discovery
include/exclude filters, pathlike and absolute path conversion, recorded paths,
and fallback `doc2path` behavior. Remaining C3 gaps are concentrated in the
application/facade lifecycle and larger environment/domain integrations.

Docindex coverage now exercises setup, builder/feature skips, artifact path
resolution, artifact/HDT toggles, and both HTML-family builders. Its remaining
branch is the external indexer error path and belongs in the later integration
error-injection sweep.

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

## Complete branch map (2026-09-18)

This is the complete nonzero-branch inventory from the library-only LLVM
report. Entries are ordered by missed branches, so the table is also the
execution order. `missed/total` is authoritative; percentages are rounded.

| Module | Missed/total | Branch % | Upstream coverage source |
| --- | ---: | ---: | --- |
| `environment.rs` | lib 74/296; all 57/296 | lib 75.00%; all 80.74% | `test_environment/test_environment.py`, `test_environment_toctree.py`, `test_environment_record_dependencies.py` (updated 2026-09-19: added direct xref/toctree resolver cases covering explicit/shortened/unresolved refs, extlinks, hidden/caption/depth handling, plus prior discovery/YAML/read-event coverage; remaining misses are concentrated in larger lifecycle/domain branches) |
| `config.rs` | lib 28/198; all 28/198 | lib 85.86%; all 85.86% | `test_config/test_config.py`, `test_config/test_copyright.py` (updated 2026-09-19: added raw conversion wrong-shape coverage and the `read_conf_py` wrapper; remaining misses are defensive PyO3/raw-shape edges) |
| `builders/html.rs` | lib 34/126; all 32/126 | lib 73.02%; all 74.60% | `test_builders/test_build_html*.py`, `test_build_html_assets.py`, `test_build_html_toctree.py`, `test_build_warnings.py` (updated 2026-09-19: added wrapper title, dirhtml target, and static-file layout cases) |
| `builders/linkcheck.rs` | lib 32/62; all 14/62 | lib 48.39%; all 77.42% | `test_builders/test_build_linkcheck.py`, HTTP/logging tests |
| `builders/json.rs` | lib 19/84; all 19/84 | lib 77.38%; all 77.38% | `test_builders/test_build.py`, JSON parity fixtures |
| `app_facade.rs` | 7/48 | 85% | `test_application.py`, `test_events.py`, extension tests (updated 2026-09-19: added ~25 native unit tests covering config/env facades, event dispatch/connect, node/directive/role/domain/theme/builder registration, and error conversion; remaining misses are defensive `map_err` arms on effectively-infallible PyO3 conversions and the `doctree: Some(_)` arm of `HtmlPageContext`) |
| `theme_render.rs` | lib 29/138; all 22/138 | lib 78.99%; all 84.06% | `test_theming/*`, `test_build_html_5_output.py`, parity fixtures (updated 2026-09-19: added URI/content-root, checksum/resource, context conversion, static-asset filtering, and unresolvable-theme fallback cases) |
| `domains/scan.rs` | 31/148 | 79% | `test_domains/*`, `test_environment_toctree.py` (updated 2026-09-19: added malformed label, glossary, role, xref, index, and domain-context cases) |
| `autodoc.rs` | lib 28/122; all 28/122 | lib 77.05%; all 77.05% | `test_ext_autodoc/*.py` (updated 2026-09-19: added option-pair/selector matrices and complex signature-expression coverage) |
| `application.rs` | 32/52 | 38% | `test_application.py`, `test_extension.py` (checked 2026-09-19: added `AppError` Display/From and `SphinxApp` Debug/outdir-is-a-file tests, which raised line/region coverage but did not touch the still-missing branches, which are concentrated in `load_extension`'s Rust-equivalent/version-guard/Python-fallback logic (~396-611) and `sync_registered_themes` (~684-840, 1061-1062)) |
| `http_client.rs` | 18/24 | 25% | linkcheck/intersphinx HTTP tests |
| `make_mode.rs` | 11/41 | 73% | `test_command_line.py`, make-mode tests (updated 2026-09-19: covered explicit doctree paths, recursive clean, build short-circuiting, `latexpdfja`, `info`, and `gettext` dispatch) |
| `builders/latex.rs` | 16/38 | 58% | `test_builders/test_build_latex.py` |
| `util_strypes.rs` | 7/76 | 91% | `test_util/test_util_rst.py`, writer tests (updated 2026-09-19: added PO escape alternatives, malformed escapes, attribute-name edges, XML `>`, and C1/ESC terminal controls) |
| `extensions/docindex.rs` | 2/14 | 86% | extension/inventory tests (updated 2026-09-19: added the broken-symlink indexing error conversion; remaining misses include defensive artifact/HDT write paths) |
| `search.rs` | 12/52 | 77% | `test_search.py`, HTML toctree tests |
| `builders/manpage.rs` | 4/12 | 67% | `test_builders/test_build_manpage.py` (updated 2026-09-19: added `man_pages` shape/duplicate validation, configured output, stored-doctree and source fallback paths) |
| `locale.rs` | 11/62 | 82% | `test_intl/test_locale.py`, `test_intl/test_intl.py` |
| `quickstart/validate.rs` | 11/20 | 45% | `test_quickstart.py` |
| `autodoc_runtime.rs` | 10/52 | 81% | `test_ext_autodoc/test_ext_autodoc_importer.py` |
| `apidoc/generate.rs` | lib 13/52; all 29/78 | lib 75.00%; all 62.82% | `test_extensions/test_ext_apidoc.py` (updated 2026-09-19: added generator unit coverage for package/module filtering, namespace handling, sorted walks, write modes, stale-file removal, separate modules, and TOC deduplication) |
| `intl.rs` | 10/64 | 84% | `test_intl/test_catalogs.py`, `test_intl/test_intl.py` |
| `theme_static.rs` | 10/26 | 62% | `test_theming/*`, `test_build_html_assets.py` |
| `util_matching.rs` | 8/54 | 85% | `test_util/test_util_matching.py` (updated 2026-09-19: added nested include/exclude and unrelated-path fallback cases) |
| `domains/py_sig.rs` | 7/20 | 65% | `test_domains/test_domain_py*.py` |
| `intersphinx.rs` | 7/36 | 81% | `test_ext_intersphinx/*.py` |
| `quickstart/parser.rs` | 7/26 | 73% | `test_quickstart.py` |
| `util_console.rs` | 6/28 | 79% | `test__cli/test__cli_util_errors.py`, `test_util_display.py` (updated 2026-09-19: covered all named colors, environment aliases, and the Python registration surface; remaining edges are short-circuit branches attributed to already-covered lines) |
| `autogen/generate.rs` | 6/26 | 77% | autosummary/apidoc tests |
| `build/native_runner.rs` | 6/10 | 40% | `test_command_line.py`, build lifecycle tests |
| `builders/changes.rs` | 6/20 | 70% | `test_builders/test_build_changes.py` |
| `extensions/webmcp.rs` | 6/10 | 40% | native WebMCP contract tests |
| `util_docstrings.rs` | 5/32 | 84% | `test_util/test_util_docstrings.py` |
| `util_rst.rs` | 5/18 | 72% | `test_util/test_util_rst.py` |
| `autogen/scan.rs` | 4/26 | 85% | autosummary/apidoc tests |
| `build/logging.rs` | 0/18 | 100% | `test_util/test_util_logging.py`, `test_build_warnings.py` (updated 2026-09-19: covered prefix preservation, explicit color, suppression, and warning-file failure) |
| `builders/gettext.rs` | 4/14 | 71% | `test_builders/test_build_gettext.py` |
| `builders/singlehtml.rs` | 4/16 | 75% | `test_builders/test_build_html*.py` |
| `cli/io.rs` | 4/8 | 50% | `test_command_line.py` |
| `domains/py_domain.rs` | 4/12 | 67% | `test_domains/test_domain_py*.py` |
| `domains/std_domain.rs` | 4/12 | 67% | `test_domains/test_domain_std.py` |
| `addnodes.rs` | 3/10 | 70% | `test_addnodes.py` |
| `assets.rs` | 3/6 | 50% | asset/integrity tests |
| `builders/texinfo.rs` | 1/4 | 75% | `test_builders/test_build_texinfo.py` (updated 2026-09-19: added trait metadata, nested output, explicit docnames, and missing-source paths) |
| `domains/js_domain.rs` | 3/12 | 75% | `test_domains/test_domain_js.py` |
| `project.rs` | 3/14 | 79% | `test_project.py`, `test_util/test_util_matching.py` |
| `quickstart/generate.rs` | 3/16 | 81% | `test_quickstart.py` |
| `util_lines.rs` | 3/20 | 85% | `test_util/test_util_lines.py` |
| `versioning.rs` | 3/28 | 89% | `test_versioning.py` |
| `build/args.rs` | 1/24 | 96% | `test_command_line.py` (updated 2026-09-19: covered `ConfValue` display and empty `--confdir`; one defensive/parser-generated branch remains) |
| `builders/epub.rs` | 1/4 | 75% | `test_builders/test_build_epub.py` (updated 2026-09-19: added configured metadata and explicit-docname build coverage) |
| `builders/pseudoxml.rs` | 1/4 | 75% | writer tests (updated 2026-09-19: added explicit-docname and missing-source paths) |
| `builders/text.rs` | 1/4 | 75% | `test_builders/test_build_text.py` (updated 2026-09-19: added explicit-docname and missing-source paths) |
| `builders/xml.rs` | 1/4 | 75% | XML writer tests (updated 2026-09-19: added explicit-docname and missing-source paths) |
| `extension.rs` | 2/12 | 83% | `test_extensions/test_extension.py`, `test_events.py` |
| `genindex.rs` | 2/4 | 50% | HTML builder tests |
| `registry.rs` | 2/16 | 88% | `test_application.py`, extension tests |
| `apidoc/parser.rs` | 1/6 | 83% | `test_extensions/test_ext_apidoc.py` |
| `autogen/templates.rs` | 1/2 | 50% | autosummary templates |
| `extensions/mod.rs` | 1/4 | 75% | extension loading tests |
| `util_uri.rs` | 1/66 | 98% | `test_util/test_util_uri.py` |

Modules with zero branch points remain in the line/error test inventory but do
not affect branch gates. Generated/static template data, platform-exclusive
wrappers, and unavailable FFI branches require an explicit policy entry rather
than a blanket exclusion.

## Upstream-to-native execution backlog

1. **Application boundary:** `app_facade.rs`, `application.rs`, and
  `extensions/docindex.rs` from `test_application.py`, `test_events.py`, and
  extension tests. Cover lifecycle event ordering, Python callback argument
  conversion, extension load failures, registered assets, and docindex setup.
2. **Environment/config completion:** remaining `environment.rs` and
  `config.rs` branches from `test_environment*` and `test_config/test_config.py`:
  YAML/notebook registration, incremental persistence, dependency changes,
  config status transitions, typed override failures, and eventful reads.
3. **Builder branch sweep:** JSON, HTML fallback/theme errors, linkcheck,
  manpage, LaTeX, and text/XML/pseudo-XML from their `test_builders` files;
  use temporary files and wiremock instead of broad docs builds.
4. **Domain/search/theme sweep:** `domains/scan.rs`, `theme_render.rs`,
  `autodoc.rs`, `autodoc_runtime.rs`, `search.rs`, locale/intl, and
  intersphinx from the matching domain/extension/util suites.
5. **Tail and policy:** close every 1-10 missed-branch module, classify
  unreachable/FFI/platform-only branches, and add ratcheted branch gates at
  70%, 80%, 90%, 95%, and finally 100%.
