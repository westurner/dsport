# DocIndex, Sphinx Extensions, and WASM Implementation Plan

Status: planning

This plan covers three related deliverables:

1. Port SustainableFactory's DocIndex packages from Python to Rust.
2. Keep the shared DocIndex implementation usable on native targets and
   `wasm32-unknown-unknown`.
3. Add built-in Sphinx extensions to `sphinxdocrs` for DocIndex and WebMCP.

The extension work begins with the existing `sphinxdocrs` extension-loading
infrastructure. The DocIndex and WebMCP implementations must not bypass that
lifecycle or introduce a second extension mechanism.

## 1. Confirmed Source and Repository State

The SustainableFactory checkout is now available at:

```text
src/sustainablefactory
```

The checkout is on upstream `main` at commit `81602f5acd86affcb4775a20991014d9e6014994`.

The recovered source tree contains:

```text
src/docindex/
  src/docindex-core/
  src/docindex-cli/
  src/docindex-sphinx/
  src/docindex-sustainablefactory/
  src/docindex_workspace/
  tests/

src/docindex-cli/
```

The DocIndex core currently contains:

- typed document models and configuration in `config.py`;
- `DocumentIndexer` orchestration in `indexer.py`;
- HTML and chat parsers;
- glossary and synonym managers;
- a common backend interface in `backends/base.py`;
- `milli.py`, `oxirs.py`, and `multi.py` backends;
- tests for the models, parsers, indexer, and each backend.

The Sphinx integration currently lives in:

```text
src/sustainablefactory/src/docindex/src/docindex-sphinx/
```

Its hooks configure the backend, listen for Sphinx build events, index generated
HTML, and optionally emit static RDF/HDT and OxiRS WASM assets.

The WebMCP reference implementation is the separate
`sphinxcontrib-webmcp` repository/submodule at commit
`87e88f8cc8f49e99be976739309bf3d81e552c9c`. Its observable contract includes:

- a `webmcp.json` build manifest;
- a static `webmcp.js` browser runtime;
- page context and navigation tools;
- documentation metadata and search tools;
- optional DocIndex, OxiRS, and Meilisearch search modes.

## 2. Existing `sphinxdocrs` Boundary

Before implementing the new built-in extensions, review and use the extension
work already recorded in `docs/sphinxdocrs-port-plan.md`.

The relevant existing Rust surfaces are:

- `src/sphinxdocrs/src/application.rs`
  - `SphinxApp` owns configuration, environment, registry, events, assets,
    builders, and loaded extensions.
  - `SphinxApp::load_extension` is the extension-loading entry point.
  - `NATIVE_BUILDERS` currently includes `html` and `singlehtml` support along
    with the other native builders.
- `src/sphinxdocrs/src/extension.rs`
  - extension metadata and version requirement checks.
- `src/sphinxdocrs/src/registry.rs`
  - component, builder, domain, directive, role, node, CSS, JavaScript, and
    static-directory registration.
- `src/sphinxdocrs/src/app_facade.rs`
  - Python-compatible `app` facade used by loaded Python extensions.
- `src/sphinxdocrs/src/app_events.rs`
  - native event dispatch used by the Rust application lifecycle.
- `src/sphinxdocrs/src/assets.rs`
  - asset registration and integrity handling.
- `src/sphinxdocrs/src/environment.rs`
  - document, doctree, metadata, and navigation state available to builders
    and extensions.
- `src/sphinxdocrs/src/search.rs`
  - Sphinx-compatible `searchindex.js` generation. This is complementary to
    DocIndex, not a replacement for it.

### 2.1 Extension prerequisite

The first Sphinx-specific milestone is to make the extension abstraction
sufficient for native built-ins. This must be completed before adding either
`docindex` or `webmcp`.

Create a public module namespace equivalent to:

```rust
sphinxdocrs::extensions::docindex
sphinxdocrs::extensions::webmcp
```

The extension layer should provide a common built-in contract covering:

- stable extension name;
- version metadata;
- configuration registration;
- event registration;
- builder applicability;
- native setup invocation;
- optional Python fallback or compatibility setup;
- parallel-read and parallel-write declarations.

The built-in extension registry should be explicit and deterministic. It should
not depend on Python package discovery for these two extensions.

