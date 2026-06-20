# sphinxdocrs CLI port & test plan

Drop-in replacements for the Sphinx command-line entry points, ported to
native Rust in [`src/sphinxdocrs`](../src/sphinxdocrs). Target binaries:

| upstream console script | Rust binary | upstream module | priority | status |
| --- | --- | --- | --- | --- |
| `sphinx-quickstart` | `sphinx-quickstart-rs` | `sphinx.cmd.quickstart` | **C1** | **done** |
| `sphinx-build`      | `sphinx-build-rs`      | `sphinx.cmd.build` + `sphinx.cmd.make_mode` | **C2** | **partial** (make-mode native; direct-mode delegates to Python) |
| `sphinx-apidoc`     | `sphinx-apidoc-rs`     | `sphinx.ext.apidoc` | **C3** | **done** (module generation + `--full` native; parity verified vs Python 9.1.0) |
| `sphinx-autogen`    | `sphinx-autogen-rs`    | `sphinx.ext.autosummary.generate` | **C4** | **done** (RST scan + arg-parse + native stub generation; Python fallback on `--use-python-impl`) |

~~Current state: all four binaries in
[src/sphinxdocrs/src/bin](../src/sphinxdocrs/src/bin) are **PyO3
shell-out shims** that exec `python -c "from <mod> import main; ..."`.~~

Current state (post C1/C2a/C2b/C3):
- `sphinx-quickstart-rs` — **fully native**; Python fallback only on
  `--use-python-impl` / `SPHINXDOCRS_PY_FALLBACK=1`.
- `sphinx-build-rs` — **make-mode (`-M`) native**; direct mode (`-b`)
  validates args natively then delegates to Python `Sphinx`.
- `sphinx-apidoc-rs` — **fully native**: module/package/TOC generation and
  `--full` quickstart; Python fallback only on `--use-python-impl` / `SPHINXDOCRS_PY_FALLBACK=1`.
- `sphinx-autogen-rs` — **fully native**: RST scan, arg-parse, and stub file
  generation; Python fallback only on `--use-python-impl` / `SPHINXDOCRS_PY_FALLBACK=1`.

New modules in `src/sphinxdocrs/src/`:

```
cli/
  mod.rs          — re-exports
  io.rs           — Terminal, Fs, Clock, Runner traits + Real* impls +
                    FixedClock, CapturingRunner, ScriptedTerminal test helpers
quickstart/
  mod.rs
  validate.rs     — 7 validator fns (boolean, suffix, nonempty, …)
  settings.rs     — QuickstartSettings, EXTENSIONS table
  templates.rs    — QuickstartTemplates (jinja2rs wrapper + repr filter)
  generate.rs     — valid_dir, ask_user, generate
  parser.rs       — clap parser, parse_args, is_fully_specified
build/
  mod.rs
  parser.rs       — clap parser mirroring sphinx-build get_parser()
  args.rs         — BuildArgs, ConfValue, jobs_argument, _parse_* helpers
  logging.rs      — LoggingConfig, parse_logging
  make_mode.rs    — MakeMode, BUILDERS, run_make_mode
apidoc/
  mod.rs
  settings.rs     — ApidocOptions (20 fields), DEFAULT_AUTOMODULE_OPTIONS
  templates.rs    — ApidocTemplates (heading/heading2/repr filters; 3 vendored templates)
  generate.rs     — is_initpy, module_join, is_excluded, walk, recurse_tree,
                    create_module_file, create_package_file,
                    create_modules_toc_file, remove_old_files
  parser.rs       — clap parser, parse_args (full flag grammar + --ext-*)
autogen/
  mod.rs
  scan.rs         — AutosummaryEntry, find_autosummary_in_lines,
                    find_autosummary_in_files
  templates.rs    — AutogenTemplates (underline + _ identity filters; 3 vendored stub templates)
  parser.rs       — AutogenArgs, build_parser, parse_args
  generate.rs     — ObjType, infer_obj_type, split_fqn, StubContext,
                    generate_stub, generate_stubs
assets/quickstart/
  conf.py.jinja   — vendored from sphinx/templates/quickstart/
  root_doc.rst.jinja
  Makefile.new.jinja
  make.bat.new.jinja
assets/apidoc/
  module.rst.jinja  — vendored from sphinx/templates/apidoc/
  package.rst.jinja — (patched: heading(2) → heading2 filter)
  toc.rst.jinja
assets/autosummary/
  base.rst        — vendored from sphinx/ext/autosummary/templates/autosummary/
  class.rst
  module.rst
```

