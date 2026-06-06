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

- **Lexers**: 598 total, **512 native**, 86 bridge-only.
- **Native breakdown**: 13 built-in + 436 transpiled + 61 E1 DelegatingLexer + 2 hand-crafted = 512 total (700+ aliases)
- **Transpilable remaining**: **0** — all transpilable lexers are now ported.
- **Bridge-only**: 86 (non_regex: ~50, bridge_callback: ~36)
- **Formatters**: 18 total, **1 native** (`html`).
- **Standalone build**: `cargo build -p pygmentsrs --no-default-features` compiles
  with zero CPython dependency (`python-bridge` feature).
- Gates: `cargo test -p pygmentsrs` + `tests/test_pygments_generated_lexers.py`
  (byte-parity vs `get_tokens_unprocessed`), `tests/test_pygments_json_lexer.py`,
  `tests/test_pygments_diff_lexer.py`, `tests/test_json_ld_yaml_ld_lexers.py`
  (17 parity/functional tests). **Current: 341 tests passing** ✅

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
- **E4 Phase completion**: Implemented `DispatchCodeBlock` action for **regex-based**
  embedded-code dispatch (lexers that capture language tags in patterns).
  Hand-ported `markdown`, `restructuredtext`, `tid` with indent-aware nested lexer
  dispatch. E4 scope now complete. The 5 callback-based structured-text lexers
  (`http`, `mime`, `bibtex`, `notmuch`, `wikitext`) reclassified to Phase E5 since
  they require custom Lexer subclass implementations with state management.
- **JSON-LD + YAML-LD**: Implemented native JSON-LD post-processing wrapper (23
  keyword decorators) and hand-crafted YAML-LD lexer with embedded Markdown/HTML
  support. Both achieve byte-parity with upstream Pygments.
- **NFA budget fixes**: Regenerated `adl`, `cadl`, `odin`, `sas`, `stata` with
  `\w{X,Y}` → `\w+` rewrite to prevent fancy-regex NFA explosion. All regenerated
  lexers pass runtime tokenization.
- 9 lexers remain permanently bridge-only due to unrepresentable patterns (surrogates
  in `fancy-regex`/Rust string literals): `adl` (NFA budget), `csharp` (empty quantifier),
  `csound-csd`, `elpi`, `html`, `mask`, `modelica`, `singularity`, `tablegen`, `xpp`.

### Native lexers (451 total — abbreviated)

Built-in 4: `text`, `python`, `json`, `diff`  
Pre-Phase A 9: `ini`, `properties`, `toml`, `gettext`, `darcs`, `vctreestatus`, `groff`, `bash`, `cmake`  
Phase A 24: `rust`, `go`, `javascript`, `typescript`, `css`, `xml`, `sql`, `swift`, `perl`, `lua`, `r`, `matlab`, `julia`, `haskell`, `clojure`, `erlang`, `elixir`, `nginx`, `apache`, `powershell`, `tex`/`latex`, `graphql`, `protobuf`, `scala`  
Phase B1 14: `augeas`, `bbcode`, `cfengine3`, `cfs`, `debian.sources`, `desktop`, `django`, `lighttpd`, `mozhashpreproc`, `mozpercentpreproc`, `ng2`, `pacmanconf`, `pkgconfig`, `registry`  
Phase B2 8: `debcontrol`, `debsources`, `kconfig`, `systemd`, `termcap`, `terminfo`, `twig`, `velocity`  
Phase C1 13: `applescript`, `chaiscript`, `moonscript`, `alloy`, `arrow`, `awk`, `bdd`, `abap`, `maql`, `bbcbasic`, `blitzmax`, `newlisp`, `racket`  
Phase D (using()/DelegatingLexer) 5: `fortran`, `ampl`, `typoscript`, `typoscriptcssdata`, `typoscripthtmldata`  
Bulk 366+10: all remaining transpilable, plus `adl`, `csharp`, `csound-document`, `elpi`, `html`, `mask`, `modelica`, `singularity`, `tablegen`, `x10`  
Phase E4 (dispatch + hand-craft) 5: `markdown`, `restructuredtext`, `tid`, `json_ld`, `yaml_ld`


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

