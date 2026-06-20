# sphinx port inventory (Phase 4)

Generated from `src/sphinx/tests/` and `src/sphinx/sphinx/` to plan the
incremental Rust port (`sphinxdocrs`). Each row tags a test or
subsystem with its phase priority. **P1** = port now (small, pure
Python, few deps). **P2** = port after extension/event scaffolding.
**P3** = depends on a builder, environment, or domain pipeline that is
not yet ported — keep as parity probes only.

## Subsystem priorities

| subsystem (sphinx) | sphinxdocrs target | priority | notes |
| --- | --- | --- | --- |
| `errors.py` | `sphinxdocrs::errors` | **P1** | pure exception hierarchy; `pyo3::create_exception!` |
| `events.py` | `sphinxdocrs::events` | **P1** | `EventManager`: connect/disconnect/emit/emit_firstresult + priority sort + `allowed_exceptions` + `pdb` re-raise + `ExtensionError` wrapping |
| `project.py` | `sphinxdocrs::project` | **P1** | **mirrored** — `path2doc`/`doc2path`/`discover` landed in `src/sphinxdocrs/src/project.rs`; `discover()` uses Rust `util_matching` for glob exclusion (`EXCLUDE_PATHS` parity) |
| `addnodes.py` | n/a (Python re-export) | **P1** | extends docutils.nodes — keep as Python shim that imports vendored `sphinx.addnodes` until our doctree gains Sphinx-specific node types |
| `extension.py` | `sphinxdocrs::extension` | **P2** | **mirrored** — `Extension` wrapper + `verify_needs_extensions` landed in `src/sphinxdocrs/src/extension.rs`; gated by `tests/test_sphinxdocrs_extension.py` |
| `registry.py` | `sphinxdocrs::registry` | **P2** | builder/parser/transform/translator registries; depends on extension + events |
| `config.py` | `sphinxdocrs::config` | **P2** | depends on `util.typing`, complex value coercion; port `Config` after util |
| `cmd/quickstart.py` | `sphinxdocrs::quickstart` | **C1** | **mirrored** — all 7 validators, `ask_user`, `generate`, `valid_dir`, full clap parser in `src/sphinxdocrs/src/quickstart/`; 50 Rust-side integration tests; `sphinx-quickstart-rs` binary native by default |
| `cmd/build.py` + `cmd/make_mode.py` | `sphinxdocrs::build` | **C2** | **partial** — arg parser, all `_parse_*` helpers, `jobs_argument`, `MakeMode` (`build_clean`, `build_help`, `run_generic_build`, full `BUILDERS` table, target dispatch) ported in `src/sphinxdocrs/src/build/`; 35 Rust-side integration tests; `sphinx-build -M` runs natively; `sphinx-build -b` delegates to Python until builders land |
| `ext/apidoc.py` | `sphinxdocrs::apidoc` | **C3** | **partial** — `ApidocOptions`, `recurse_tree`, `create_{module,package,modules_toc}_file`, `remove_old_files`, full clap parser in `src/sphinxdocrs/src/apidoc/`; 24 Rust-side integration tests; `sphinx-apidoc-rs` native by default; `--full` delegates to Python |
| `ext/autosummary/generate.py` | `sphinxdocrs::autogen` | **C4** | **done** — RST scan, full clap parser, `underline`+`_` identity filters + 3 vendored stub templates + `generate_stub`/`generate_stubs` (heuristic type detection, empty member lists, autodoc fills at build time) in `src/sphinxdocrs/src/autogen/`; 32 Rust-side tests; `sphinx-autogen-rs` fully native |
| `roles.py` / `directives/` | `sphinxdocrs::roles` etc | **P3** | needs the doctree converter (already in `docutilsrs::python`) and the directive/role registry |
| `domains/` | `sphinxdocrs::domains` | **P3** | each domain is a substantial subsystem (`py`, `c`, `cpp`, `js`, `rst`, `std`) |
| `environment/` | `sphinxdocrs::environment` | **P3** | the build environment, large and stateful |
| `builders/` | `sphinxdocrs::builders` | **P3** | one builder at a time (`html`, `latex`, `epub`, ...) |
| `ext/*` | n/a (Python plugins) | **P3** | keep as Python; loaded via `Extension` registry |
| `util/*` | `sphinxdocrs::util::*` | **P2** | **mirrored (matching + console)** — `compile_matchers` / `Matcher` / `get_matching_files` ported to `src/sphinxdocrs/src/util_matching.rs`; `sphinx.util.console` + `sphinx._cli.util.colour` + `sphinx._cli.util.errors` colour/escape surface (`colourise`, `disable_colour`/`enable_colour`, `strip_escape_sequences`, `terminal_safe`, 22 colour escape codes) ported to `src/sphinxdocrs/src/util_console.rs`; other `util/*` pieces ported on demand |
| `theming.py` | n/a | **P3** | jinja2-bound; keep Python until templating story decided |
| `search/` | n/a | **P3** | indexer + JS bridge; keep Python |

