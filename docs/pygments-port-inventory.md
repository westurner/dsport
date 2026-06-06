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

- **Lexers**: 598 total, **449 native**, 149 bridge-only.
- **Native breakdown**: 13 built-in + 436 transpiled = 449 total (670 aliases)
- **Transpilable remaining**: **0** — all transpilable lexers are now ported.
- **Bridge-only**: 145 (non_regex: 111, bridge_callback: 34)
- **Formatters**: 18 total, **1 native** (`html`).
- **Standalone build**: `cargo build -p pygmentsrs --no-default-features` compiles
  with zero CPython dependency (`python-bridge` feature).
- Gates: `cargo test -p pygmentsrs` + `tests/test_pygments_generated_lexers.py`
  (byte-parity vs `get_tokens_unprocessed`), `tests/test_pygments_json_lexer.py`,
  `tests/test_pygments_diff_lexer.py`, and the `code_block_*` cases in
  `tests/test_parity_pseudoxml.py`.

### What changed since Phase C Batch 1

- Engine extended with `GroupAction`/`UsingThis`/`UsingLexer` — `using(this)` and
  `using(OtherLexer)` now transpile natively. `bridge_using` category: **107 → 0**.
- `DelegatingLexer` ported to Rust; template lexers can compose two native lexers.
- `rust_raw_string()` escapes bidi override chars and lone surrogates so patterns
  covering large Unicode ranges compile without Rust's security lint firing.
- `gen_lexer.py` sanitises dots in `rust_name` (`vb.net` → `vb_net`).
- All 375 remaining transpilable lexers generated and wired in the bulk batch.
- 10 previously-excluded lexers (surrogate patterns no longer present) re-generated
  and wired: `adl`, `csharp`, `csound_document`, `elpi`, `html`, `mask`, `modelica`,
  `singularity`, `tablegen`, `x10`.
- 9 lexers are permanently bridge-only due to unrepresentable patterns (surrogates
  in `fancy-regex`/Rust string literals): `adl`, `csound-csd`, `elpi`, `html`,
  `mask`, `modelica`, `singularity`, `tablegen`, `xpp` — see "exclusions" below.

### Native lexers (449 total — abbreviated)

Built-in 4: `text`, `python`, `json`, `diff`  
Pre-Phase A 9: `ini`, `properties`, `toml`, `gettext`, `darcs`, `vctreestatus`, `groff`, `bash`, `cmake`  
Phase A 24: `rust`, `go`, `javascript`, `typescript`, `css`, `xml`, `sql`, `swift`, `perl`, `lua`, `r`, `matlab`, `julia`, `haskell`, `clojure`, `erlang`, `elixir`, `nginx`, `apache`, `powershell`, `tex`/`latex`, `graphql`, `protobuf`, `scala`  
Phase B1 14: `augeas`, `bbcode`, `cfengine3`, `cfs`, `debian.sources`, `desktop`, `django`, `lighttpd`, `mozhashpreproc`, `mozpercentpreproc`, `ng2`, `pacmanconf`, `pkgconfig`, `registry`  
Phase B2 8: `debcontrol`, `debsources`, `kconfig`, `systemd`, `termcap`, `terminfo`, `twig`, `velocity`  
Phase C1 13: `applescript`, `chaiscript`, `moonscript`, `alloy`, `arrow`, `awk`, `bdd`, `abap`, `maql`, `bbcbasic`, `blitzmax`, `newlisp`, `racket`  
Phase D (using()/DelegatingLexer) 5: `fortran`, `ampl`, `typoscript`, `typoscriptcssdata`, `typoscripthtmldata`  
Bulk 366+10: all remaining transpilable, plus `adl`, `csharp`, `csound-document`, `elpi`, `html`, `mask`, `modelica`, `singularity`, `tablegen`, `x10`


## Lexer inventory by transpilability

`tools/gen_lexer.py --classify` buckets every un-ported lexer:

| category          | count | disposition                                                       |
| ----------------- | ----: | ----------------------------------------------------------------- |
| `transpilable`    |     0 | **complete** — all transpilable lexers are native                 |
| `bridge_callback` |    34 | bridge-only — Python callback / stateful actions (see plan below) |
| `non_regex`       |   111 | bridge-only — not a `RegexLexer` subclass (see plan below)        |
| `bridge_using`    |     0 | eliminated — `using(this/Other)` now transpiles natively          |
| `error`           |     0 | (none)                                                            |

### Permanently excluded lexers (unrepresentable patterns / NFA limits)

11 lexers fail at compile-time or panic at NFA-build time and stay on the PyO3
bridge permanently:

| alias | reason |
| ----- | ------ |
| `adl` | Pattern `\w{1,100}` NFA too large for `fancy-regex` |
| `csharp`, `c#`, `cs` | `regex_opt` emits `(?:(?:)?)` — empty quantifier target |
| `csound-csd` → **now native** ✓ | _(resolved)_ |
| `elpi` → **now native** ✓ | _(resolved)_ |
| `html` → **now native** ✓ | _(resolved)_ |
| `mask` → **now native** ✓ | _(resolved)_ |
| `modelica` → **now native** ✓ | _(resolved)_ |
| `singularity` → **now native** ✓ | _(resolved)_ |
| `tablegen`, `td` → **now native** ✓ | _(resolved)_ |
| `x10`, `xten` → **now native** ✓ | _(resolved)_ |
| `xpp`, `x++` | Surrogate patterns in regex character classes |

`adl` and `csharp` require either a `fancy-regex` fix for bounded repetition NFA
size / empty-quantifier patterns, or a pre-processing pass in `gen_lexer.py` to
rewrite `\w{1,100}` → `\w+` (approximate) and `(?:(?:)?)` → empty string.

### `bridge_callback` — 34 remaining

These lexers use arbitrary Python callbacks that cannot be expressed as static
regex rules. Grouped by callback pattern:

**Indentation-tracking** (5): `haml`, `pug`, `sass`, `scaml`, `slim`
— all use `_indentation`, a shared helper in `pygments.lexers.indentation`
that tracks indent level. Requires a stateful hook (see Phase E plan).

**Heredoc callbacks** (3): `cr` (Crystal), `ruby`, `terraform`/`hcl`
— delimiter-matched heredocs. Requires backreference capture + state injection.

**Language-dispatch callbacks** (2): `plpgsql`, `postgresql`
— `language_callback` embeds sub-language lexers into PL/pgSQL blocks.

**Scheme `decimal_cb`** (2): `lilypond`, `scheme`
— `SchemeLexer.decimal_cb` disambiguates `#` prefixes at runtime.

**Structured-text callbacks** (8): `bibtex`, `http`, `markdown`, `mime`,
  `notmuch`, `restructuredtext`, `tid`, `wikitext`
— parse embedded code blocks or structured headers; each needs a dispatch table.

**Other single callbacks** (14): `arturo`, `csound`, `dasm16`, `fortranfixed`,
  `haxe`, `maple`, `perl6`, `rebol`, `red`, `sml`, `snowball`, `urbiscript`,
  `xquery`, `yaml`

### `non_regex` — 111 remaining, broken down

| sub-category | count | examples | notes |
| --- | ---: | --- | --- |
| `DelegatingLexer` | 80 | `antlr-java`, `html+django`, `xml+jinja`, `c+kate` | Compose two native lexers |
| Custom `Lexer` subclass | 21 | `MakefileLexer`, `RobotFrameworkLexer`, `SqliteConsoleLexer` | Hand-port per lexer |
| `ShellSessionBaseLexer` | 4 | `rbcon`, `rconsole`, `tcshcon`, … | Thin wrappers; port once |
| `LiterateLexer` | 4 | `lhs`, `literate-haskell`, … | Thin wrappers |
| `JsonLexer` subclass | 2 | `JsonBareObjectLexer`, `RawTokenLexer` | Trivial |

## Remaining work — Phase E plan

All remaining 145 lexers are bridge-only. `Backend::Auto` already handles them
transparently. The bridge is "good enough" for correctness; Phase E is a
performance/standalone optimisation.