The existing Python extension path must remain available for unrelated
extensions and for fallback behavior. Native built-ins should be selected by
name before attempting a Python import, following the existing extension
resolution and fallback rules documented in the Sphinx plan.

Required prerequisite tests:

- built-in names resolve without Python imports;
- `sphinxdocrs::extensions::docindex` and
  `sphinxdocrs::extensions::webmcp` register through the same application path;
- duplicate loading is harmless and does not duplicate event handlers or assets;
- extension metadata is visible through the registry;
- a Python extension with the same name is not accidentally preferred over the
  built-in implementation;
- Python fallback remains available for extensions without a native equivalent;
- configuration and event state are shared correctly with `SphinxApp` and
  `BuildEnvironment`;
- extension errors propagate through the existing `AppError`/event error path.

Do not make `add_directive`, `add_role`, or custom node support a prerequisite
for these two extensions unless an actual DocIndex/WebMCP requirement needs
that behavior. Use the existing native registry and event surfaces first.

## 3. Target Crate Layout

Use separate crates for platform-independent behavior and platform adapters.
The exact names may follow the repository's existing naming convention, but the
responsibilities must remain separate.

```text
docindexrs-core/
  Document model and metadata
  Configuration types
  HTML and chat parsing
  Glossary and synonym expansion
  Normalization and tokenization
  In-memory indexing and ranking
  Backend-independent search results
  Versioned deterministic serialization
  Backend traits

docindexrs-native/
  Filesystem discovery and file metadata
  Native persistent index adapters
  Meilisearch HTTP adapter
  `milli` adapter where its public API is suitable
  Native OxiRS adapter
  Optional RDF/HDT export

docindexrs-wasm/
  `wasm-bindgen` API
  Serialized artifact loading
  Browser-side in-memory search
  Browser fetch/storage adapters where needed
  Optional OxiRS WASM integration

docindexrs-cli/
  Native DocIndex command line interface

sphinxdocrs::extensions::docindex
  Sphinx lifecycle integration
  Document extraction and indexing
  Static artifact export

sphinxdocrs::extensions::webmcp
  WebMCP manifest generation
  WebMCP JavaScript asset registration
  Built-in browser tool contract
```

The shared core must not depend on:

- PyO3;
- `std::fs` or `std::net` as required capabilities;
- Meilisearch client types;
- native-only OxiRS server components;
- native threads or a native async runtime;
- filesystem-backed indexes.

Native and WASM crates may depend on the core. The dependency direction must
not be reversed.

## 4. Compatibility Contract

Port behavior from the recovered Python source and tests before optimizing or
changing the public model.

The initial compatibility contract includes:

- `DocumentType` values:
  - `chat`;
  - `chat_input`;
  - `chat_thinking`;
  - `chat_output`;
  - `sphinx_rst`;
  - `sphinx_md`;
  - `sphinx_nb`;
  - `sphinx_html`;
  - `json`.
- document IDs, titles, content, filenames, URLs, summaries, code snippets,
  build IDs, and metadata;
- source file, tags, concepts, word count, heading level, breadcrumbs, build
  time, and Sphinx role metadata;
- configuration defaults and environment variable names;
- backend selection, including comma-separated multi-backend configuration;
- index names `all`, `chats`, `sphinx`, and `myst`;
- document routing to those indexes;
- indexing statistics and success-rate calculation;
- search result fields, snippets, relevance scores, and matched fields;
- batch submission, retries, task finalization, and error accounting;
- skip-unchanged behavior where a cache is available;
- glossary and synonym behavior;
- static N-Triples/HDT export behavior.

Every ported behavior must be marked as one of:

- exact parity;
- accepted Rust-specific deviation;
- pending parity.

The Python tests in the recovered checkout are the initial fixture inventory.
Rust tests should use equivalent inputs and compare normalized structured
outputs rather than incidental Python representation details.

## 5. Phase A: Core Models and Serialization

Implement `docindexrs-core` first.

### A1. Models and errors

Port the typed configuration and model types from `config.py`:

- `DocumentType`;
- `CodeSnippet`;
- `DocumentMetadata`;
- `Document`;
- `DocIndexConfig`;
- `SearchResult`;
- `IndexingStats`;
- backend-independent error types.

Use `serde` with an explicit versioned schema. Preserve enum string values in
JSON and avoid serializing Rust implementation details.