## Test triage

Tagged from `src/sphinx/tests/`. Status legend: **mirrored** = a
parity-checked Rust-side test exists; **stub** = test file scaffolded
without a Rust impl; **deferred** = remains pure Python until the
underlying subsystem is ported.

| test file | subsystem | tier | status this phase |
| --- | --- | --- | --- |
| `test_errors.py` | errors | P1 | mirrored — `tests/test_sphinxdocrs_errors.py` |
| `test_events.py` | events | P1 | mirrored — `tests/test_sphinxdocrs_events.py` |
| `test_project.py` | project | P1 | mirrored — `tests/test_sphinxdocrs_project.py` + `tests/test_sphinxdocrs_project_discover.py` (basic discovery, exclude patterns, multi-suffix, recorded `doc2path`, default `EXCLUDE_PATHS`) |
| `test_addnodes.py` | addnodes | P1 | deferred (no Sphinx-specific nodes in Rust doctree yet) |
| (no upstream test_extension.py) | extension | P2 | mirrored — `tests/test_sphinxdocrs_extension.py` (8 cases: defaults, kwargs-pop semantics, explicit-None preservation, `verify_needs_extensions` parity) |
| `test_application.py` | application | P3 | deferred |
| `test_command_line.py`, `test__cli/` | cli | P3 | **partial** — arg-parsing layer ported natively (`build::parser`, `build::args`, `build::logging`, `build::make_mode`); full `Sphinx()` invocation deferred |
| `test_config/` | config | P2 | deferred |
| `test_directives/` | directives | P3 | deferred |
| `test_domains/` | domains | P3 | deferred |
| `test_environment/` | environment | P3 | deferred |
| `test_ext_*` | extensions | P3 | deferred (run as-is against vendored sphinx) |
| `test_extensions/` | extension loader | P2 | deferred |
| `test_highlighting.py` | highlighting | P3 | depends on Pygments port (`pygmentsrs`) |
| `test_intl/` | intl | P3 | deferred |
| `test_markup/` | markup | P3 | depends on docutils converter |
| `test_pycode/` | pycode | P3 | deferred |
| `test_quickstart.py` | quickstart | **C1** | **mirrored** — `quickstart::validate` (all 7 validators), `quickstart::parser` (full clap flag grammar), `quickstart::generate`, `quickstart::ask_user`, `quickstart::valid_dir` ported; 50 Rust-side tests in `tests/quickstart.rs` (11 validator `#[case]` tables, 8 parser flag tests, 4 `valid_dir` tests, 4 tree-layout insta snapshots, `conf_py_snapshot`, newline-mode assertions, `ask_user` scripted-terminal test, help-text snapshot); `sphinx-quickstart-rs` binary now runs natively, falling back to Python only on `--use-python-impl` / `SPHINXDOCRS_PY_FALLBACK=1` |
| `test_roles.py` | roles | P3 | deferred |
| `test_search.py` | search | P3 | deferred |
| `test_theming/` | theming | P3 | deferred |
| `test_transforms/` | transforms | P3 | deferred (per-transform port) |
| `test_util/` | util | P2 | mirrored (matching + console) — `tests/test_sphinxdocrs_util_matching.py` (12 cases) and `tests/test_sphinxdocrs_util_console.py` (40 byte-parity cases over `colourise` for all 22 colours, `disable_colour`/`enable_colour` round-trip, 8 `strip_escape_sequences` fixtures including SGR + EL, 6 `terminal_safe` fixtures including non-ASCII + emoji + tabs/newlines, and the `util:console` capability flag). Other `test_util/*` pieces deferred. |
| `test_versioning.py` | versioning | P2 | deferred |
| `test_writers/` | writers | P3 | deferred (one writer at a time) |
| `test_builders/` | builders | P3 | deferred (one builder at a time) |
| `test_ext_autodoc/`, `test_ext_autosummary/`, `test_ext_imgconverter/`, `test_ext_intersphinx/`, `test_ext_napoleon/` | extensions | P3 | deferred — these run against vendored Python sphinx |
| `test_ext_apidoc/` | apidoc | **C3** | **mirrored** — `recurse_tree` + file generators ported; 24 Rust-side tests in `tests/apidoc.rs`; `--full` now wired to `quickstart::generate` |
| `test_ext_autosummary/` | autosummary/autogen | **C4** | **done** — RST scan + native stub generation ported; 32 Rust-side tests in `tests/autogen.rs` |
| `js/` | search JS | n/a | external |

