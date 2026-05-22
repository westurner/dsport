# Handoff — Pygments syntax highlighting for `code`/`code-block`/`sourcecode`

**Status**: ✅ **Landed (Phase 3 wire-up complete)**. The native
backend lives at [src/docutilsrs/src/code_block.rs](../../src/docutilsrs/src/code_block.rs)
and dispatches: in-workspace `pygmentsrs` crate first (Rust→Rust, no
GIL hop) → PyO3 bridge to `docutils.utils.code_analyzer.Lexer` for
languages pygmentsrs hasn't ported yet → `None` (raw text). Remaining
work is widening pygmentsrs lexer coverage and tightening byte-parity
for the python lexer; see [src/pygmentsrs/docs/compat.md](../../src/pygmentsrs/docs/compat.md).

The original handoff brief is preserved below for context.

---

**Target agent**: implements native (Rust-side) syntax-highlighting parity
with `docutils.parsers.rst.directives.body.CodeBlock`, replacing the
opt-in plugin-bridge stub at `src/docutilsrs/python/docutilsrs_pygments.py`.

**Status going in**: parser emits a plain
`LiteralBlock { text, classes: "code [lang]" }` for the three directives
with no token analysis. Upstream emits a `literal_block` whose children
are `inline` nodes carrying Pygments token classes.

## What "done" looks like (parity target)

Input

```rst
.. code-block:: python

   def f():
       return 1
```

Upstream pseudo-XML (verified with vendored docutils
`writer="pseudoxml"`, default `_disable_config=True`):

```
<document source="<string>">
    <literal_block classes="code python" xml:space="preserve">
        <inline classes="keyword">
            def
        <inline classes="whitespace">
             
        <inline classes="name function">
            f
        <inline classes="punctuation">
            ():
        <inline classes="whitespace">
            \n            
        <inline classes="keyword">
            return
         
        <inline classes="literal number integer">
            1
```

Notes on the upstream shape:

- Container is `literal_block` with `classes="code <lang>"` and
  `xml:space="preserve"` (the parser's writer already emits this for
  `LiteralBlock` — verify with `tests/test_parity_pseudoxml.py`).
- Children are `inline` nodes; class names come from
  `docutils.utils.code_analyzer.Lexer` with `tokennames='long'`
  (downcased, dotted token type with the leading `Token.` stripped and
  `.` replaced by space — e.g. `Token.Name.Function` → `"name function"`).
- Tokens of type `Token`, `Token.Text`, or empty short-name are emitted
  as raw text (no `<inline>` wrapper). See
  `docutils/docutils/docutils/utils/code_analyzer.py` (`unstyled_tokens`).
- Adjacent tokens of the same type are merged before emission.
- A trailing `\n` added by Pygments must be stripped from the last
  token's value.

## Existing infrastructure (use these — do not duplicate)

- `NodeKind::LiteralBlock { text, classes }` and `NodeKind::Inline { classes }`
  are already defined in [src/docutilsrs/src/doctree.rs](src/docutilsrs/src/doctree.rs).
- `parse_to_pseudoxml` already emits both correctly; round-trip parity
  is covered by [tests/test_parity_pseudoxml.py](src/tests/test_parity_pseudoxml.py).
- Directive dispatch happens in [src/docutilsrs/src/parser.rs](src/docutilsrs/src/parser.rs)
  around line 1441 (`"code" | "code-block" | "sourcecode" =>`). Currently
  builds `Block::LiteralBlock { text, classes }`. This is the single
  rewrite site.
- Plugin bridge (`crate::plugins`) is unrelated to this work and
  should NOT be the implementation path — the goal is native parity,
  not a Python callback per code block.

## Implementation approach (recommended)

> **Landed approach** (supersedes Path A/B below): a hybrid was used.
> A new in-workspace crate, [`pygmentsrs`](../../src/pygmentsrs/), ports
> the Pygments engine + token hierarchy to Rust and is called natively
> by the parser. The Python `code_analyzer.Lexer` bridge from Path A
> is preserved as a fallback for languages pygmentsrs has not yet
> ported. See [src/docutilsrs/src/code_block.rs](../../src/docutilsrs/src/code_block.rs)
> for the dispatcher; see [README.md §Phase 2.5](../../README.md) for
> the per-phase status of the pygmentsrs port.