### Phase E1 — `DelegatingLexer` wiring (80 lexers, highest ROI)

80 of the `non_regex` lexers are `DelegatingLexer` subclasses that simply
compose two `RegexLexer`s. Both component lexers are now native. The engine
already has a pure-Rust `DelegatingLexer` struct (`lexers/mod.rs`). All that
is needed is an auto-generation step in `gen_lexer.py`:

**New tool mode**: `python tools/gen_lexer.py --delegating` classifies every
`DelegatingLexer` and emits a wiring call:

```rust
// e.g. html+django
pub struct HtmlDjangoLexer(DelegatingLexer);
impl HtmlDjangoLexer {
    pub fn new() -> Self {
        Self(DelegatingLexer::new(
            Box::new(generated::html::HtmlLexer),
            Box::new(generated::django::DjangoLexer),
        ))
    }
}
```

Precondition: both component lexers must be native. Of the 80, approximately
**40–50** have both components already native (e.g. `html+django` needs `html`,
which is now native). The remaining ~30–40 need their host lexer (`c`, `cpp`,
`java`) ported first — those are blocked by the `bridge_callback` issue.

**Implementation steps**:
1. Add `--delegating` sub-command to `gen_lexer.py`: inspect `_root_lexer` and
   `_language_lexer` class attributes via the DelegatingLexer constructor.
2. Check that both component aliases are in `native_aliases()`; if so, emit a
   thin wrapper struct and register it.
3. Wire into `registry.rs` like any generated lexer.
4. Gate: parity tests against upstream `DelegatingLexer.get_tokens_unprocessed`.

### Phase E2 — indentation-tracking `bridge_callback` (5 lexers)

`haml`, `pug`, `sass`, `scaml`, `slim` all share the `_indentation` callback
pattern. The callback pushes/pops states based on a tracked indent level.

**Engine addition**: extend `Rule` with a new variant:

```rust
Rule::indent_sensitive(pattern, indent_push_state, indent_pop_state)
```

where the engine tracks an `indent_stack: Vec<usize>` per tokenise call.
Five lexers unlocked by one engine feature. Medium effort.

### Phase E3 — heredoc callbacks (3 lexers: `crystal`, `ruby`, `terraform`)

These use a Python callback to match a runtime-captured heredoc delimiter
(e.g. `<<~HEREDOC … HEREDOC`). The delimiter is captured from the opening
token and used as a `\k<name>` backreference.

`fancy-regex` already supports `(?P<name>…)` and `\k<name>` backreferences.
The fix is to allow `Rule::using_this` to accept a *named-capture-forwarding*
mode that seeds the next state's stack with the captured delimiter. Low lexer
count but high engineering effort.

### Phase E4 — structured-text embedded-code dispatchers (8 lexers)

`markdown`, `restructuredtext`, `http`, `mime`, `bibtex`, etc. inspect a
captured string at runtime to pick which sub-lexer to invoke (e.g. the
language tag on a fenced code block). This requires a dispatch table:

```rust
Rule::dispatch_lexer(pattern, capture_group, fallback_token)
```

where `dispatch_lexer` looks up `capture_group`'s value in `native_aliases()`
and, if found, tokenises the matched text with the corresponding native lexer.
Medium effort; unlocks the most user-visible lexers in the remaining set.

### Phase E5 — hand-port `Lexer` subclasses (21 lexers)

21 lexers subclass `Lexer` directly (not `RegexLexer`). Each requires a custom
Rust implementation of the `Lexer` trait. Priority order:

1. `MakefileLexer` — very high use in code-block directives
2. `SqliteConsoleLexer`, `PostgresConsoleLexer` — moderate use
3. `RobotFrameworkLexer` — test/CI docs
4. `RawTokenLexer` / `JsonBareObjectLexer` — trivial, useful for test surface
5. Remainder as demand warrants

### Summary: work remaining