Test suites (all green, 563 total):

| suite | tests | covers |
| --- | --- | --- |
| lib (unit) | 237 | existing + inline tests in `cli/io.rs`, `quickstart/validate.rs`, `build/args.rs`, `build/make_mode.rs`, `build/logging.rs`, `apidoc/generate.rs`, `apidoc/parser.rs`, `autogen/scan.rs`, `autogen/parser.rs`, `autogen/generate.rs`, `registry.rs`, `versioning.rs`, `util_rst.rs`, `util_osutil.rs`, `util_uri.rs`, `util_lines.rs`, `util_docstrings.rs` |
| `tests/util_extra.rs` | 35 | `is_url` (7 `#[case]`), `encode_uri` (4), `parse_line_num_spec` (8 + 4 `#[case]` errors), `prepare_docstring` (3), `prepare_commentdoc` (4), `separate_metadata` (6) |
| `tests/versioning.rs` | 29 | `levenshtein_distance` (8 `#[case]`), `get_ratio` (5), `add_uids` (3), `merge_doctrees` mirrors (modified/added/deleted/deleted_end/insert/insert_beginning/insert_similar + edge cases) |
| `tests/registry.rs` | 32 | `add_source_suffix` (3, incl. `#[case]` suffix variants), `add_source_parser`/`get_source_parser`/`get_source_parsers` (6), `add_transform`/`get_transforms` (5), `add_post_transform`/`get_post_transforms` (2), CSS assets (3), JS assets (2), static dirs (2), LaTeX packages (7), HTML themes (2), empty registry (1) |
| `tests/autogen.rs` | 32 | parser flags (7 incl. 3 `#[case]`), `find_autosummary_in_lines` (7 cases), `find_autosummary_in_files` (1), template/help snapshots (2), `infer_obj_type` (5 `#[case]`), `split_fqn` (2), `generate_stub` (5), `generate_stubs` (2), stub content snapshots (2) |
| `tests/apidoc.rs` | 24 | parser flags (8), `is_initpy` (4 `#[case]`), `module_join` (4 `#[case]`), `is_excluded` (1), `recurse_tree` basic/no-private/with-private (3), module/TOC/help snapshots (3) |
| `tests/quickstart.rs` | 50 | validators (11 `#[case]` tables), parser (8), `valid_dir` (4), tree-layout snapshots (4), `conf_py_snapshot`, newline modes (2), `ask_user` scripted-terminal, help-text snapshot |
| `tests/build.rs` | 35 | `jobs_argument` (6), `parse_confdir` (4), `parse_doctreedir` (2), `validate_filenames` (2), `parse_confoverrides` (5), `parse_color` (3), `build_clean` safety (4), `run_generic_build` (2), dispatch (2), `run_make_mode` (1), BUILDERS completeness (1), help snapshot |
| `tests/config.rs` | 9 | existing |
| `tests/assets.rs` | 6 | existing |
| `tests/snapshot.rs` | 1 | existing |
| `tests/parity.rs` | 2 (parity-gated) | cross-language harness; enabled with `--features parity`; `quickstart_parity_flat` and `apidoc_parity_basic` pass (Python 9.1.0) |

---

## 1. Porting principles

- **Drop-in CLI contract first.** Argument grammar, exit codes, stdout/
  stderr text, and on-disk artifacts must match upstream byte-for-byte
  where observable. The argparse surface is the spec; mirror flag names,
  defaults, `dest`, and help strings.