There is no native Rust port of Pygments. Two viable paths:

### Path A — bridge to Python `docutils.utils.code_analyzer.Lexer` (recommended first step)

- In the `"code" | "code-block" | "sourcecode"` arm, instead of emitting
  a flat `LiteralBlock { text, classes }`, build a new
  `Block::CodeBlock { lang: String, text: String }` (or extend
  `LiteralBlock` to allow pre-tokenized children — see "data model"
  below).
- At doctree-build time (`emit_block`), if `lang` is non-empty and
  non-`"text"`, call into Python via PyO3:
  - import `docutils.utils.code_analyzer`
  - construct `Lexer(code, language, tokennames='long')`
  - iterate `(ttype, value)` tuples
  - convert each to a child `Inline { classes: ttype }` text node, or
    raw text when `ttype` is in the unstyled set
- If Python or Pygments is unavailable, or `Lexer.__init__` raises
  `LexerError`, fall back to the current single-text-child shape
  (matches docutils' behavior with `syntax_highlight = "none"`).

This is the only path that gives byte-parity with `docutils` for
`tests/test_parity_pseudoxml.py` and matches the byte-parity strategy
already used for the ODT writer's `compat=True` mode (see
[src/docutilsrs/src/lib.rs](src/docutilsrs/src/lib.rs)
`py_parse_to_odt`).

### Path B — wrap pygments via a Rust crate

Skip. There is no maintained pure-Rust port of Pygments; `syntect` uses
a different grammar set and would not produce identical token classes.
Path A is the only route to parity.

## Data model decision

> **Landed**: option #1. `Block::LiteralBlock` (parser) grew an
> `Option<Vec<(Option<String>, String)>>` field; `emit_block` walks it
> to produce `<inline classes="…">` children when `Some`, falling back
> to the original single Text child when `None`.

Pick one:

1. **Extend `LiteralBlock`** to optionally carry pre-tokenized children:
   add `tokens: Option<Vec<(Option<String>, String)>>` (None = raw text
   path; `Some` = list of `(class, text)` pairs where `class=None` means
   unstyled). Pro: smallest diff. Con: bifurcates the node shape.

2. **New `NodeKind::CodeBlock { lang, text }`** processed by a new
   transform that, at emit time, calls the Python lexer and replaces
   the node with a `LiteralBlock` whose children are `Inline`s.
   Pro: separates concerns; mirrors how upstream applies the
   `code_block_lexer` transform. Con: more boilerplate.

Recommend #1 for the first cut; refactor to #2 only if test surface
demands the indirection.

## Tests to add

> **Landed**: smoke gate at
> [src/tests/test_pygments_native.py](../../src/tests/test_pygments_native.py)
> (5 tests covering python token inlines, text passthrough, no-lang,
> sourcecode alias, unknown-lang fallthrough). Byte-parity gate at
> [src/tests/test_parity_pseudoxml.py](../../src/tests/test_parity_pseudoxml.py)
> picked up `code_block_language_text` and `code_sourcecode_text_alias`
> (102 → **104**). The `code_block_python_*` byte-parity fixtures are
> deferred until pygmentsrs' PythonLexer reaches byte-parity with
> vendored pygments (tracked in pygmentsrs compat.md).

Add new cases to [tests/test_parity_pseudoxml.py](../../src/tests/test_parity_pseudoxml.py)
(the byte-parity gate). Each must produce identical pseudo-XML to
`docutils.publish_string(src, writer="pseudoxml", settings_overrides={"_disable_config": True})`.