Inject the clock used for `date_indexed`, so tests and deterministic builds do
not depend on wall-clock time.

### A2. Artifact format

Define a versioned artifact containing:

- schema version;
- index settings;
- documents and metadata;
- normalized searchable fields;
- synonym/glossary configuration;
- optional RDF identifiers and source links.

Start with deterministic JSON for parity and debugging. Add a compact binary
format only after the JSON contract is stable. Both formats must reject an
unknown major schema version and report malformed input without panicking.

Required tests:

- model JSON round trips;
- enum and optional-field compatibility;
- deterministic serialization independent of insertion order;
- malformed and unsupported artifact versions;
- indexing statistics for empty, successful, skipped, and failed inputs.

## 6. Phase B: Parsing, Normalization, and In-Memory Search

Port the pure behavior from:

```text
src/docindex/src/docindex-core/src/docindex_core/html_parser.py
src/docindex/src/docindex-core/src/docindex_core/chat_parser.py
src/docindex/src/docindex-core/src/docindex_core/glossary_manager.py
src/docindex/src/docindex-core/src/docindex_core/synonyms_manager.py
```

Implement in this order:

1. text normalization and tokenization;
2. HTML title, heading, body, code, link, and source extraction;
3. chat document and turn extraction;
4. glossary loading and lookup;
5. synonym loading, expansion, and serialization;
6. document ID and URL generation;
7. in-memory indexing;
8. filtering, pagination, snippets, matched fields, and ranking.

Use Rust-native parsers already present in the workspace where they match the
required behavior. Keep parser output independent of Sphinx and backend
clients.

The in-memory index is also the first WASM implementation. It must support
loading a serialized artifact and searching it without a network or filesystem.

Required tests:

- port all parser fixtures from `test_html_parser.py` and
  `test_chat_parser.py`;
- glossary and synonym parity from the corresponding tests;
- stable IDs and URLs;
- headings, breadcrumbs, code snippets, and metadata;
- empty queries and whitespace-only queries;
- exact, partial, multi-term, and synonym-expanded searches;
- result ordering and stable tie-breaking;
- filters, offsets, limits, and snippets;
- large-document and Unicode input cases.

## 7. Phase C: Backend Traits and Native Adapters

Replace the Python `BaseSearchBackend` contract with smaller Rust traits so a
WASM in-memory implementation does not need to implement native operations.

Recommended separation:

```rust
trait DocumentIndexer {
    fn create_or_update_index(&mut self, settings: &IndexSettings) -> Result<()>;
    fn add_documents(&mut self, documents: &[Document]) -> Result<IndexingStats>;
    fn clear_index(&mut self) -> Result<()>;
}

trait DocumentSearcher {
    fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>>;
}

trait BackendAdmin {
    fn verify_connection(&self) -> Result<bool>;
    fn stats(&self) -> Result<BackendStats>;
    fn delete_index(&mut self) -> Result<bool>;
}

trait IndexArtifact {
    fn export(&self) -> Result<Vec<u8>>;
    fn import(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}
```

The final trait names may differ, but read, write, administration, and artifact
capabilities must be independently implementable.

### C1. In-memory backend

Implement the core in-memory backend first. It is the reference backend for
unit tests, CLI tests, and WASM.

### C2. Meilisearch and `milli`

Port the behavior from `backends/milli.py`:

- connection and health checks;
- index creation and settings updates;
- batch submission;
- task waiting and failure reporting;
- search response mapping;
- statistics, clear, delete, and synonym operations.

Use an HTTP client only in the native adapter. If `milli` can be used directly
without native-only assumptions, add it as an optional native implementation;
do not force its storage model into the shared core.

Preserve API-key boundaries. Credentials must never be written into browser
manifests or WASM artifacts.

### C3. OxiRS

Port the behavior from `backends/oxirs.py` using the existing Rust OxiRS
interfaces where possible:

- local in-memory/native RDF store;
- HTTP/SPARQL endpoint mode;
- named graph per index;
- document-to-RDF quad mapping;
- RDF search and result conversion;
- `dump_ntriples()` equivalent;
- optional HDT export through the existing `rdfhdt` crate.

OxiRS is the semantic/RDF backend. It should not be required to emulate every
ranked full-text feature of Meilisearch. Shared results must make the backend
and relevance semantics explicit where they differ.

