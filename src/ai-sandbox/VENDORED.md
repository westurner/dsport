# Vendored ai-sandbox

Source: https://github.com/Teckwin/sandbox
Crate version: 0.2.1 (MIT, as declared by the source Cargo.toml)
Reviewed base commit: 4cef879225e3bd177aa2fff1ca7d8771ae108a67
Local executor extension commits:
- fffaac01b5ed98bf74ce6bf9fa0f86eb09efb3c0: read-only roots and piped stdio
- 8a03e7f: piped stdio in a new Unix process group

This snapshot contains the crate manifest, README, and runtime `src/` tree.
Fuzz targets, repository tooling, tests, and build artifacts are omitted. The
extension does not add resource limits or process-tree cleanup beyond the Unix
process-group spawn option. Re-review and update this file when refreshing it.