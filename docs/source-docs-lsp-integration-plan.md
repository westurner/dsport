# Source Documentation and Optional LSP Integration Plan

Status: in progress
Last updated: 2026-10-15
Scope: `sphinxdocrs` source-aware documentation for Rust and Lean, with optional
integration through a client-side LSP adapter

Implementation status (2026-10-15): phases 1 through 4 have working
implementations with remaining contract, cache, and consumer audit gaps. The
off-by-default LSP client supports document symbols, hover enrichment, safe
definition metadata, normalized diagnostics, bounded framing, provider-scoped
process reuse, and fake-server coverage. Protected LSP selects `mxc` (the
default) or `ai-sandbox` through `source_lsp_sandbox_engine`; both consume the
same `source_lsp_mxc_config` policy. MXC is pinned at
`4b147b0c9491b1d7247824d5677ee4d75c7ca8a1`, and the ai-sandbox adapter at
`7538c3386af8cb6216caaaea457981b69704ad0c`. The adapter supports a strict
policy-only subset of MXC JSON and rejects unrepresentable settings. Linux
omits the Bubblewrap child procfs mount by default while retaining private
PID/network namespaces; explicit procfs mounting remains configurable.
Rust/Lean apidoc has a separate opt-in LSP backend; Python apidoc remains
unchanged. The public inspection tool is `sphinx-source-status` (not
`sphinx-source-port`):
`cargo run -p sphinxdocrs --bin sphinx-source-status`. Its default invocation is
process-free. With `lsp-source-analysis`, `--live --live-fake` exercises the
checked-in fake peer; real servers require explicit
`--live --live-language rust|lean --live-server ... --trusted-local` arguments.

Still incomplete: Sphinx-relative definition-link rendering, references and
workspace-symbol enrichment, the full source-port mapping audit, resource
limits, native macOS runtime validation, adversarial path-replacement tests,
and installed rust-analyzer/Lean live CI. No-proc Bubblewrap, fake-LSP startup,
read-only write denial, and parent-loopback network denial pass in this
container. The opt-in protected no-proc rust-analyzer 1.96.1 test and an
end-to-end Rust LSP apidoc/HTML build pass locally. Lean and other server
versions remain unverified. Protected-build remains unavailable and fails
closed. Static remains the default and requires no LSP dependency or process.

## Summary

H14 currently provides a shared source declaration model, Rustdoc JSON and Lean
static analyzers, source domains, autodoc rendering, apidoc discovery, search
metadata, persistence, and parser-native directives.

The next architectural step is to make those capabilities one source-documentation
pipeline with replaceable providers. Static analysis remains the deterministic
default. An optional Language Server Protocol (LSP) provider can enrich or, when
explicitly requested, replace static analysis for interactive or environment-specific
workflows.

The LSP integration must not make ordinary documentation builds depend on a live
editor process, installed language server, network access, or asynchronous process
availability.

## Decisions

### Keep `source_analysis` as the compatibility boundary

Do not rename `source_analysis` to `apidocrs`.

`source_analysis` is already consumed by:

- `environment` persistence and invalidation
- source domains and cross-reference resolution
- autodoc rendering
- apidoc discovery
- search metadata

The name `apidocrs` would incorrectly imply that source analysis belongs only to
apidoc. It would also make future LSP, IDE, search, and diagnostics integration
look like an apidoc implementation detail.

Introduce a public `source_docs` facade instead. Keep compatibility re-exports from
`source_analysis`, `autodoc`, and `domains::source_domain` during migration.

### Static analysis remains authoritative by default

The default provider is static:

- Rust uses Rustdoc JSON with an explicitly matched `rustdoc-types` schema.
- Lean uses the current Arborium syntax-level analyzer.
- Python continues through its existing runtime/static autodoc paths.

LSP is optional enrichment or an explicitly selected backend. It is not silently
used just because a server happens to be installed.

### Do not make `dscode-lsp` a prerequisite

