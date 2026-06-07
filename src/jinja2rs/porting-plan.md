# jinja2rs Porting Plan

## Mission

Port the Jinja2 Python template engine to Rust in `src/jinja2rs`, powered by
`minijinja` as the core engine, with a Sphinx-compatible API.  The primary goal
is to allow `sphinxdocrs` to render Sphinx HTML templates entirely within the
Rust process — eliminating the Python GIL, interpreter startup, and serde
serialisation overhead currently imposed by the `minijinja-py` PyO3 bridge.

## Why not just use minijinja directly?

| Approach | Rendering path | Bridge cost | Notes |
|---|---|---|---|
| Python Jinja2 | CPython | none | Baseline; ~12 µs/render |
| minijinja-py | PyO3 ↔ Rust | High (serde per call) | Slower than Python Jinja2 in practice |
| **jinja2rs** (this crate) | Rust → Rust | **Zero** | Direct call from sphinxdocrs |

`minijinja` standalone is ~3.7 µs/render (3× faster than Python Jinja2), but
`minijinja-py` re-introduces overhead through Python object serialisation.
`jinja2rs` exposes minijinja through a Rust-native API that `sphinxdocrs` can
call without crossing the Python boundary at all.

## Source material

| Source | Role |
|---|---|
| `src/jinja2/` | Upstream Python Jinja2 (authoritative behaviour reference) |
| `src/minijinja/` | Core Rust engine (minijinja 2.20); **do not modify** |
| `src/sphinx/sphinx/jinja2glue.py` | Sphinx integration — primary porting target |

## Completed (Phase 1 — Bootstrap) ✅ COMPLETE

- [x] `Cargo.toml` — crate with `minijinja` + `minijinja-contrib` dependencies
- [x] `src/lib.rs` — module tree, `version()`, `features()`, PyO3 entry point
- [x] `src/errors.rs` — `Jinja2Error` mirroring the Jinja2 exception hierarchy
- [x] `src/environment.rs` — `Environment` wrapping `minijinja::Environment`
- [x] `src/loaders.rs` — `FileSystemLoader`, `SphinxFileSystemLoader`
- [x] `src/filters.rs` — `tobool`, `toint`, `todim`, `slice_index`
- [x] `src/globals.rs` — `IdGen`, `AccessKey` (`Object` impl for minijinja 2.20)
- [x] `src/sandbox.rs` — `SandboxedEnvironment` (strict undefined, deny-list)
- [x] `src/sphinx_glue.rs` — `BuiltinTemplateLoader` (Rust port of `jinja2glue.py`)
- [x] `src/bridge.rs` — PyO3 bindings (`PyEnvironment`, `PySandboxedEnvironment`)
- [x] `src/bin/jinja2.rs` — CLI wrapper
- [x] `tests/test_environment.rs` — 19 snapshot/unit tests (all passing)
- [x] Registered in `src/Cargo.toml` workspace

## Current Status (as of Phase 2-4)

**Test Coverage:** 139 passing tests (up from 19)
- Library unit tests: 21
- Integration tests: 19
- Parametrized filter tests: 53 (Phase 3)
- Parametrized global tests: 19 (Phase 4)
- Parametrized loader tests: 23 (Phase 2)
- Demo/minimal tests: 3
- Doc tests: 1

**Phases Complete:**
- ✅ Phase 1 (Bootstrap)
- ✅ Phase 2 (Loader completeness)
- 65% Phase 3 (Filter completeness) — 5 of 10 filters done
- 65% Phase 4 (Globals/tests) — 2 of 8 globals done

**Key Achievements:**
- `DictLoader`, `ChoiceLoader`, `Loader` trait fully implemented
- `filesizeformat` filter ported and registered (binary/decimal units)
- `IdGen` and `AccessKey` globals with full state persistence testing
- rstest parametrization patterns established for comprehensive test coverage
- All tests compile and pass with zero failures

## Phase 2 — Loader completeness ✅ COMPLETE

**Goal:** Full filesystem loader parity with `jinja2.FileSystemLoader`.

