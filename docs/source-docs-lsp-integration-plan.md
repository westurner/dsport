# Source Documentation and Optional LSP Integration Plan

Status: in progress
Last updated: 2026-10-04
Scope: `sphinxdocrs` source-aware documentation for Rust and Lean, with optional
integration through a client-side LSP adapter

Implementation status (2026-10-02): phases 1 and 2 are implemented; phase 3 has
an off-by-default trusted-local JSON-RPC client for
`textDocument/documentSymbol`, bounded framing, source mapping, diagnostics,
provider-scoped process reuse, and fake-server tests. The public inspection tool
is `sphinx-source-status` (not `sphinx-source-port`):
`cargo run -p sphinxdocrs --bin sphinx-source-status`. Its default invocation is
process-free. With `lsp-source-analysis`, `--live --live-fake` exercises the
checked-in fake peer; real servers require explicit
`--live --live-language rust|lean --live-server ... --trusted-local` arguments.

Security review update (2026-10-04): `libc` is an optional Unix-target dependency
used to signal the LSP process group; `std::process::Child::kill()` only
terminates the direct child. Windows uses an optional `windows-sys` Job Object
with kill-on-close and a 64-process limit. Assignment occurs just after spawn,
so this is best-effort cleanup containment rather than a race-free security
boundary. The adapter bounds header reads before allocation, rejects
duplicate/malformed `Content-Length`, enforces a five-minute per-request and
30-minute provider-session ceiling, caps source documents at 1 MiB, outgoing and
incoming frames at 8 MiB, each analysis at 2,048 files/100,000 directory
entries/10,000 combined declarations and diagnostics, and permits only canonical
symlink targets within the workspace while skipping external targets and cycles.
Executables are resolved to absolute paths in the parent; the child starts with
a cleared environment and no inherited `PATH`. Diagnostic URIs are decoded and
resolved only to canonical files inside the configured workspace. These are
trusted-local hardening controls, not an OS sandbox or a protected-mode claim.
Linux tests cover process-group descendant cleanup. The Windows Job Object API
was checked in isolation, but full Windows runtime/cross-build validation was
unavailable because the container lacks its native C/PyO3 cross toolchain.
macOS uses the Unix process-group path but still needs native runtime validation.

Still incomplete: hover/definition/references/workspace-symbol enrichment,
server-version probing, the full source-port mapping audit, CPU/memory/output
limits, native Windows/macOS runtime process-tree tests, an audited
`SandboxProvider`, platform boundary tests, and installed rust-analyzer/Lean live
CI. Protected modes remain unavailable and fail closed. Unix process groups and
Windows Job Objects provide best-effort cleanup, not OS sandboxing. Static remains
the default and requires no LSP dependency or process.

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

### Use `ai-sandbox` as the sandbox provider, not the process manager

