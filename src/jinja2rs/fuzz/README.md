# jinja2rs fuzzing

This is an independent `cargo-fuzz` workspace so it does not affect the parent
Cargo workspace. The targets exercise template parsing and rendering with
structured, recursively nested contexts:

- `render` fuzzes the regular and Django-mode Rust environments.
- `sandbox` fuzzes strict undefined handling and sandbox rendering.
- `python_bridge` fuzzes the PyO3 module's `Environment` and
  `SandboxedEnvironment` classes with Python-native JSON contexts.

Run both Rust-only and Python-enabled profiles with the default five-minute
budget per target:

```text
./full_fuzz.sh
```

Use a shorter budget for a local smoke run:

```text
FUZZ_MAX_TOTAL_TIME=10 ./full_fuzz.sh
```

The Python profile requires a Python development/runtime installation usable by
PyO3. To run one target directly:

```text
cargo +nightly fuzz run --no-default-features render -- -max_total_time=60
cargo +nightly fuzz run --no-default-features --features python python_bridge -- -max_total_time=60
```