- **Three-layer architecture per command** so logic is unit-testable
  without a process boundary:
  1. **parse layer** — pure `argv: &[String] -> Result<Args, CliError>`.
     No I/O. Mirrors each `get_parser()` / `_parse_*` helper.
  2. **core layer** — pure-ish functions taking an injected filesystem +
     clock + terminal trait, returning planned actions or rendered
     strings. Mirrors `generate()`, `ask_user()`, `build_main()` body.
  3. **shell layer** — `main()` wiring real stdio, real FS, real exit.
- **Dependency injection at boundaries only** (FS, time, terminal input,
  subprocess). Everything else stays concrete. This is what makes
  `mockall` + `rstest` fixtures cheap (see test plan).
- **Reuse already-ported subsystems**: `config` (conf.py reader),
  `util_console` (colour/escape parity), `util_matching`, `project`,
  `errors`, `events`, `extension`. Do not re-implement these.
- **Templating**: quickstart renders Jinja templates. Use the vendored
  `minijinja`/`jinja2rs` already in the workspace
  (`src/minijinja`, `src/jinja2rs`) rather than shelling to Python. The
  four template files
  ([sphinx/templates/quickstart](../src/sphinx/sphinx/templates/quickstart))
  are vendored as crate assets.
- **Fallback ladder**: every command keeps a `--use-python-impl` escape
  hatch (and an env `SPHINXDOCRS_PY_FALLBACK=1`) that runs the existing
  shell-out shim. Until a command reaches parity it defaults to the
  Python path; a feature flag flips the default once snapshots are green.

---

## 2. Command: `sphinx-quickstart` (C1)

Smallest blast radius, no builder/environment dependency, pure file
generation — the correct first target.

### 2.1 Surface to port (`sphinx.cmd.quickstart`)

| upstream symbol | Rust target | notes |
| --- | --- | --- |
| validators (`is_path`, `nonempty`, `choice`, `boolean`, `suffix`, `ok`, `allow_empty`) | `quickstart::validate` | pure `fn(&str) -> Result<Value, ValidationError>`; table-test each |
| `do_prompt` / `term_input` | `quickstart::prompt` over a `Terminal` trait | `mockall`-mocked in tests; readline behavior is out of scope |
| `ask_user(d)` | `quickstart::ask_user(&mut Settings, &dyn Terminal)` | drives prompts; the conflict rule (imgmath + mathjax) and existing-conf.py / existing-master guards must match |
| `QuickstartRenderer` | `quickstart::render` over minijinja | `_has_custom_template` + `templatedir` override semantics |
| `generate(d, ...)` | `quickstart::generate(&Settings, &dyn Fs, &dyn Clock)` | dir layout (`sep`/`dot`), `exclude_patterns`, `copyright`, `now`, `project_underline` (column-width!), newline modes (`\n` Makefile, `\r\n` make.bat) |
| `valid_dir(d)` | `quickstart::valid_dir` | reserved-name collision check |
| `get_parser()` / `main()` | `quickstart::{parser, run}` | flag parity incl. `--ext-*` append_const, `--no-sep`/`--no-makefile` |

### 2.2 Parity-critical details

- `project_underline = column_width(project) * '='` — uses `unicode-width`
  crate (`UnicodeWidthStr::width`) matching docutils' east-Asian width.
- `copyright = "<year>, <author>"` and `now = time.asctime()` — injected
  via `Clock` trait; `FixedClock::snapshot()` for deterministic snapshots.
- `extensions` ordering follows the fixed `EXTENSIONS` dict order.
- Makefile written with binary LF via `to_lf()`; make.bat with CRLF via
  `to_crlf()`. Snapshot tests assert both.
- "Creating file %s." / "File %s already exists, skipping." stdout lines
  honor the `quiet` key.
- `| repr` Jinja2 filter registered manually — not present in minijinja
  builtins or `minijinja-contrib` pycompat (which only covers method
  syntax like `str.upper()`). Registered via `env.add_filter("repr", …)`
  in `QuickstartTemplates::vendored()`.

### 2.3 Definition of done (C1) — ✅ COMPLETE

- `sphinx-quickstart-rs` produces a tree matching upstream layout for
  the matrix `{sep×nosep} × {makefile×none} × {default exts, all exts}`
  — verified by 4 insta tree-layout snapshots.