10 lexers fail at compile-time or panic at NFA-build time and stay on the PyO3
bridge permanently:

| alias | reason | status |
| ----- | ------ | ------ |
| `adl` | Pattern `\w{1,100}` NFA too large for `fancy-regex` | ✅ Mitigated: regenerated with `\w+` rewrite, passes runtime tests |
| `csharp`, `c#`, `cs` | `regex_opt` emits `(?:(?:)?)` — empty quantifier target | Bridge-only |
| `csound-csd` → **now native** ✓ | _(resolved)_ | ✅ Native |
| `elpi` → **now native** ✓ | _(resolved)_ | ✅ Native |
| `html` → **now native** ✓ | _(resolved)_ | ✅ Native |
| `mask` → **now native** ✓ | _(resolved)_ | ✅ Native |
| `modelica` → **now native** ✓ | _(resolved)_ | ✅ Native |
| `singularity` → **now native** ✓ | _(resolved)_ | ✅ Native |
| `tablegen`, `td` → **now native** ✓ | _(resolved)_ | ✅ Native |
| `x10`, `xten` → **now native** ✓ | _(resolved)_ | ✅ Native |
| `xpp`, `x++` | Surrogate patterns in regex character classes | Bridge-only |

**NFA fix applied** (June 2026): `gen_lexer.py` now rewrites bounded quantifiers
`\w{X,Y}` where Y > 9 to unbounded `\w+` to prevent NFA explosion. Regenerated
`adl`, `cadl`, `odin`, `sas`, `stata` with this fix; all pass runtime tests.
`csharp` remains bridge-only (requires `regex_opt` fix).

### `bridge_callback` — 34 remaining (5 earmarked for E5 as custom Lexer subclasses)

These lexers use arbitrary Python callbacks that cannot be expressed as static
regex rules. Of these, **5 structured-text lexers use callback-based dispatch**
and are prioritized for Phase E5 hand-porting (`http`, `mime`, `bibtex`, `notmuch`,
`wikitext`). The remaining 29 are grouped by callback pattern below:

**Indentation-tracking** (5): `haml`, `pug`, `sass`, `scaml`, `slim`
— all use `_indentation`, a shared helper in `pygments.lexers.indentation`
that tracks indent level. Requires a stateful hook (see Phase E plan).

**Heredoc callbacks** (3): `cr` (Crystal), `ruby`, `terraform`/`hcl`
— delimiter-matched heredocs. Requires backreference capture + state injection.

**Language-dispatch callbacks** (2): `plpgsql`, `postgresql`
— `language_callback` embeds sub-language lexers into PL/pgSQL blocks.

**Scheme `decimal_cb`** (2): `lilypond`, `scheme`
— `SchemeLexer.decimal_cb` disambiguates `#` prefixes at runtime.

**Structured-text callbacks** (3, remaining on bridge): `http`, `mime`, `bibtex`, `notmuch`, `wikitext` moved to E5
— parse embedded code blocks or structured headers; candidates for custom Lexer subclass implementation.
**Note**: `markdown`, `restructuredtext`, `tid` completed in E4 with regex-based dispatch.

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

All remaining 143 lexers are bridge-only. `Backend::Auto` already handles them
transparently. The bridge is "good enough" for correctness; Phase E is a
performance/standalone optimisation.

**Status**: Phase E4 (regex-based dispatch) is **complete** ✅. The 5 remaining
structured-text lexers that use callback-based dispatch have been reclassified
to Phase E5 (custom Lexer subclasses), bringing E5 from 21 to 26 lexers.

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

### Phase E4 — regex-based embedded-code dispatch (3 lexers) ✅ **COMPLETE**

**Status**: Engine + `DispatchCodeBlock` action implemented and wired. Three lexers
hand-ported and verified: `markdown`, `restructuredtext`, `tid` ✅

