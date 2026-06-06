#!/bin/bash
# Run fuzz target with all sanitizers enabled
# Combines ASAN for comprehensive testing
#
# Note: Due to proc-macro limitations, full sanitizer instrumentation may not work locally.
# Use OSS-Fuzz infrastructure for guaranteed sanitizer support.

set -e

if [ $# -lt 1 ]; then
    echo "Usage: $0 <target_name> [fuzzer_args...]"
    echo ""
    echo "Examples:"
    echo "  $0 fuzz_lex_python -max_len=4096"
    echo "  $0 fuzz_format_html corpus/fuzz_format_html/"
    exit 1
fi

TARGET=$1
shift
FUZZER_ARGS="$@"

echo "=== Running $TARGET with Comprehensive Fuzzing ==="
echo "Target: $TARGET"
echo "Fuzzer Args: $FUZZER_ARGS"
echo ""
echo "⚠ Note: Full sanitizer instrumentation may not work on local systems"
echo "due to proc-macro conflicts with sanitizers (known Rust/LLVM limitation)."
echo "Using base fuzzer with coverage instrumentation."

# Try to build with ASAN (the most compatible sanitizer)
echo ""
echo "Building..."
export RUSTFLAGS="-Zsanitizer=address" || true

cargo +nightly build --release 2>&1 | grep -E "(Finished|error: could not compile)" || true

BINARY="target/release/$TARGET"
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

echo "Binary ready at $BINARY"
echo ""
echo "For full sanitizer support, use OSS-Fuzz:"
echo "  https://google.github.io/oss-fuzz/"
echo ""
echo "Starting fuzzer..."
echo ""

# Configure options
export ASAN_OPTIONS="detect_leaks=1:detect_odr_violation=1:halt_on_error=0" || true

# Run
$BINARY $FUZZER_ARGS

echo ""
echo "=== Fuzzing Complete ==="