- Non-interactive (`-q -p -a -v ...`) and interactive (`ScriptedTerminal`)
  paths both covered.
- 50 passing tests in `tests/quickstart.rs`.

---

## 3. Command: `sphinx-build` (C2)

Two entry modes share the binary:

- **make mode** `sphinx-build -M <target> <src> <out>` →
  `sphinx.cmd.make_mode` (porting first; it is mostly arg routing +
  subprocess + `clean`).
- **direct mode** `sphinx-build -b <builder> <src> <out>` →
  `sphinx.cmd.build.build_main`, which constructs `Sphinx(...)` and runs
  a builder. **The builder/environment pipeline is P3 and not yet
  ported**, so direct mode delegates to the Python `Sphinx` app via the
  existing shim until builders land.

### 3.1 Port now (native)

| upstream symbol | Rust target | notes |
| --- | --- | --- |
| `get_parser()` | `build::parser` | full flag grammar parity (builder, jobs, `-a/-E`, path opts, `-D/-A/-t/-n`, console/warning opts) |
| `jobs_argument` | `build::jobs_argument` | `'auto'` → cpu count; positive-int validation + error text |
| `_parse_confdir` / `_parse_doctreedir` / `_validate_filenames` / `_validate_colour_support` / `_parse_confoverrides` | `build::args::*` | pure; high-value unit/param tests |
| `make_mode.Make` (`build_clean`, `build_help`, `run_generic_build`, target dispatch, `BUILDERS` table) | `build::make_mode` | `build_clean` safety checks (same-dir, src-under-build) are security-relevant — port faithfully; subprocess calls go through an injected `Runner` trait |
| `handle_exception` / `_parse_logging` (status/warning/TeeStripANSI/warnfile) | `build::logging` | colour disable via `util_console` |

### 3.2 Delegate (for now)

- `build_main` → constructing and running `Sphinx`. Keep shell-out shim;
  gate native takeover behind `feature = "native-build"` once a builder
  exists. make-mode's `run_generic_build` shells to `sphinx-build` direct
  mode anyway, so make mode can be native while direct mode is Python.

### 3.3 Definition of done (C2) — ⚠️ PARTIAL

- ✅ `-M help`, `-M clean`, unknown-target, and arg-validation errors
  match upstream stdout/stderr + exit codes natively (35 passing tests).
- ✅ `build_clean` safety checks (same-dir, src-under-build) ported and
  tested with real `TempDir`.
- ✅ `jobs_argument`, all `_parse_*` helpers, `BUILDERS` table complete.
- ⏳ Direct-mode invocations transparently forwarded (Python delegation
  path is in place; parity harness §5.5 pending).

---

## 4. Commands: `sphinx-apidoc` (C3) & `sphinx-autogen` (C4)

### 4.1 `sphinx-apidoc` (C3) — ✅ DONE

File-generator with no builder/environment dependency. Implementation
status:

| upstream symbol | Rust target | status |
| --- | --- | --- |
| `ApidocOptions` dataclass | `apidoc::settings::ApidocOptions` | ✅ all 20 fields |
| `_generate.py` helpers | `apidoc::generate` | ✅ `is_initpy`, `module_join`, `is_excluded`, `is_skipped_package/module`, `walk`, `recurse_tree`, `create_module_file`, `create_package_file`, `create_modules_toc_file`, `remove_old_files` |
| RST templates | `assets/apidoc/*.jinja` | ✅ all 3 vendored; `heading(2)` call patched to `heading2` filter |
| `_cli.get_parser()` | `apidoc::parser` | ✅ full clap grammar, all flags incl. `--ext-*`, `SPHINX_APIDOC_OPTIONS` env |
| `--full` mode | `run_full_quickstart` in binary | ✅ wired to `quickstart::generate` natively |
| exclude regex compilation | `fnmatch_to_regex` in binary | ✅ simplified fnmatch→regex |

Definition of done (C3) — ✅ DONE:
- ✅ `recurse_tree` generates correct package/module `.rst` layout.
- ✅ 24 passing tests in `tests/apidoc.rs`.
- ✅ `--full` wired to `quickstart::generate` natively (`run_full_quickstart`).
- ✅ Parity vs Python 9.1.0 (`apidoc_parity_basic` passes; snapshot committed).

