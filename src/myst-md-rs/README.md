# myst-md-rs

Rust port of [MyST-Parser](https://github.com/executablebooks/MyST-Parser),
built on top of [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark).

## Objectives

- Port MyST Markdown to Rust as `myst-md-rs`.
- Reuse `pulldown-cmark` for CommonMark/GFM core; layer MyST extensions on top.
- Provide HTML rendering plus a native doctree bridge to `docutilsrs` /
  `sphinxdocrs`.
- Expose a Python module mirroring the `pygmentsrs` / `docutilsrs` pattern so
  upstream `myst_parser` can dispatch to the Rust path when available.

## Status — Phase 0 + W6 partial

Implemented:

- CommonMark + GFM tables/strikethrough/tasklists/footnotes via `pulldown-cmark`.
- YAML front matter (`---` fences at top of document) extracted into a struct.
- MyST inline roles: `` {role}`content` `` rendered as `<span class="myst-role" data-role="…">…</span>`.
- MyST inline math: `$…$` (single line) rendered as `<span class="math">…</span>`.
- MyST block math: `$$…$$` rendered as `<div class="math">…</div>`.
- Colon fences: `:::name … :::` rewritten to fenced code with info string
  `{name}` and rendered as `<div class="myst-directive" data-name="…">…</div>`.
- Native `parse_to_doctree(source, source_path, options)` lowering for sections,
  inline markup, links, images, lists, tables, definition lists, directives,
  roles, math, simple substitutions, relative includes, eval-rst, source
  metadata, and source-line tracking.
- YAML-backed directive option parsing with quoted/multiline scalars, block
  scalars, comments, and typed option coercion.
- Sphinx integration through `source_suffix = {'.md': 'myst'}` with persisted
  doctree output and native HTML writing.

Pending (later phases): full directive registry/argument validation, field
lists, recursive substitution diagnostics, include options/errors, table
spans, dollarmath label support, reporter warning nodes, full upstream
doctree XML parity, and Python plugin fallback.

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
│   ├── doctree.rs    # Markdown → docutilsrs doctree bridge
│   └── role.rs       # inline role detection
└── tests/
    └── snapshot.rs
```