Mandatory fixtures (mirror docutils' own coverage):

- `code_block_python_def` — the example above.
- `code_block_python_string` — exercises `Token.Literal.String` (multi-word class).
- `code_block_no_language` — `.. code::` with no arg; should emit
  `classes="code"` and no inline children (raw text).
- `code_block_language_text` — `.. code-block:: text`; same as above.
- `code_block_unknown_language` — `.. code-block:: not-a-real-language`;
  upstream emits a system_message. Match it, or document as
  accepted-deviation.
- `code_block_sourcecode_alias` — same as `code-block` but using the
  `sourcecode` directive name.
- `code_block_inside_list_item` — nested context, exercises indentation.
- `code_block_with_options` — `:number-lines:` and `:class:` options
  (these may be accepted-deviation if not yet supported by the parser;
  document the gap explicitly).

Also extend [tests/test_plugin_bridge.py](src/tests/test_plugin_bridge.py) or add
`tests/test_pygments_native.py` with:

- `test_pygments_missing_falls_back_to_plain` — monkey-patch
  `docutils.utils.code_analyzer.with_pygments = False` and verify the
  output is the plain `<literal_block>...text...</literal_block>` shape
  (no `<inline>` children, no exception).
- `test_unknown_language_emits_system_message_or_plain` — explicit
  decision recorded.

Run gates after each step:

```sh
cd /workspaces/dsport/src
cargo build --release -p docutilsrs
cargo test --release -p docutilsrs
.venv/bin/maturin develop --release -m docutilsrs/Cargo.toml
.venv/bin/pytest -q tests/
```

Current baseline (pre-handoff): **24 cargo tests + 184 pytest, zero
skips**; **102** of those pytest are byte-parity cases in
`test_parity_pseudoxml.py`. The new fixtures must keep that count
monotonically increasing; no parity regressions.

> **Post-landing**: pytest is now at **235 passed, zero skips**; the
> byte-parity count is at **104**. Pygments-related cargo unit tests
> (4 in `code_block.rs` + 3 token + 5 pygmentsrs snapshot) sit
> alongside the existing docutilsrs suite.

## Documentation to update on landing

- [README.md](README.md) Phase 2 — Writers/Plugin-bridges section: move
  Pygments out of "Plugin bridges" into the parser line, and bump the
  pseudo-XML parity-gate count.
- [docs/compat.md](docs/compat.md) — find the `code`/`code-block`/
  `sourcecode` rows and flip them from accepted-deviation to
  byte-parity for the implemented fixture set.
- Add an ADR under `docs/adr/` for the Python-bridge decision (Path A),
  citing the absence of a pure-Rust Pygments port and the precedent set
  by the ODT writer's `compat=True` bridge.

## Out of scope

- Any direct dependency on `syntect`, `tree-sitter-highlight`, or other
  non-Pygments highlighters.
- Removing or breaking the existing
  `src/docutilsrs/python/docutilsrs_pygments.py` example (it is a
  *different* feature — a user-facing `pyghl` directive that emits raw
  HTML). Leave it in place.
- HTML writer integration. The pseudo-XML parity gate is the only
  required signal; HTML5 writer is already accepted-deviation.

## Files the next agent will touch

- [src/docutilsrs/src/parser.rs](src/docutilsrs/src/parser.rs) — the
  `"code" | "code-block" | "sourcecode"` arm + `Block` enum.
- [src/docutilsrs/src/doctree.rs](src/docutilsrs/src/doctree.rs) — only
  if extending `LiteralBlock` (Path #1 data model).
- [src/docutilsrs/src/writer.rs](src/docutilsrs/src/writer.rs) — only
  if `Inline` emission inside `LiteralBlock` needs adjustment (verify
  with a quick read first; the existing emit path is likely sufficient).
- [src/docutilsrs/src/lib.rs](src/docutilsrs/src/lib.rs) — wire the
  Python-bridge call; add a feature flag (suggested:
  `parser:code_block_pygments`).
- [src/tests/test_parity_pseudoxml.py](src/tests/test_parity_pseudoxml.py)
  — add the fixtures listed above.

## Useful one-liners

Print upstream pseudo-XML for any rST source (use this to author new
parity fixtures):

```sh
cd /workspaces/dsport/src && .venv/bin/python -c "
import sys, docutils.core
src = sys.stdin.read()
print(docutils.core.publish_string(source=src, writer='pseudoxml',
    settings_overrides={'_disable_config': True}).decode(), end='')
" <<'EOF'
.. code-block:: python

   x = 1
EOF
```

Dump the Lexer's token stream directly:

```sh
.venv/bin/python -c "
from docutils.utils.code_analyzer import Lexer
for t in Lexer('def f():\n    return 1\n', 'python', 'long'):
    print(repr(t))
"
```
