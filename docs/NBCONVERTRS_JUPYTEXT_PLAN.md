# nbconvertrs Jupytext Port Plan

## Goal

`nbconvertrs` is the Rust implementation of the optional Markdown and notebook
transforms currently invoked by sustainablefactory's `transform-md` workflow.
It is both a standalone Clap CLI and an in-process library that DocIndex can
invoke before indexing. Notebook structures and serialization are provided by
the `runtimed` `nbformat` crate rather than by a second notebook model.

## Current foundation

- Markdown fenced code cells are converted to nbformat v4 code cells.
- `{raw-cell}` and `raw` fences are converted to raw cells.
- Ordinary Markdown becomes Markdown cells.
- `--transform-cell-split m1` splits Markdown at level-one headings.
- `myst`, `md`, and `markdown` output formats render paired Markdown; `ipynb`
  renders a deterministic nbformat JSON document.
- `docindex index`, `index-chats`, and `index-html` can run this transform
  in-process with `--transform-markdown`, avoiding a child process.
- Existing `.ipynb` files are directly ingestible by DocIndex, including
  runtimed's legacy and v3 upgrade paths.

## Jupytext compatibility phases

### Phase 1: paired Markdown and MyST

Port the behaviors needed by the current sustainablefactory corpus:

- Markdown fenced code cells, including language identifiers and MyST
  `{code-cell}` metadata.
- HTML cell markers: `<!-- #region -->`, `<!-- #markdown -->`,
  `<!-- #raw -->`, and matching end markers.
- Raw cells and preservation of cell boundaries.
- Stable generated cell IDs and deterministic JSON serialization.
- Round-trip tests against representative `.md`, `.myst.md`, and `.ipynb`
  fixtures from `src/jupytext/tests`.

### Phase 2: script formats

Add parsers and writers for Jupytext's light and percent formats:

- Python/R/Julia/MATLAB `# %%` cells.
- Light script `# +` and `# -` markers.
- Language-specific comment prefixes and notebook metadata.
- `py`, `R`, `jl`, and `m` format selection in the CLI.

### Phase 3: metadata and outputs

- Preserve notebook-level kernelspec and language metadata.
- Preserve cell tags, names, attachments, execution counts, and outputs.
- Implement Jupytext-compatible `formats`, `text_representation`, and
  `jupytext` metadata where it is present.
- Add fixture parity tests that compare normalized notebook structures rather
  than incidental JSON key order.

### Phase 4: workflow parity

- Port incremental manifest fingerprints and atomic output replacement from
  `tools/workflow_transform.py`.
- Add `--dry-run`, `--out-format`, and transform configuration loading from
  `_toc.yml`.
- Preserve the `transform-md` compatibility binary while documenting
  `nbconvertrs` as the canonical command.
- Add a Python-to-Rust parity matrix for files that remain delegated to the
  upstream Jupytext implementation.

## API shape

The library should remain independent of filesystem indexing details:

```text
markdown_to_notebook(source, options) -> nbformat::v4::Notebook
notebook_to_markdown(notebook) -> String
notebook_to_json(notebook) -> String
transform_file(source, output_base, formats, options) -> outputs
```

DocIndex owns file discovery and document creation. It passes in-memory source
text to `nbconvertrs`, then indexes the transformed representation as
`DocumentType::SphinxNb`. This keeps the CLI and library paths behaviorally
aligned and makes tests independent of an installed Python environment.

## Test gates

1. Unit tests for each marker and format parser.
2. Round-trip tests for Markdown -> notebook -> Markdown.
3. JSON compatibility tests using runtimed nbformat fixtures.
4. Native DocIndex tests proving no subprocess is started.
5. CLI subprocess tests for single-file, directory, and incremental workflows.
6. Corpus smoke tests over `src/sustainablefactory/data` with bounded output.
7. Differential tests against Python Jupytext for supported fixtures before
   expanding the supported format matrix.

## Non-goals

- Executing notebooks or kernels.
- Replacing runtimed's notebook protocol or execution clients.
- Claiming complete Jupytext parity before the format matrix and fixture tests
  are implemented.