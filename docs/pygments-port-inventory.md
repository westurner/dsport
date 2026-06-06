# Pygments Port Inventory & Plan

Tracks the port of vendored Pygments (`src/pygments/`) to native Rust in
`src/pygmentsrs/`. Lexers are transpiled with `tools/gen_lexer.py`; the
end-to-end workflow is the **`port-pygments-lexer`** skill
(`.agents/skills/port-pygments-lexer/SKILL.md`). This doc is the
roadmap and progress tracker — it does **not** restate the per-lexer
mechanics (the skill owns those).

Counts below come from `python tools/gen_lexer.py --classify` against the
pinned vendored Pygments. Re-run after an upstream bump.

## Current state (as of June 6, 2026)

- **Lexers**: 598 total, **72 native**, 526 un-ported.
- **Native breakdown**: 13 built-in + 37 Phase A + 22 Phase B + 13 Phase C = 72 total
- **Transpilable remaining**: 296 (of 331 originally)
- **Bridge-only**: 230 (non_regex: 111, bridge_callback: 107, bridge_using: 12)
- **Formatters**: 18 total, **1 native** (`html`).
- Gates: `cargo test -p pygmentsrs` + `tests/test_pygments_generated_lexers.py`
  (byte-parity vs `get_tokens_unprocessed`), `tests/test_pygments_json_lexer.py`,
  `tests/test_pygments_diff_lexer.py`, and the `code_block_*` cases in
  `tests/test_parity_pseudoxml.py`.

### Native lexers (72)

Built-in: `text`, `python`, `json`, `diff`
Pre-Phase A: `ini`, `properties`, `toml`, `gettext`, `darcs`, `vctreestatus`, `groff`, `bash`, `cmake`
**Phase A (24)**: `rust`, `go`, `javascript`, `typescript`, `css`, `xml`, `sql`, `swift`, `perl`, `lua`, `r`, `matlab`, `julia`, `haskell`, `clojure`, `erlang`, `elixir`, `nginx`, `apache`, `powershell`, `tex`, `graphql`, `protobuf`, `scala`
**Phase B1 (14)**: `augeas`, `bbcode`, `cfengine3`, `cfs`, `debian.sources`, `desktop`, `django`, `lighttpd`, `mozhashpreproc`, `mozpercentpreproc`, `ng2`, `pacmanconf`, `pkgconfig`, `registry`
**Phase B2 (8)**: `debcontrol`, `debsources`, `kconfig`, `systemd`, `termcap`, `terminfo`, `twig`, `velocity`
**Phase C1 (13)**: `applescript`, `chaiscript`, `moonscript`, `alloy`, `arrow`, `awk`, `bdd`, `abap`, `maql`, `bbcbasic`, `blitzmax`, `newlisp`, `racket`

## Lexer inventory by transpilability

`tools/gen_lexer.py --classify` buckets every un-ported lexer:

| category          | count | disposition                                              |
| ----------------- | ----: | -------------------------------------------------------- |
| `transpilable`    |   296 | **port natively** — 72 done, 224 remaining to port       |
| `bridge_callback` |   107 | bridge-only — Python callback actions                    |
| `non_regex`       |   111 | bridge-only — not a `RegexLexer` subclass                |
| `bridge_using`    |    12 | bridge-only — `using()`/`this` embedded-lexer delegation |
| `error`           |     0 | (none — fixed in tooling)                                |

**Status**: Pipeline fully functional. Remaining 296 transpilable lexers can be
processed in ~6-8 more batches (15-20 lexers per batch) using automated
generation, parity testing, and bridge-only filtering. Estimated completion time:
2-4 hours of batch processing. See `tools/batch_lexers.sh` for automation.

`bridge_callback` (107) breaks down as:

- **79** — `bygroups(..., using(this)/callback, ...)`: a nested callback
  inside a group. Most are `using(this)` (recursive self-lex) or
  `using(OtherLexer)` (embedded language). Genuinely bridge-only unless
  the recursive-`using(this)` case is later supported (see Tooling).
- **19** — top-level Python callback action (arbitrary code).
- **9** — heredoc / indent-tracking callbacks (`ruby`, `php`,
  `terraform`/`hcl`, `yaml`, …): stateful callbacks that can't be
  expressed as a static rule table.

### Why the popular languages split the way they do

