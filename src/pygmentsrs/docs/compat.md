# pygmentsrs compatibility matrix

Tracks how close each lexer/formatter in this crate is to the
vendored `pygments` at `src/pygments/`. Columns:

- **tokens** — byte-identical `(ttype, value)` stream vs
  `pygments.lex(code, LexerClass())` for the fixture set in
  `src/tests/test_*_lexer_parity.py`.
- **html** — byte-identical `HtmlFormatter().format(tokens, out)`
  output for the same fixtures.
- **notes** — known gaps / accepted deviations.

Status legend: ✅ byte-parity · 🟡 partial · 🔲 accepted-deviation · — not yet

## Lexers

| name     | aliases                       | tokens | html | notes                                                                                    |
| -------- | ----------------------------- | :----: | :--: | ---------------------------------------------------------------------------------------- |
| `text`   | `text`, `plain`, `""`         |   ✅   |  ✅  | Trivial passthrough.                                                                     |
| `python` | `python`, `py`, `python3`     |   ✅   |  ✅  | Byte-parity for 33 fixtures in `src/tests/test_parity_pseudoxml.py` (`code_block_python_*`). Constructs covered: `def NAME` / `def __dunder__` → `Name.Function` / `Name.Function.Magic`; `class NAME` → `Name.Class`; `from MOD import NAME` via `fromimport_state` → `fromimport_plain` (module → `Name.Namespace`, imported names → `Name`); `import MOD, MOD2 as alias` via `import_state` (names → `Name.Namespace`, comma → `Operator`); relative imports (`from . import X`); parenthesised imports (`from X import (a, b)`); `True`/`False`/`None` → `Keyword.Constant`; walrus `:=` → `Operator`; line-continuation `\\\n`/`\\` → `Text`; `@deco` → `Name.Decorator`, bare `@` → `Operator` (matrix-mul fallback); escape sequences (`\n`, `\xhh`, `\uhhhh`, etc. → `String.Escape`); raw strings (`r"…"` — no escape tokenization); triple-quoted strings (`"""…"""`, `'''…'''` → `String.Double`/`String.Single`); prefixed strings (`b"…"`, `rb"…"`); f-strings with `{expr}`, format specs, conversion flags, literal braces, triple f-strings, **nested string literals inside `{…}`**; `Name.Builtin` (69 builtins: `print`, `len`, etc.); `Name.Builtin.Pseudo` (`self`, `cls`, `Ellipsis`, `NotImplemented`); `Name.Exception` (all stdlib exception classes); `Name.Variable.Magic` (`__name__`, `__file__`, etc.); `#` comments; integers, hex/oct/bin/float numbers; operators; `in`/`is`/`and`/`or`/`not` → `Operator.Word` inside f-string `{…}` expressions. Whitespace bifurcated (`\n` → `Token.Text.Whitespace`, horizontal → `Token.Text`; all whitespace → `Token.Text.Whitespace` inside f-string expressions). **Accepted deviations**: standalone triple-string docstrings emit `String.Double` instead of `String.Doc`; `match`/`case` soft keywords emit `Name` (deferred; not yet ported — the engine now supports the lookaheads upstream uses); complex-number `j` suffix is `Name` (matches actual upstream behavior). |
| `json`   | `json`, `json-object`         |   ✅   |  ✅  | Hand-written state-machine port of `pygments.lexers.data.JsonLexer`. Byte-parity for 10 fixtures in `src/tests/test_pygments_json_lexer.py` (objects, arrays, constants, numbers, escape sequences, line + block comments). Strings are queued and re-tokenized as `Name.Tag` on the next `:` (JSON object keys). |
| `diff`   | `diff`, `udiff`               |   ✅   |  ✅  | RegexLexer-engine port of `pygments.lexers.diff.DiffLexer`. Byte-parity for 6 fixtures in `src/tests/test_pygments_diff_lexer.py` (unified, context, ed-style, git/index headers, bang lines, plain context). |

### Transpiled lexers (`tools/gen_lexer.py`)

Generated into `src/pygmentsrs/src/lexers/generated/` by the transpiler
(see "Lexer transpiler" below). All byte-parity-gated by
`src/tests/test_pygments_generated_lexers.py` against
`<UpstreamLexer>().get_tokens_unprocessed(...)`.

