# myst-md-rs

Rust port of [MyST-Parser](https://github.com/executablebooks/MyST-Parser),
built on top of [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark).

## Objectives

- Port MyST Markdown to Rust as `myst-md-rs`.
- Reuse `pulldown-cmark` for CommonMark/GFM core; layer MyST extensions on top.
- Provide HTML rendering and (later) an AST bridge to `docutilsrs` / `sphinxdocrs`.
- Expose a Python module mirroring the `pygmentsrs` / `docutilsrs` pattern so
  upstream `myst_parser` can dispatch to the Rust path when available.

## Status — Phase 0

Implemented:

- CommonMark + GFM tables/strikethrough/tasklists/footnotes via `pulldown-cmark`.
- YAML front matter (`---` fences at top of document) extracted into a struct.
- MyST inline roles: `` {role}`content` `` rendered as `<span class="myst-role" data-role="…">…</span>`.
- MyST inline math: `$…$` (single line) rendered as `<span class="math">…</span>`.
- MyST block math: `$$…$$` rendered as `<div class="math">…</div>`.
- Colon fences: `:::name … :::` rewritten to fenced code with info string
  `{name}` and rendered as `<div class="myst-directive" data-name="…">…</div>`.

Pending (later phases): full directive option/argument parsing, substitutions,
field lists, definition lists, attrs, dollarmath label support, doctree bridge,
Python plugin fallback.

## Layout

```
src/myst-md-rs/
├── Cargo.toml
├── pyproject.toml
├── src/
│   ├── lib.rs        # public surface + Python module
│   ├── frontmatter.rs
│   ├── preprocess.rs # colon fence + dollar math → cmark-friendly source
│   ├── render.rs     # event-stream HTML renderer with MyST hooks
│   └── role.rs       # inline role detection
└── tests/
    └── snapshot.rs
```
