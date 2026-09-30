# Source Documentation and Optional LSP Integration Plan

Status: proposed
Date: 2026-09-30
Scope: `sphinxdocrs` source-aware documentation for Rust and Lean, with optional
integration through a client-side LSP adapter

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
requirement. A small internal client built on `lsp-types`, `serde_json`, and
`tokio` avoids coupling the source-documentation API to DSCode, allows stricter
process policy, and keeps the dependency graph smaller. A `dscode-lsp` adapter
remains reasonable if rapid experimentation is more important than owning the
lifecycle and security policy, but it must be pinned and wrapped behind the
provider boundary.

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

## LSP adapter design

### Client lifecycle

The adapter should expose an internal `SourceLspClient` trait. It may be backed by
`dscode-lsp` initially or by a local client implementation. The expected lifecycle
is:

```text
Stopped -> Starting -> Initializing -> Ready -> ShuttingDown -> Stopped
                      \-> Crashed ---------------> Ready/Stopped
```

The adapter owns:

- server registration by source language
- workspace-root initialization
- `LspServerPool` strategy
- document open/change/save/close state
- request timeout handling
- server shutdown and crash diagnostics

Before spawning the server, the client passes the executable, arguments, working
directory, and filtered environment through the internal `SandboxProvider` when
the selected backend mode requires protection. `ai-sandbox` prepares the command;
the client remains responsible for executing it with piped stdio and supervising
the resulting process. The prepared command must retain the exact argv boundary:
no shell, string interpolation, or shell-based wrapper is permitted.

The local implementation must use a single serialized writer, a reader task that
dispatches responses and notifications, a pending-request map keyed by JSON-RPC
IDs, and explicit message/header/body limits. It must reject malformed framing,
unknown response IDs, oversized messages, and responses after the session has been
closed. Request timeout must remove the pending entry and make the session
unusable unless the server is restarted; merely abandoning a future leaves a
possibly busy server and is not sufficient cleanup.

The lifecycle requirements are more than `Command::spawn()` and `Child::kill()`:

1. Validate an executable policy and argument vector; never invoke a shell.
2. Start with a filtered environment, an explicit workspace `current_dir`, and
  only the standard input/output/error handles intended for the protocol.
3. Bound startup, initialize, request, shutdown, stderr, and total session time.
4. Monitor process exit independently of request handling and reject all pending
  requests on EOF, protocol failure, or crash.
5. Send `shutdown` as a JSON-RPC request, wait briefly for its response, then send
  `exit`; escalate to process-group termination if the child remains alive.
6. On Unix, terminate the child process group where possible. On Windows, use a
  Job Object with kill-on-close and a process limit. Cleanup must be idempotent.
7. Disable automatic restart during a documentation build unless an explicit
  interactive policy enables it. Restarting a compromised or runaway server can
  turn one failure into an unbounded resource loop.

A build must call shutdown on normal completion and best-effort shutdown on error.
Server reuse is permitted only within one analysis session and one workspace root.

### Synchronous build boundary

The LSP client is asynchronous, while the current source analysis and build APIs
are synchronous. Do not spread async through `BuildEnvironment` in the first
phase.

Use one of these two implementations behind the same provider trait:

1. A worker owning a Tokio runtime and an `LspServerPool`, with synchronous request
   and response channels exposed to the build thread.
2. A blocking wrapper supplied by dscode-lsp, if the pinned version provides one
   and its runtime-drop behavior is safe in all test contexts.

The worker approach is the default recommendation because it makes lifecycle,
timeouts, and shutdown explicit. The worker must never hold a `RefCell` borrow or a
Python GIL interaction while waiting for an LSP response.

### LSP-to-snapshot mapping

Convert LSP responses into the existing normalized declaration model:

| LSP data | Snapshot field |
| --- | --- |
| `DocumentSymbol.name` | `short_name` and qualified-name input |
| symbol container hierarchy | `parent` and `children` |
| `DocumentSymbol.kind` | `DeclarationKind` |
| `range` | `SourceSpan` |
| `selection_range` | declaration anchor position when available |
| `detail` | signature/type text |
| hover markdown/plaintext | documentation, after normalization |
| definition locations | canonical source location when needed |
| diagnostics | `AnalysisDiagnostic` |
| unsupported server metadata | `attributes`, namespaced by backend |

The adapter must preserve the source language's namespace separator: `::` for
Rust and `.` for Lean. LSP names that cannot be normalized safely become a
structured diagnostic rather than a guessed cross-reference target.

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

The CLI should eventually expose equivalent options:

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
3. Query LSP only for configured enrichments or missing information.
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

Add a tiny test server or scripted stdio peer covering:

- initialize and initialized notifications
- document symbol response
- workspace symbol response
- hover and definition response
- diagnostics notification
- delayed response and timeout
- malformed JSON-RPC response
- server crash before initialization
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

## Reducing future porting overhead

### Source-port manifest

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

### Porting command

Add a focused developer command that:

- lists upstream symbols mapped to source-documentation owners
- reports missing declaration-kind/directive/role mappings
- runs the shared provider contract suite
- runs selected Python/Rust parity fixtures
- reports accepted deviations and their test names
- optionally exercises configured LSP servers

The command should be usable without LSP installed. Its output should distinguish
"not implemented", "accepted deviation", "backend unavailable", and "test skipped".

## Delivery phases

