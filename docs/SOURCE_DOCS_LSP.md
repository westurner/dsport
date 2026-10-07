# Optional Source Documentation LSP

Static source analysis remains the default and does not require a language
server. LSP support is optional and requires the `lsp-source-analysis` Cargo
feature. Protected LSP additionally requires `source-sandbox` and a supported
Linux host; macOS policy generation exists but native runtime validation is
pending. Protected-build is not available.

## Fake Server

The status command is process-free unless a live mode is explicitly selected.
Run its checked-in fake peer with:

```sh
cargo run -p sphinxdocrs --features lsp-source-analysis --bin sphinx-source-status -- --live --live-fake
```

## Protected Rust Test

On Linux, install rust-analyzer through rustup and run the opt-in protected
no-proc integration test:

```sh
rustup component add rust-analyzer --toolchain stable
SPHINXDOCRS_RUST_ANALYZER="$(rustup which --toolchain stable rust-analyzer)" \
  cargo test -p sphinxdocrs --features source-sandbox --lib \
  source_docs::lsp_backend::tests::protected_rust_analyzer_live_returns_document_symbols_without_proc \
  -- --exact --nocapture --test-threads=1
```

The test skips when `SPHINXDOCRS_RUST_ANALYZER` is unset. It invokes the
configured executable through an argv-only `/usr/bin/env` wrapper, exposes the
installed toolchain's library directory to rust-analyzer, and uses Bubblewrap
without mounting child `/proc`. It requires the host to support the sandbox's
namespace and mount operations; it never falls back to an unsandboxed launch.

## Lean Test

Lean servers and project launch requirements vary. Set
`SPHINXDOCRS_LEAN_LSP_COMMAND` to a JSON array containing the exact executable
and argument tokens for the current project, then run the opt-in test:

```sh
SPHINXDOCRS_LEAN_LSP_COMMAND='["/absolute/path/to/server","<server-argument>"]' \
  cargo test -p sphinxdocrs --features lsp-source-analysis --lib \
  source_docs::lsp_backend::tests::configured_lean_server_live_returns_document_symbols \
  -- --exact --nocapture --test-threads=1
```

The test skips when the variable is unset. It runs the configured Lean command
as `trusted-local`; only use it with a server and project you trust. The command
is parsed as JSON argv and is not interpreted by a shell.

## Status Command

The status command can also make an explicitly requested live request to a
trusted local server. Its live mode is not the protected no-proc test and does
not provide a sandbox boundary. Use an absolute source path and pass each server
argument separately with `--live-arg`:

```sh
cargo run -p sphinxdocrs --features lsp-source-analysis --bin sphinx-source-status -- \
  --live --live-language rust \
  --live-server /absolute/path/to/server \
  --live-arg <argument> \
  --source /absolute/path/to/project/src/lib.rs \
  --trusted-local
```

`--trusted-local` is required because this command launches the configured
process without an OS sandbox. The default status invocation starts no process.

LSP startup errors include up to 8 KiB of captured server stderr when available.