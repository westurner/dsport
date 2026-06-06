# OSS-Fuzz Integration Guide for dsport

## Quick Overview

This guide shows how to register dsport with Google Cloud Fuzz (OSS-Fuzz) for continuous fuzzing with ASAN/MSAN/UBSAN instrumentation.

## Why OSS-Fuzz?

- **Free continuous fuzzing** from Google's infrastructure
- **Automatic sanitizer builds** (ASAN, MSAN, UBSAN)
- **Crash reporting** to GitHub issues
- **Corpus management** for regression testing
- **Coverage tracking** and reports
- **Scale**: Runs on dozens of machines 24/7

## Step 1: Prepare Your Repository

Your repository is already prepared! The required files are in place:

✅ `src/pygmentsrs/fuzz/Cargo.toml` - Fuzz workspace
✅ `src/pygmentsrs/fuzz/fuzz_targets/` - Fuzz target implementations
✅ `src/pygmentsrs/fuzz/corpus/` - Seed corpus
✅ `src/pygmentsrs/fuzz/oss-fuzz/` - OSS-Fuzz integration files

## Step 2: Create PR to oss-fuzz Repository

1. Fork https://github.com/google/oss-fuzz
2. Create branch: `git checkout -b add-dsport`
3. Create directory: `projects/dsport/`
4. Copy these files:

```bash
# From dsport repo
mkdir -p projects/dsport

cp src/pygmentsrs/fuzz/oss-fuzz/Dockerfile projects/dsport/
cp src/pygmentsrs/fuzz/oss-fuzz/build.sh projects/dsport/
cp src/pygmentsrs/fuzz/oss-fuzz/project.yaml projects/dsport/

# Create OWNERS file (your GitHub username)
echo "# For questions about this project, contact:"  > projects/dsport/OWNERS
echo "westurner" >> projects/dsport/OWNERS
```

5. Verify structure:
```
projects/dsport/
├── Dockerfile
├── build.sh
├── project.yaml
└── OWNERS
```

6. Commit and push:
```bash
git add projects/dsport/
git commit -m "Add dsport project to OSS-Fuzz"
git push origin add-dsport
```

7. Submit PR to https://github.com/google/oss-fuzz/

## Step 3: Configure OSS-Fuzz Project Settings

In the PR, OSS-Fuzz maintainers will:
1. Enable the project
2. Set up GitHub integration
3. Configure sanitizer matrix
4. Create automatic issue reporting

## Step 4: Ongoing Maintenance

### When Fuzzing Finds Crashes

OSS-Fuzz will:
1. Create GitHub issue with crash details
2. Provide reproducer script
3. Include stack trace
4. Upload crash input

### Example Issue:

```
Title: AddressSanitizer: heap-buffer-overflow in fuzz_lex_python

To reproduce:
  ./fuzz_lex_python < crash-abc123def

Stack trace:
  #0 0x5555... in some_function
  #1 0x5555... in bridge::lex
```

### Responding to Crashes

1. Download crash input
2. Reproduce locally:
   ```bash
   cd src/pygmentsrs/fuzz
   ./target/release/fuzz_lex_python crash-input
   ```
3. Fix the bug
4. Add test case to `corpus/`
5. Comment on GitHub issue with fix PR link

## Step 5: Monitoring Coverage

Check coverage reports at:
```
https://oss-fuzz-build-logs.storage.googleapis.com/dsport/latest/report/index.html
```

**What to look for**:
- Line coverage %
- Branch coverage %
- Uncovered functions
- Code complexity

**Improve coverage by**:
- Adding seed corpus files
- Creating targeted fuzz targets
- Fixing found bugs

## Step 6: Local Testing Before Push to OSS-Fuzz

Before submitting PR to oss-fuzz, verify locally:

```bash
cd src/pygmentsrs/fuzz

# Test build without sanitizers (works on all systems)
cargo +nightly build --release
./target/release/fuzz_lex_python -max_total_time=10

# Test with corpus
./target/release/fuzz_lex_python corpus/fuzz_lex_python/ -max_total_time=10

# All fuzz targets
for target in target/release/fuzz_*; do
    name=$(basename $target)
    timeout 10 $target -max_len=4096 &
done
```

## Step 7: Sanitizer Verification

OSS-Fuzz will automatically test with:

1. **AddressSanitizer (ASAN)**: Memory safety
2. **UndefinedBehaviorSanitizer (UBSAN)**: Undefined behavior
3. **Memory Sanitizer (MSAN)**: Uninitialized memory (Linux only)

You'll see builds like:
- `dsport-asan`
- `dsport-ubsan`
- `dsport-msan`

Each runs continuously against your fuzz targets.

## Architecture: How OSS-Fuzz Works