### C4. Multi-backend

Port `backends/multi.py` after the individual adapters:

- fan-out indexing;
- backend-specific failures;
- merged search results;
- deterministic deduplication;
- combined statistics;
- clear and administration semantics.

Add mock-based tests for partial failure and backend ordering.

## 8. Phase D: `DocumentIndexer` Orchestration

Port `indexer.py` over the backend traits.

Required operations:

- initialize the configured backend set;
- initialize `all`, `chats`, `sphinx`, and `myst` indexes;
- route documents by `DocumentType`;
- index chat directories;
- index Sphinx HTML directories;
- batch documents;
- retry transient submission failures;
- finalize asynchronous tasks where supported;
- optimize where supported;
- return deterministic `IndexingStats`.

Filesystem traversal, modification times, and progress bars belong in the
native crate. The orchestration should accept an injected document source so
WASM can index already-loaded documents without filesystem access.

Do not use sleeps or native runtime assumptions in the core. Retry timing and
execution policy belong to the native adapter or an injected retry policy.

## 9. Phase E: Native CLI

Port `docindex-cli` after the core and orchestration APIs stabilize.

The CLI must preserve the useful upstream command surface for:

- indexing a directory;
- indexing generated Sphinx HTML;
- choosing OxiRS, Meilisearch/`milli`, or multiple backends;
- batch size and retry controls;
- skipping unchanged files;
- progress output;
- search;
- status and health checks;
- clearing and rebuilding indexes;
- exporting JSON/binary artifacts;
- exporting RDF and HDT.

Use a three-layer design:

1. pure argument parsing;
2. injectable command logic;
3. thin native `main()` with real filesystem, terminal, clock, and network
   implementations.

Required tests:

- help and option compatibility;
- invalid configuration and exit codes;
- in-memory indexing and search;
- filesystem directory indexing;
- skip-unchanged behavior;
- mocked HTTP backend operations;
- artifact and RDF/HDT export;
- progress-disabled deterministic output.

## 10. Phase F: `sphinxdocrs::extensions::docindex`

Implement the DocIndex integration only after the extension prerequisite and
native/core APIs exist.

The built-in extension should:

- register through `sphinxdocrs::extensions::docindex`;
- use Sphinx configuration values and environment variables;
- collect documents from Sphinx output or the native environment;
- support native backend indexing;
- optionally emit a local serialized DocIndex artifact;
- optionally emit RDF/HDT artifacts;
- preserve native Sphinx search as a fallback;
- avoid failing an otherwise successful Sphinx build merely because an
  optional external backend is unavailable, unless explicitly configured as
  required.

Configuration should use namespaced options such as:

```text
docindex_backend
docindex_enabled
docindex_index_name
docindex_batch_size
docindex_html_exclude_patterns
docindex_artifact_enabled
docindex_artifact_path
docindex_rdf_hdt_enabled
docindex_static_wasm_enabled
```

The final defaults must be decided from the upstream behavior and documented
in the compatibility table. Native indexing and artifact generation should be
independently selectable.

The extension should consume `BuildEnvironment` data directly where possible,
not re-parse output HTML unnecessarily. HTML parsing remains available for
post-build indexing and parity with the Python implementation.

Builder applicability:

- `html`: supported;
- `singlehtml`: supported;
- other builders: skip post-build HTML indexing unless they produce a compatible
  HTML artifact, while still allowing explicit artifact generation from the
  environment if implemented.

Required tests:

- built-in extension setup and configuration registration;
- minimal HTML build indexing;
- minimal `singlehtml` build indexing;
- disabled external backend with local artifact enabled;
- failed external backend with configured warning/error policy;
- generated artifact contents and deterministic ordering;
- RDF/HDT export;
- no duplicate indexing on repeated extension loading.

## 11. Phase G: `sphinxdocrs::extensions::webmcp`

WebMCP is a built-in `sphinxdocrs` extension. It is not discovered as the
Python package name `sphinxcontrib.webmcp`.

The native extension name and module path are:

```text
sphinxdocrs::extensions::webmcp
```

The WebMCP extension is always enabled for supported HTML output. It must not
be gated by `docindex_webmcp_enabled`.

The extension applies to both:

- `html` builds;
- `singlehtml` builds.

