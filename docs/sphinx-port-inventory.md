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
| `project.py` | `sphinxdocrs::project` | **P1** | `Project.path2doc` / `doc2path` first; `discover()` defers globbing logic — port after `util.matching` is available |
| `addnodes.py` | n/a (Python re-export) | **P1** | extends docutils.nodes — keep as Python shim that imports vendored `sphinx.addnodes` until our doctree gains Sphinx-specific node types |
| `extension.py` | `sphinxdocrs::extension` | **P2** | `Extension` wrapper class + version metadata; depends on event/registry |
| `registry.py` | `sphinxdocrs::registry` | **P2** | builder/parser/transform/translator registries; depends on extension + events |
| `config.py` | `sphinxdocrs::config` | **P2** | depends on `util.typing`, complex value coercion; port `Config` after util |
| `roles.py` / `directives/` | `sphinxdocrs::roles` etc | **P3** | needs the doctree converter (already in `docutilsrs::python`) and the directive/role registry |
| `domains/` | `sphinxdocrs::domains` | **P3** | each domain is a substantial subsystem (`py`, `c`, `cpp`, `js`, `rst`, `std`) |
| `environment/` | `sphinxdocrs::environment` | **P3** | the build environment, large and stateful |
| `builders/` | `sphinxdocrs::builders` | **P3** | one builder at a time (`html`, `latex`, `epub`, ...) |
| `ext/*` | n/a (Python plugins) | **P3** | keep as Python; loaded via `Extension` registry |
| `util/*` | `sphinxdocrs::util::*` | **P2** | port the pieces actually depended on by P1/P2 modules; do not port the whole tree |
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
| `test_project.py` | project | P1 | partial-mirror (`path2doc`/`doc2path`); `discover` deferred |
| `test_addnodes.py` | addnodes | P1 | deferred (no Sphinx-specific nodes in Rust doctree yet) |
| `test_application.py` | application | P3 | deferred |
| `test_command_line.py`, `test__cli/` | cli | P3 | deferred |
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
| `test_quickstart.py` | quickstart | P3 | deferred |
| `test_roles.py` | roles | P3 | deferred |
| `test_search.py` | search | P3 | deferred |
| `test_theming/` | theming | P3 | deferred |
| `test_transforms/` | transforms | P3 | deferred (per-transform port) |
| `test_util/` | util | P2 | deferred (port piecewise on demand) |
| `test_versioning.py` | versioning | P2 | deferred |
| `test_writers/` | writers | P3 | deferred (one writer at a time) |
| `test_builders/` | builders | P3 | deferred (one builder at a time) |
| `test_ext_autodoc/`, `test_ext_autosummary/`, `test_ext_imgconverter/`, `test_ext_intersphinx/`, `test_ext_napoleon/` | extensions | P3 | deferred — these run against vendored Python sphinx |
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
