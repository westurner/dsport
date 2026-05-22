# myst-md-rs test port plan

Generated from `src/MyST-Parser/tests/` to plan the incremental port
into `src/myst-md-rs/tests/` (Rust) and a thin Python parity harness.

Status legend:

* **P1** — port now: no docutils/Sphinx dependency; exercises only the
  parser, AST, options/directive parser, HTML tokenizer, front matter,
  or string-level renderer behaviour.
* **P2** — port after the doctree bridge to `docutilsrs` lands.
* **P3** — port after Sphinx environment / builder support in
  `sphinxdocrs`.
* **K-PY** — keep as Python: CLI, docutils Publisher, sphinx-pytest
  fixtures. Will be exercised via the PyO3 module surface (so the
  upstream test still runs against `myst_md_rs.*` once we publish
  compat shims), not re-implemented in Rust.

The plan groups tests by what they exercise, not by their upstream
folder, so a wave can be closed end-to-end without dragging in
docutils/sphinx prematurely.

## Wave summary

| wave | rust deliverable | upstream tests folded in | rough case count |
| --- | --- | --- | --- |
| W0 (done) | bootstrap snapshot harness | — | 10 |
| W1 | CommonMark spec parity (HTML out) | `test_commonmark/` | 649 (minus skips) |
| W2 | MyST inline & block extensions, HTML out | parts of `test_renderers/fixtures/*.md`, `test_html/html_round_trip.md` | ~120 |
| W3 | options + directive text parser | `test_renderers/fixtures/option_parsing*.yaml`, `directive_parsing.txt` | 72 |
| W4 | HTML tokenizer + AST | `test_html/html_ast.md`, `html_round_trip.md`, `test_html_to_nodes.py` (html→nodes deferred) | ~30 |
| W5 | front matter / anchors / CLI surface | `test_anchors.py`, `test_inventory.py` (data only), `myst-config.txt` (string slice only) | small |
| W6 (P2) | doctree bridge → docutilsrs | `docutil_*`, `containers.md`, `tables.md`, `dollarmath.md`, `amsmath.md`, `definition_lists.md`, `attributes.md`, `mock_include*`, `reporter_warnings.md`, `eval_rst.md`, `directive_options.md` | ~250 |
| W7 (P3) | sphinx bridge → sphinxdocrs | `sphinx_*`, `test_sphinx/`, `test_myst_refs/`, `test_include_directive.py` (Sphinx half) | very large; keep mostly K-PY |

## Wave details

### W1 — CommonMark spec

Upstream: `tests/test_commonmark/test_commonmark.py` runs the 649 cases
from `commonmark.json` (CommonMark 0.29) through `MarkdownIt` and
compares HTML byte-for-byte. Skips: examples `14`, `66`, `68`; spec-bug
rewrites for `187`, `209`, `210`, `593`.

Port mechanics:

* Add `src/myst-md-rs/tests/commonmark.rs` (`#[test] fn commonmark_spec()`).
* Load `tests/data/commonmark.json` (copy the file into the crate so we
  don't reach into the vendored Python repo at test time).
* For each entry, run `myst_md_rs::render_html` and compare to
  `entry["html"]`. Apply the same case-by-case patches as upstream.
* Track parity: emit a single failure summary listing failing example
  numbers + section. Use `insta` only for the *summary*, so the suite
  doesn't generate 600+ snapshot files; raw `assert_eq!` per case keeps
  flake low.
* `pulldown-cmark` already targets CommonMark 0.31 — we expect mostly
  green with a small allowlist for known semantic deltas (autolink,
  list-loose detection, raw-html). Track deviations in
  `docs/compat.md` under a new `myst-md-rs::commonmark` heading.

Acceptance: every spec example either passes or appears in a documented
allowlist with a one-line reason.

### W2 — MyST inline & block extensions (HTML-only checks)

These fixtures already encode expected docutils XML, *not* HTML, so we
can't reuse them as-is. Instead:

* For each fixture below, hand-port the **input** halves to
  `tests/data/myst/<fixture>.md` and write expected **HTML** snapshots
  (one per case) under `tests/snapshots/`. This keeps the Rust suite
  HTML-only until W6.
* Fixtures in scope:
  * `dollarmath.md` (9 cases) — inline `$…$`, display `$$…$$`.
  * `amsmath.md` (6 cases) — amsmath environments (Phase 1 of W2).
  * `containers.md` (6 cases) — colon-fence directives.
  * `attributes.md` (4 cases) — inline `{#id .class}` attrs.
  * `definition_lists.md` (1 case).
  * `tables.md` (7 cases).
  * `test_html/html_round_trip.md` (already HTML-oriented).
* Add a tiny `param_file.rs` test helper that mirrors upstream's
  `pytest-param-files` dot-fence format (`title\n.\ninput\n.\nexpected\n.\n`)
  so the same fixture files can be lifted later for the doctree wave.

Acceptance: each ported case has either a passing snapshot or an
explicit `// PENDING: feature X` skip with the upstream case title.

### W3 — Options / directive text parser

Pure-Python today (`myst_parser/parsers/options.py`,
`parsers/directives.py`). No docutils objects in
`option_parsing*.yaml` — values are stringly typed. `directive_parsing.txt`
does reference `Note`/`Admonition`/`CodeBlock` classes; for the Rust
port we'll represent those as a `DirectiveSpec { required_arguments,
optional_arguments, has_content, option_spec: HashMap<&str, OptionType> }`
loaded from a small JSON table mirroring the upstream classes.

Port mechanics:

* Add `myst_md_rs::options::options_to_items` matching upstream signature
  (returns `(Vec<(String, String)>, ParseState { has_comments })`).
* Add `myst_md_rs::directives::parse_directive_text(spec, first_line,
  body) -> Result<ParsedDirective, MarkupError>`.
* Tests: `tests/options.rs`, `tests/directives.rs`. Load YAML fixtures
  with `serde_yaml`. Reuse upstream YAML files verbatim (vendored copy
  in `tests/data/`).
* Errors compared via `Display` rendering — keep messages
  byte-for-byte unless documented in `docs/compat.md`.

This is also where `nom` becomes worth pulling in (option list grammar,
quoted-string scanning). Confirm need against
`options_to_items` complexity before adding the dep.

Acceptance: all 28 `directive_parsing.txt` cases, all 16 `option_parsing.yaml`
cases, all 14 `option_parsing_errors.yaml` cases.

### W4 — HTML tokenizer + AST

Upstream: `myst_parser.parsers.parse_html.tokenize_html` — a tiny
HTML5-ish tokenizer producing a tree of `Element` nodes with
attributes. Used by both the `eval-rst` flow and the inline HTML
attrs.

Port mechanics:

* Add `myst_md_rs::html::tokenize(&str) -> HtmlAst`.
* Snapshot tests for `tests/data/html_ast.md` (repr-walk) and
  `html_round_trip.md` (stringify).
* `test_html_to_nodes.py` depends on docutils nodes — defer to W6.

### W5 — front matter / anchors / CLI surface

* `test_anchors.py` — already trivial; reproduce as
  `myst_md_rs::cli::print_anchors` + a Python entry point on the PyO3
  module. Rust test: feed `# a\n\n## b\n\ntext`, assert
  `<h1 id="a"></h1>`.
* `test_inventory.py` — pure data parsing of Sphinx inventory `.inv`
  files. Not strictly MyST; we can either port it under
  `sphinxdocrs::inventory` (likely already on the roadmap there) or
  keep K-PY. **Recommendation:** out of scope for `myst-md-rs`; cross-
  reference into the sphinxdocrs port plan.
* `myst-config.txt` exercises docutils CLI plumbing — K-PY. Once the
  PyO3 module is published, we can run the upstream test against it
  by configuring the Publisher to use the Rust parser via the existing
  Python `Parser` wrapper.

### W6 — doctree bridge (depends on `docutilsrs`)

All fixtures whose expected output is `<document source=...>` belong
here:

| fixture | cases | notes |
| --- | --- | --- |
| `docutil_syntax_elements.md` | 63 | core HTML/AST parity |
| `docutil_syntax_extensions.txt` | 25 | enable_extensions matrix |
| `docutil_directives.md` | 46 | all built-in directives |
| `docutil_roles.md` | 18 | all built-in roles |
| `docutil_link_resolution.md` | 10 | reference resolution |
| `directive_options.md` | 16 | option coercion into AST |
| `containers.md` | 6 | re-asserted at doctree level |
| `tables.md` | 7 | colwidths / thead / tbody |
| `dollarmath.md` | 9 | `<math>` / `<math_block>` |
| `amsmath.md` | 6 | `<math_block>` ams flavour |
| `definition_lists.md` | 1 | `<definition_list>` |
| `attributes.md` | 4 | id/class attrs on AST nodes |
| `eval_rst.md` | 3 | inline rST escape hatch |
| `mock_include.md` | 7 | `{include}` directive |
| `mock_include_errors.md` | 4 | error paths |
| `reporter_warnings.md` | 28 | warning surface; needs reporter shim |
| `test_html_to_nodes.py` (`html_to_nodes.md`) | n/a | uses docutils nodes |

Mechanics:

* Reuse the W2 `param_file` helper but assert against pretty-printed
  doctree from `docutilsrs::doctree`.
* Many fixtures embed `<src>/index.md` as the document source string —
  expose that as a parser knob so we don't have to rewrite fixtures.

### W7 — sphinx bridge (depends on `sphinxdocrs`)

`sphinx_syntax_elements.md` (58), `sphinx_directives.md` (51),
`sphinx_link_resolution.md` (13), `sphinx_roles.md` (85),
`test_sphinx/test_sphinx_builds.py` (15 builders, each with multiple
output files) — all gated on Sphinx environment availability. Realistic
plan: keep **K-PY** indefinitely; after `myst_md_rs` is wired into
`myst_parser` as a backend, run upstream's existing tests against the
Rust parser via the Python module surface and record any deltas in
`docs/compat.md`.

## Cross-cutting infrastructure

1. **`param_file` helper** (`tests/common/mod.rs`): a 60-line parser
   for the upstream `title\n.\ninput\n.\nexpected\n.\n` format.
   Returns an iterator of `(title, description, content, expected)`.
   Reused by W2/W3/W4/W6.
2. **Fixture vendoring**: copy (don't symlink) every upstream fixture
   we depend on into `src/myst-md-rs/tests/data/`. Track upstream
   commit hash in `tests/data/UPSTREAM.md` for refresh diffs.
   Rationale: `src/MyST-Parser/` is a vendored upstream tree we don't
   want test runs to depend on path-traversing into.
3. **Compatibility log** (`docs/compat.md`, new section
   `myst-md-rs`): one line per accepted deviation, with the upstream
   case ID, the actual output, and the rationale (e.g. `commonmark 25:
   pulldown-cmark 0.13 normalises HTML entities to NFC`).
4. **Parity dashboard**: a single `cargo test -p myst-md-rs
   --test parity` aggregator that prints `N/M passing` per wave, so a
   CI badge can read the same number.
5. **PyO3 surface coverage**: add Python tests under
   `src/myst-md-rs/tests/python/` that import `myst_md_rs` and exercise
   `version`, `features`, `render_html`, `parse_front_matter`,
   `parse_to_html`. Lets us run upstream `test_commonmark.py`
   unmodified by registering `myst_md_rs` as an alternate parser
   backend in a thin shim.

## Out of scope (this plan)

* Re-implementing docutils transforms, Sphinx domains, or the Sphinx
  build pipeline inside `myst-md-rs`.
* Porting upstream's pytest plugins (`pytest-param-files`,
  `sphinx_pytest`); we re-implement the *fixture format* in Rust, not
  the plugin.
* Performance benchmarks — track separately under `benches/` after W3.

## Immediate next actions

1. Land the `param_file` helper + the CommonMark spec runner (W1).
2. Vendor the four W3 YAML fixtures and stand up
   `myst_md_rs::options` against them.
3. Stand up the `parity` aggregator harness and add a section in
   `docs/compat.md`.