## Exit criteria for Phase 4 (incremental)

1. P1 modules (`errors`, `events`, `project`) land in Rust with PyO3
   bindings.
2. Mirrored tests for each P1 module pass and are gated on `pytest -q`.
3. A `sphinxdocrs_hybrid` Python wrapper exists that exposes the Rust
   `EventManager` and falls back to `sphinx.events.EventManager` if the
   Rust extension is missing — so existing extensions can keep
   working.
4. Inventory (this file) is updated whenever a row's status changes.

## CLI porting milestone (C-phase) — current status

| step | target | status |
| --- | --- | --- |
| **C1** `sphinx-quickstart` | `sphinxdocrs::quickstart` | **done** — 50 tests green; native binary default; Python fallback on `--use-python-impl` |
| **C2a** `sphinx-build` parser + `_parse_*` | `sphinxdocrs::build::args` | **done** — 35 tests green (all `jobs_argument`, `parse_confdir`, `parse_doctreedir`, `validate_filenames`, `parse_confoverrides`, `parse_color` param tables) |
| **C2b** `sphinx-build -M` make-mode | `sphinxdocrs::build::make_mode` | **done** — `build_clean` safety checks, `build_help`, `run_generic_build`, `BUILDERS` table, target dispatch all ported and tested via `CapturingRunner` |
| **C2c** `sphinx-build -b` direct mode | delegates to Python `Sphinx` | pending builders |
| **C2.3** parity harness scaffold | `tests/parity.rs` | **done** — `quickstart_parity_flat` and `apidoc_parity_basic` both pass vs Python 9.1.0; snapshots committed |
| **C3** `sphinx-apidoc` | `sphinxdocrs::apidoc` | **done** — 50 tests green (24 apidoc + 2 parity); native binary default; `--full` wired to `quickstart::generate`; parity test passes vs Python 9.1.0 |
| **C4** `sphinx-autogen` | `sphinxdocrs::autogen` | **done** — 32 tests green; RST scan + native stub generation + full parser native; `sphinx-autogen-rs` default fully native |

### New modules landed (C-phase)

