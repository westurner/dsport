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
| `python` | `python`, `py`, `python3`     |   🟡   |  🟡  | Phase 1 starter: keywords / `def`/`class`-name / simple strings (no escapes, no triples) / integers / `#` comments / punctuation / operators. f-strings, decorators, raw/byte strings, numeric variants (hex/bin/oct/float), formatted-string state machine: not yet. |

## Formatters

| name   | tokens accepted | html | notes                                                                       |
| ------ | --------------- | :--: | --------------------------------------------------------------------------- |
| `html` | any             |  🟡  | Default-options shape only. Uses placeholder short-name mapping until the full `STANDARD_TYPES` table lands (will be needed for byte-parity vs `pygments.formatters.html.HtmlFormatter`). |

## Engine coverage (`pygmentsrs::lexer::engine`)

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