```
GitHub Repo (dsport)
    ↓
OSS-Fuzz (google/oss-fuzz)
    ├─ Dockerfile (builds fuzz targets)
    ├─ build.sh (compilation with sanitizers)
    └─ project.yaml (configuration)
    ↓
Docker Build
    ├─ Checkout dsport
    ├─ Run build.sh
    ├─ Instrument with ASAN/UBSAN/MSAN
    ├─ Compile fuzz targets
    └─ Output binaries to /out
    ↓
Continuous Fuzzing Infrastructure
    ├─ Machine 1: fuzz_lex_python
    ├─ Machine 2: fuzz_lex_javascript
    ├─ Machine 3: fuzz_format_html
    └─ ... (24/7 fuzzing)
    ↓
Crash Discovery
    ├─ Create GitHub issue
    ├─ Include stack trace
    ├─ Attach crash input
    └─ Link reproducer script
    ↓
Developer Response
    ├─ Fix bug
    ├─ Add regression test
    └─ Close issue
```

## Build Configuration Details

### Dockerfile

- **Base Image**: `gcr.io/oss-fuzz-base/base-builder-rust`
  - Pre-configured Rust + LLVM with sanitizer support
  - libFuzzer built-in
  - Correct permissions for `/out`

- **Environment Variables Set**:
  ```
  SANITIZERS=address,undefined  (in OSS-Fuzz)
  LIB_FUZZER_CFLAGS             (from OSS-Fuzz)
  RUSTUP_TOOLCHAIN=nightly      (from build.sh)
  ```

### build.sh Script

The script:
1. Clones dsport into `/src`
2. Runs `cargo +nightly build --release` with ASAN/UBSAN
3. Copies binaries to `/out/`
4. Verifies all 6 fuzz targets

### project.yaml

Configures:
- Sanitizer matrix (ASAN, UBSAN, MSAN)
- Timeout per fuzz run (10 seconds)
- Maximum input size (8KB recommended)
- Job templates

## Environment Variables

OSS-Fuzz provides these during build:

| Variable | Purpose | Value |
|----------|---------|-------|
| `OUT` | Output directory | `/out` |
| `SRC` | Source directory | `/src` |
| `WORK` | Work directory | `/work` |
| `SANITIZERS` | Sanitizers to use | `address`, `undefined`, `memory` |
| `LIB_FUZZER_CFLAGS` | libFuzzer flags | `-fsanitize=fuzzer` |
| `FUZZER_CXXFLAGS` | C++ flags | (not used for Rust) |
| `FUZZER_LDFLAGS` | Linker flags | (handled by Rust) |

## Debugging OSS-Fuzz Builds Locally

To simulate OSS-Fuzz build environment:

```bash
# Install Docker
# Then in repo root:

docker run -it \
  -v $(pwd):/src \
  -v /tmp/oss-fuzz:/out \
  gcr.io/oss-fuzz-base/base-builder-rust \
  bash -c "cd /src/dsport/src/pygmentsrs/fuzz && \
           /src/oss-fuzz/build.sh"

# Check outputs
ls -lh /tmp/oss-fuzz/
```

## Troubleshooting

### "Build script timed out"
- Reduce dependencies
- Simplify fuzz targets
- Check for infinite loops

### "No crashes found" (good!)
- Increase fuzzing time in project.yaml
- Add more corpus files
- Ensure code paths are reachable

### "Too many crashes" (also good!)
- Prioritize critical bugs
- Fix them systematically
- Add regression tests

### "Build failed"
- Check build.sh script
- Verify Cargo.toml syntax
- Check for missing dependencies

## Next Steps After Setup

1. ✅ Create PR to oss-fuzz repository
2. ⏳ Wait for OSS-Fuzz maintainers to merge
3. 📊 Monitor coverage reports (usually within 24 hours)
4. 🔔 Start receiving crash reports on GitHub
5. 🐛 Fix bugs and close reports
6. 📈 Track coverage improvements

## Resources

- [OSS-Fuzz Documentation](https://google.github.io/oss-fuzz/)
- [Rust Fuzzing Book](https://rust-fuzz.github.io/book/)
- [Adding New Projects to OSS-Fuzz](https://google.github.io/oss-fuzz/getting-started/new-project-guide/)
- [Project Example (PostgreSQL)](https://github.com/google/oss-fuzz/tree/master/projects/postgresql)

## Questions?

- GitHub Issues in dsport repository
- OSS-Fuzz Slack channel (after merge)
- Follow-up issues in oss-fuzz repository

---

**Status**: Ready for OSS-Fuzz integration
**Files**: Prepared at `src/pygmentsrs/fuzz/oss-fuzz/`
**Fuzz Targets**: 6 targets (lex × 3, format × 3)
**Sanitizers**: ASAN, UBSAN, MSAN support configured