| crate path | mirrors | notes |
| --- | --- | --- |
| `src/sphinxdocrs/src/cli/io.rs` | — | `Terminal`, `Fs`, `Clock`, `Runner` traits + `RealTerminal`, `RealFs`, `SystemClock`, `ProcessRunner` impls; `FixedClock`, `CapturingRunner`, `ScriptedTerminal` test helpers |
| `src/sphinxdocrs/src/quickstart/validate.rs` | `sphinx.cmd.quickstart` validators | 7 functions: `is_path`, `is_path_or_empty`, `allow_empty`, `nonempty`, `choice`, `boolean`, `suffix`, `ok` |
| `src/sphinxdocrs/src/quickstart/settings.rs` | `d: dict` in quickstart | `QuickstartSettings`, `EXTENSIONS` table |
| `src/sphinxdocrs/src/quickstart/templates.rs` | `QuickstartRenderer` | wraps `jinja2rs::Environment`; vendored templates in `assets/quickstart/`; registers `repr` filter (Jinja2 built-in missing from minijinja) |
| `src/sphinxdocrs/src/quickstart/generate.rs` | `ask_user`, `generate`, `valid_dir` | |
| `src/sphinxdocrs/src/quickstart/parser.rs` | `get_parser` / `main` | full clap flag grammar; `--ext-*` per-extension flags; `--use-python-impl` escape hatch |
| `src/sphinxdocrs/src/build/parser.rs` | `sphinx.cmd.build.get_parser` | |
| `src/sphinxdocrs/src/build/args.rs` | `_parse_confdir`, `_parse_doctreedir`, `_validate_filenames`, `_parse_confoverrides`, `jobs_argument`, `parse_color` | `BuildArgs`, `ConfValue` |
| `src/sphinxdocrs/src/build/logging.rs` | `_parse_logging` | `LoggingConfig` |
| `src/sphinxdocrs/src/build/make_mode.rs` | `sphinx.cmd.make_mode.Make` | `MakeMode`, `BUILDERS`, `run_make_mode` |
| `src/sphinxdocrs/src/apidoc/settings.rs` | `ApidocOptions` dataclass | 20 fields; `effective_automodule_options()`; `DEFAULT_AUTOMODULE_OPTIONS` |
| `src/sphinxdocrs/src/apidoc/templates.rs` | `ReSTRenderer` in apidoc | `heading`/`heading2`/`repr` filters; 3 vendored templates in `assets/apidoc/` |
| `src/sphinxdocrs/src/apidoc/generate.rs` | `sphinx.ext.apidoc._generate` | `is_initpy`, `module_join`, `is_excluded`, `is_skipped_package/module`, `walk`, `recurse_tree`, `create_module_file`, `create_package_file`, `create_modules_toc_file`, `remove_old_files` |
| `src/sphinxdocrs/src/apidoc/parser.rs` | `sphinx.ext.apidoc._cli.get_parser` | full clap grammar; `--ext-*` flags; `SPHINX_APIDOC_OPTIONS` env |
| `src/sphinxdocrs/src/autogen/scan.rs` | `sphinx.ext.autosummary.generate.find_autosummary_in_lines` | `AutosummaryEntry`, `find_autosummary_in_lines`, `find_autosummary_in_files` |
| `src/sphinxdocrs/src/autogen/templates.rs` | `AutosummaryRenderer` | `underline` + `_` identity filters; 3 vendored stub templates in `assets/autosummary/` |
| `src/sphinxdocrs/src/autogen/parser.rs` | `sphinx.ext.autosummary.generate.get_parser` | `AutogenArgs`; all 6 flags including `--respect-module-all`, `--imported-members` |
| `src/sphinxdocrs/src/autogen/generate.rs` | `generate_autosummary_docs` (stub writing) | `ObjType`, `infer_obj_type`, `split_fqn`, `StubContext`, `generate_stub`, `generate_stubs`; heuristic type detection; `--remove-old` support |
| `src/sphinxdocrs/assets/quickstart/` | `sphinx/templates/quickstart/` | 4 vendored Jinja templates embedded via `include_str!` |
| `src/sphinxdocrs/assets/apidoc/` | `sphinx/templates/apidoc/` | 3 vendored Jinja templates; `package.rst.jinja` patched: `heading(2)` → `heading2` filter |
| `src/sphinxdocrs/assets/autosummary/` | `sphinx/ext/autosummary/templates/autosummary/` | 3 vendored RST stub templates embedded via `include_str!` |
| `src/sphinxdocrs/tests/parity.rs` | — | Cross-language parity harness; `#[cfg(feature="parity")]`; skips without Python |
