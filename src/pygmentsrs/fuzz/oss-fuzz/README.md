# OSS-Fuzz Integration for pygmentsrs

This directory contains configuration for integrating pygmentsrs fuzzing with [Google OSS-Fuzz](https://google.github.io/oss-fuzz/).

## Project Information

- **Project Name**: dsport
- **Repository**: https://github.com/westurner/dsport
- **Fuzz Targets Location**: `src/pygmentsrs/fuzz/fuzz_targets/`
- **Language**: Rust
- **Fuzzer**: libFuzzer

## Files

- `Dockerfile` - Build environment for OSS-Fuzz
- `build.sh` - Build script for fuzzing
- `project.yaml` - OSS-Fuzz project configuration

## Building Locally

If you want to test the OSS-Fuzz build locally:

```bash
# Clone OSS-Fuzz
git clone https://github.com/google/oss-fuzz.git
cd oss-fuzz

# Build pygmentsrs fuzzer
python3 infra/helper.py build_image dsport
python3 infra/helper.py build_fuzz dsport
```

## Running Fuzz Targets Directly

To run fuzz targets without OSS-Fuzz infrastructure:

```bash
cd src/pygmentsrs/fuzz
cargo +nightly fuzz run fuzz_lex_python -- -max_len=4096 -timeout=10
```

## Adding New Fuzz Targets

1. Create a new file: `src/pygmentsrs/fuzz/fuzz_targets/fuzz_*.rs`
2. Add a `[[bin]]` entry in `src/pygmentsrs/fuzz/Cargo.toml`
3. Implement using `libfuzzer_sys::fuzz_target!` macro
4. Update `build.sh` to copy the new binary (if needed)

## Resources

- [OSS-Fuzz Documentation](https://google.github.io/oss-fuzz/getting-started/new-project-guide/)
- [cargo-fuzz Documentation](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [libFuzzer Documentation](https://llvm.org/docs/LibFuzzer/)