The [Teckwin/sandbox](https://github.com/Teckwin/sandbox) crate is named
`ai-sandbox`. The initial review was against tag `v0.2.1` (`d9df470`); a
follow-up review of the workspace's security-hardening commit
`4cef879225e3bd177aa2fff1ca7d8771ae108a67` changes the executor assessment
below. The manifest still reports version `0.2.1`, so that version number alone
does not identify the reviewed implementation. The crate provides a
cross-platform policy and command-transformation API:

- `SandboxManager`, `SandboxPolicy`, and `SandboxCommand` describe the requested
  filesystem and network policy.
- `SandboxExecRequest` returns a transformed argv, working directory, and
  environment, and now exposes `spawn()`, `run(timeout)`, and `wait()` entry
  points for its supported executors.
- Linux command transformation and execution use Bubblewrap with explicit
  filesystem mounts, user/PID/IPC namespaces, session isolation, and network
  namespace isolation for `NoAccess`. `spawn()` probes whether Bubblewrap can
  create the required namespaces before starting the transformed command.
- macOS command transformation invokes Seatbelt through `sandbox-exec`; Linux
  and macOS are the only platforms currently wired into the
  `SandboxExecRequest` execution path.

The bundled `sandbox-exec` CLI now creates a request and waits for the protected
child instead of only printing a request. The executor is still not a complete
LSP process manager or a cross-platform security guarantee:

- The bundled CLI's `workspace` mode currently passes `.` as its writable root,
  while `SandboxPolicy::is_safe()` requires absolute roots. Canonicalize CLI
  roots before policy validation or correct the CLI before relying on that mode.

- Bubblewrap enforces Linux filesystem mounts and `NoAccess` networking; it
  rejects unsupported `Localhost` and `Proxy` network policies. Its writable
  roots are canonicalized and filesystem root is rejected, but mount paths are
  still passed by name, leaving a path-replacement race between validation and
  Bubblewrap startup. It does not provide CPU, memory, output, or process-count
  limits, and it does not install Landlock or a seccomp policy.
- Landlock APIs now explicitly describe capability/ruleset metadata only; they
  do not apply Landlock syscalls and are not part of the active executor.
- Seatbelt generates and launches a policy on macOS, rejects unsupported Proxy
  policy, and has policy-generation tests. Runtime filesystem/network boundary
  tests on macOS are still required before making a platform-specific isolation
  claim.
- Windows Restricted Token and BSD Capsicum/pledge helpers remain in the crate,
  but those backends return unsupported from the `SandboxExecRequest`
  transformation/execution path. They must not be advertised as available
  protected execution through this API.
- `SandboxExecRequest::spawn()` inherits stdin/stdout/stderr. `run(timeout)`
  supervises and kills only the direct child on timeout; `wait()` has no
  timeout. Neither provides piped LSP stdio, process-group/job cleanup, or
  descendant-process limits. The LSP adapter must own that lifecycle and must
  not use these methods as a substitute for its JSON-RPC process manager.
- The command policy now rejects executable-prefix aliases for Allows, requires
  exact literal argument tokens, applies component-aware cwd and path checks,
  gives matching Deny/restriction rules precedence, and detects absolute-path
  `chmod` SUID/SGID modes. These policy checks complement OS isolation; they do
  not replace it.

The crate's `is_safe` check is input validation, not a canonical-path, symlink,
environment, resource, or process-tree security boundary. Bubblewrap currently
filters selected environment keys and clears inherited environment in its
generated command, but the provider still owns the exact environment allowlist
and workspace-root policy.

Adopt `ai-sandbox` behind an internal `SandboxProvider` rather than exposing its
types or assuming that a transformed argv is already sandboxed:

```rust
pub trait SandboxProvider {
    fn prepare(
        &self,
        command: &SandboxCommand,
        policy: &SandboxPolicy,
    ) -> Result<PreparedCommand, SandboxError>;

    fn capabilities(&self) -> SandboxCapabilities;
}
```

The provider owns version pinning, capability probes, workspace-root
canonicalization, private temporary-directory setup, environment filtering, and
fail-closed policy selection. The LSP client still owns stdio pipes, JSON-RPC
framing, request timeouts, process-group cleanup, and shutdown. A build runner
may use the same provider to wrap the `sphinxdocrs` process itself. Do not nest a
second sandbox around an LSP server unless the platform-specific interaction has
been tested; prefer one outer build sandbox with inherited child confinement, or
one independently wrapped LSP process when the build is trusted.

The initial Cargo integration should be optional and pinned to the exact audited
source. Do not assume the crates.io `=0.2.1` package contains commit
`4cef879225e3bd177aa2fff1ca7d8771ae108a67`; either publish and audit a release
that includes the required fixes or pin the reviewed fork/revision. The provider
can reuse the Linux Bubblewrap and macOS Seatbelt command transformations, but
must add capability checks, root/environment policy, LSP-specific piped stdio,
timeouts, and descendant-process cleanup. Protected mode must return
`SandboxUnavailable` on unsupported platforms or failed probes; it must never
downgrade to the direct command. An explicitly named `unsafe-local` mode may
bypass the provider for trusted development only.

### Direct-client effort estimate

There are two materially different scopes:

| Scope | Contents | Estimate |
| --- | --- | --- |
| Trusted local developer tool | stdio framing, request dispatch, initialize/initialized, the small read-only method set, per-request timeouts, crash detection, and best-effort shutdown | 3-5 engineer-days |
| Maintained build feature | all of the above plus bounded messages, cancellation, process-group cleanup, sync/async bridge, fake-server tests, cache/session rules, diagnostics, and cross-platform failure handling | 2-4 engineer-weeks |
| Strong isolation for untrusted workspaces | maintained build feature plus pinned `ai-sandbox` provider integration, verified executor on each supported platform, resource and descendant-process enforcement, executable policy, environment filtering, adversarial tests, and platform-specific CI | 4-8+ engineer-weeks after the executor boundary is available, depending on platform guarantees |

The first estimate is enough to prove the provider contract, not enough to claim
that arbitrary project-selected servers are safe. The second is the minimum
credible implementation for an opt-in local feature. The third is a security
project and should not be hidden inside an LSP adapter estimate.

The direct client work breaks down roughly as follows:

- transport and JSON-RPC dispatcher: 2-4 days
- lifecycle, timeouts, crash monitoring, and process-group termination: 3-6 days
- synchronous build worker and LSP-to-snapshot mapping: 3-5 days
- fake-server, malformed-input, timeout, crash, and cleanup tests: 3-5 days
- `ai-sandbox` provider integration, LSP stdio/process lifecycle, executor validation, and platform-specific sandbox/resource controls: 1-4 weeks

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
| `textDocument/hover` | Fill a missing description/type summary for a statically discovered declaration | P1 | Not implemented. Normalize plaintext/Markdown, apply only to empty static fields, track LSP provenance, and diagnose disagreement. Never silently replace static documentation. |
| `textDocument/definition` | Optional “Defined in” source link or cross-file source location | P1 | Not implemented. Convert only in-workspace locations to Sphinx-relative source links; never publish editor `file://` URIs directly. |
| `textDocument/publishDiagnostics` | Separate build diagnostics page/report, optionally grouped by file/severity | P1 | Notifications are decoded into `AnalysisDiagnostic`; dedicated mapping/report tests and a Sphinx diagnostics page are not implemented. Diagnostics should not become API prose by default. |
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
directory, and filtered environment through the internal `SandboxProvider` when
the selected backend mode requires protection. `ai-sandbox` prepares the command;
the client remains responsible for executing it with piped stdio and supervising
the resulting process. The prepared command must retain the exact argv boundary:
no shell, string interpolation, or shell-based wrapper is permitted.

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
  TERM/KILL escalation. Windows assigns the child to a Job Object with
  kill-on-close and a 64-process limit; assignment just after spawn leaves a short
  race and Windows runtime/failure-path tests remain necessary. Neither mechanism
  is a protected sandbox. Cleanup is idempotent for the owned process tree where
  the platform mechanism applies.
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
| hover markdown/plaintext | documentation, after normalization (not implemented) |
| definition locations | canonical source location when needed (not implemented) |
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
source_backend = "static"
source_lsp_servers = {
    "rust": ["rust-analyzer", "--stdio"],
    "lean": ["lake", "env", "lean", "--server"],
}
source_lsp_timeout = 30000
source_lsp_allow_fallback = True
source_lsp_workspace_root = None
source_lsp_sandbox = "off"  # off, trusted-local, protected-lsp
source_build_sandbox = "off"  # off or protected-build
```

The Lean command must remain configurable because Lean installations differ by
project and package manager.

The source autodoc CLI exposes an opt-in subset. Current options are
`--source-backend {auto,static,lsp,hybrid}`, `--source-lsp-server EXECUTABLE`,
repeatable `--source-lsp-arg TOKEN`, `--source-lsp-timeout MILLISECONDS`,
`--source-lsp-no-fallback`, and `--source-lsp-sandbox {off,trusted-local,protected-lsp}`.
The command vector remains an argv list and is never shell-interpolated.
The status tool supports `--live --live-fake` or explicit real-server options.
The full build CLI and configuration-to-provider wiring remain future work.

The planned equivalent option set is:

```text
--source-backend {auto,static,lsp,hybrid}
--source-lsp-server LANGUAGE=COMMAND [ARGS...]
--source-lsp-timeout MILLISECONDS
--source-lsp-no-fallback
--source-lsp-sandbox {off,trusted-local,protected-lsp}
--source-build-sandbox {off,protected-build}
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

Keep sandbox policy separate from backend selection:

| Mode | `sphinxdocrs` build | LSP server | Failure behavior |
| --- | --- | --- | --- |
| `static` | normal process; no server | not started | static errors only |
| `trusted-local` | normal process | direct child, or the Linux/macOS `ai-sandbox` executor when explicitly enabled | may report an unsafe-local diagnostic |
| `protected-lsp` | normal process unless the caller selected protected build execution | must run through `SandboxProvider` with LSP-owned piped stdio and process lifecycle | fail closed if the pinned backend, capability probe, or required lifecycle control is unavailable |
| `protected-build` | run through `SandboxProvider` with workspace/output policy | inherit the outer boundary by default; do not nest automatically | fail closed before starting the build |

`protected-build` and `protected-lsp` are distinct because a sandboxed LSP does
not make Python extensions, themes, directives, or the `sphinxdocrs` process safe.
When the build itself is untrusted, wrap the entire build with one outer
`ai-sandbox` policy and keep the LSP process inside that boundary. When only the
server is untrusted, wrap the server as a child and keep the build outside. The
provider must record which scope was established in diagnostics and cache
metadata. A child process spawned by a correctly enforced outer OS sandbox is
confined by inheritance, but it is still subject to the LSP client's timeout and
process-tree cleanup rules.

For the first protected policy, use:

- read-only source and toolchain roots;
- a dedicated writable output/cache root only where required;
- a private temporary directory;
- an empty or explicit allowlist home directory;
- a filtered environment containing only declared compiler and locale settings;
- no network access unless a future policy explicitly permits a local proxy;
- bounded CPU, memory, output, process count, and wall-clock resources.

Do not treat `ai-sandbox`'s `SandboxPolicy::ReadOnly` as sufficient for LSP
protection. Linux Bubblewrap currently builds read-only system/cwd mounts,
isolated temporary mount points, clears inherited environment, and supports
network namespace isolation; macOS builds a Seatbelt policy. The provider must
still prove the concrete workspace/home/temp exposure, enforce an explicit
environment allowlist, validate symlink behavior and writable-root boundaries,
and supervise descendants. The crate currently supplies no CPU, memory, output,
or process-count limits, and its LSP-facing request methods do not support
piped stdio or process-group/job cleanup.

### `ai-sandbox` integration audit

The reviewed implementation is commit
`4cef879225e3bd177aa2fff1ca7d8771ae108a67` in the workspace, with crate version
`0.2.1`. It is a usable Linux/macOS executor foundation, not a drop-in LSP
process manager or a cross-platform security claim that can be copied from its
README:

| Area | What `ai-sandbox` supplies | Required `sphinxdocrs` work or current limitation |
| --- | --- | --- |
| Policy model | read-only, workspace-write, network enum, path checks, command safety check, exact argument literals, executable alias protection, Deny precedence, absolute-path chmod guard | validate canonical roots and symlink policy at the provider boundary; policy matching is not OS isolation |
| Linux | Bubblewrap executor and namespace capability probe; explicit RO/RW mounts; isolated `/tmp`, `/home`, `/root`; selected environment filtering; NoAccess network namespace; fail-closed unsupported network/filesystem policies | no Landlock syscall enforcement or seccomp filter; no resource limits; writable-root path replacement TOCTOU remains; integration must test mount exposure, filesystem boundaries, network denial, and descendant behavior |
| macOS | Seatbelt command transformation and `sandbox-exec` launch; quoted policy paths; NoAccess/Localhost/FullAccess policy generation; Proxy rejected | add native boundary tests for filesystem/network behavior, child inheritance, path escapes, and unavailable/rejected `sandbox-exec` |
| Windows | restricted-token, ACL, and process-launch implementation exists; tests ensure protected policies do not select unrestricted launch | `SandboxExecRequest` currently reports backend unsupported on Windows; wire it to process creation, add Job Object kill-on-close/process limits, and validate filesystem/network semantics |
| BSD | Capsicum/pledge policy helpers and enforcement adapter functions; pledge setup is represented in child execution APIs | `SandboxExecRequest` currently reports backend unsupported on FreeBSD/OpenBSD; prove the target child enters the capability boundary and test filesystem/network behavior before enabling provider support |
| Execution | `SandboxExecRequest::spawn()`, `run(timeout)`, and `wait()` execute the immutable prepared command; `run` terminates the direct child on timeout | stdio is inherited; `wait` is unbounded; `run` does not provide process-group/job or descendant cleanup. LSP must retain stdio framing and lifecycle ownership, likely via a provider API that prepares the sandbox command separately from spawning |
| Fallbacks | unsupported protected execution returns an explicit error; Linux spawn runs a Bubblewrap capability probe; unavailable Linux/macOS transformations fail closed | keep provider-level capability reporting explicit; never fall through to a direct command when protected mode is selected |
| Supply chain | crate manifest remains `0.2.1`; audited workspace commit is `4cef879225e3bd177aa2fff1ca7d8771ae108a67` | crates.io `=0.2.1` does not identify this commit; publish/verify a release or pin the reviewed fork/revision, record the source and lockfile, and run supported-platform adversarial CI |

The integration must start with the Linux Bubblewrap capability probe (and an
equivalent native Seatbelt probe/test on macOS), then verify that a disposable
child cannot read a sentinel outside allowed roots, write outside the output
root, access the network under `NoAccess`, or escape by symlink. Add a test for
replacement/racing of a configured writable path; if the API cannot eliminate
that race, document and constrain the trust assumptions rather than claiming
race-free root containment. Also verify process-tree timeout/cleanup separately:
the current `run()` only kills the direct child. A policy-construction success
or successful capability probe alone is not a passing boundary test.

### DSCode sandbox audit

DSCode provides useful reference implementations, but its sandbox should not be
treated as a security guarantee for `sphinxdocrs` without additional work:

| DSCode facility | Reusable idea | Limitation for this project |
| --- | --- | --- |
| `dscode-lsp` client | lifecycle shape, stdio protocol, pending responses | not a sandbox; current client has fixed policy and best-effort cleanup |
| Linux `bwrap` path | namespaces, `--unshare-net`, `--unshare-pid`, `--die-with-parent` | only used when `bwrap` is present; fallback is memory `setrlimit` only; the implementation exposes the whole `HOME` read-only and host `/tmp` writable |
| Linux fallback | address-space limit | no filesystem, network, child-process, or CPU isolation; `max_cpu_percent` is not enforced |
| macOS path | address-space limit | current code does not invoke `sandbox-exec`; it sets `NODE_ENV` and a memory limit only |
| Windows Job Object | memory/process-count limits and kill-on-close | no filesystem or network isolation; assignment/configuration failures are logged and treated as success |
| `PathValidator` | canonicalized allowlist for DSCode-mediated file API calls | does not constrain direct filesystem access by a child language server and remains subject to TOCTOU concerns |
| permissions/rate limiter | capability vocabulary and request throttling | only useful when every operation passes through DSCode IPC; neither controls an arbitrary LSP executable nor enforces OS permissions |
| binary verifier | integrity check for a bundled executable | checks DSCode's Node binary, not a configured `rust-analyzer`, Lean server, or wrapper |

The most important practical issue is that the Linux `bwrap` implementation binds
the entire home directory read-only. That prevents writes but still exposes
contents such as SSH keys, cloud credentials, package tokens, and editor state.
It also always binds host `/tmp` read-write. If `bwrap` is unavailable or cannot
create namespaces, the code silently falls back to resource limiting, so the
documented network and filesystem restrictions no longer apply. The macOS README
and security documentation describe stronger isolation than the current sandbox
code supplies. Windows Job Objects are useful for containment and cleanup, but
they are not a filesystem/network sandbox.

For `sphinxdocrs`, reuse DSCode as a design reference rather than depending on
`dscode-extension-host`. `ai-sandbox` now supplies active Bubblewrap execution on
Linux and Seatbelt execution on macOS, but does not remove the need for explicit
workspace binds, an empty or allowlisted home, a private temporary directory,
filtered environment variables, network denial, resource limits, and fail-closed
behavior when the requested isolation cannot be established. Its request
executor is not wired for Windows or BSD, and its process API has no LSP stdio,
process-group/job cleanup, or descendant/resource limits. If those guarantees
cannot be provided on a platform, the LSP backend should be unavailable there
rather than silently downgraded to an unsandboxed process when the caller
selected a protected mode. An explicitly named `unsafe-local` mode could permit
that downgrade for trusted developer use.

The static backend remains the security and reproducibility default. LSP should be
classified as trusted-local by default, with a separate protected mode only after
platform-specific sandbox tests demonstrate the promised boundary.

## Testing strategy

### Provider contract tests

Create one shared contract suite that every static, LSP, and hybrid provider must
pass:

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
- server crash during a request (pre-initialize crash coverage remains to add)
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
provider-scoped process reuse, timeout, crash during a request, framing limits,
malformed JSON-RPC/version, UTF-16 position conversion, symbol-kind mapping,
missing executable resolution, workspace escape rejection before spawn, and
configuration/cache secrecy. The status command can invoke the shared fake server
with `--live --live-fake`; the command remains process-free without that flag.
Gaps include diagnostics notification fixtures, pre-initialize crash, true request
cancellation, Windows process-tree cleanup, and cross-platform lifecycle tests.
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

Status: the public facade, compatibility re-exports, provider contract, backend
identity, core consumer imports, and initial session tests are present. Existing
H14 source tests pass in focused runs. A comprehensive shared provider contract
suite for every static/LSP/hybrid provider remains incomplete.

### Phase 2: provider and persistence cleanup — implemented with cache-audit gaps

- Extract `SourceAnalysisSession`.
- Centralize backend selection and request identity.
- Add snapshot provenance internally.
- Add source-status manifest and focused status command.
- Verify static builds remain process-free and deterministic.

Status: static is the default; session mode, request/cache identity, per-field
in-memory provenance, hybrid conflict diagnostics, unmatched-LSP opt-in, and
manifest command exist. Static input identity remains provided by the backend;
verify every Rust/Lean toolchain/configuration input for cache invalidation. LSP
identity hashes configured command arguments, workspace, the client-requested
capabilities, timeout, and source request, but server-version probing is not
implemented yet.

### Phase 3: optional LSP adapter — trusted-local document-symbol slice implemented

- Define the internal `SourceLspClient` interface and fake-server contract.
- Choose either a pinned `dscode-lsp` adapter or a local client based on the
  trusted-local effort and dependency review.
- The current local adapter uses `serde_json`, standard library threads/channels,
  and optional `libc`; no dscode or `lsp-types` dependency is present.
- Add `lsp-source-analysis` as an off-by-default Cargo feature.
- Deferred security work: add an off-by-default `source-sandbox` feature pinned
  to an audited `ai-sandbox` release or exact fork revision, behind an internal
  `SandboxProvider`.
- Deferred security work: implement `AiSandboxProvider` for Linux Bubblewrap and
  macOS Seatbelt, with command preparation separate from stdio/process lifecycle.
  Do not enable protected Windows or BSD modes until their executors and boundary
  tests exist.
- Deferred security tests: verify Linux mounts/network/path replacement/descendant
  cleanup and native macOS filesystem/network boundaries before labeling either
  platform protected.
- Implement the lifecycle worker and normalized document-symbol mapper.
- Add fake-server tests and timeout/shutdown coverage.
- Add explicit source-backend configuration and trusted-local CLI selection.

Status: optional feature is off by default, fake server produces snapshots, and
provider instances reuse one child within their own lifetime/workspace. Source
paths are canonicalized and constrained before process startup. Protected modes
fail closed because there is no `SandboxProvider`. Unix process-group cleanup is
implemented and tested on Linux; Windows Job Object kill-on-close is implemented
but not runtime-tested here. macOS uses the Unix process-group implementation but
has not had native validation. Resource limits and actual sandboxed stdio remain
unimplemented. `trusted-local` is not a security boundary. The status command's
fake-server path produces a normalized document-symbol snapshot; real
rust-analyzer/Lean test runs remain explicit opt-ins.

The provider mutex serializes access to its single child; reusing that child is
limited to the lifetime of one `LspSnapshotProvider` instance. The current CLI
constructs a provider per invocation, so this is not a cross-build or global
server pool. Startup/initialize uses the configured request timeout. The client
does not yet query server version or implement hover/definition/references.

### Phase 4: hybrid merge and source consumers — core merge implemented

- Implement static-base/LSP-enrichment merge rules.
- Add provenance and conflict diagnostics.
- Wire merged snapshots into domains, autodoc, apidoc, and search.
- Add cache invalidation for LSP configuration and server versions.

Status: core consumers use normalized records and hybrid mode retains static
policy fields with conflict diagnostics and provenance. Search/xref parity when
LSP adds no information and full apidoc/environment cache integration still need
dedicated end-to-end tests.

### Phase 5: optional live-server workflows — partially available

- Add opt-in rust-analyzer integration tests.
- Add opt-in Lean language-server integration tests.
- Document server installation and project-specific commands.
- Add opt-in `ai-sandbox` boundary tests for LSP-only and whole-build scopes on
  each supported platform.
- Evaluate `dscode-session` separately; it must not replace the sandbox provider
  or process-policy boundary.

Status: status command can run the fake peer or an explicitly supplied
trusted-local server. Installed rust-analyzer/Lean CI, platform boundary tests,
and server installation documentation remain pending.

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
- `sphinx-source-status` is the inspection command name; protected modes remain
  unavailable until a sandbox provider is audited and tested.

Still open:

1. Which Lean server command/capabilities should be documented as the recommended
   configuration, if any?
2. Should diagnostics be exposed as a generated Sphinx page, a build warning stream,
   or both? What severity/count policy should each use?
3. Which server-version probe is reliable across rust-analyzer and Lean servers and
   should therefore participate in cache identity?
4. Should the optional client remain in `sphinxdocrs` or move to a separate
   `sphinxdocrs-lsp` crate before publication?
5. Which exact audited sandbox source/revision and platform guarantees are acceptable
   for a future `protected-lsp` implementation?

## Acceptance criteria and remaining work

Implemented criteria:

- `source_docs` is the documented public facade.
- Existing `source_analysis` imports continue to compile during migration.
- Static Rust and Lean analysis remains deterministic and fully usable without LSP.
- The current local LSP client is optional and isolated behind a feature; no
  third-party LSP client type crosses the provider boundary.
- `protected-lsp` and `protected-build` currently fail closed because no
  `SandboxProvider` is integrated; no protected mode runs a direct child. This
  satisfies fail-closed behavior but does not satisfy the sandbox-integration
  acceptance criterion below.
- LSP lifecycle failures produce explicit errors; notification diagnostics are
  normalized, but dedicated diagnostics-reporting policy/tests remain open.
- Domains, autodoc, apidoc, persistence, and search consume only normalized source
  records.
- Future upstream port work can be tracked through the source-port manifest and
  focused porting command.

The plan is not complete until the remaining acceptance criteria below land:

- Add and audit an optional `SandboxProvider` pinned to a reviewed release or
  exact source revision; never treat `ai-sandbox` command transformation alone as
  proof of isolation.
- Add Linux Bubblewrap and macOS Seatbelt boundary tests for read/write roots,
  network denial, symlink/path replacement, and descendant cleanup. Add native
  Windows Job Object process-tree cleanup and failure-path tests before
  advertising validated cleanup support there. Until the sandbox boundary tests
  pass, protected modes remain unavailable.
- Add hover/definition (and any selected references/workspace-symbol) mapping,
  with source links converted to Sphinx-relative URIs under an explicit policy.
- Add server version/capability identity and ensure cache invalidation covers
  every relevant server/toolchain input.
- Complete shared provider contract coverage and static/hybrid parity tests for
  domains, autodoc, apidoc, environment persistence, and search.
- Add optional rust-analyzer and Lean live tests plus diagnostics-notification,
  pre-initialize crash, malformed-response, and cross-platform lifecycle cases.
- Complete source-port mapping audits and shared contract tests for every static,
  LSP, and hybrid provider, including static/hybrid parity in domains, autodoc,
  apidoc, environment persistence, and search.
- Audit source visibility/type mapping and the source-port manifest against
  upstream behavior; do not label local tests as exact parity without an
  upstream comparison fixture.
