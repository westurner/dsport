# jinja2rs fuzzing

This is an independent `cargo-fuzz` workspace so it does not affect the parent
Cargo workspace. The targets exercise template parsing and rendering with
structured, recursively nested contexts:

- `render` fuzzes the regular and Django-mode Rust environments.
- `sandbox` fuzzes strict undefined handling and sandbox rendering.
- `python_bridge` fuzzes the PyO3 module's `Environment` and
  `SandboxedEnvironment` classes with recursively generated native Python
  contexts.

Each target keeps a raw-template case for parser robustness and generates
several valid template shapes for runtime coverage. The Python bridge also
passes a deliberately non-JSON bytes value through the conversion layer.

Run both Rust-only and Python-enabled profiles with the default five-minute
budget per target:

```text
./full_fuzz.sh
```

Use a shorter budget for a local smoke run:

```text
FUZZ_MAX_TOTAL_TIME=10 ./full_fuzz.sh
```

The runner stores corpora and crash artifacts separately for each mode and
target under `corpus/<mode>/<target>` and `artifacts/<mode>/<target>`.
Override those roots with `FUZZ_CORPUS_ROOT` and `FUZZ_ARTIFACT_ROOT` for CI or
disposable runs. `FUZZ_TIMEOUT` limits an individual input to ten seconds by
default.

The Python profile requires a Python development/runtime installation usable by
PyO3. To run one target directly:

```text
cargo +nightly fuzz run --no-default-features render -- -max_total_time=60
LSAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz run --no-default-features --features python python_bridge -- -max_total_time=60
```

LeakSanitizer suppression is applied only to `python_bridge`: CPython reports
interpreter-shutdown allocations after the bridge has finished. The Rust
targets in the Python profile retain sanitizer leak checks.