### 4.2 `sphinx-autogen` (C4) — ✅ DONE

Scans sources for `autosummary` directives and emits stub `.rst` files.
Architecture split:

| layer | status | notes |
| --- | --- | --- |
| RST scan (`find_autosummary_in_lines`) | ✅ native | regex-based parser matching upstream exactly; 7 parametrized test cases |
| Argument parser | ✅ native | full clap grammar; `--imported-members`, `--respect-module-all`, `--remove-old`, `--templates` |
| Stub templates | ✅ vendored | `base.rst`, `class.rst`, `module.rst` in `assets/autosummary/`; `underline` + `_` identity filters registered |
| Stub generation | ✅ native | `autogen::generate` — heuristic type detection (CamelCase→class, else→module), `StubContext`, `generate_stub`, `generate_stubs`; member lists empty (autodoc fills at build time) |

Definition of done (C4) — ✅ DONE:
- ✅ `find_autosummary_in_lines` / `find_autosummary_in_files` ported and tested.
- ✅ 32 passing tests in `tests/autogen.rs`.
- ✅ Native stub generation: `generate_stub`/`generate_stubs` write `.rst` stubs with correct headings, `.. automodule::` / `.. autoclass::` directives, and `--remove-old` support.

---

## 5. Testing plan

Tooling: `rstest` (fixtures + parametrization), `mockall` (trait
mocks), `insta` / `cargo-insta` (snapshots), `wiremock`/`rvcr` (only if a
command ever touches the network — currently none do). Dev-deps already
present in [Cargo.toml](../src/sphinxdocrs/Cargo.toml).

### 5.1 Injection traits (enable mocking) — ✅ IMPLEMENTED

All four traits and their production impls are live in
[`src/sphinxdocrs/src/cli/io.rs`](../src/sphinxdocrs/src/cli/io.rs).
The `#[cfg_attr(test, mockall::automock)]` attributes generate `Mock*`
types for inline `#[cfg(test)]` use. External test crates use the
concrete test helpers instead:

```rust
// src/sphinxdocrs/src/cli/io.rs
#[cfg_attr(test, mockall::automock)]
pub trait Terminal { … }

#[cfg_attr(test, mockall::automock)]
pub trait Fs { … }

#[cfg_attr(test, mockall::automock)]
pub trait Clock { … }

#[cfg_attr(test, mockall::automock)]
pub trait Runner { … }
```

External test helpers (public, no mockall needed in integration tests):

```rust
pub struct FixedClock { pub asctime_str: String, pub year_val: i32 }
pub struct CapturingRunner { … }  // records (program, args) pairs
pub struct ScriptedTerminal { … } // feeds pre-set answers, captures print()
```

### 5.2 `rstest` fixtures — ✅ IMPLEMENTED

Used in `tests/quickstart.rs`. Note: external test crates cannot see
`Mock*` types generated by `#[cfg_attr(test, mockall::automock)]`, so
fixtures use the concrete `FixedClock` helper instead of `MockClock`:

```rust
#[fixture]
fn fixed_clock() -> FixedClock { FixedClock::snapshot() }

#[fixture]
#[once]
fn templates() -> QuickstartTemplates { QuickstartTemplates::vendored() }
```

### 5.3 Parametrization (`#[case]`)

Validators — one table per function:

```rust
#[rstest]
#[case("y", Some(true))] #[case("YES", Some(true))]
#[case("n", Some(false))] #[case("maybe", None)]
fn boolean_parity(#[case] input: &str, #[case] want: Option<bool>) {
    assert_eq!(quickstart::validate::boolean(input).ok(), want);
}

#[rstest]
#[case(".rst", true)] #[case(".txt", true)]
#[case("rst", false)] #[case(".", false)] #[case("", false)]
fn suffix_parity(#[case] input: &str, #[case] ok: bool) {
    assert_eq!(quickstart::validate::suffix(input).is_ok(), ok);
}
```

build arg parsing:

```rust
#[rstest]
#[case("auto", Ok(num_cpus))]
#[case("4", Ok(4))]
#[case("0", Err("job number should be a positive number"))]
#[case("-1", Err("job number should be a positive number"))]
fn jobs_argument(#[case] v: &str, #[case] want: Result<usize, &str>) { /* ... */ }

#[rstest]
#[case(true,  None,        "src", None)]          // noconfig
#[case(false, Some("cfg"), "src", Some("cfg"))]   // explicit confdir
#[case(false, None,        "src", Some("src"))]   // default → sourcedir
fn parse_confdir(/* ... */) { /* ... */ }
```

### 5.4 `cargo-insta` snapshots — ✅ IMPLEMENTED

Two snapshot families in `tests/snapshots/`:

1. **Rendered strings** — `conf_py_snapshot` (full `conf.py` content),
   `quickstart_help_snapshot`, `build_help_snapshot`.
2. **Generated trees** — `quickstart_tree_snapshot` with per-case suffix
   via `insta::with_settings!({snapshot_suffix => …})` so each
   `#[case]` variant gets its own snapshot file:
   - `quickstart_tree_snapshot@flat_make.snap`
   - `quickstart_tree_snapshot@sep.snap`
   - `quickstart_tree_snapshot@no-makefile_no-batchfile.snap`
   - `quickstart_tree_snapshot@ext-autodoc_ext-mathjax.snap`

Tree manifest stores the sorted list of relative paths (not SHA256 hashes
because template timestamps would cause churn). The newline tests assert
byte-level LF/CRLF correctness separately.

```rust
#[rstest]
#[case::flat_make(&["-q","-p","P","-a","A"])]
#[case::sep(&["-q","--sep","-p","P","-a","A"])]
#[case::no_make(&["-q","--no-makefile","--no-batchfile","-p","P","-a","A"])]
fn quickstart_tree_snapshot(clock: MockClock, templates: &QuickstartTemplates,
                            #[case] argv: &[&str]) {
    let tmp = TempDir::new().unwrap();
    let s = Settings::from_args(argv).with_path(tmp.path());
    quickstart::generate(&s, &RealFs, &clock, templates).unwrap();
    insta::assert_yaml_snapshot!(tree_manifest(tmp.path()));
}
```

Interactive `ask_user` via `MockTerminal` with `Sequence` to assert
prompt order and feed answers:

```rust
let mut term = MockTerminal::new();
let mut seq = mockall::Sequence::new();
for ans in ["", "n", "_", "MyProj", "Me", "1.0", "1.0", "en", ".rst", "index"] {
    term.expect_prompt().times(1).in_sequence(&mut seq)
        .return_once(move |_| Ok(ans.into()));
}
term.expect_print().returning(|_| ());
quickstart::ask_user(&mut settings, &term);
```

### 5.5 Cross-language parity harness — ✅ DONE

Integration tests in `tests/parity.rs` gated behind
`cfg(feature = "parity")`:
- `quickstart_parity_flat`: runs Python `sphinx-quickstart` and
  `sphinx-quickstart-rs` on identical inputs, diffs file trees.
- `apidoc_parity_basic`: runs Python `sphinx-apidoc` and
  `sphinx-apidoc-rs` on a synthetic Python package, diffs file trees.

Both **pass** against Python 9.1.0. Reference snapshots committed in
`tests/snapshots/parity__*.snap`.

Enable with:
```
cargo test -p sphinxdocrs --features parity --test parity
```

### 5.6 Coverage / triage tagging

Mirror the inventory legend: tag each ported function **exact parity**,
**accepted deviation**, or **pending**, in
[docs/sphinx-port-inventory.md](sphinx-port-inventory.md). Branch-coverage
target ≥ the existing crate bar; `build_clean` safety branches and
validator error paths are mandatory-covered.

---

## 6. Sequencing

1. ✅ **C1.0** Land `cli::io` traits + real impls + minijinja template assets.
2. ✅ **C1.1** Port validators + `do_prompt` + `parser`; full `#[case]` tables.
3. ✅ **C1.2** Port `generate` + `ask_user` + `valid_dir`; tree snapshots;
   `sphinx-quickstart-rs` default flipped to native (all 50 tests green).