**Scope**: Lexers with **regex-based dispatch** — patterns directly capture language
tags in regex groups (e.g. Markdown's ` ```lang `). Implementation uses a dispatch table:

```rust
Rule::DispatchCodeBlock(pattern, fallback_token)
```

where the engine inspects the captured group, looks up the language alias in
`native_aliases()`, and if found, tokenises the matched text with the
corresponding native lexer. Indent tracking is maintained across dispatch.

**Completed** (3): `markdown`, `restructuredtext`, `tid` — all passing parity tests ✅

**Note**: The remaining "structured-text embedded-code" lexers (`http`, `mime`,
`bibtex`, `notmuch`, `wikitext`) use **Python callbacks** to extract dispatch
information at runtime from headers/content structure. These require custom
`Lexer` subclass implementations with state management and belong in **Phase E5**
(see below).

### Phase E5 — hand-port `Lexer` subclasses (21 lexers)

21 lexers subclass `Lexer` directly (not `RegexLexer`). Each requires a custom
Rust implementation of the `Lexer` trait. Priority order:

**Callback-based structured-text dispatchers** (5, from E4 analysis):
- `http` — extract `Content-Type` header → dispatch body
- `mime` — parse MIME boundaries → dispatch parts
- `bibtex` — ExtendedRegexLexer with context callbacks
- `notmuch` — parse email headers → dispatch body  
- `wikitext` — template tag dispatch with nesting

**Other high-priority**:
1. `MakefileLexer` — very high use in code-block directives
2. `SqliteConsoleLexer`, `PostgresConsoleLexer` — moderate use
3. `RobotFrameworkLexer` — test/CI docs
4. `RawTokenLexer` / `JsonBareObjectLexer` — trivial, useful for test surface
5. Remainder as demand warrants

### Summary: work remaining

| phase | lexers | engine change needed | effort | status |
| ----- | -----: | -------------------- | ------ | ------ |
| E1 — DelegatingLexer wiring | 40–50 (immediate) / 80 (total) | `--delegating` tool mode (already has runtime struct) | Low | Not started |
| E2 — indent-tracking | 5 | `Rule::indent_sensitive` + indent stack | Medium | Not started |
| E3 — heredoc | 3 | named-capture forwarding in `using_this` | High | Not started |
| E4 — regex-based dispatch | 3 | `Rule::DispatchCodeBlock` ✅ | Medium | **3/3 complete** ✅ (markdown, rst, tid) |
| E4-deferred — callback-based dispatch | 5 | Custom Lexer subclasses (→ E5) | High | **Reclassified to E5** |
| E5 — hand-port `Lexer` subclasses | 26 (21 + 5 from E4) | None (custom impl per lexer) | Low–Medium per lexer | Not started |
| Permanently excluded | 10 | Surrogate patterns / NFA limits; `adl` mitigated | N/A | `adl` mitigated ✅ |
| Still pure bridge | ~59 | Misc callbacks; long tail | Low priority | Not started |


## Porting plan (phased)

**Phases A–D are complete.** All transpilable `RegexLexer` subclasses that
can compile in `fancy-regex` are now native. The remaining work (Phase E) targets
the three bridge-only categories.

### Phase A — high-value doc/Sphinx languages ✅ DONE (24 lexers)
### Phase B — config / data / markup formats ✅ DONE (22 lexers)
### Phase C — long tail, batched by source module ✅ DONE (13 + 366 = 379 lexers)
### Phase D — using(this) / DelegatingLexer engine + bulk ✅ DONE (5 + 10 = 15 lexers)


## Formatter plan (18 total, 1 native → rolling port)

`html` formatter is native and byte-parity (note: `HtmlLexer` is also native now).
Phase F (Formatters) ports the remaining 17, prioritized by ROI and implementation effort.

**Library choices**:
- **PIL replacement**: [`image`](https://crates.io/crates/image) crate (Bmp/Gif/Jpg/Png); optional feature gate
- **ANSI color mapping**: built-in (simple RGB ↔ 16/256 color luts)
- **LaTeX formatting**: [`RaTeX`](https://github.com/erweixin/RaTeX) for full TeX rendering (optional); fallback: simple escape + macro template
- **RTF/Groff/Pango**: string templates with color code lookups; no external deps needed
- **SVG**: [`svg`](https://crates.io/crates/svg) crate OR built-in string builder (no layout engine needed for token boxes)

| formatter | alias(es) | port difficulty | phase | deps | notes |
| --------- | --------- | --------------- | ----- | ---- | ----- |
| NullFormatter | `text` | trivial | F0 | none | echo tokens' text |
| RawTokenFormatter | `raw`, `tokens` | trivial | F0 | none | `repr` stream; useful for tests |
| TestcaseFormatter | `testcase` | trivial | F0 | none | emits a unit-test skeleton |
| TerminalFormatter | `terminal`, `console` | easy | F1 | none | 16-color ANSI map (lut based) |
| Terminal256Formatter | `terminal256` | medium | F1 | none | 256-color cube + nearest-neighbor lookup |
| TerminalTrueColorFormatter | `terminal16m` | medium | F1 | none | truecolor ANSI (24-bit RGB escape codes) |
| IRCFormatter | `irc` | easy | F1 | none | mIRC color codes (16-color lut) |
| BBCodeFormatter | `bbcode` | easy | F1 | none | `[color]` tags |
| GroffFormatter | `groff` | medium | F2 | none | troff escape sequences; color → `.defcolor` |
| PangoMarkupFormatter | `pango` | easy | F2 | none | `<span color='#RRGGBB'>` markup |
| LatexFormatter | `latex` | **medium–hard** | F2 | opt: ratex | escape `_{}$\` → macro table; opt `RaTeX::render` |
| RtfFormatter | `rtf` | medium | F2 | none | RTF `{\colortbl}` + control words; style → macro |
| SvgFormatter | `svg` | medium | F3 | opt: `svg` | token bounding boxes; CSS style injection |
| Bmp/Gif/Img/Jpg/Png | `bmp`/`gif`/`img`/`jpg` | hard | F4 | opt: `image` | pixel rendering + layout; **permanent bridge** for `bmp`/`gif` (low demand) |

**Phase summary**:

| phase | formatters | effort | status | notes |
| ----- | ---: | ------ | ------ | ----- |
| **F0** — trivial (3) | text, raw, testcase | trivial | Not started | 0 deps; test harness friendly |
| **F1** — terminal ANSI (6) | terminal, terminal256, terminal16m, irc, bbcode, (+ pango in F2) | low–med | Not started | 5 formatters share color lut infrastructure; one engine pass |
| **F2** — markup (4) | groff, pango, latex, rtf | low–med | Not started | latex: opt `RaTeX` for full TeX; fallback escape+template |
| **F3** — vector (1) | svg | medium | Not started | opt `svg` crate or manual builder |
| **F4** — raster (5) | bmp, gif, img, jpg, png | hard | **bridge-only** | `image` crate optional; low demand, heavy deps → keep on bridge |

**Recommendation**: port F0–F2 incrementally (low-effort, high coverage). F3 when SVG CLI use case emerges. **F4 permanently on bridge** — raster rendering is out of scope for text highlighting and pulls in heavy image processing deps.

**Definition of done (formatter)**:
1. Implement `Formatter` trait in `src/formatters/{name}.rs`
2. Register in `formatters/registry.rs` (both getter + aliasing)
3. Byte-parity tests vs Pygments output for ≥3 representative token streams
4. `cargo build -p pygmentsrs`, `cargo test -p pygmentsrs --lib`, `make test-python` green
5. Add to `docs/compat.md` table; update tracking table below

---

## Formatter implementation plan (Phase F)

### F0 — Trivial formatters (text, raw, testcase) — 3 formatters

**No external dependencies. Pure Rust string builders.**

#### `NullFormatter` (`text`)
- **I/O**: consume tokens, emit `.text` for each token, strip leading/trailing whitespace at EOF
- **Complexity**: ~30 lines
- **Test**: verify whitespace handling matches Pygments

#### `RawTokenFormatter` (`raw`, `tokens`)  
- **I/O**: consume tokens, emit `repr(token)` (tab-sep: type, string value)
- **Complexity**: ~40 lines
- **Test**: byte-parity with Pygments repr format (includes escape codes for special chars)

#### `TestcaseFormatter` (`testcase`)
- **I/O**: consume tokens, emit Rust unit-test boilerplate
- **Complexity**: ~50 lines
- **Test**: verify generated code compiles + contains expected token type names

**ROI**: High. These are self-contained, improve test harness independence, and ship with zero cost.

---

### F1 — Terminal ANSI formatters (terminal, terminal256, terminal16m, irc, bbcode) — 5 formatters

**Shared infrastructure: color lookup tables (RGB ↔ ANSI/IRC/BBCode)**

#### Build once, use 5 times

```rust
// lib/ansi.rs
pub struct ColorLut {
    // 16-color ANSI (3-bit + bright bit)
    ansi_16: [(u8, u8, u8); 16],
    // 256-color cube: 6×6×6 RGB cube + grayscale
    ansi_256: [(u8, u8, u8); 256],
    // IRC mIRC color table (16 standard codes)
    irc: [&str; 16],
    // BBCode hex lookup
}

impl ColorLut {
    fn rgb_to_ansi_16(&self, r: u8, g: u8, b: u8) -> u8 { /* nearest neighbor */ }
    fn rgb_to_ansi_256(&self, r: u8, g: u8, b: u8) -> u8 { /* cube lookup + grayscale fallback */ }
    fn rgb_to_irc(&self, r: u8, g: u8, b: u8) -> u8 { /* nearest in 16 mIRC colors */ }
}
```

#### `TerminalFormatter` (`terminal`)
- **I/O**: tokens → ANSI escape codes (3-bit color, bold/italic/underline via SGR)
- **Complexity**: ~80 lines (tokentype → SGR, ColorLut calls)
- **Shared**: ColorLut (RGB → 16-color)
- **Test**: byte-parity with Pygments on 5 token streams (includes style overrides)

#### `Terminal256Formatter` (`terminal256`)
- **I/O**: tokens → 256-color ANSI codes (6×6×6 RGB cube + grayscale)
- **Complexity**: ~90 lines (tokentype → SGR, ColorLut calls)
- **Shared**: ColorLut (RGB → 256-color), **Reuse SGR logic** from TerminalFormatter
- **Test**: verify color rounding (e.g. `#FF6600` → nearest 256 index)

#### `TerminalTrueColorFormatter` (`terminal16m`)
- **I/O**: tokens → 24-bit truecolor ANSI codes (ESC[38;2;R;G;Bm)
- **Complexity**: ~60 lines (trivial — direct RGB, no lookup)
- **Shared**: None (RGB passed through directly)
- **Test**: verify 24-bit RGB escape format

#### `IRCFormatter` (`irc`)
- **I/O**: tokens → mIRC color codes (\x03NN)
- **Complexity**: ~70 lines
- **Shared**: ColorLut (RGB → mIRC 16-color index)
- **Test**: byte-parity on IRC chat-like output

#### `BBCodeFormatter` (`bbcode`)
- **I/O**: tokens → `[color=#RRGGBB][bold]…[/bold][/color]` markup
- **Complexity**: ~80 lines
- **Shared**: None (hex pass-through)
- **Test**: verify tag nesting order

**Infrastructure cost**: ~150 lines for ColorLut (one-time); **formatters add ~80 lines each**.

**ROI**: Very high. ANSI trio is heavily used in CLI/CI environments. One shared LUT serves all.

---

### F2 — Markup formatters (groff, pango, latex, rtf) — 4 formatters

#### `GroffFormatter` (`groff`)
- **I/O**: tokens → troff escape sequences (`.ft`, `.nr`, `.defcolor`)
- **Complexity**: ~110 lines
- **Dependencies**: none
- **Color mapping**: RGB → troff `.defcolor` names (auto-allocate 0–1024 slots)
- **Test**: verify `.ft` font changes match token style, color index consistency
- **Notes**: Groff is a rarely-used backend; lower priority within F2

#### `PangoMarkupFormatter` (`pango`)
- **I/O**: tokens → Pango markup (`<span color='#RRGGBB' weight='bold'>…</span>`)
- **Complexity**: ~90 lines
- **Dependencies**: none
- **Color**: pass-through hex; weight/style via XML attributes
- **Test**: verify XML structure, color format (#RRGGBB), attribute escaping
- **Notes**: Often paired with GTK+ rendering; moderate demand

#### `LatexFormatter` (`latex`)
- **I/O**: tokens → LaTeX macros (escape `_`, `{`, `}`, `$`, `\`; map style → `\textbf{}`, `\textit{}`, color → `\textcolor{name}{}`)
- **Complexity**: ~150 lines (escape logic + macro table)
- **Dependencies**: **optional** `ratex` (for full TeX rendering); fallback string template
- **Variant A (escape + template)**: ~150 lines Rust, zero deps. No live rendering; just produces LaTeX source. Suitable for `latexmk` pipelines.
  ```rust
  // e.g. escape + macro table
  fn escape_latex(s: &str) -> String { /* _ -> \_, $ -> \$, etc. */ }
  fn style_to_latex(token: TokenType) -> &str { match … { Bold => r"\textbf", … } }
  ```
- **Variant B (RaTeX integration)**: ~50 lines Rust + `ratex` dep. Render tokens → PDF/SVG on demand.
  ```rust
  use ratex::{render_latex, RenderOptions};
  let tex_src = build_latex_document(&tokens);
  let pdf = render_latex(&tex_src, RenderOptions::pdf())?;
  ```
- **Decision**: Start with **Variant A** (escape + template); optionally gate `RaTeX` behind feature `ratex-render` for V2.
- **Test**: byte-parity on LaTeX-safe token streams; verify escape sequences

#### `RtfFormatter` (`rtf`)
- **I/O**: tokens → RTF (`{\colortbl;...}` + `{\*\fonttbl}`, tokens as `\cf1\b text\b0`)
- **Complexity**: ~140 lines (color table generation, RTF syntax)
- **Dependencies**: none
- **Color mapping**: allocate color slots in RTF preamble, map token colors to indices
- **Test**: verify RTF structure (header, color table, body); open in Word/LibreOffice
- **Notes**: RTF is aging but still used in some legacy workflows; moderate complexity but self-contained

**Infrastructure cost**: ~100 lines for shared style-→-macro mapping; **formatters add ~90–150 lines each**.

**ROI**: Medium. LaTeX is high-value (Sphinx use); RTF/Pango lower demand but straightforward.

---

### F3 — Vector formatter (svg) — 1 formatter

#### `SvgFormatter` (`svg`)
- **I/O**: tokens → `<svg>` with `<text>` elements, CSS `<style>` for token colors
- **Complexity**: ~180 lines (layout bounding boxes, monospace font metrics)
- **Dependencies**: optional `svg` crate (or manual XML builder)
- **Layout**: 
  - Monospace font assumption (or accept `font-width` option)
  - Track line height, max line width
  - Position each token as `<text x='..' y='..'>`
  - Group by line or emit runs
- **Color mapping**: token type → CSS class → `.token-type { color: #RRGGBB; }`
- **Test**: verify SVG structure, validate with `xmllint`, render in browser
- **Notes**: Less common than HTML for web (HTML formatter preferred); occasional use in documentation generators

**ROI**: Low–medium. Niche use case; implement when SVG CLI demand appears.

---

### F4 — Raster formatters (bmp, gif, img, jpg, png) — 5 formatters

**Status: Bridge-only (permanent). Rationale**:
- Heavy `image` crate dependency (300+ KB; pulls in `tiff`, `jpeg`, `deflate`)
- Require font rasterization (either `fontdue`, `rusttype`, or system fonts)
- Low demand in typical code-highlighting workflows (HTML/SVG/ANSI dominate)
- PIL in Pygments is already a "convenience" formatter, not core

**Fallback**: users who need raster can pipe HTML to `wkhtmltoimage` or use `pandoc` with wkhtmltopdf.

**Decision**: If raster demand surfaces, gate behind feature `raster-formatters` + `image` dep, implement later. For now, bridge serves these via PyO3.

---

## Formatter infrastructure (shared, all phases)

### `Formatter` trait (core)
```rust
pub trait Formatter {
    fn format(&self, tokens: &[(TokenType, &str)], writer: &mut dyn std::io::Write) -> std::io::Result<()>;
}
```

### Style engine (shared by F1–F2)
```rust
// lib/style.rs
pub struct Style {
    pub color: Option<(u8, u8, u8)>,        // RGB
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub bgcolor: Option<(u8, u8, u8)>,
}

impl Style {
    pub fn from_token(token: TokenType) -> Self { /* pygments token type → style */ }
}
```

### Color utilities
```rust
// lib/color.rs
pub fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> u8;
pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8;
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String;
```

---

## Updated tracking table

| date | native lexers | native formatters | notes |
| ---- | -----: | -----: | ------ |
| (init) | 13 | 1 | html only |
| Phase A | 37 | 1 | – |
| Phase B | 59 | 1 | – |
| Phase C | 72 | 1 | – |
| Phase D (bulk) | 449 | 1 | – |
| Phase E4 (dispatch) | **451** | 1 | Lexers complete ✅ |
| **Phase F0** | 451 | **4** | text, raw, testcase + html = 4 |
| **Phase F1** | 451 | **9** | + terminal trio + irc + bbcode = 9 |
| **Phase F2** | 451 | **13** | + groff, pango, latex, rtf = 13 |
| **Phase F3** | 451 | **14** | + svg = 14 |
| **Phase F4** | 451 | **14** | bmp/gif/jpg/png bridge-only (no change) |

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

| date | native lexers | transpilable remaining | native formatters | tests passing | status |
| ---- | ------------: | ---------------------: | ----------------: | --------: | ------ |
| (init) | 13 | 355 | 1 | – | bootstrap |
| Phase A | 37 | 331 | 1 | – | high-value docs |
| Phase B Batch 1 | 51 | 317 | 1 | – | config/markup |
| Phase B Batch 2 | 59 | 309 | 1 | – | – |
| Phase C Batch 1 | 72 | 296 | 1 | – | long tail |
| Phase D (engine + bulk) | 449 | 10 | 1 | 280 | regexlexer bulk |
| Final transpilable batch | 447 | 0 | 1 | 322 | transpilable exhausted |
| **Phase E4** (dispatch + hand-craft) | **451** | **0** | 1 | **341** ✅ | Lexers complete |
| **Phase F0** | 451 | 0 | **4** | TBD | text, raw, testcase, html |
| **Phase F1** | 451 | 0 | **9** | TBD | terminal trio, irc, bbcode |
| **Phase F2** | 451 | 0 | **13** | TBD | groff, pango, latex, rtf |
| **Phase F3** | 451 | 0 | **14** | TBD | svg |
| **Phase F4** | 451 | 0 | **14** | TBD | bmp/gif/jpg/png bridge-only |

_Lexers: 451 total (436 generated + 4 built-in + 2 hand-crafted markdown/rst/tid + 9 hand-crafted pre-Phase A).
`adl` excluded from generation but now mitigated via NFA-safe rewrite. `csharp` remains excluded.
Formatters: phases F0–F2 targeted for port; F3 optional (SVG low demand); F4 permanent bridge (raster deps)._
