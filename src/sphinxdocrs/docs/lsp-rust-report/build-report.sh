#!/usr/bin/env bash
set -euo pipefail

docs_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$docs_dir/../../../.." && pwd)"
cd "$repo_root"

rust_analyzer="${SPHINXDOCRS_RUST_ANALYZER:-$(rustup which --toolchain stable rust-analyzer)}"
rust_analyzer="$(readlink -f "$rust_analyzer")"
export SPHINXDOCRS_RUST_ANALYZER="$rust_analyzer"
cargo_home="$(readlink -f "${CARGO_HOME:-$HOME/.cargo}")"
rustup_home="$(readlink -f "${RUSTUP_HOME:-$HOME/.rustup}")"
toolchain_bin="$(dirname "$rust_analyzer")"
toolchain_lib="$(dirname "$toolchain_bin")/lib"
server_path="$cargo_home/bin:$toolchain_bin:/usr/bin:/bin"
features="source-sandbox,rust-source-analysis,lsp-source-analysis"
generated_report="$docs_dir/.api-generated.rst"
trap 'rm -f "$generated_report"' EXIT

cargo run -p sphinxdocrs --features "$features" --bin sphinx-autodoc-rs -- \
  "$docs_dir/rust-project" \
  --source-kind rust \
  --source-backend lsp \
  --source-lsp-timeout 120000 \
  --source-lsp-sandbox protected-lsp \
  --source-lsp-server /usr/bin/env \
  --source-lsp-arg "HOME=$HOME" \
  --source-lsp-arg "PATH=$server_path" \
  --source-lsp-arg "CARGO_HOME=$cargo_home" \
  --source-lsp-arg "RUSTUP_HOME=$rustup_home" \
  --source-lsp-arg CARGO_TARGET_DIR=/tmp/sphinxdocrs-lsp-target \
  --source-lsp-arg "LD_LIBRARY_PATH=$toolchain_lib" \
  --source-lsp-arg "$rust_analyzer" \
  --source-lsp-read-only-root "$cargo_home" \
  --source-lsp-read-only-root "$rustup_home" \
  --output "$generated_report"

{
  printf 'LSP-discovered Rust API\n=======================\n\n'
  printf 'Declarations below were returned by rust-analyzer for the local fixture crate using protected LSP with a private PID namespace and no child ``/proc`` mount.\n\n'
  cat "$generated_report"
} > "$docs_dir/api.rst"
sed -i -e '${/^$/d;}' "$docs_dir/api.rst"
rm -f "$generated_report"
trap - EXIT

cargo run -p sphinxdocrs --features "$features" --bin sphinx-build-rs -- \
  -b html -E -a -d "$docs_dir/_build/final-doctrees" \
  "$docs_dir" "$docs_dir/_build/final-html"
