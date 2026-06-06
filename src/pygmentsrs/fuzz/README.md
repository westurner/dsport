# Fuzzing Infrastructure for pygmentsrs

This directory contains fuzzing targets for the pygmentsrs project using **libFuzzer** and **cargo-fuzz**. This setup integrates directly with Google Cloud Fuzz.

## Overview

The fuzzing infrastructure provides continuous input fuzzing for critical components:
- **Lexers** (arbitrary languages and code)
- **Formatters** (arbitrary token streams)
- **Bridge integration** (end-to-end lex→format)

## Fuzz Targets

| Target | Purpose | Cloud Fuzz Job |
|--------|---------|-----------------|
| `fuzz_lex_arbitrary_lexer` | Fuzz lexing with arbitrary lexer names and code | Primary fuzzer |
| `fuzz_lex_python` | Targeted fuzzing of Python lexer | Language-specific |
| `fuzz_lex_javascript` | Targeted fuzzing of JavaScript lexer | Language-specific |
| `fuzz_format_html` | Fuzz HTML formatter with arbitrary tokens | Formatter-specific |
| `fuzz_format_arbitrary` | Fuzz formatting with arbitrary formatter names | Comprehensive |
| `fuzz_bridge_e2e` | End-to-end lex→format roundtrip | Integration |

## Running Locally

### Compile All Fuzz Targets

```bash
cd /workspaces/dsport/src/pygmentsrs/fuzz
cargo build --release
```

### Run a Specific Fuzz Target

```bash
# Run Python lexer fuzzer for 60 seconds
cargo +nightly fuzz run fuzz_lex_python -- -max_len=4096 -timeout=10

# Run arbitrary lexer fuzzer with specific seed corpus
cargo +nightly fuzz run fuzz_lex_arbitrary_lexer -- corpus/ -max_len=8192

# Run formatter fuzzer with continuous output
cargo +nightly fuzz run fuzz_format_html -- -verbosity=2
```

### Use Existing Corpus

If you have a corpus directory from Cloud Fuzz or previous runs:

```bash
cargo +nightly fuzz run fuzz_lex_python -- corpus/fuzz_lex_python/ -max_len=4096
```

## Installation Requirements

### Nightly Rust

Fuzzing with libFuzzer requires the nightly toolchain:

```bash
rustup install nightly
```

### libFuzzer (Linux)

Already included in Rust's LLVM when using `cargo +nightly fuzz`.

## Cloud Fuzz Integration

### Configuration (for Cloud Fuzz platform)

1. **Project Setup**
   - Repository: github.com/westurner/dsport
   - Branch: main
   - Language: Rust

2. **Fuzzing Build**
   ```dockerfile
   FROM gcr.io/oss-fuzz-base/base-builder-rust

   COPY . $SRC/dsport
   WORKDIR $SRC/dsport/src/pygmentsrs/fuzz

   COPY build.sh $SRC/
   RUN chmod +x $SRC/build.sh
   ```

3. **Build Script** (`build.sh`)
   ```bash
   #!/bin/bash
   cd $SRC/dsport/src/pygmentsrs/fuzz
   cargo +nightly fuzz build --release
   
   # Copy artifacts
   for target in fuzz_targets/fuzz_*.rs; do
       target_name=$(basename "$target" .rs | sed 's/^fuzz_//')
       cp target/x86_64-unknown-linux-gnu/release/$target_name $OUT/
   done
   ```

### Environment Variables

- `MSAN_TRACK_ORIGINS=2` - Enable memory sanitizer tracking (for CI)
- `ASAN_OPTIONS=detect_leaks=1` - Enable address sanitizer leak detection
- `UBSAN_OPTIONS=print_stacktrace=1` - Enable undefined behavior sanitizer

## Analyzing Crashes

When libFuzzer finds a crash:

```bash
# The crash input is saved as a file (e.g., crash-deadbeef123)
# Reproduce with:
cargo +nightly fuzz run fuzz_lex_python crash-deadbeef123

# Or with more verbose output:
LIBFUZZER_VERBOSITY=2 cargo +nightly fuzz run fuzz_lex_python crash-deadbeef123
```

### Minimizing Crashes

Use libFuzzer's built-in minimization:

```bash
# Minimize the crash to the smallest input that reproduces it
cargo +nightly fuzz cmin fuzz_lex_python -- crash-deadbeef123
```

## Performance Tuning

### Corpus

Place interesting seed inputs in `corpus/` directories to improve coverage:

```
fuzz/
  corpus/
    fuzz_lex_python/
      file1.py
      file2.py
    fuzz_lex_javascript/
      file1.js
      file2.js
```

### Build Optimization

Use release mode for better performance:

```bash
cargo +nightly fuzz build --release
```

### Memory Limits

Adjust for large inputs:

```bash
cargo +nightly fuzz run fuzz_lex_python -- -max_len=1048576  # 1MB
```

## Maintenance

### Adding New Fuzz Targets

1. Create a new file in `fuzz_targets/` following the naming pattern `fuzz_*.rs`
2. Add a `[[bin]]` entry in `fuzz/Cargo.toml`
3. Implement the target using `libfuzzer_sys::fuzz_target!`

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Your fuzzing code here
    let _ = some_function(data);
});
```

### Updating Dependencies

When updating `libfuzzer-sys` or `arbitrary`:

```bash
cd fuzz
cargo update
```

## Resources

- [cargo-fuzz book](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [libFuzzer documentation](https://llvm.org/docs/LibFuzzer/)
- [Arbitrary crate docs](https://docs.rs/arbitrary/)
- [Google OSS-Fuzz integration](https://google.github.io/oss-fuzz/)

## CI/CD Integration

The fuzz targets are built as part of Cloud Fuzz's continuous fuzzing pipeline. Local fuzzing can be integrated into CI with:

```yaml
# Example GitHub Actions workflow
- name: Fuzz for 60 seconds
  run: |
    rustup install nightly
    cd src/pygmentsrs/fuzz
    cargo +nightly fuzz run fuzz_lex_python -- -max_total_time=60
```