The existing `docindex_webmcp_enabled` option should therefore be removed from
the native gating logic. For compatibility, it may remain accepted as a
 deprecated configuration value that has no disabling effect, but the plan's
implementation and tests must treat WebMCP as enabled regardless of whether
that value is present or false. A future major release can remove the option
entirely after documenting the behavior change.

### G1. Manifest

Generate a deterministic `webmcp.json` manifest containing the reference
implementation's public schema:

- `schema_version`;
- project, version, and builder;
- public search configuration;
- enabled tool configuration;
- page records;
- navigation root and children;
- native search page and search-index artifact paths;
- doctree schema metadata;
- optional DocIndex artifact metadata;
- optional RDF/HDT artifact metadata.

For `html`, page records point to individual generated pages. For
`singlehtml`, page records and navigation must resolve correctly against the
single generated document rather than assuming one HTML file per source
 document.

Write the manifest atomically and sort pages, navigation children, tools, and
metadata keys for reproducible builds.

### G2. Static JavaScript

Port or embed the reference `webmcp.js` as a package asset managed by the
existing `sphinxdocrs` asset system.

The runtime should:

- detect `document.modelContext` and safely no-op when unavailable;
- load `webmcp.json` relative to the generated site;
- register only configured tools;
- expose page context;
- expose navigation;
- expose documentation metadata;
- expose search;
- expose same-origin navigation;
- support native Sphinx search;
- support local DocIndex/WASM search when an artifact is present;
- support explicitly configured OxiRS and Meilisearch HTTP modes;
- never expose private API keys;
- handle missing, malformed, or unavailable search backends without breaking
  the documentation site.

### G3. WebMCP tools

Preserve the reference tool names and input/output contract:

- `sphinx.get_page_context`;
- `sphinx.list_navigation`;
- `sphinx.get_documentation_metadata`;
- `sphinx.search`;
- `sphinx.navigate`.

The tool schema should be represented in the manifest sufficiently for clients
to understand enabled modes without executing arbitrary Sphinx or Python code.

Required tests:

- WebMCP assets are present in an `html` build even when
  `docindex_webmcp_enabled` is false or absent;
- the same is true for `singlehtml`;
- the old option cannot disable WebMCP;
- manifest page URLs work for both builders;
- manifest contents are deterministic;
- JavaScript registers the expected tools;
- missing `document.modelContext` is harmless;
- same-origin navigation rejects external URLs;
- local artifact search and native search fallback work;
- OxiRS and Meilisearch HTTP modes map responses correctly.

## 12. Phase H: WASM and Browser Delivery

Expose a stable `docindexrs-wasm` API for loading and searching local artifacts.
The first release should prefer a serialized local index over trying to run a
native backend in the browser.

The WASM API should support:

- loading JSON or binary artifacts from bytes;
- querying documents and metadata;
- full-text search and pagination;
- deterministic JSON result output;
- loading optional RDF/HDT data where the existing OxiRS/HDT WASM support is
  proven suitable;
- clear JavaScript errors for malformed artifacts and unsupported operations.

Keep the WASM dependency graph free of native-only filesystem, networking, and
persistent storage requirements. Browser HTTP adapters may be separate from
the artifact-search API.

Add:

- `wasm-bindgen-test` coverage;
- a browser fixture containing a generated DocIndex artifact;
- WebMCP integration tests with a real browser page;
- a `wasm32-unknown-unknown` build in CI;
- size and initialization-time checks once the API stabilizes.

## 13. Phase I: End-to-End Sphinx/WebMCP Workflow

Build a small fixture project containing:

- at least two source documents;
- a nested toctree;
- one `singlehtml` build;
- one ordinary `html` build;
- headings, code, links, and searchable terms;
- DocIndex and WebMCP built-ins enabled by default through the native
  application.

Verify:

1. `sphinx-build-rs -b html` completes.
2. `sphinx-build-rs -b singlehtml` completes.
3. both outputs contain `webmcp.json`;
4. both outputs contain and load `webmcp.js`;
5. both manifests contain valid page and navigation records;
6. a local DocIndex artifact can be loaded and searched in WASM;
7. native Sphinx search remains available;
8. optional OxiRS/HDT assets are generated only when configured;
9. external backend failure follows the configured warning/error policy;
10. repeated builds are deterministic.