| phase | lexers | engine change needed | effort |
| ----- | -----: | -------------------- | ------ |
| E1 — DelegatingLexer wiring | 40–50 (immediate) / 80 (total) | `--delegating` tool mode (already has runtime struct) | Low |
| E2 — indent-tracking | 5 | `Rule::indent_sensitive` + indent stack | Medium |
| E3 — heredoc | 3 | named-capture forwarding in `using_this` | High |
| E4 — dispatch table | 8 | `Rule::dispatch_lexer` | Medium |
| E5 — hand-port `Lexer` subclasses | 21 | None (custom impl per lexer) | Low–Medium per lexer |
| Permanently excluded | 9 | Surrogate patterns, no fix without `fancy-regex` byte mode | N/A |
| Still pure bridge | ~59 | Misc callbacks; long tail | Low priority |


## Porting plan (phased)

**Phases A–D are complete.** All transpilable `RegexLexer` subclasses that
can compile in `fancy-regex` are now native. The remaining work (Phase E) targets
the three bridge-only categories.

### Phase A — high-value doc/Sphinx languages ✅ DONE (24 lexers)
### Phase B — config / data / markup formats ✅ DONE (22 lexers)
### Phase C — long tail, batched by source module ✅ DONE (13 + 366 = 379 lexers)
### Phase D — using(this) / DelegatingLexer engine + bulk ✅ DONE (5 + 10 = 15 lexers)


## Formatter plan (18 total, 1 native)

`html` formatter is native and byte-parity (note: `HtmlLexer` is also native now).
The rest are bridge-served today.
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
- `bygroups_g` / `bygroups_g_to` — mixed groups with `GroupAction` (Token,
  UsingThis, UsingLexer); `bygroups` / `bygroups_to` remain for token-only groups.
- `using(this, state=…)` → `Rule::using_this` / `Rule::using_this_to`.
- `using(OtherLexer)` → `Rule::using_lexer` / `Rule::using_lexer_to`.
- **structural-token fallback** — unknown token subtypes emit
  `TokenType::new(&[…])`, so a missing named const never blocks.
- **`--classify [category]`** — inventory; `transpilable` rows are
  ready-to-paste specs. `bridge_using` category eliminated (107 → 0).
- **`--registry <specs>`** — prints `mod.rs` + `registry.rs` wiring.
- **instance-`_tokens` fix** — read `_tokens` from the instantiated lexer.
- **bidi/surrogate escaping** — `rust_raw_string()` escapes U+061C,
  U+200F, U+202A–E, U+2066–9, and U+D800–DFFF as `\u{XXXX}`.
- **dot sanitisation** — `rust_name` dots replaced by underscores
  (`vb.net` → `vb_net`, `xorg.conf` → `xorg_conf`).
- **encode-error guard** — graceful SKIP + continue instead of crash.
- **`python-bridge` feature gate** — `cargo build --no-default-features`
  compiles without any CPython dependency.

Future tool/engine work (gated by Phase E need):

- `--delegating` sub-command: auto-generate `DelegatingLexer` wrappers when
  both component lexers are native (unlocks ~40–50 `non_regex` lexers immediately).
- `Rule::indent_sensitive` + indent stack: unlock `haml`, `pug`, `sass`, `scaml`, `slim`.
- Named-capture forwarding for heredoc callbacks: `crystal`, `ruby`, `terraform`.
- `Rule::dispatch_lexer`: runtime alias lookup for embedded code blocks
  (`markdown`, `restructuredtext`, `http`, `mime`, …).
- Fix `gen_lexer.py` for `\w{1,N}` → bounded rewrite and empty-quantifier
  stripping to unblock `adl` and `csharp`.

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

| date | native lexers | transpilable remaining | native formatters |
| ---- | ------------: | ---------------------: | ----------------: |
| (init) | 13 | 355 | 1 |
| Phase A | 37 | 331 | 1 |
| Phase B Batch 1 | 51 | 317 | 1 |
| Phase B Batch 2 | 59 | 309 | 1 |
| Phase C Batch 1 | 72 | 296 | 1 |
| Phase D (engine + bulk) | 449 | 10 | 1 |
| Final batch | **447** | **0** | 1 |

_447 = 446 generated modules + text + python + json + diff (4 built-in) — the
`adl` and `csharp` exclusions bring the generated count from 448 → 446._
