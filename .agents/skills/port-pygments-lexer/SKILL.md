---
name: port-pygments-lexer
description: >-
  Port vendored Pygments lexers to native Rust in src/pygmentsrs using the
  tools/gen_lexer.py transpiler: classify which lexers are transpilable,
  generate them, wire the registry, and byte-parity-gate against upstream.
  Use when adding native syntax-highlighting lexers, expanding pygmentsrs
  coverage, or deciding whether a lexer must stay on the PyO3 bridge.
---

# Port a Pygments lexer to native Rust (`pygmentsrs`)

`pygmentsrs` ports Pygments' `RegexLexer` engine to Rust. Most lexers are
pure data — `tools/gen_lexer.py` transpiles a vendored Pygments lexer's
*processed* token table into an equivalent native Rust lexer, byte-parity
with upstream. This skill is the end-to-end workflow.

## When to use

- Add one or more native Rust lexers to `src/pygmentsrs/src/lexers/generated/`.
- Survey which of the ~585 un-ported lexers can be transpiled now.
- Decide whether a lexer must stay on the runtime PyO3 `pygments` bridge.

The overall roadmap, phased batches, and progress tracker live in
`docs/pygments-port-inventory.md`. This skill is the per-batch mechanics;
that doc says *which* lexers to do next.

## Prerequisites

- Run everything from `src/` (the cargo workspace root). The venv is `src/.venv`.
- Vendored Pygments must be importable: `src/.venv/bin/python -c "import pygments"`.
- After Rust changes, rebuild the extension before pytest:
  `src/.venv/bin/maturin develop --release -m pygmentsrs/Cargo.toml`.

## Background: what transpiles, what doesn't

The transpiler emits a `Rule` per processed `(pattern, action, new_state)`:

- token action → `Rule::token` / `Rule::token_to`
- `bygroups(t1, …)` → `Rule::bygroups` / `Rule::bygroups_to` (args recovered
  from the closure; each group must be a token or `None`)
- `default(state)` → `Rule::default` (zero-width)
- `include()` / `words()` / `combined()` are already expanded by
  `process_tokendef`, so they need no special handling
- unknown token dotted-names fall back to a structural
  `TokenType::new(&["Name","Variable","Anonymous"])` — never blocks on a
  missing named const

**Bridge-only** (the tool prints `SKIP`, no file written — these resolve
through the PyO3 bridge at runtime): `using(OtherLexer)` / `this`
delegation, arbitrary Python callback actions, and `bygroups` with a nested
callback. Do **not** try to hand-port these unless you are also porting the
embedded lexer.

The engine compiles with `fancy-regex`, so lookahead, lookbehind, and
backreferences (e.g. the `bash` heredoc `\2`) all work.

## 1. Classify — find transpilable lexers

```sh
cd src
.venv/bin/python tools/gen_lexer.py --classify              # all buckets
.venv/bin/python tools/gen_lexer.py --classify transpilable # ready-to-paste specs
```

The `transpilable` bucket prints ready-to-use `module:ClassName:rust_name`
specs (with rule counts). Pick targets from there. Categories:
`transpilable`, `bridge_using`, `bridge_callback`, `non_regex` (all
bridge-only). `error` should be empty — if a lexer errors, it is almost
always a token-variant lexer whose `_tokens` lives on the instance (the
tool already reads the instance); a new error category usually means an
upstream API change worth investigating.

Prefer the next unfinished phase in `docs/pygments-port-inventory.md`
(Phase A is the high-value doc/Sphinx languages). To work a single source
module end-to-end: `--classify transpilable | grep <module>`.

## 2. Generate

```sh
.venv/bin/python tools/gen_lexer.py \
    pygments.lexers.rust:RustLexer:rust \
    pygments.lexers.data:YamlLexer:yaml
```

`rust_name` becomes the file stem `generated/<rust_name>.rs` and the struct
`<RustName>Lexer` (each `_`-separated part capitalized: `my_lang` →
`MyLangLexer`). Confirm `WROTE …` and `0 skipped`. A `SKIP` means
bridge-only — drop it and leave it to the bridge.