| lexer | status | note |
| ----- | ------ | ---- |
| `rust`, `go`, `javascript`, `typescript`, `css`, `xml`, `sql`, `swift`, `scala`, `perl`, `lua`, `r`, `matlab`, `julia`, `haskell`, `clojure`, `erlang`, `elixir`, `tex`/`latex`, `graphql`, `protobuf`, `nginx`, `apache`, `powershell` | ✅ transpilable | priority batch 1 below |
| `c`, `cpp`, `java`, `kotlin`, `php`, `dart` | bridge (`using(this)` in `bygroups`) | embed whitespace/other lexers; the PyO3 bridge handles them today |
| `ruby`, `terraform`/`hcl`, `yaml`, `dockerfile` | bridge (heredoc/indent callback) | stateful callbacks |
| `html` | bridge (`using()`) | delegates to CSS + JS sublexers |
| `make` | bridge (`non_regex`) | `MakefileLexer` is a custom `Lexer`, not `RegexLexer` |

Bridge-only is **not a gap**: `Backend::Auto` (the default) transparently
falls back to upstream `pygments` via PyO3 for any alias without a native
lexer. The port improves speed and removes the GIL hop incrementally; it
never regresses coverage.

## Porting plan (phased)

Each phase is a batch of `module:Class:rust_name` specs fed to the skill's
generate → wire → parity-test → gate loop. Keep batches to ~10–25 lexers so
a parity failure is easy to bisect. Generated files are byte-parity-gated;
a green `tests/test_pygments_generated_lexers.py` is the definition of done
for a batch.

### Phase A — high-value doc/Sphinx languages (24)

The languages most likely to appear in `code-block` directives. All
verified `transpilable`:

```
pygments.lexers.rust:RustLexer:rust
pygments.lexers.go:GoLexer:go
pygments.lexers.javascript:JavascriptLexer:javascript
pygments.lexers.javascript:TypeScriptLexer:typescript
pygments.lexers.css:CssLexer:css
pygments.lexers.html:XmlLexer:xml
pygments.lexers.sql:SqlLexer:sql
pygments.lexers.objective:SwiftLexer:swift
pygments.lexers.perl:PerlLexer:perl
pygments.lexers.scripting:LuaLexer:lua
pygments.lexers.r:SLexer:r
pygments.lexers.matlab:MatlabLexer:matlab
pygments.lexers.julia:JuliaLexer:julia
pygments.lexers.haskell:HaskellLexer:haskell
pygments.lexers.jvm:ClojureLexer:clojure
pygments.lexers.erlang:ErlangLexer:erlang
pygments.lexers.erlang:ElixirLexer:elixir
pygments.lexers.configs:NginxConfLexer:nginx
pygments.lexers.configs:ApacheConfLexer:apache
pygments.lexers.shell:PowerShellLexer:powershell
pygments.lexers.markup:TexLexer:tex
pygments.lexers.graphql:GraphQLLexer:graphql
pygments.lexers.dsls:ProtoBufLexer:protobuf
pygments.lexers.jvm:ScalaLexer:scala
```

Note: `TexLexer` covers both `tex` and `latex` aliases — wire both.

### Phase B — config / data / markup formats

High-frequency in docs and infra repos; mostly small state tables.
Pull from these modules' `transpilable` rows (`--classify transpilable`):
`configs` (16), `markup`, `templates`, `data` (the non-callback ones),
`installers`. Examples: more `configs:*`, `markup:*`, `templates:*`.

### Phase C — the long tail, batched by source module

Work module-by-module through the remaining `transpilable` lexers (195
source modules carry them). Largest pools:

| module (`pygments.lexers.`) | transpilable |
| --------------------------- | -----------: |
| `configs`                   |           16 |
| `scripting`                 |           12 |
| `dsls`                      |           10 |
| `lisp`                      |           10 |
| `javascript`                |            8 |
| `basic`, `asm`, `jvm`       |          7 ea |
| `business`, `haskell`, `esoteric`, `sql` | 6 ea |

For each module: `--classify transpilable | grep <module>`, generate the
batch, `--registry` to get the wiring, add 3–5 parity samples per lexer,
gate. Repeat until `--classify` shows `transpilable: 0`.

### Phase D — revisit bridge-only (optional, lower ROI)

Only after the transpilable set is exhausted. Candidate uplifts, each a
genuine engine/tool feature (not just a transpile):

