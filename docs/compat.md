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
| doctree (slice) | `docutils.nodes` | `src/docutilsrs/src/doctree.rs` | accepted-deviation | Document, Paragraph, Text, Emphasis, Strong, Literal, BulletList, EnumeratedList, ListItem, Reference, Target |
| Python FFI (`Doctree`/`Node`) | `docutils.nodes.Element` | `src/docutilsrs/src/python.rs` | accepted-deviation | `.tag`, `.attributes`, `.children`, `.text`, `.pformat()` only |
| pseudo-XML writer | `docutils.writers.pseudoxml` | `src/docutilsrs/src/writer.rs` | exact-parity | byte-for-byte on the supported node kinds |
| rST parser — paragraphs (incl. multi-line) | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | exact-parity | blank-line splitting, source-line preservation |
| rST parser — bullet lists | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | accepted-deviation | `-`/`*`/`+`, single-paragraph items with continuation lines; no nested lists, no multi-paragraph items |
| rST parser — inline (emphasis/strong/literal) | `docutils.parsers.rst.states` | `src/docutilsrs/src/parser.rs` | exact-parity | flat (non-nesting) per rST spec |
| rST parser — backslash escapes | `docutils.parsers.rst.states` | `src/docutilsrs/src/parser.rs` | exact-parity | `\X` → `X`, `\<ws>` consumed |
| rST parser — simple references + explicit targets | `docutils.parsers.rst` + `docutils.transforms.references` | `src/docutilsrs/src/parser.rs` | accepted-deviation | `name_` + `.. _name: uri` only; phrase refs and unresolved-target system messages deferred |
| rST parser — enumerated lists | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | accepted-deviation | arabic / lower-/upperalpha / lower-/upperroman / auto-`#`; bare, `N.`, `N)`, `(N)`; `start` attribute; continuation lines; roman-vs-alpha disambiguation by peek-ahead; no nested lists, no multi-paragraph items |
| rST parser — definition/field lists | `docutils.parsers.rst` | — | pending | phase 2 |
| rST parser — tables/directives/roles/substitutions | `docutils.parsers.rst` | — | pending | phase 2 |
| transforms (full) | `docutils.transforms` | — | pending | phase 2 |
| HTML5 writer | `docutils.writers.html5_polyglot` | — | pending | phase 2 |

## sphinxdocrs

| Subsystem | Upstream module | Rust location | Status | Notes |
|-----------|-----------------|---------------|--------|-------|
| crate scaffold | — | `src/sphinxdocrs/src/lib.rs` | exact-parity | M1: `version()` only |
| config | `sphinx.config` | — | pending | phase 4 |
| environment | `sphinx.environment` | — | pending | phase 4 |
| events | `sphinx.events` | — | pending | phase 4 (priority for plugins) |
| builders | `sphinx.builders.*` | — | pending | phase 4 |