The upstream [DSCode repository](https://github.com/dmwarenet/dscode) describes
`dscode-lsp` as a standalone Rust crate exposing an `LspClient`, `LspManager`, and
`LspServerPool`. It manages language-server subprocesses over stdio and provides
operations such as document symbols, workspace symbols, hover, definitions,
references, diagnostics, and lifecycle management.

The current `dscode-lsp` manifest inspected on the main branch depends on
`lsp-types`, `tokio`, `serde`, `serde_json`, `tracing`, and `thiserror`; it does
not depend on `tower-lsp`. Its client and `Content-Length` framing are implemented
directly. `tower-lsp` is primarily a server-side `LanguageServer`/`LspService`
framework, so it is not a drop-in replacement for a client that launches and
supervises another language-server process.

For this project, `dscode-lsp` is optional convenience code, not an architectural
requirement. The current experimental internal client uses `serde_json`, standard
library process/thread/channel APIs, and optional `libc` for Unix process-group
signals. It intentionally does not add `lsp-types`, Tokio, or a DSCode dependency
to the root workspace. Any future client-library adapter must still be pinned
and hidden behind the provider boundary.

The [dscode-lsp README](https://github.com/dmwarenet/dscode/tree/main/crates/dscode-lsp)
and [LSP integration guide](https://github.com/dmwarenet/dscode/blob/main/docs/architecture/lsp-integration.md)
are implementation references only. No `dscode_lsp` type should cross the
`sphinxdocrs` source-documentation boundary.

The published crates.io package currently reports `dscode-lsp` `0.1.0`, while the
repository's main branch documents newer examples and a `0.3` ecosystem. Before
adding the dependency, select and verify one exact release or commit. Do not add
an unbounded git dependency.

### Use MXC Policy With Selectable Protected LSP Providers

[Microsoft MXC](https://github.com/microsoft/mxc) is the default protected
provider, pinned as the `src/mxc` submodule at
`4b147b0c9491b1d7247824d5677ee4d75c7ca8a1`. The optional
`source_lsp_sandbox_engine` setting selects `mxc` or `ai-sandbox`; the latter is
pinned as `src/ai-sandbox-review` at
`7538c3386af8cb6216caaaea457981b69704ad0c`. Both optional dependencies are
behind `source-sandbox` and hidden behind an internal provider boundary. The
LSP client owns JSON-RPC framing, piped stdio, request timeouts, shutdown, and
response mapping; sandbox implementation types do not cross the
source-documentation API.

The `mxc` provider builds a typed `ContainerRequest` from argv, the canonical
workspace, explicit server environment, source/toolchain read-only roots, MXC
filesystem/network policy, and a bounded timeout. The `ai-sandbox` provider
serializes the same typed policy to the supported policy-only MXC JSON subset;
the workload command remains a separate argv vector. It rejects denied paths,
mixed read-only/read-write roots, explicit network rules, and partial network
actions rather than silently weakening them. Both providers clear inherited
environment, apply the configured network policy, and fail closed on startup
errors. Safe `conf.py` defaults use `process` containment, deny-by-default
filesystem access, no writable roots, no procfs mount on Linux, and a 30-minute
timeout. The workspace and explicitly configured toolchain roots are mounted
read-only.

MXC exposes procfs selection through the additive typed `ProcfsPolicy` execution
API, not through `FilesystemPolicy` or its stable JSON contract. Its new API
default is `Omit`; legacy `spawn`/`run` entrypoints and existing V1 JSON requests
retain their backend behavior during the compatibility period. The `mxc`
provider maps `source_lsp_mxc_config.filesystem.mount_procfs` to `Omit` or
`Mount` through `spawn_with_procfs_policy`. The `ai-sandbox` adapter applies the
same setting to the generated Bubblewrap argv. On Linux, omission suppresses
only `--proc /proc` and retains the private PID namespace, allowing startup in
nested containers that reject procfs mounts. `LegacyBackendDefault` remains
available for compatibility-sensitive MXC callers.

Known boundaries are explicit: Linux Bubblewrap startup and LSP read-only/network
denial are tested in this container; macOS Seatbelt has not had native runtime
boundary validation. Protected Windows and BSD are unsupported. MXC does not
currently impose CPU, memory, output-size, or process-count limits, and
path-replacement races and adversarial descendant behavior remain under audit.
Do not infer stronger guarantees from policy construction or a successful
capability probe alone. Protected mode fails closed; it never falls back to a
direct child.

`SPHINXDOCRS_SANDBOXING=off` is an explicit global override: a configured
`protected-lsp` process becomes `trusted-local`, runs without an OS boundary,
and is logged as such. This is not a silent failure fallback and does not turn
off LSP itself. `protected-build` remains unavailable; protecting an LSP does
not sandbox Python extensions, directives, themes, or the build process.

The internal MXC provider keeps the sandbox SDK behind the LSP process
boundary. The adapter constructs a typed request and MXC owns the protected
process launch; the LSP client retains stdio and JSON-RPC lifecycle ownership.

The optional dependency is pinned through the `src/mxc` submodule. The adapter
canonicalizes and validates workspace/toolchain roots, supplies an explicit
environment and read-only policy, and fails closed if protected startup fails.
The LSP client owns piped stdio, JSON-RPC framing, request timeouts, shutdown,
and process cleanup. There is no whole-build MXC provider; do not treat LSP
containment as protection for the Sphinx build or its Python extensions.

### Direct-client effort estimate

There are two materially different scopes:

| Scope | Contents | Estimate |
| --- | --- | --- |
| Trusted local developer tool | stdio framing, request dispatch, initialize/initialized, the small read-only method set, per-request timeouts, crash detection, and best-effort shutdown | 3-5 engineer-days |
| Maintained build feature | all of the above plus bounded messages, cancellation, process-group cleanup, sync/async bridge, fake-server tests, cache/session rules, diagnostics, and cross-platform failure handling | 2-4 engineer-weeks |
| Strong isolation for untrusted workspaces | maintained build feature plus pinned MXC integration, verified executor on each supported platform, resource and descendant-process enforcement, environment policy, adversarial tests, and platform-specific CI | 4-8+ engineer-weeks after the executor boundary is available, depending on platform guarantees |

The first estimate is enough to prove the provider contract, not enough to claim
that arbitrary project-selected servers are safe. The second is the minimum
credible implementation for an opt-in local feature. The third is a security
project and should not be hidden inside an LSP adapter estimate.

The direct client work breaks down roughly as follows:

- transport and JSON-RPC dispatcher: 2-4 days
- lifecycle, timeouts, crash monitoring, and process-group termination: 3-6 days
- synchronous build worker and LSP-to-snapshot mapping: 3-5 days
- fake-server, malformed-input, timeout, crash, and cleanup tests: 3-5 days
- MXC provider integration, LSP stdio/process lifecycle, executor validation, and platform-specific sandbox/resource controls: 1-4 weeks

Using `dscode-lsp` removes much of the first three bullets, but not the security
policy, trust model, adapter mapping, cache identity, or tests. Its current client
also needs review before adoption: the implementation uses a fixed 30-second
request timeout, kills rather than gracefully waits for the child in `stop()`,
does not expose a process-group policy, and its pool bookkeeping does not provide
the full isolation and lifecycle guarantees needed by a documentation build.

## Target architecture

```text
sphinxdocrs::source_docs
├── model.rs          declarations, spans, snapshots, diagnostics
├── provider.rs       provider and session contracts
├── static_backend.rs Rustdoc JSON and Arborium Lean adapters
├── lsp_backend.rs    optional client adapter
├── merge.rs          static/LSP reconciliation and provenance
├── domain.rs         source indexes and xref resolution
├── autodoc.rs        source declaration rendering
└── apidoc.rs         source discovery and directive generation
```

The first migration does not need to move all implementation files physically.
Create the facade and re-export existing types first. Relocate code only after the
new ownership boundaries are covered by tests.

### Current-to-target ownership

| Current surface | Target ownership | Migration rule |
| --- | --- | --- |
| `source_analysis.rs` | `source_docs::model` and `source_docs::provider` | preserve type aliases and re-exports |
| `source_analysis/rust.rs` | `source_docs::static_backend::rust` | preserve Rustdoc schema boundary |
| `source_analysis/lean.rs` | `source_docs::static_backend::lean` | preserve syntax-only limitation |
| `domains/source_domain.rs` | `source_docs::domain` | consume only normalized snapshots |
| source portion of `autodoc.rs` | `source_docs::autodoc` | leave Python autodoc/runtime bridge separate |
| source portion of `apidoc/generate.rs` | `source_docs::apidoc` | leave Python module/package apidoc separate |
| `environment.rs` snapshot lifecycle | `source_docs::provider` integration | environment owns storage, not backend logic |
| `search.rs` source objects | `source_docs::domain` projection | retain legacy Python/JS index output |

## Provider contract

Generalize the current `SourceAnalyzer` contract without breaking existing callers.

```rust
pub trait SourceSnapshotProvider {
    fn analyze(
        &self,
        request: &SourceAnalysisRequest,
    ) -> Result<AnalysisSnapshot, AnalysisError>;
}
```

Keep `SourceAnalyzer` as a compatibility alias or blanket adapter during the
migration. The downstream contract is always `AnalysisSnapshot`; downstream code
must not inspect Rustdoc, Arborium, LSP, or subprocess-specific types.

Add explicit backend identity:

```rust
pub enum SourceBackendKind {
    Static,
    Lsp,
    Hybrid,
}
```

The existing `BackendPolicy` can remain the request-level fallback policy, but the
selected backend should be represented separately from error handling. This avoids
confusing "LSP is selected" with "fallback is allowed".

### Session contract

A build may use a short-lived analysis session:

```rust
pub struct SourceAnalysisSession {
    pub static_provider: Option<Box<dyn SourceSnapshotProvider>>,
    pub lsp_provider: Option<Box<dyn SourceSnapshotProvider>>,
}
```

The session owns provider configuration and cache policy. `BuildEnvironment` owns
only normalized snapshots, diagnostics, and invalidation state. Live LSP clients
must not be serialized into doctrees or environment files.

## Documentation products from LSP

LSP is most valuable when it fills gaps in the static source snapshot or adds
optional, clearly scoped reports. It should not replace the deterministic Rustdoc
JSON and Arborium paths for ordinary builds.

| LSP method/data | Sphinx documentation product | Priority | Current state and policy |
| --- | --- | --- | --- |
| `textDocument/documentSymbol` | Nested Rust/Lean API pages and declaration directives | P0 | Implemented behind `lsp-source-analysis`; opt-in trusted-local process; static analysis remains the default. |
| `DocumentSymbol.detail`, `range`, `selectionRange` | Signatures, source spans, and declaration anchors | P0 | Detail/ranges are mapped; selection positions are retained as LSP attributes. Static declaration IDs and policy metadata remain authoritative in hybrid mode. |
| `textDocument/hover` | Fill a missing description/type summary for a statically discovered declaration | P1 | Implemented: bounded plaintext/Markdown is normalized and fills only empty static documentation fields; fake-server coverage exists. Never silently replace static documentation. |
| `textDocument/definition` | Optional “Defined in” source link or cross-file source location | P1 | In-workspace target paths/spans are normalized as safe metadata; rendering a Sphinx-relative link remains open pending a project source-page mapping policy. Never publish editor `file://` URIs directly. |
| `textDocument/publishDiagnostics` | Separate build diagnostics page/report, optionally grouped by file/severity | P1 | Notifications are decoded into normalized `AnalysisDiagnostic` records; a fake-server test checks severity, message, source path, and span. A generated diagnostics page/report remains unimplemented; diagnostics do not become API prose by default. |
| `textDocument/references` | “Used by” lists or reverse-reference reports | P2 | Not implemented. Keep opt-in because results can be large, server-dependent, and expensive. Do not use them to define API membership. |
| `workspace/symbol` | Workspace-wide API index or namespace landing pages | P2 | Not implemented. Prefer static apidoc discovery for page membership; use workspace symbols only for explicitly requested enrichment/discovery. |
| `textDocument/completion`, `signatureHelp`, `semanticTokens` | Interactive completion/signature/token display | Not a generated-doc priority | Better suited to editor integrations. Semantic tokens may eventually help render signatures, but must not be a prerequisite for generated docs. |

Recommended first user-facing additions after document symbols are hover
description enrichment, safe definition links, and a diagnostics report. Each must
be independently configurable, use the normalized snapshot contract, preserve
static provenance/policy, and remain unavailable in the default static build.

## LSP adapter design

### Client lifecycle

The adapter should expose an internal `SourceLspClient` trait. It may be backed by
`dscode-lsp` initially or by a local client implementation. The expected lifecycle
is:

```text
Stopped -> Starting -> Initializing -> Ready -> ShuttingDown -> Stopped
                      \-> Crashed ---------------> Ready/Stopped
```

The target adapter owns:

- server registration by source language
- workspace-root initialization
- provider/session-scoped server reuse, never reuse across workspace roots
- document open/change/save/close state
- request timeout handling
- server shutdown and crash diagnostics

Current implementation has no `LspServerPool`: one `LspSnapshotProvider` owns a
single mutex-guarded child process and serializes calls through it. Reuse is
limited to that provider instance. The current `sphinx-autodoc-rs` and
`sphinx-source-status` entry points create short-lived providers, so reuse across
separate CLI invocations does not occur. Documents are opened, queried for
symbols, and closed for each selected source file; didChange/didSave are not
implemented.

Before spawning the server, the client passes the executable, arguments, working
Before spawning a protected server, the MXC adapter builds a typed request from
the executable argv, canonical workspace, filtered explicit environment,
read-only roots, network policy, and timeout. It launches through MXC with
process stdin/stdout/stderr pipes; the LSP client owns JSON-RPC framing and
request/session lifecycle. Commands remain argv vectors and are never
shell-interpolated.

The local implementation uses a single serialized writer, a reader thread that
dispatches responses and notifications, a pending-request map keyed by JSON-RPC
IDs, an 8 KiB header limit, an 8 MiB default frame limit, and a bounded
notification queue. It rejects malformed framing/JSON-RPC versions, unknown
response IDs, and oversized messages. A request timeout terminates and discards
the session process; it is not reused after timeout. Unknown notifications are
ignored; a full notification queue drops excess notifications instead of
blocking response delivery.

Current implementation covers framing bounds, malformed JSON/version handling,
unknown response IDs, pending-request failure on EOF, per-request timeout, and
document-symbol capability negotiation. Startup and initialize are bounded by
the same request timeout. It drains capped stderr separately but does not yet
attach that capture consistently to returned diagnostics. The client is
synchronous; it does not own an async runtime. Server version is not queried yet.

The lifecycle requirements are more than `Command::spawn()` and `Child::kill()`:

1. Validate an executable policy and argument vector; never invoke a shell.
2. Resolve the configured executable in the parent process, preserve argv
  boundaries, clear the child environment, set an explicit workspace `current_dir`,
  and provide piped stdin/stdout plus a separate drained stderr pipe. The current
  environment allowlist only permits declared locale variables; PATH is used for
  parent-side executable resolution and is not passed to the child by default.
3. Bound initialize and each request by the configured request timeout; shutdown
  uses at most 500 ms. Stderr is drained and capped at 8 KiB, but the captured text
  is not yet attached to diagnostics. There is no independent total-session limit.
4. Monitor child exit while waiting for responses and fail pending calls on EOF,
  protocol failure, or crash.
5. Send `shutdown`, wait briefly for its response, send `exit`, then escalate to
  process-group termination if the child remains alive.
6. On Unix, the current implementation creates a process group and applies
  TERM/KILL escalation. Windows Job Object cleanup, process limits, and guaranteed
  descendant cleanup are not implemented. Cleanup is idempotent for the owned
  process and its Unix process group.
7. Disable automatic restart during a documentation build unless an explicit
  interactive policy enables it. Restarting a compromised or runaway server can
  turn one failure into an unbounded resource loop.

A provider instance owns one process for its lifetime, shuts it down on analysis
errors, and performs best-effort shutdown on drop. This provides reuse only when
the caller reuses that provider instance; the current CLI builds a short-lived
provider per invocation. Server reuse across unrelated roots is not supported.
The target build contract remains explicit session ownership and shutdown on both
normal and error completion.

### Synchronous build boundary

The LSP client is asynchronous, while the current source analysis and build APIs
are synchronous. Do not spread async through `BuildEnvironment` in the first
phase.

The current implementation uses standard-library threads and channels behind the
synchronous provider interface. It does not use Tokio, hold a Python GIL, or hold
a `RefCell` borrow while waiting. A Tokio worker remains an option only if later
APIs require async features; it is not a current dependency.

The original alternatives, if the client is replaced, are:

1. A worker owning a Tokio runtime and an `LspServerPool`, with synchronous request
   and response channels exposed to the build thread.
2. A blocking wrapper supplied by dscode-lsp, if the pinned version provides one
   and its runtime-drop behavior is safe in all test contexts.

The existing thread/channel approach makes lifecycle, timeouts, and shutdown
explicit. Any future async worker must not hold a `RefCell` borrow or Python GIL
interaction while waiting for an LSP response.

### LSP-to-snapshot mapping

Convert LSP responses into the existing normalized declaration model:

| LSP data | Snapshot field |
| --- | --- |
| `DocumentSymbol.name` | `short_name` and qualified-name input |
| symbol container hierarchy | `parent` and `children` |
| `DocumentSymbol.kind` | `DeclarationKind` |
| `range` | `SourceSpan` |
| `selection_range` | namespaced `lsp:selection_start` / `lsp:selection_end` attributes |
| `detail` | signature/type text |
| hover markdown/plaintext | normalized documentation for declarations lacking static documentation |
| definition locations | safe workspace-relative path and span metadata; not rendered as a documentation link by default |
| diagnostics | `AnalysisDiagnostic` |
| unsupported server metadata | `attributes`, namespaced by backend |

The current adapter maps hierarchical `DocumentSymbol` arrays and preserves the
source language namespace separator (`::` for Rust, `.` for Lean). It records
selection positions in `lsp:` attributes and maps `detail` to signature text for
functions/methods or type text for other kinds. It accepts string or markup-object
documentation values. Unsupported kinds, missing names, and malformed ranges
become diagnostics. LSP-only declarations use `Visibility::Unknown`; aliases,
re-exports, deprecation, and noindex policy are not inferred. LSP names that cannot
be normalized safely must remain diagnostics rather than guessed xref targets.

LSP servers generally do not provide all Sphinx policy metadata. Static analysis
therefore remains authoritative for:

- visibility
- aliases and reexports
- `deprecated`
- `noindex`
- stable declaration IDs
- documentation ordering

## Backend modes

Expose four effective modes:

```text
static  # deterministic static provider only
lsp     # require the configured LSP provider
hybrid  # static base plus optional LSP enrichment
auto    # static unless explicitly configured otherwise
```

Recommended behavior:

| Mode | Server unavailable | Static backend unavailable | Intended use |
| --- | --- | --- | --- |
| `static` | never starts a server | error | CI and reproducible releases |
| `lsp` | error | error | interactive, server-driven workflows |
| `hybrid` | diagnostic plus static result | error unless another fallback is configured | richer local builds |
| `auto` | static result | existing backend policy applies | compatibility default |

`--full` apidoc generation must remain static by default. An explicit option is
required to use LSP so a project cannot unexpectedly start a compiler or language
server during a normal documentation build.

## Configuration

Add source-documentation settings rather than hard-coding server commands:

```python
source_backend = "hybrid"
source_lsp_servers = {
  "rust": ["rust-analyzer", "--stdio"],
  "lean": ["lake", "env", "lean", "--server"],
}
source_lsp_sandbox = "protected-lsp"
source_lsp_sandbox_engine = "mxc"  # mxc or ai-sandbox; default mxc
source_lsp_mxc_config = {
  "containment": "process",
  "filesystem": {
    "mount_procfs": False,
    "readonly_paths": [],
    "readwrite_paths": [],
    "denied_paths": [],
  },
  "network": {
    "egress": {"default": "deny"},
    "ingress": {"default": "deny", "host_loopback": "deny"},
  },
  "timeout_ms": 1800000,
}
```

The MXC config shown here makes the protected-mode defaults explicit. The typed
`source_lsp_mxc_config` override accepts only containment, filesystem paths and
`mount_procfs`, directional network actions, and a timeout up to 30 minutes.
`source_lsp_sandbox_engine` selects the direct MXC SDK or the ai-sandbox adapter;
both consume this same policy model. The ai-sandbox adapter serializes it to the
supported MXC policy JSON subset and rejects filesystem/network settings it
cannot preserve.
Writable paths and network allowances should be added only for a documented
server requirement. `SPHINXDOCRS_SANDBOXING=off` deliberately changes a
configured protected LSP to trusted-local; it is logged and is not used as an
automatic fallback. The Lean command remains configurable because installations
differ by project and package manager.

`sphinx-apidoc-rs` keeps source language and analysis backend separate. Its
`--source-mode {python,rust,lean,auto}` selects discovery rules;
`--source-backend {static,lsp}` selects Rust/Lean analysis, defaulting to
static; and `--conf-file PATH` supplies LSP server and MXC settings. Python
apidoc remains on its existing path. The status tool supports
`--live --live-fake` or explicit real-server options. Commands are argv vectors,
not shell strings.

```text
--source-mode {python,rust,lean,auto}
--source-backend {static,lsp}
--conf-file PATH
```

Commands should report the selected backend and fallback decision in verbose mode,
without printing server arguments that may contain sensitive paths or tokens.

## Hybrid merge policy

The merge order is deterministic:

1. Run the static provider.
2. Normalize and sort its declarations.
3. Current client queries document symbols; future clients should query other
  methods only for explicitly configured enrichments or missing information.
4. Match declarations by stable source path and range first, then qualified name.
5. Apply only fields for which the LSP response has sufficient confidence.
6. Preserve static policy fields and stable IDs.
7. Append unmatched LSP declarations only when the request explicitly allows them.
8. Sort declarations and diagnostics through the shared snapshot normalizer.

Every enriched field should have provenance internally, even if provenance is not
initially serialized in the public snapshot:

```text
static
lsp
merged-static-lsp
```

Conflicts must not be silently resolved. Record an `AnalysisDiagnostic` with both
backend names and retain the static value for policy-sensitive fields.

## Persistence and cache identity

Extend snapshot cache identity with:

- backend kind
- client adapter implementation and version
- server command and arguments
- server version, when available
- workspace-root URI
- requested LSP capabilities
- timeout and fallback policy
- source request identity
- source input hash

Example metadata:

```text
backend = "lsp:<adapter>"
backend_version = "<adapter-version>"
```

A cached LSP snapshot is invalid when the server configuration, workspace root,
capabilities, source files, or relevant toolchain version changes. A live server
must never be persisted or reused across unrelated projects.

Current LSP identity hashes the source request, language, workspace root, request
timeout, message limit, configured argv, and explicit environment entries. The
argv/environment values are hashed rather than written to persisted metadata.
The server version is not queried, and the current implementation requests one
fixed document-symbol capability set, so server-version/capability negotiation
invalidation remains to implement.

The environment should continue to persist only `AnalysisSnapshot`, diagnostics,
and backend metadata. On reread or invalidation, clear source-domain records before
registering the replacement snapshot.

## Domains, autodoc, apidoc, and search

All consumers remain backend-neutral:

- **Domains** resolve xrefs against the merged `SourceIndex`.
- **Autodoc** renders declarations from `SourceSnapshot`.
- **Apidoc** discovers source files statically, then asks the selected provider for
  declarations.
- **Search** indexes stable `SourceObjectEntry` records.
- **Environment** stores snapshots and controls invalidation.
- **Parser directives** consume normalized directive/signature records.

LSP definition locations are source-analysis inputs, not final documentation URLs.
Do not emit editor-specific or `file://` links into generated documentation unless a
separate project policy explicitly requests them.

Python `autodoc` and Python module/package `apidoc` remain separate paths. The
`source_docs` facade should own only Rust/Lean source-aware behavior and shared
contracts.

## Security and operational policy

Starting a language server executes a project- or user-selected process. The LSP
feature must therefore be opt-in and observable.

Required controls:

- no server startup in default static mode
- executable and arguments resolved through explicit configuration
- workspace root constrained to the configured project tree
- no shell interpolation when spawning commands
- bounded request and startup timeouts
- bounded response size and diagnostic count
- server stderr captured separately from generated documentation
- clear process shutdown and crash reporting
- no network access requirement in the core build path
- no secrets copied into logs or snapshot metadata
- optional feature disabled in minimal/package builds

These controls protect the build process from a broken client or server; they do
not by themselves sandbox a language server. An LSP server is an executable with
the permissions of its parent and may inspect the workspace, home directory,
credentials, network, and child processes unless an OS boundary prevents it.

### Sandbox modes and scope

Keep sandbox policy separate from analysis backend selection:

| Mode | `sphinxdocrs` build | LSP server | Failure behavior |
| --- | --- | --- | --- |
| `static` | normal process; no server | not started | static errors only |
| `trusted-local` | normal process | direct child with explicit environment | trusted input only; no OS security boundary |
| `protected-lsp` | normal process | selected engine (`mxc` by default or `ai-sandbox`), using Bubblewrap on Linux and Seatbelt on macOS | fail closed if the selected provider or platform setup fails |
| `protected-build` | unavailable | no build sandbox provider is enabled | fail before starting the build |

`protected-build` and `protected-lsp` are distinct: protecting the language
server does not make Python extensions, themes, directives, or the Sphinx build
process safe. No whole-build sandbox provider is implemented, so do not imply
that the LSP policy covers the builder.

Protected LSP defaults are intentionally narrow:

- deny-by-default filesystem policy; mount the workspace and declared toolchain
  roots read-only;
- no writable paths by default;
- deny network egress, ingress, and host loopback;
- clear inherited environment and pass only explicit server settings plus the
  locale-only internal allowlist;
- use a bounded MXC timeout (30 minutes by default);
- omit child `/proc` on Linux while retaining Bubblewrap's private PID namespace.

`source_lsp_mxc_config` is a typed `conf.py` override surface shared by both
engines. It permits
containment selection (`process`, `bubblewrap`, or `seatbelt`), filesystem
read-only/read-write/denied paths and `mount_procfs`, network default actions,
and a timeout capped at 30 minutes. Configured writable paths and network
allowances expand server authority and should be justified by a concrete
toolchain requirement. Configured roots are checked for absolute non-root paths
and canonicalized before MXC launch; adversarial path replacement still needs
testing.

MXC's procfs choice is not serialized in the stable JSON request contract.
`ProcfsPolicy::Omit` is the least-privilege selection for new typed callers;
`Mount` explicitly requests a fresh procfs, and `LegacyBackendDefault` preserves
the historical behavior. Existing `spawn`/`run` and V1 JSON entrypoints remain
compatible in V1.0.x. The tentative schedule is to deprecate the unqualified
typed entrypoints in V1.1 without changing runtime behavior, then make them use
`Omit` in V2.0 while retaining `LegacyBackendDefault` and V1 JSON semantics. See
the MXC [Bubblewrap procfs policy and migration notes](../src/mxc/docs/backends/bwrap/bubblewrap-backend.md#procfs-policy-and-migration).

The sandbox-off override is deliberately unsafe and explicit:
`SPHINXDOCRS_SANDBOXING=off` changes a configured protected LSP to
`trusted-local`, logs the override, and does not disable LSP or silently handle
MXC failure. Protected mode otherwise fails closed. The static backend remains
the reproducible default and starts no process.

### MXC integration status

MXC is the `src/mxc` Git submodule pinned at
`4b147b0c9491b1d7247824d5677ee4d75c7ca8a1`; the optional Rust SDK is behind
`source-sandbox`. The ai-sandbox adapter is pinned at
`7538c3386af8cb6216caaaea457981b69704ad0c`. The `mxc` provider owns typed
policy construction and MXC spawn; the ai-sandbox provider maps the shared
policy to its supported MXC JSON subset. The LSP client owns JSON-RPC stdio and
session lifecycle.

| Area | Current implementation and remaining limit |
| --- | --- |
| Linux | Bubblewrap protected launch, no-proc control, explicit read-only roots, cleared environment, network deny, fake-server startup, write-denial and host-loopback denial tests pass in this container. Path-replacement races and resource limits remain open. |
| macOS | MXC Seatbelt selection is wired, but native runtime filesystem/network boundary validation remains pending. |
| Windows and BSD | Protected LSP is unsupported and fails closed. |
| Process/resource limits | MXC timeout is configured; CPU, memory, output-size, and process-count enforcement are not implemented. Descendant lifecycle behavior requires additional adversarial testing. |
| Supply chain | Both providers are pinned by submodule commit and Cargo.lock; update deliberately and review provider changes before moving either pin. |

The protected rust-analyzer 1.96.1 no-proc test and a Rust LSP apidoc plus HTML
build pass locally. A policy-construction success alone is not a boundary test;
continue testing denied filesystem/network access, symlink/path replacement,
descendant cleanup, and each platform before expanding security claims.

## Testing strategy

### Provider contract tests

- deterministic declaration ordering
- stable IDs and source spans
- visibility filtering
- aliases and parent/child relationships
- deprecated and noindex filtering
- diagnostics preservation
- request/cache identity
- serialization round-trip

### Pure LSP mapping tests

Use canned LSP-shaped values to test:

- symbol-kind mapping
- nested document symbols
- ranges and UTF-16-to-UTF-8 position conversion
- hover markdown normalization
- detail/signature extraction
- definition location normalization
- malformed and incomplete responses
- unsupported symbols becoming diagnostics

These tests must not require a language server executable.

### Fake stdio server tests

The checked-in fake peer at `src/sphinxdocrs/tests/fixtures/h14/fake_lsp.py`
currently covers:

- initialize and initialized notifications
- document symbol response
- workspace symbol response
- hover and definition response
- diagnostics notification (notification decoding is implemented; dedicated mapping coverage remains to add)
- delayed response and timeout
- malformed JSON-RPC response
- server crash during a request and during initialize (captured stderr, no timeout)
- shutdown after success and failure

The fake server should live in test support and never be used by production code.

### Hybrid and integration tests

Cover:

- static mode never spawns a process
- required LSP mode fails clearly when the command is absent
- hybrid mode falls back while retaining a diagnostic
- LSP enrichment does not change stable IDs or policy fields
- conflicting static/LSP declarations produce deterministic diagnostics
- server configuration changes invalidate snapshots
- repeated builds reuse valid snapshots
- server processes terminate after the build
- static and hybrid search output is identical when LSP adds no data
- source-domain xrefs remain deterministic across backend modes

Live tests using installed `rust-analyzer` or Lean servers should be optional and
excluded from normal CI. They should be enabled by an explicit feature or
environment variable and report a skipped result when the server is unavailable.

Current automated LSP tests cover fake-server initialize/document-symbol/shutdown,
provider-scoped process reuse, initialize-time exit with stderr capture, timeout,
crash during a request, diagnostics notification normalization, framing limits,
malformed JSON-RPC/version, UTF-16 position conversion, symbol-kind mapping,
missing executable resolution, workspace escape rejection before spawn, and
configuration/cache secrecy. The status command can invoke the shared fake server
with `--live --live-fake`; the command remains process-free without that flag.
Remaining lifecycle gaps include true request cancellation, Windows process-tree
cleanup, and cross-platform lifecycle tests.
The full Sphinx library suite has known environment failures in Python 3.14
`typing` import behavior, theme/config expectations, and a symlink fixture; the
focused source-doc and LSP suites pass.

## Reducing future porting overhead

### Source-status manifest

Add a manifest under `docs/` or `src/sphinxdocrs/tests/fixtures/` recording one row
per ported behavior:

```text
upstream Python symbol
Rust owner
source backend
public contract
parity fixture
accepted deviation
feature gate
```

For LSP-backed behavior, also record:

```text
LSP method
required capability
fallback behavior
provenance policy
live-server test
```

### Contract-first workflow

For each upstream behavior:

1. Identify the Sphinx symbol and its observable output.
2. Add or update one source-neutral contract test.
3. Implement the static provider, LSP adapter, or merge rule.
4. Run the same domain, autodoc, apidoc, persistence, and search tests.
5. Record accepted deviations in the manifest and compatibility documentation.
6. Run live-server tests only when the required executable is explicitly present.

This prevents each backend from acquiring a separate interpretation of visibility,
anchors, aliases, or xrefs.

### Status command

The implemented `sphinx-source-status` command reads the manifest and can run
source-doc contract tests, the parity integration target, the checked-in fake LSP,
or an explicitly configured trusted-local server. It reports live checks as
skipped when the optional LSP feature is unavailable. It does not yet inspect the
Rust directive/role mapping tables or discover accepted deviations from tests.
Remaining command work:

- lists upstream symbols mapped to source-documentation owners
- reports missing declaration-kind/directive/role mappings
- runs the shared provider contract suite
- runs selected Python/Rust parity fixtures
- reports accepted deviations and their test names
- optionally exercises configured LSP servers (single-language analysis only)

The command should be usable without LSP installed. Its output should distinguish
"not implemented", "accepted deviation", "backend unavailable", and "test skipped".

## Delivery phases and status

Status labels describe implementation progress, not security approval:
**implemented** means code and focused tests exist; **implemented with gaps** and
**partially available** mean exit criteria remain open. No protected sandbox mode
is approved by these labels.

### Phase 1: facade and contracts — implemented with contract-coverage gaps

- Add `source_docs` module with compatibility re-exports.
- Introduce `SourceSnapshotProvider` and backend identity.
- Move source-specific autodoc/apidoc/domain contracts behind the facade.
- Add the shared provider contract test suite.
- No dscode dependency yet.

Status: the public facade and compatibility re-exports are present. Shared
provider-contract tests cover the Rustdoc, Lean, and LSP providers plus static,
LSP, and hybrid sessions. Broader consumer parity remains tracked in phase 4.

### Phase 2: provider and persistence cleanup — implemented with cache-audit gaps

- Extract `SourceAnalysisSession`.
- Centralize backend selection and request identity.
- Add snapshot provenance internally.
- Add source-status manifest and focused status command.
- Verify static builds remain process-free and deterministic.

Status: static is the default; session mode, request/cache identity, per-field
in-memory provenance, hybrid conflict diagnostics, unmatched-LSP opt-in, and
manifest command exist. Rust cache identity includes the rustdoc schema and
hashed toolchain identity; Lean identity includes the pinned grammar version.
LSP identity includes configured command arguments, workspace, client-requested
and negotiated capabilities, timeout, serverInfo when available, and source
request. Continue auditing every toolchain/configuration input for invalidation.

### Phase 3: optional LSP adapter — selectable protected providers integrated with gaps

- Define the internal `SourceLspClient` interface and fake-server contract.
- Use the local JSON-RPC client behind `SourceLspClient`; no dscode or
  `lsp-types` dependency crosses the provider boundary.
- The current local adapter uses `serde_json`, standard library threads/channels,
  and optional `libc`; no dscode or `lsp-types` dependency is present.
- Add `lsp-source-analysis` as an off-by-default Cargo feature.
- The off-by-default `source-sandbox` feature provides an MXC SDK provider and
  an ai-sandbox provider, selected by `source_lsp_sandbox_engine` (default
  `mxc`). They share `source_lsp_mxc_config`; ai-sandbox accepts only the
  documented, fail-closed MXC policy JSON subset.
- Linux protected LSP uses MXC Bubblewrap with explicit read-only roots, cleared
  explicit environment, network denial, and piped stdio. The typed no-proc
  option omits only the child `/proc` mount and retains the private PID
  namespace. macOS uses MXC Seatbelt selection but still needs native runtime
  boundary validation.
- Protected Windows and BSD modes remain unavailable. CPU, memory, output, and
  process-count limits are not implemented; descendant cleanup and path-race
  resistance need further adversarial tests.
- `source_lsp_mxc_config` exposes narrow `conf.py` overrides with safe defaults.
  `SPHINXDOCRS_SANDBOXING=off` is a logged, explicit switch from protected LSP
  to trusted-local, not a silent fallback.
- Sphinx configuration supports per-language server environment and additional
  read-only roots. Inherited environment remains cleared; unsafe loader/runtime
  keys, relative PATH entries, and invalid filesystem roots are rejected.
- Implement the lifecycle worker and normalized document-symbol mapper.
- Add fake-server tests and timeout/shutdown coverage.
- Add explicit source-backend configuration and trusted-local CLI selection.

Status: the optional feature remains off by default; fake-server snapshots,
hover/definition normalization, both protected provider paths, config overrides,
and the sandbox-off switch are implemented. MXC procfs parsing and the
ai-sandbox policy adapter have dedicated cargo-fuzz targets; bounded Linux
campaigns completed without crashes. Source paths are canonicalized and
constrained before launch. Linux no-proc startup, fake-LSP analysis, read-only
write denial, and parent-loopback network denial pass in this container. The
protected no-proc rust-analyzer 1.96.1 test passes with an explicit toolchain
library path. Native macOS validation, Windows/BSD support, path-replacement
races, descendant behavior, and resource limits remain open. `trusted-local` is
not a security boundary. Real server runs remain explicit opt-ins.

The provider mutex serializes access to its single child; reusing that child is
limited to the lifetime of one `LspSnapshotProvider` instance. The current CLI
constructs a provider per invocation, so this is not a cross-build or global
server pool. Startup/initialize uses the configured request timeout. Optional
serverInfo and negotiated capabilities participate in provider identity. Hover
enrichment and safe definition metadata are implemented; references are not.
Definition metadata is not yet rendered as documentation links because source
page mapping is project-specific.

### Phase 4: hybrid merge and source consumers — core merge implemented

- Implement static-base/LSP-enrichment merge rules.
- Add provenance and conflict diagnostics.
- Wire merged snapshots into domains, autodoc, apidoc, and search.
- Add cache invalidation for LSP configuration and server versions.

Status: core consumers use normalized records and hybrid mode retains static
policy fields with conflict diagnostics and provenance. Search/xref parity when
LSP adds no information and full environment-cache integration still need
dedicated tests. Rust/Lean apidoc separates language discovery from backend
selection; its opt-in LSP path adds an aggregate `lsp-api.rst` to the generated
TOC, while Python apidoc remains unchanged. A real Rust LSP apidoc and native
HTML build pass locally. The automated fake-server CLI test now generates the
aggregate Rust API page and TOC, runs the native HTML builder against those
generated sources, and asserts the LSP-derived declaration appears in the
rendered page.

### Phase 5: optional live-server workflows — Rust validated locally; CI pending

- Add opt-in rust-analyzer integration tests (`SPHINXDOCRS_RUST_ANALYZER` supplies
  the executable path; unset means the test skips).
- Add opt-in Lean language-server integration tests
  (`SPHINXDOCRS_LEAN_LSP_COMMAND` supplies a JSON argv array; unset means skip).
- Document server installation and project-specific commands in
  [`SOURCE_DOCS_LSP.md`](SOURCE_DOCS_LSP.md).
- Maintain MXC boundary tests for protected LSP on each supported platform;
  whole-build sandboxing remains unavailable and is not implied by these tests.
- Evaluate `dscode-session` separately; it must not replace the sandbox provider
  or process-policy boundary.

Status: status command can run the fake peer or an explicitly supplied
trusted-local server. The opt-in rust-analyzer 1.96.1 test passes in protected
Linux no-proc mode; its explicit `/usr/bin/env` argv supplies the installed
toolchain's `LD_LIBRARY_PATH`, without relaxing the sandbox environment filter.
The Lean live test accepts a JSON argv array and skips when no command is
configured. Installed-server CI, broader version/project compatibility,
platform boundary tests, and Lean live-server validation remain pending.

Exit criteria: live integrations are useful for development but remain unnecessary
for normal package builds, release builds, and deterministic CI; protected mode
fails closed when its platform capability probe or boundary test cannot pass.

## Decisions resolved and open questions

Resolved:

- Use the local JSON-RPC client behind `SourceLspClient`; do not add dscode-lsp or
  `lsp-types` to the root workspace currently.
- The client is synchronous and uses standard threads/channels. No Tokio runtime is
  needed for the current method set.
- Keep LSP optional in the main `sphinxdocrs` crate behind
  `lsp-source-analysis`; the default feature set does not include it.
- Unmatched LSP declarations are excluded by default and require an explicit hybrid
  session option to append.
- Field provenance remains in-memory and is not serialized in `AnalysisSnapshot`.
- Diagnostics remain normalized snapshot metadata only by default; no generated
  diagnostics page or build-warning stream is emitted without a future explicit
  output policy.
- Definition locations remain safe workspace-relative metadata. They are not
  rendered as links until a project-specific source-page mapping is configured.
- Cache identity uses optional LSP `serverInfo` when supplied; do not shell-probe
  server versions. Missing `serverInfo` remains explicitly unreported.
- Keep the optional client inside `sphinxdocrs` until its API is stable enough to
  justify a separate crate.
- Do not prescribe one Lean server command; live tests accept the project's
  explicit JSON argv so Lake/toolchain setup remains caller-controlled.
- `sphinx-source-status` is the inspection command name. Protected LSP is opt-in
  and fails closed; protected-build remains unavailable.

Still open:

1. What evidence should gate making the experimental source-doc APIs stable before
  publication?

## Acceptance criteria and remaining work

Implemented criteria:

- `source_docs` is the documented public facade.
- Existing `source_analysis` imports continue to compile during migration.
- Static Rust and Lean analysis remains deterministic and fully usable without LSP.
- The current local LSP client is optional and isolated behind a feature; no
  third-party LSP client type crosses the provider boundary.
- `protected-lsp` is opt-in through `source-sandbox`, selects MXC by default or
  the explicit ai-sandbox adapter, shares the documented MXC policy defaults,
  and fails closed if protected startup is unavailable; it never falls back to
  a direct child. The explicit
  `SPHINXDOCRS_SANDBOXING=off` override selects trusted-local and is logged.
  `protected-build` remains unavailable and fails closed.
- LSP lifecycle failures produce explicit errors; notification diagnostics are
  normalized and retained only in snapshots by default.
- Domains, autodoc, apidoc, persistence, and search consume only normalized source
  records.
- Future upstream port work can be tracked through the source-port manifest and
  focused porting command.

The plan is not complete until the remaining acceptance criteria below land:

- Continue reviewing MXC changes against the pinned submodule commit and the
  adapter's typed policy construction; policy construction alone is not proof of
  isolation.
- Linux Bubblewrap no-proc startup, read-only write denial, and parent-loopback
  network denial are tested. Add path-replacement and descendant-lifecycle
  tests; validate Seatbelt boundaries natively on macOS before claiming runtime
  support there. Windows and BSD remain unsupported. Do not claim broader
  guarantees than the tests establish.
- References/workspace-symbol output remains optional and must not define API
  membership. Definition links remain deferred until a project source-page mapping
  policy is explicitly configured.
- Continue auditing cache invalidation for every relevant server/toolchain input;
  optional serverInfo and negotiated capabilities already contribute to LSP
  provider identity.
- Complete shared provider contract coverage and static/hybrid parity tests for
  domains, autodoc, apidoc, environment persistence, and search.
- Opt-in rust-analyzer and Lean live-test harnesses, diagnostics-notification,
  pre-initialize crash, and malformed-response tests exist. Run installed-server
  CI and add cross-platform lifecycle cases before treating coverage as complete.
- Complete source-port mapping audits and shared contract tests for every static,
  LSP, and hybrid provider, including static/hybrid parity in domains, autodoc,
  apidoc, environment persistence, and search.
- Audit source visibility/type mapping and the source-port manifest against
  upstream behavior; do not label local tests as exact parity without an
  upstream comparison fixture.