| name           | aliases                  | tokens | html | upstream                                      |
| -------------- | ------------------------ | :----: | :--: | --------------------------------------------- |
| `ini`          | `ini`, `cfg`, `dosini`   |   ✅   |  ✅  | `pygments.lexers.configs.IniLexer`            |
| `properties`   | `properties`, `jproperties` | ✅ |  ✅  | `pygments.lexers.configs.PropertiesLexer`     |
| `toml`         | `toml`                   |   ✅   |  ✅  | `pygments.lexers.configs.TOMLLexer`           |
| `gettext`      | `pot`, `po`              |   ✅   |  ✅  | `pygments.lexers.textfmts.GettextLexer`       |
| `darcs`        | `dpatch`                 |   ✅   |  ✅  | `pygments.lexers.diff.DarcsPatchLexer`        |
| `vctreestatus` | `vctreestatus`           |   ✅   |  ✅  | `pygments.lexers.console.VCTreeStatusLexer`   |
| `groff`        | `groff`, `nroff`, `man`  |   ✅   |  ✅  | `pygments.lexers.text.GroffLexer`             |
| `bash`         | `bash`, `sh`, `ksh`, `zsh`, `shell`, `openrc` | ✅ | ✅ | `pygments.lexers.shell.BashLexer` (heredoc `\2` backref via fancy-regex) |
| `cmake`        | `cmake`                  |   ✅   |  ✅  | `pygments.lexers.make.CMakeLexer` (named backref `(?P=level)`) |

Coverage status (from `python tools/gen_lexer.py --classify`): of ~585
un-ported lexers, **355 are transpilable now** (token / `bygroups` /
`default` only — structural-token fallback means a missing named const no
longer blocks). The rest are bridge-only: ~103 use arbitrary Python
callbacks, ~111 are not `RegexLexer` subclasses, ~12 use
`using()`/`this` delegation, 4 fail `process_tokendef`. Bridge-only
lexers resolve through the PyO3 `pygments` fallback at runtime.

## Formatters

| name   | tokens accepted | html | notes                                                                       |
| ------ | --------------- | :--: | --------------------------------------------------------------------------- |
| `html` | any             |  ✅  | Full `STANDARD_TYPES` short-name table landed in `src/pygmentsrs/src/token.rs`. `short_name()` walks to the nearest known ancestor for unknown subtypes (matches `HtmlFormatter._get_css_classes` semantics). Default-options output shape now byte-compatible with `pygments.formatters.html.HtmlFormatter` for the lexers above. |

## Fallback bridge (`pygmentsrs::bridge`)

When `pygmentsrs` has no native Rust lexer for an alias, the
`Backend::Auto` (default) dispatch path calls upstream `pygments` via
PyO3:

- ✅ `pygmentsrs.lex(alias, code)` / `pygmentsrs.lex(alias, code, backend="auto")` —
  Rust-native first, Python fallback on `None`
- ✅ `backend="rust"` — native-only, returns `None` if not implemented
- ✅ `backend="python"` — skip native, call upstream directly
- ✅ `pygmentsrs.has_native_lexer(alias)` — query without allocating
- ✅ `pygmentsrs.native_aliases()` — list all native aliases
- ✅ `pygmentsrs.highlight(code, alias)` also accepts `backend=` kwarg



Ported from `pygments.lexer.RegexLexer.get_tokens_unprocessed`:

- ✅ state stack (`Vec<&'static str>`), initial `["root"]`
- ✅ named state transitions (push named, push tuple-of-names)
- ✅ `#pop` and `#pop:N`
- ✅ `#push`
- ✅ `default(...)` zero-width transition
- ✅ `bygroups(...)` per-group tokenization
- ✅ implicit adjacent-same-type token merging
- ✅ no-match fallback: at `\n` → reset to root + emit `Whitespace`,
  otherwise emit `Error <char>` and advance one char
- 🔲 `combined(...)` anonymous combined states — deferred until a
  lexer needs them
- 🔲 `using(...)` / `this` cross-lexer callbacks — deferred
- 🔲 `ExtendedRegexLexer` (context-based) — deferred
- 🔲 inheritance via `inherit` token — deferred (concrete lexers
  currently compose their own state tables directly)
- 🔲 `RegexLexerMeta` lazy compilation — N/A in Rust; state tables
  are built once via `OnceLock`

## Regex backend (`fancy-regex`)

