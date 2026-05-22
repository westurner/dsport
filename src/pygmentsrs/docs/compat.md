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
| `python` | `python`, `py`, `python3`     |   ✅   |  🟡  | Byte-parity for 33 fixtures in `src/tests/test_parity_pseudoxml.py` (`code_block_python_*`). Constructs covered: `def NAME` / `def __dunder__` → `Name.Function` / `Name.Function.Magic`; `class NAME` → `Name.Class`; `from MOD import NAME` via `fromimport_state` → `fromimport_plain` (module → `Name.Namespace`, imported names → `Name`); `import MOD, MOD2 as alias` via `import_state` (names → `Name.Namespace`, comma → `Operator`); relative imports (`from . import X`); parenthesised imports (`from X import (a, b)`); `True`/`False`/`None` → `Keyword.Constant`; walrus `:=` → `Operator`; line-continuation `\\\n`/`\\` → `Text`; `@deco` → `Name.Decorator`, bare `@` → `Operator` (matrix-mul fallback); escape sequences (`\n`, `\xhh`, `\uhhhh`, etc. → `String.Escape`); raw strings (`r"…"` — no escape tokenization); triple-quoted strings (`"""…"""`, `'''…'''` → `String.Double`/`String.Single`); prefixed strings (`b"…"`, `rb"…"`); f-strings with `{expr}`, format specs, conversion flags, literal braces, triple f-strings, **nested string literals inside `{…}`**; `Name.Builtin` (69 builtins: `print`, `len`, etc.); `Name.Builtin.Pseudo` (`self`, `cls`, `Ellipsis`, `NotImplemented`); `Name.Exception` (all stdlib exception classes); `Name.Variable.Magic` (`__name__`, `__file__`, etc.); `#` comments; integers, hex/oct/bin/float numbers; operators; `in`/`is`/`and`/`or`/`not` → `Operator.Word` inside f-string `{…}` expressions. Whitespace bifurcated (`\n` → `Token.Text.Whitespace`, horizontal → `Token.Text`; all whitespace → `Token.Text.Whitespace` inside f-string expressions). **Accepted deviations**: standalone triple-string docstrings emit `String.Double` instead of `String.Doc`; `match`/`case` soft keywords emit `Name` (Rust `regex` crate lacks lookaheads; deferred); complex-number `j` suffix is `Name` (matches actual upstream behavior). |

## Formatters

| name   | tokens accepted | html | notes                                                                       |
| ------ | --------------- | :--: | --------------------------------------------------------------------------- |
| `html` | any             |  🟡  | Default-options shape only. Uses placeholder short-name mapping until the full `STANDARD_TYPES` table lands (will be needed for byte-parity vs `pygments.formatters.html.HtmlFormatter`). |

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
