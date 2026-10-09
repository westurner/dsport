# Optional Source Documentation LSP

Static source analysis remains the default and does not require a language
server. LSP support is optional and requires the `lsp-source-analysis` Cargo
feature. Protected LSP additionally requires `source-sandbox` and a supported
Linux host; macOS policy generation exists but native runtime validation is
pending. Protected-build is not available.

## Sphinx Configuration

Configure argv, environment, and protected filesystem roots by language in
`conf.py`. For example, an elan-managed Lean server can be configured as:

```python
from pathlib import Path

elan_home = Path.home() / ".elan"

source_backend = "hybrid"
source_lsp_sandbox = "protected-lsp"
source_lsp_servers = {
  "lean": [str(elan_home / "bin" / "lake"), "serve"],
}
source_lsp_server_environment = {
  "lean": {
    "ELAN_HOME": str(elan_home),
    "PATH": f"{elan_home / 'bin'}:/usr/bin:/bin",
  },
}
source_lsp_server_read_only_roots = {
  "lean": [str(elan_home)],
}
```

`source_lsp_servers` is a language-to-argv mapping; arguments are passed
literally, without a shell. `source_lsp_server_environment` is a
language-to-environment mapping. The process starts with inherited environment
cleared, and receives only explicit per-server values plus the existing
locale-only internal allowlist. Protected mode also applies
`source_lsp_sandbox_environment`; per-server values override same-named shared
values there.

The existing `source_lsp_read_only_roots` list remains available for roots shared
by all servers. `source_lsp_server_read_only_roots` adds roots only for the
matching language. Both are additional read-only mounts; the workspace is
already mounted read-only. Paths must be absolute, non-root, and contain no
`..`; protected startup fails if a configured path cannot be mounted or if the
selected server has no effective toolchain roots. Additional roots cannot
replace `/dev` or `/proc` (including descendants), or the base `/tmp`, `/home`,
or `/root` mounts. Explicit nested toolchain directories such as
`/home/user/.elan` remain configurable.

For safety, environment keys that can inject code or alter dynamic loading (such
as `LD_*`, `DYLD_*`, `PYTHONPATH`, and `NODE_OPTIONS`) are rejected. Configured
`PATH` entries must all be non-empty absolute paths. The server executable is
resolved using the Sphinx process's host `PATH` before the child environment is
constructed, so prefer an absolute executable path when the toolchain is not on
that host path. Do not put credentials in `conf.py`; configured values are
visible to the configured server.

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

## Built LSP Report

The quickstarted project at
`src/sphinxdocrs/docs/lsp-rust-report/` demonstrates the end-to-end flow. It
contains a small Rust crate, protected per-language settings in `conf.py`, and
the current rust-analyzer output in `api.rst`. Regenerate the report and build
HTML from the workspace root with:

```sh
bash src/sphinxdocrs/docs/lsp-rust-report/build-report.sh
```

The script runs `sphinx-autodoc-rs` in protected LSP mode to write `api.rst`,
then runs `sphinx-build-rs` to render that report. The HTML build itself reads
the generated RST and does not start another language server. The output is
written to `src/sphinxdocrs/docs/lsp-rust-report/_build/final-html/`.
Rust object signatures are syntax-highlighted through PygmentsRS because this
project sets `html_highlight_object_signatures = True`; the setting defaults to
off for other projects.

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