## 14. Testing and Parity Gates

Use focused gates at each phase:

```text
cargo test -p docindexrs-core
cargo test -p docindexrs-native
cargo test -p docindexrs-cli
cargo test -p sphinxdocrs
cargo build --target wasm32-unknown-unknown -p docindexrs-wasm
wasm-bindgen-test
```

Also run the recovered Python tests while the parity harness is active:

```text
pytest -q src/sustainablefactory/src/docindex
pytest -q src/sustainablefactory/src/docindex-cli
```

Add parity reports for:

- parsed document JSON;
- search result JSON and ordering;
- indexing statistics;
- backend request payloads;
- generated Sphinx manifests;
- generated static assets and file trees;
- CLI help and exit behavior.

Normalize only known nondeterministic values such as timestamps, temporary
paths, task IDs, and generated hashes. Do not normalize away semantic
behavioral differences.

## 15. Sequencing and Milestones

### M0: Source and extension prerequisite

- Confirm the SustainableFactory checkout and submodule sources.
- Freeze the Python fixture inventory.
- Review and complete the `sphinxdocrs` extension abstraction.
- Add the `sphinxdocrs::extensions` namespace and built-in registry.
- Add extension-loading and duplicate-registration tests.

### M1: WASM-safe core

- Port models, errors, serialization, parsers, normalization, synonyms, and
  in-memory search.
- Establish Python/Rust parity fixtures.
- Produce the first searchable serialized artifact.

### M2: Native backends and orchestration

- Port OxiRS, Meilisearch/`milli`, multi-backend behavior, and the indexer.
- Add native filesystem and HTTP adapters.
- Add RDF/HDT export.

### M3: CLI

- Port `docindex-cli` with native-only I/O in the shell layer.
- Add help, exit-code, mock backend, and artifact tests.

### M4: Built-in DocIndex extension

- Add `sphinxdocrs::extensions::docindex`.
- Integrate Sphinx environment and builder output.
- Support both `html` and `singlehtml`.

### M5: Built-in WebMCP extension

- Add `sphinxdocrs::extensions::webmcp`.
- Make it unconditional for `html` and `singlehtml`.
- Generate manifest and register static runtime assets.
- Integrate native, DocIndex, OxiRS, and Meilisearch search modes.

### M6: WASM/browser integration

- Add `docindexrs-wasm` bindings.
- Load local artifacts from WebMCP.
- Add browser and WASM tests.

### M7: Release hardening

- Complete parity reports.
- Document accepted deviations.
- Add native and WASM CI jobs.
- Publish migration and configuration documentation.

## 16. Acceptance Criteria

The work is complete when:

- DocIndex core parsing, models, indexing, and search run without PyO3 or
  native-only dependencies;
- the same serialized artifact can be searched natively and in WASM;
- OxiRS and Meilisearch/`milli` integrations are implemented as adapters;
- the native CLI covers the supported upstream workflow;
- `sphinxdocrs::extensions::docindex` is a built-in native extension;
- `sphinxdocrs::extensions::webmcp` is a built-in native extension;
- WebMCP is emitted for every successful `html` and `singlehtml` build,
  regardless of `docindex_webmcp_enabled`;
- WebMCP manifests and assets are deterministic and browser-tested;
- native Sphinx search remains functional as a fallback;
- Python/Rust parity tests identify every remaining deviation;
- native and WASM builds pass in CI.

## 17. Open Decisions to Resolve During M0

- Whether `docindexrs` crates belong in the root Cargo workspace or remain in a
  nested workspace like the existing OxiRS repository.
- Which existing Rust HTML and Markdown parser best matches the Python parser
  fixtures.
- Whether the `milli` version available in the environment exposes a stable
  embeddable API or should be used only behind a Meilisearch-compatible adapter.
- Which OxiRS crate and feature set is suitable for `wasm32-unknown-unknown`.
- Whether RDF/HDT browser loading is part of the first WASM release or a second
  artifact-only milestone.
- Whether the deprecated `docindex_webmcp_enabled` configuration value should
  be retained for one compatibility cycle or removed immediately.
- Whether WebMCP's JavaScript should be embedded with `include_str!` or copied
  from a vendored asset directory during packaging.
- How much of Sphinx's Python extension compatibility must be retained for
  third-party extensions while the two built-ins are native.