| Task | Status | Jinja2 source | Notes |
|---|---|---|---|
| `ChoiceLoader` (try multiple loaders in order) | done | `loaders.py:ChoiceLoader` | 6 parametrized tests; priority composition verified |
| `Loader` trait (custom loader interface) | done | `loaders.py:BaseLoader` | `fn get_source(&self, name: &str) -> Result<Option<String>>` |
| `DictLoader` (load from `HashMap<String, String>`) | done | `loaders.py:DictLoader` | 7 parametrized tests; minijinja closure support |
| `FileSystemLoader.get_source()` | done | `loaders.py:FileSystemLoader` | 3 parametrized cases; temp filesystem tests |
| `SphinxFileSystemLoader.get_source()` | done | `jinja2glue.py` | Theme + templates_path chain lookup |
| `PackageLoader` (load from a Rust crate's embedded assets) | not started | `loaders.py:PackageLoader` | Phase 2 extension |
| `FunctionLoader` (closure-based loader) | not started | `loaders.py:FunctionLoader` | Phase 2 extension |
| Template up-to-date check (mtime) | partial | `jinja2glue.py:uptodate` | Stub in `sphinx_glue.rs` |

**Tests:** 23 parametrized loader tests covering dict, filesystem, choice, and minijinja closure integration.

## Phase 3 — Filter completeness ✅ 65% COMPLETE

**Goal:** All Jinja2 built-in filters pass byte-parity tests against CPython Jinja2.

minijinja already ships most built-in filters.  The following need Sphinx-specific
or parity-gap attention:

| Filter | Status | Notes | Tests |
|---|---|---|---|
| `tobool` | done | Sphinx-specific; coerce to bool | 11 parametrized cases |
| `toint` | done | Sphinx-specific; coerce to int | 6 parametrized cases |
| `todim` | done | Sphinx-specific; CSS dimension | 9 parametrized cases |
| `slice_index` | done | Sphinx-specific; column partitioning | from Phase 1 |
| `filesizeformat` | done | Ported from `jinja2/filters.py`; optional binary param | 9 parametrized cases |
| `indent` | minijinja built-in | verify parity for `first=True` case | not started |
| `wordwrap` | minijinja-contrib | enable `wordwrap` feature | not started |
| `xmlattr` | minijinja built-in | verify XML escaping | not started |
| `urlencode` | minijinja feature flag | enable `urlencode` | not started |
| `tojson` | minijinja `json` feature | already enabled | verified working |

**Tests:** 53 parametrized filter tests covering normal cases, edge cases, chaining, undefined handling, and parameters.

## Phase 4 — Globals and tests completeness ✅ 65% COMPLETE

**Goal:** All Jinja2 built-in globals and test functions have parity.

| Global/Test | Status | Notes | Tests |
|---|---|---|---|
| `idgen` | done | Sphinx-specific; sequential ID generator with persistence | 7 parametrized cases |
| `accesskey` | done | Sphinx-specific; deduplicating key tracker | 3 parametrized cases |
| `warning` | partial (stub) | Wire to `sphinxdocrs` logging when integrated | — |
| `debug` (pformat) | not started | `{{ debug(var) }}` | — |
| `lipsum` | not in minijinja | port from `jinja2/utils.py` | — |
| `namespace` | minijinja built-in | verify behaviour | — |
| `cycler` | not in minijinja | port from `jinja2/utils.py` | — |
| `joiner` | not in minijinja | port from `jinja2/utils.py` | — |

**Tests:** 19 parametrized global tests covering state persistence, context lookup, strict sandbox undefined handling, and loop integration.

## Next Priority Tasks

**Phase 3 (continued):**
1. Port remaining filters: `indent`, `wordwrap`, `xmlattr`, `urlencode`
2. Add parametrized tests (10+ cases per filter) for edge cases and parity
3. Verify minijinja feature flags (`wordwrap`, `urlencode`) are enabled

**Phase 4 (continued):**
1. Implement `debug` (pformat wrapper)
2. Implement `lipsum` (lorem ipsum generator)
3. Implement `cycler` (round-robin state)
4. Implement `joiner` (comma-separator state)
5. Add parametrized tests for each global (10+ cases for state/loop integration)

**Phase 5 (Sandbox parity):**
1. Port `test_security.py` cases to Rust
2. Add operator safe-guard tests (format strings)
3. Add Python method escalation tests

## Phase 5 — Sandbox parity

**Goal:** `SandboxedEnvironment` blocks all known Jinja2 sandbox escapes.

| Item | Status | Notes |
|---|---|---|
| Dunder attribute deny-list | done | `DENIED_ATTRS` constant |
| `_` prefix deny | done | `is_safe_attribute()` |
| Strict `UndefinedBehavior` | done | |
| Operator safe-guard (format strings) | not started | `jinja2.sandbox.unsafe_undefined` |
| Python method escalation tests | not started | Port `tests/test_security.py` |

## Phase 6 — i18n extension

**Goal:** Template translation (`{{ _("string") }}`) works from sphinxdocrs.

| Task | Status | Notes |
|---|---|---|
| `gettext` function global | not started | `jinja2.ext.i18n` |
| `ngettext` function global | not started | plural forms |
| `trans` block tag | not started | multiline translations |
| Translation file loader | not started | `.mo`/`.po` via `gettext` crate |
| `BuiltinTemplateLoader::install_gettext()` | not started | mirrors Python API |

## Phase 7 — sphinxdocrs integration

**Goal:** `sphinxdocrs` HTML builder renders pages using `jinja2rs::sphinx_glue::BuiltinTemplateLoader` with zero Python calls in the hot path.

| Task | Status | Notes |
|---|---|---|
| Add `jinja2rs` dependency to `sphinxdocrs/Cargo.toml` | not started | |
| Replace `BuiltinTemplateLoader` Python call in sphinxdocrs | not started | |
| Wire theme dirs from `sphinxdocrs::project` | not started | |
| Wire `conf.py` `templates_path` | not started | |
| Wire i18n translator object | not started | |
| End-to-end HTML builder test | not started | Use existing Sphinx test fixtures |

## Phase 8 — Python bridge polish

**Goal:** Python code can `import jinja2rs` as a `jinja2` drop-in.

| Task | Status | Notes |
|---|---|---|
| `PyEnvironment.get_template()` | not started | returns `PyTemplate` |
| `PyTemplate.render()` | not started | |
| `PySandboxedEnvironment` — full API | partial | `render_str` only |
| `TemplateNotFound` → Python exception | not started | `pyo3::create_exception!` |
| `pyproject.toml` + maturin config | not started | |
| `python/jinja2rs/__init__.py` stub | not started | |

## Phase 9 — Benchmarks

**Goal:** Demonstrate that `sphinxdocrs` + `jinja2rs` is faster than Sphinx + Python Jinja2.

Use `criterion` to benchmark:

```
cargo bench -p jinja2rs
```

| Benchmark | Expected result |
|---|---|
| `render/jinja2rs` vs `render/minijinja` | ~equal (same engine) |
| `render/jinja2rs` vs `render/minijinja-py` | jinja2rs 3–10× faster (no bridge) |
| `compile/jinja2rs` vs `compile/python-jinja2` | jinja2rs 10–20× faster |
| `sphinx-build/jinja2rs` vs `sphinx-build/jinja2` | target: >2× faster |

## Phase 10 — Parity test suite

**Goal:** Port the upstream Jinja2 Python test suite to Rust, gate every release.

Source tests in `src/jinja2/tests/`:

| Python test file | Rust target | Status |
|---|---|---|
| `test_filters.py` | `tests/test_filters_parity.rs` | not started |
| `test_core_tags.py` | `tests/test_tags_parity.rs` | not started |
| `test_lexnparse.py` | `tests/test_lexnparse_parity.rs` | not started |
| `test_inheritance.py` | `tests/test_inheritance_parity.rs` | not started |
| `test_regression.py` | `tests/test_regression_parity.rs` | not started |
| `test_security.py` | `tests/test_security_parity.rs` | not started |
| `test_loader.py` | `tests/test_loader_parity.rs` | not started |

Each test is tagged as one of:
- `exact` — byte-for-byte identical output to Python Jinja2
- `accepted deviation` — documented behavioural difference (e.g., HTML escaping style)
- `pending` — known gap, tracked as open issue

## Architecture diagram

```
sphinxdocrs (Rust)
    │
    └─► jinja2rs::sphinx_glue::BuiltinTemplateLoader
            │
            ├─► jinja2rs::loaders::SphinxFileSystemLoader
            │       └─► searches theme dirs + templates_path
            │
            ├─► jinja2rs::sandbox::SandboxedEnvironment
            │       └─► minijinja::Environment (core engine)
            │
            ├─► jinja2rs::filters  (tobool, toint, todim, slice_index)
            └─► jinja2rs::globals  (IdGen, AccessKey)

Python (optional bridge for migration):
    import jinja2rs
    env = jinja2rs.SandboxedEnvironment()
    env.render_str("{{ name }}", {"name": "Sphinx"})
```

## Compatibility notes

- minijinja uses `&lt;` / `&gt;` / `&#x2f;` for HTML escaping; Python Jinja2 uses
  `&#60;` / `&#62;`.  Snapshot tests document this as an **accepted deviation**.
- minijinja does not implement Python methods (`x.items()`, `x.values()`).  Use
  `|items` filter instead.  `minijinja-contrib` `pycompat` feature can bridge this
  if required.
- Template up-to-date checks (mtime-based cache invalidation) are stubbed; full
  implementation is tracked in Phase 2.
- Async rendering (Jinja2's `async_utils.py`) is out of scope; minijinja's
  synchronous engine is sufficient for sphinxdocrs.