## 3. Wire the registry

Generate the exact wiring snippet, then paste the three blocks into place:

```sh
.venv/bin/python tools/gen_lexer.py --registry \
    pygments.lexers.rust:RustLexer:rust
```

- `pub mod <name>;` → `src/pygmentsrs/src/lexers/generated/mod.rs`
- match arm → `get_lexer_by_name` in `src/pygmentsrs/src/lexers/registry.rs`
- alias lines → `native_aliases()` in the same file

Keep `get_lexer_by_name` and `native_aliases()` in sync — a parity test
asserts every generated alias is in both.

## 4. Byte-parity test

Add the lexer to `GENERATED` in
`src/tests/test_pygments_generated_lexers.py` with 3–5 representative
samples (cover the tricky states: strings, comments, nested/heredoc,
numbers):

```python
"rust": (
    "pygments.lexers.rust", "RustLexer",
    [
        "fn main() {\n    let x = 1;\n}\n",
        "// comment\n/* block */\nlet s = \"str\";\n",
        "struct S { a: u32 }\n",
    ],
),
```

The test compares native `pygmentsrs.lex(alias, src, backend="rust")` against
`UpstreamLexer().get_tokens_unprocessed(src)` token-for-token.

## 5. Build, install, verify

```sh
cd src
cargo build -p pygmentsrs                                   # must be warning-free
.venv/bin/maturin develop --release -m pygmentsrs/Cargo.toml
.venv/bin/pytest -q tests/test_pygments_generated_lexers.py
```

Quick ad-hoc parity probe while iterating:

```sh
.venv/bin/python - <<'PY'
import importlib, pygmentsrs
mod, cls, alias, src = "pygments.lexers.rust", "RustLexer", "rust", "fn f(){}\n"
up = [(repr(t), v) for _, t, v in getattr(importlib.import_module(mod), cls)().get_tokens_unprocessed(src)]
nat = [(a, b) for a, b in pygmentsrs.lex(alias, src, backend="rust")]
print("OK" if nat == up else "DIFF")
for i, (a, b) in enumerate(zip(nat, up)):
    if a != b:
        print("first diff @", i, "native=", a, "upstream=", b); break
PY
```

## 6. Full gate + docs

```sh
cargo test -p pygmentsrs          # cargo unit + snapshot tests
make -C .. test-python            # full pytest + coverage report
```

Then update the transpiled-lexers table and counts in
`src/pygmentsrs/docs/compat.md`, and the tracking row in
`docs/pygments-port-inventory.md`.

## Batching many lexers

For a whole phase/module, pass all specs to one `generate` and one
`--registry` invocation, then paste once. Keep a batch to ~10–25 lexers so
a parity failure is easy to bisect. The `--registry` output is already
sorted/de-duplicated for direct pasting. After wiring, add every alias's
samples to `GENERATED` before running the parity suite so a single
`pytest` run validates the whole batch.

## Troubleshooting

- **DIFF in parity**: dump both token streams side by side (probe above).
  A common cause is a state the samples didn't reach — add a sample that
  exercises it. If upstream uses an `ExtendedRegexLexer` context or a
  callback the transpiler couldn't see, reclassify as bridge-only.
- **Runtime panic `uncompilable pattern`**: a pattern uses a regex
  construct `fancy-regex` rejects. Check `--classify` didn't mislabel it;
  most Python constructs (named groups `(?P<>)`, `(?P=name)`, `\g<n>`,
  lookaround, backrefs) are supported. If genuinely unsupported, the lexer
  is bridge-only.
- **`SKIP … using()/callback`**: expected for embedded-language lexers.
  Leave on the bridge; do not hand-port unless porting the embedded lexer too.
- **New token const wanted**: the structural `TokenType::new(&[…])` fallback
  already handles any subtype. Only add a named const to `token.rs` if it is
  widely shared and you want readable generated output.

## Regeneration / upstream bumps

Generated files carry a header with the exact regenerate command. After a
vendored-Pygments bump, re-run the generate command for every lexer and
re-run the parity suite — that suite *is* the drift detector.
