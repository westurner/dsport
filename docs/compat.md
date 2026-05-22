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
| rST parser — simple references + explicit targets | `docutils.parsers.rst` + `docutils.transforms.references` | `src/docutilsrs/src/parser.rs` | accepted-deviation | named, anonymous (`name__` + `__ uri` / FIFO), and embedded-URI references; unresolved-target system messages deferred |
| rST parser — enumerated lists | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | accepted-deviation | arabic / lower-/upperalpha / lower-/upperroman / auto-`#`; bare, `N.`, `N)`, `(N)`; `start` attribute; continuation lines; roman-vs-alpha disambiguation by peek-ahead; no nested lists, no multi-paragraph items |
| rST parser — sections + transitions | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | accepted-deviation | underlined and overlined title sections; arbitrary section punctuation; document title + subtitle promotion |
| rST parser — block quotes | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | accepted-deviation | indented blocks; attribution line (`--`/`---`) split into `<attribution>`; multi-paragraph attribution variant deferred |
| rST parser — literal blocks (`::`) | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | exact-parity | expanded (`Intro::`), partially-expanded (`Intro ::`), and quoted (`::`) forms |
| rST parser — definition lists | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | accepted-deviation | term / `term : classifier` / definition; single-paragraph definitions |
| rST parser — field lists + docinfo | `docutils.parsers.rst` + `docutils.transforms.frontmatter` | `src/docutilsrs/src/parser.rs` | accepted-deviation | `:name: value` lists; `<docinfo>` promotion of recognized bibliographic fields after document title |
| rST parser — comments | `docutils.parsers.rst` | `src/docutilsrs/src/parser.rs` | exact-parity | `..` block form |
| rST parser — admonitions | `docutils.parsers.rst.directives.admonitions` | `src/docutilsrs/src/parser.rs` | accepted-deviation | note/warning/tip/hint/important/attention/caution/danger/error |
| rST parser — image / figure | `docutils.parsers.rst.directives.images` | `src/docutilsrs/src/parser.rs` | accepted-deviation | `:alt:` / `:width:` / `:height:` options; figure with caption + legend body |
| rST parser — footnotes & citations | `docutils.parsers.rst` + `docutils.transforms.references` | `src/docutilsrs/src/parser.rs` | accepted-deviation | numbered footnotes (`.. [1] body` + `[1]_`), named citations (`.. [Knuth1986] body` + `[Knuth1986]_`), autonumber footnotes (anonymous `[#]_` and named `[#name]_`), and autosymbol footnotes (`[*]_` with the standard 10-symbol rotation, doubled past the first 10). Back-references resolved; auto numbering skips manual numeric labels. |
| rST parser — code / code-block / sourcecode | `docutils.parsers.rst.directives.body` | `src/docutilsrs/src/parser.rs` | accepted-deviation | emitted as `<literal_block classes="code [lang]">`; no syntax highlighting (Pygments tokens not produced) |
| rST parser — raw directive | `docutils.parsers.rst.directives.misc` | `src/docutilsrs/src/parser.rs` | exact-parity | `format` argument, indented body |
| rST parser — inline roles | `docutils.parsers.rst.roles` | `src/docutilsrs/src/parser.rs` | accepted-deviation | `:emphasis:` / `:strong:` / `:literal:` / `:code:` / `:title:` (== `:title-reference:`); unknown roles → `<inline classes="…">` |
| rST parser — substitutions | `docutils.parsers.rst` + `docutils.transforms.references` | `src/docutilsrs/src/parser.rs` | accepted-deviation | `replace::` directive resolved inline; other substitution directives dropped |
| rST parser — tables (simple + grid) | `docutils.parsers.rst.tableparser` | `src/docutilsrs/src/parser.rs` | accepted-deviation | head + body rows; no row/col spans; no multi-paragraph cells |
| rST parser — phrase references | `docutils.parsers.rst` + `docutils.transforms.references` | `src/docutilsrs/src/parser.rs` | accepted-deviation | `` `Phrase Name`_ `` resolved against `.. _Phrase Name: uri`; embedded URIs (`` `text <uri>`_ ``) and anonymous phrase refs supported |
| transforms module | `docutils.transforms` | `src/docutilsrs/src/transforms.rs` | accepted-deviation | `resolve_references`, `promote_document_title`, `promote_docinfo`, `run_default_pipeline`, and a composable `Transform` trait + `Pipeline` builder (with stock `ResolveReferences` / `PromoteDocumentTitle` / `PromoteDocinfo` types); full transform registry / per-component priorities deferred |
| feature introspection | — | `src/docutilsrs/src/lib.rs` | exact-parity | `docutilsrs.features()` / `docutilsrs.supports("...")` advertise the Rust port's capability flags so hybrid wrappers can dispatch precisely |
| hybrid Rust/Python wrapper | — | `src/docutilsrs/python/docutilsrs_hybrid.py` | accepted-deviation | `publish_string(..., prefer='rust')` routes pseudoxml/html5 through Rust (feature-gated via `docutilsrs.supports`) and falls back to `docutils.core.publish_string`; also exposes `compare()` for parity probing and `rust_supports_writer()` / `features()` |
| Python directive plugin bridge | `docutils.parsers.rst.directives.register_directive` | `src/docutilsrs/src/plugins.rs` | accepted-deviation | `docutilsrs.register_directive(name, callable)` / `unregister_directive` / `registered_directives` / `clear_directives`; the Rust parser dispatches unknown directives to a registered Python callable receiving `(args, body)` and re-parses its returned rST string. Plugin exceptions degrade to the existing comment-swallow fallback. |
| Pygments syntax highlighting | `docutils.parsers.rst.directives.code` | `src/docutilsrs/python/docutilsrs_pygments.py` | accepted-deviation | optional add-on: registers a `pyghl` directive via the plugin bridge that pipes the body through Pygments and emits a `raw:: html` block. Demonstrates how to bolt Python-only Pygments support onto the Rust parser without bundling it in the core. |
| HTML5 writer (minimal) | `docutils.writers.html5_polyglot` | `src/docutilsrs/src/html5_writer.rs` | accepted-deviation | fragment output; semantic subset (`<p>`/`<em>`/`<strong>`/`<code>`/`<ul>`/`<ol>`/`<dl>`/`<table>`/`<section>`/headings/`<blockquote>`/`<aside>` etc.); not parity-gated against upstream |

## sphinxdocrs

| Subsystem | Upstream module | Rust location | Status | Notes |
|-----------|-----------------|---------------|--------|-------|
| crate scaffold | — | `src/sphinxdocrs/src/lib.rs` | exact-parity | M1: `version()` only |
| config | `sphinx.config` | — | pending | phase 4 |
| environment | `sphinx.environment` | — | pending | phase 4 |
| events | `sphinx.events` | — | pending | phase 4 (priority for plugins) |
| builders | `sphinx.builders.*` | — | pending | phase 4 |