### Phase 1: facade and contracts

- Add `source_docs` module with compatibility re-exports.
- Introduce `SourceSnapshotProvider` and backend identity.
- Move source-specific autodoc/apidoc/domain contracts behind the facade.
- Add the shared provider contract test suite.
- No dscode dependency yet.

Exit criteria: existing H14 tests pass unchanged and all source consumers depend
only on the normalized model.

### Phase 2: provider and persistence cleanup

- Extract `SourceAnalysisSession`.
- Centralize backend selection and request identity.
- Add snapshot provenance internally.
- Add source-port manifest and focused porting command.
- Verify static builds remain process-free and deterministic.

Exit criteria: static mode is the documented release and CI path; cache identity
covers every static backend input.

### Phase 3: optional LSP adapter

- Define the internal `SourceLspClient` interface and fake-server contract.
- Choose either a pinned `dscode-lsp` adapter or a local client based on the
  trusted-local effort and dependency review.
- If using dscode-lsp, pin and verify one release or commit; if using a local
  client, depend on `lsp-types`, `serde_json`, and the existing async runtime only.
- Add `lsp-source-analysis` as an off-by-default Cargo feature.
- Add an off-by-default `source-sandbox` feature pinned to an audited
  `ai-sandbox` release or exact fork revision, and keep its types behind an
  internal `SandboxProvider`.
- Implement the `AiSandboxProvider` around the Linux Bubblewrap and macOS
  Seatbelt backends. Expose command preparation separately from process spawn so
  the LSP client can retain piped stdio, JSON-RPC framing, timeouts, and
  process-tree cleanup. Do not enable protected Windows or BSD modes until
  those request executors are implemented and pass platform boundary tests.
- For Linux, test Bubblewrap mounts, network denial, path replacement/symlink
  behavior, and descendant cleanup; for macOS, run native Seatbelt filesystem
  and network boundary tests. Neither platform should be labeled protected
  until these integration tests pass.
- Implement the lifecycle worker and normalized LSP mapper.
- Add fake-server tests and timeout/shutdown coverage.
- Add explicit source-backend configuration.

Exit criteria: the feature-disabled build has no LSP or `ai-sandbox` dependency;
feature-enabled builds pass without a live server; a configured fake server can
produce a snapshot; trusted-local and protected modes are reported distinctly;
missing or unsupported platform executors produce an explicit protected-mode
error; and sandboxed LSP stdio plus process-tree cleanup work without bypassing
the selected OS boundary.

### Phase 4: hybrid merge and source consumers

- Implement static-base/LSP-enrichment merge rules.
- Add provenance and conflict diagnostics.
- Wire merged snapshots into domains, autodoc, apidoc, and search.
- Add cache invalidation for LSP configuration and server versions.

Exit criteria: hybrid mode cannot silently override static policy metadata, and
static/hybrid parity is proven when LSP contributes no additional information.

### Phase 5: optional live-server workflows

- Add opt-in rust-analyzer integration tests.
- Add opt-in Lean language-server integration tests.
- Document server installation and project-specific commands.
- Add opt-in `ai-sandbox` boundary tests for LSP-only and whole-build scopes on
  each supported platform.
- Evaluate `dscode-session` separately; it must not replace the sandbox provider
  or process-policy boundary.

Exit criteria: live integrations are useful for development but remain unnecessary
for normal package builds, release builds, and deterministic CI; protected mode
fails closed when its platform capability probe or boundary test cannot pass.

## Open decisions

1. Which exact dscode-lsp release or commit should be supported first?
2. Does that version expose a safe blocking API, or is the worker runtime required?
3. Which published `ai-sandbox` release contains the reviewed Linux/macOS
  executor work, or which exact fork revision should be pinned until then?
4. Which platform guarantees are strong enough to label `protected-lsp` and
  `protected-build`, and which platforms remain trusted-local only?
5. Which Lean language server command and capabilities are stable enough to document?
6. Should unmatched LSP symbols ever enter a generated API document, or only enrich
   declarations discovered statically?
7. Should provenance be serialized in `AnalysisSnapshot` version 2, or remain an
   internal merge diagnostic until consumers need it?
8. Should the optional LSP feature be published in the main crate or split into a
   separate `sphinxdocrs-lsp` integration crate?

## Acceptance criteria

The plan is complete when:

- `source_docs` is the documented public facade.
- Existing `source_analysis` imports continue to compile during migration.
- Static Rust and Lean analysis remains deterministic and fully usable without LSP.
- The LSP client implementation is optional and isolated behind a feature; if
  dscode-lsp is selected, it is pinned and hidden behind the internal client API.
- The sandbox integration is optional and isolated behind `SandboxProvider`; the
  selected published `ai-sandbox` version or exact fork revision is recorded and
  matches the audited executor behavior.
- `protected-lsp` and `protected-build` fail closed when the required executor,
  capability probe, or boundary test is unavailable; no protected mode silently
  runs a direct child.
- The build and LSP scopes are explicit, and descendant-process cleanup is tested
  independently of the sandbox policy transformation.
- LSP lifecycle failures produce structured diagnostics or explicit errors according
  to backend mode.
- Domains, autodoc, apidoc, persistence, and search consume only normalized source
  records.
- Shared provider contract tests cover every backend.
- Future upstream port work can be tracked through the source-port manifest and
  focused porting command.