4. ✅ **C2.1** Port build `parser` + `_parse_*` + `jobs_argument`; param tests.
5. ✅ **C2.2** Port `make_mode` (clean/help/dispatch via `Runner`); native
   make-mode, Python-delegated direct mode (all 35 tests green).
6. ✅ **C2.3** Parity harness scaffold (`feature = "parity"`): `tests/parity.rs`
   written for quickstart + apidoc; skips gracefully without Python.
7. ✅ **C3.1** Port apidoc `settings`, `templates` (heading/heading2/repr filters),
   `generate` (walk, recurse_tree, create_*_file, remove_old_files),
   `parser` (full clap grammar); 24 passing integration tests.
8. ✅ **C3.2** Wire `--full` to `quickstart::generate` natively
   (`run_full_quickstart` in `sphinx_apidoc.rs`).
9. ✅ **C4.1** Port autogen `scan` (`find_autosummary_in_lines` + `find_autosummary_in_files`),
   `templates` (`underline` filter, 3 vendored stubs), `parser` (full clap grammar);
   17 passing integration tests; `sphinx-autogen-rs` native scan + Python stub generation.
10. ✅ **C3.3 / C4.3** Parity harness first run: `quickstart_parity_flat` and
    `apidoc_parity_basic` both pass against Python 9.1.0; snapshots committed in
    `tests/snapshots/parity__*.snap`.
11. ✅ **C4.2** Native stub generation: `autogen::generate` — `infer_obj_type`
    (CamelCase→class, else→module), `StubContext`, `generate_stub`/`generate_stubs`;
    15 new integration tests; `sphinx-autogen-rs` now fully native by default.
    32 passing tests in `tests/autogen.rs`; 266 total (0 failures, 0 clippy warnings).
12. ✅ **P2.1 Registry** Port `SphinxComponentRegistry` P2 subset: source-suffix/parser
    registration, transforms, CSS/JS/static asset tracking, LaTeX packages, HTML
    theme registry. `RegistryError` for duplicate/not-found conditions. 27 inline
    unit tests + 32 integration tests in `tests/registry.rs`; 325 total (0 failures, 0 clippy warnings).
13. ✅ **P2.2 Versioning** Port `sphinx.versioning` pure algorithms: `VERSIONING_RATIO`,
    `levenshtein_distance`, `get_ratio`, `VersionableNode` trait, `add_uids`,
    `merge_doctrees`; mirrors all 7 Python test fixture scenarios (modified, added,
    deleted, deleted_end, insert, insert_beginning, insert_similar). Added `uuid`
    dep for UID generation. 26 inline unit tests + 29 integration tests in
    `tests/versioning.rs`; `UIDTransform` deferred to P3; 380 total tests.
14. ✅ **P2.3 util_rst + util_osutil** Port `sphinx.util.rst` (`escape`, `textwidth`,
    `heading`; east-Asian aware width) and `sphinx.util.osutil` (`SEP`, `os_path`,
    `canon_path`, `path_stabilize`, `relative_uri`, `ensuredir`, `make_filename`,
    `make_filename_from_project`, `FileAvoidWrite`). Added `unicode-normalization`
    dep. 42 inline unit tests + 52 integration tests in `tests/util_rst_osutil.rs`;
    mirrors `test_util_rst.py` and `test_util.py` (osutil subset); 480 tests.
15. ✅ **P2.4 util_uri + util_lines + util_docstrings** Port `sphinx.util._uri`
    (`is_url`, `encode_uri` with decode-then-reencode query + IDNA netloc),
    `sphinx.util._lines` (`parse_line_num_spec` with half-open ranges and error
    messages), and `sphinx.util.docstrings` (`prepare_docstring`, `prepare_commentdoc`,
    `separate_metadata`). 57 inline unit tests + 35 integration tests in
    `tests/util_extra.rs`; mirrors `test_util_uri.py`, `test_util_lines.py`,
    `test_util_docstrings.py`; 563 total tests.

Each step is independently shippable: the binary keeps working via the
shim fallback until its native path passes the parity harness.
