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

## Completed (Phase 1 — Bootstrap)

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

## Phase 2 — Loader completeness

**Goal:** Full filesystem loader parity with `jinja2.FileSystemLoader`.

| Task | Status | Jinja2 source |
|---|---|---|
| `ChoiceLoader` (try multiple loaders in order) | not started | `loaders.py:ChoiceLoader` |
| `BaseLoader` trait (custom loader interface) | not started | `loaders.py:BaseLoader` |
| `PackageLoader` (load from a Rust crate's embedded assets) | not started | `loaders.py:PackageLoader` |
| `DictLoader` (load from `HashMap<String, String>`) | not started | `loaders.py:DictLoader` |
| `FunctionLoader` (closure-based loader) | not started | `loaders.py:FunctionLoader` |
| Template up-to-date check (mtime) | partial | `jinja2glue.py:uptodate` |
| `SphinxFileSystemLoader.get_source()` — full Sphinx API | partial | `jinja2glue.py` |

## Phase 3 — Filter completeness

**Goal:** All Jinja2 built-in filters pass byte-parity tests against CPython Jinja2.

minijinja already ships most built-in filters.  The following need Sphinx-specific
or parity-gap attention:

| Filter | Status | Notes |
|---|---|---|
| `tobool` | done | Sphinx-specific |
| `toint` | done | Sphinx-specific |
| `todim` | done | Sphinx-specific |
| `slice_index` | done | Sphinx-specific |
| `indent` | minijinja built-in | verify parity for `first=True` case |
| `wordwrap` | minijinja-contrib | enable `wordwrap` feature |
| `xmlattr` | minijinja built-in | verify XML escaping |
| `filesizeformat` | not in minijinja | port from `jinja2/filters.py` |
| `urlencode` | minijinja feature flag | enable `urlencode` |
| `tojson` | minijinja `json` feature | already enabled |

## Phase 4 — Globals and tests completeness

**Goal:** All Jinja2 built-in globals and test functions have parity.

| Global/Test | Status | Notes |
|---|---|---|
| `idgen` | done | Sphinx-specific |
| `accesskey` | done (Object) | Sphinx-specific; needs full template integration test |
| `warning` | partial (stub) | Wire to `sphinxdocrs` logging when integrated |
| `debug` (pformat) | not started | `{{ debug(var) }}` |
| `lipsum` | not in minijinja | port from `jinja2/utils.py` |
| `namespace` | minijinja built-in | verify behaviour |
| `cycler` | not in minijinja | port from `jinja2/utils.py` |
| `joiner` | not in minijinja | port from `jinja2/utils.py` |

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