The engine compiles patterns with [`fancy-regex`](https://docs.rs/fancy-regex)
(`pygmentsrs/src/lexer/engine.rs::compile`), **not** the bare `regex`
crate. This is a deliberate, load-bearing decision:

- Pygments patterns are authored against Python's `re` engine. A
  survey of the 263 vendored upstream lexers shows **115 use
  lookahead** (`(?=` / `(?!`), **53 use lookbehind** (`(?<=` /
  `(?<!`), and **23 use backreferences** (`\1`…`\9`) — none of which
  the `regex` crate supports (it omits them to guarantee linear-time
  matching). That is ~44% of lexers that simply cannot be ported on
  bare `regex`.
- `fancy-regex` is pure Rust (no C toolchain), wraps the `regex` crate
  for the common linear-time case, and only drops into a backtracking
  VM for patterns that actually need lookaround / backreferences. The
  same crate is already a dependency elsewhere in this workspace
  (`RaTeX/crates/ratex-parser`).
- Catastrophic backtracking (ReDoS) on adversarial code-block input is
  bounded by `BACKTRACK_LIMIT` (1,000,000 steps, `fancy-regex`'s own
  default, set explicitly). On overflow `fancy-regex` returns `Err`,
  which the engine treats as "no match" and advances one char — so
  lexing always terminates.
- API mapping: `regex::Regex::captures_at(text, pos)` →
  `fancy_regex::Regex::captures_from_pos(text, pos)` (returns
  `Result<Option<Captures>>`); `RegexBuilder::backtrack_limit` sets the
  bound.

Proof tests (`engine.rs` `mod tests`): `lookahead_compiles_and_matches`,
`lookbehind_compiles_and_matches`, `backreference_compiles_and_matches`
(the `\1` backref is the same construct the `bash` heredoc rule needs),
and `backtrack_limit_is_bounded`. All four patterns would `panic!` at
`compile(...)` under the old bare-`regex` engine.

## Lexer transpiler (`tools/gen_lexer.py`)

Most Pygments lexers are pure data: after `RegexLexerMeta.process_tokendef`
each state is a list of `(compiled_pattern.match, action, new_state)`
triples, with `include()` / `words()` / `combined()` already expanded.
`tools/gen_lexer.py` walks that processed table and emits an equivalent
native Rust lexer into `src/pygmentsrs/src/lexers/generated/<name>.rs`.

Usage:

```sh
cd src
.venv/bin/python tools/gen_lexer.py \
    pygments.lexers.configs:IniLexer:ini \
    pygments.lexers.configs:TOMLLexer:toml
# each arg is  module:ClassName:rust_name
```

What it transpiles:

- a `_TokenType` action → `Rule::token` / `Rule::token_to`
- `bygroups(t1, t2, …)` → `Rule::bygroups` / `Rule::bygroups_to`
  (recovered from the closure's `args` freevar; every group must be a
  `_TokenType` or `None`)
- `default(state)` → `Rule::default` (zero-width, `None` action)
- newstate: `None` → `NewState::None`; int `-N` → `Pop(N)`; `'#push'`
  → `PushSame`; tuple → `Push(vec![…])` (engine handles `#pop`/`#push`
  inside the pushed list)
- per-lexer `re` flags (`IGNORECASE`/`MULTILINE`/`DOTALL`/`VERBOSE`) →
  an inline `(?ims x)` prefix on every emitted pattern (always includes
  `m`, matching Pygments' default `re.MULTILINE`)
- token dotted-name → a named Rust `token::` const when one exists
  (mapping parsed directly out of `src/pygmentsrs/src/token.rs`, the
  single source of truth), else a **structural**
  `TokenType::new(&["Name","Variable","Anonymous"])` for ad-hoc custom
  subtypes — so a missing named const never blocks transpilation
- `combined(...)` anonymous states are expanded by `process_tokendef`
  into synthesized state names, which the generator emits like any other
  state (no special handling)

Discovery / wiring modes:

- `tools/gen_lexer.py --classify [category]` — inventory every un-ported
  lexer by transpilability bucket (`transpilable` rows print ready-to-use
  `module:Class:rust_name` specs)
- `tools/gen_lexer.py --registry <specs…>` — print the `generated/mod.rs`
  `pub mod` lines plus the `registry.rs` match arms and `native_aliases`
  entries to paste in

Bridge-only (the tool prints `SKIP` and writes no file — these fall
through to the PyO3 `pygments` bridge at runtime):

- `using(OtherLexer)` / `this` cross-lexer delegation (e.g.
  `BaseMakefileLexer`, which embeds shell)
- arbitrary module-level Python callback actions
- `bygroups(...)` whose groups contain a nested callback

The `words(...)` alternation optimizer (`pygments.regexopt.regex_opt`)
is ported byte-for-byte in `src/pygmentsrs/src/regexopt.rs` (including
Python 3.7+ `re.escape` semantics), so a transpiled `words(...)` rule
compiles to the identical optimized pattern string upstream produces.
Gated by `regexopt::tests::golden_vectors_match_upstream`.

Every generated lexer is byte-parity-gated by
`src/tests/test_pygments_generated_lexers.py` against
`get_tokens_unprocessed`; regenerating after an upstream bump and
re-running that suite is the drift check.
