# Compatibility matrix

Each row tracks one upstream feature, test module, or subsystem ported to
the Rust crates. Status values:

| Status              | Meaning                                                            |
|---------------------|--------------------------------------------------------------------|
| `exact-parity`      | Behavior matches vendored Python byte-for-byte on the tested inputs |
| `accepted-deviation` | Differs from Python; deviation is intentional and documented      |
| `pending`           | Not yet ported                                                     |
| `n/a`               | Will not be ported (e.g. deprecated upstream)                      |

Columns:

| Subsystem | Upstream module | Rust location | Status | Notes |
|-----------|-----------------|---------------|--------|-------|

## docutilsrs

| Subsystem | Upstream module | Rust location | Status | Notes |
|-----------|-----------------|---------------|--------|-------|
| crate scaffold | — | `src/docutilsrs/src/lib.rs` | exact-parity | M1: `version()` only |
| doctree (minimal) | `docutils.nodes` | `src/docutilsrs/src/doctree.rs` | accepted-deviation | phase 1 slice: Document, Paragraph, Text, Emphasis, Strong, Literal only |
| pseudo-XML writer | `docutils.writers.pseudoxml` | `src/docutilsrs/src/writer.rs` | exact-parity | byte-for-byte on the phase 1 slice (see `tests/test_parity_pseudoxml.py`) |
| rST parser — paragraphs | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | exact-parity | blank-line splitting only |
| rST parser — inline (emphasis/strong/literal) | `docutils.parsers.rst.states` | `src/docutilsrs/src/parser.rs` | exact-parity | flat (non-nesting) per rST spec, on tested cases |
| rST parser — references | `docutils.parsers.rst.states` | — | pending | requires `<target>` sibling + transforms |
| rST parser — lists/tables/directives/roles | `docutils.parsers.rst` | — | pending | phase 2 |
| escape sequences in inline | — | — | pending | phase 2 |
| transforms | `docutils.transforms` | — | pending | phase 2 |
| HTML5 writer | `docutils.writers.html5_polyglot` | — | pending | phase 2 |

## sphinxdocrs

| Subsystem | Upstream module | Rust location | Status | Notes |
|-----------|-----------------|---------------|--------|-------|
| crate scaffold | — | `src/sphinxdocrs/src/lib.rs` | exact-parity | M1: `version()` only |
| config | `sphinx.config` | — | pending | phase 4 |
| environment | `sphinx.environment` | — | pending | phase 4 |
| events | `sphinx.events` | — | pending | phase 4 (priority for plugins) |
| builders | `sphinx.builders.*` | — | pending | phase 4 |