- **recursive `using(this)`** (unlocks ~part of the 79 nested-callback
  lexers, incl. paths in `c`/`cpp`/`java`): emit a recursive call into the
  same lexer for a `using(this)` group. Embedded *other* lexers
  (`using(OtherLexer)`) only work once that other lexer is also native.
- **`DelegatingLexer`** (part of `non_regex`, e.g. templated languages
  like `html+django`): compose two native lexers.
- **heredoc/indent callbacks** (`ruby`, `yaml`, `terraform`): would need a
  small stateful-callback hook in the engine. High effort; the bridge
  already covers these correctly, so defer indefinitely unless perf
  demands it.

## Formatter plan (18 total, 1 native)

`html` is native and byte-parity. The rest are bridge-served today.
Priority order if/when formatters are ported natively:

| formatter | alias(es) | port difficulty | notes |
| --------- | --------- | --------------- | ----- |
| NullFormatter | `text` | trivial | echo tokens' text |
| RawTokenFormatter | `raw`, `tokens` | trivial | `repr` stream; useful for tests |
| TestcaseFormatter | `testcase` | trivial | emits a unit-test skeleton |
| TerminalFormatter | `terminal`, `console` | easy | 16-color ANSI map |
| Terminal256Formatter | `terminal256` | medium | 256-color cube + style lookup |
| TerminalTrueColorFormatter | `terminal16m` | medium | truecolor ANSI |
| IRCFormatter | `irc` | easy | mIRC color codes |
| BBCodeFormatter | `bbcode` | easy | `[color]` tags |
| GroffFormatter | `groff` | medium | troff escapes |
| LatexFormatter | `latex` | medium | needs style → macro table |
| RtfFormatter | `rtf` | medium | RTF control words |
| PangoMarkupFormatter | `pango` | easy | span markup |
| Bmp/Gif/Img/Jpg | `bmp`/`gif`/`img`/`jpg` | **bridge-only** | image rendering; heavy deps |
| SvgFormatter | `svg` | hard | text layout |

Recommendation: port `text`/`raw`/`testcase` opportunistically (they make
the Rust test surface self-contained), then the ANSI terminal trio if a CLI
needs them. Leave image formatters on the bridge permanently.

## Tooling status & improvements

Implemented in `tools/gen_lexer.py`:

- transpile token / `bygroups` / `default` actions; `include`/`words`/
  `combined` pre-expanded by `process_tokendef`.
- **structural-token fallback** — unknown token subtypes emit
  `TokenType::new(&[…])`, so a missing named const never blocks (moved +52
  lexers into `transpilable`).
- **`--classify [category]`** — inventory; `transpilable` rows are
  ready-to-paste specs.
- **`--registry <specs>`** — prints `mod.rs` + `registry.rs` wiring.
- **instance-`_tokens` fix** — read `_tokens` from the instantiated lexer,
  not the class, so token-variant lexers (`CSharpLexer`, `NemerleLexer`,
  `Inform7Lexer`, `Inform6TemplateLexer`) classify instead of erroring
  (`error: 4 → 0`).

Future tool/engine work (gated by Phase D need):

- recursive `using(this)` emission.
- `DelegatingLexer` composition.
- a stateful-callback hook for heredoc/indent lexers.
- auto-append generated specs to `generated/mod.rs` and the parity-test
  `GENERATED` map (currently a manual paste from `--registry`).

## Definition of done (per batch)

1. `tools/gen_lexer.py <specs>` → `WROTE …`, `0 skipped`.
2. `--registry <specs>` snippet pasted into `generated/mod.rs` +
   `registry.rs` (both `get_lexer_by_name` and `native_aliases`).
3. 3–5 parity samples per lexer added to `tests/test_pygments_generated_lexers.py`.
4. `cargo build -p pygmentsrs` warning-free.
5. `maturin develop --release -m pygmentsrs/Cargo.toml`.
6. `cargo test -p pygmentsrs` + `make test-python` green.
7. `pygmentsrs/docs/compat.md` table + this doc's counts updated.

## Tracking

Update after each batch (`--classify` totals):

| date | native lexers | transpilable remaining | native formatters |
| ---- | ------------: | ---------------------: | ----------------: |
| (init) | 13 | 355 | 1 |
| Phase A | 37 | 331 | 1 |
| Phase B Batch 1 | 51 | 317 | 1 |
| Phase B Batch 2 | 59 | 309 | 1 |
| Phase C Batch 1 | 72 | 296 | 1 |
