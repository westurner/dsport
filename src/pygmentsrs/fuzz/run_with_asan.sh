#!/bin/bash
# Run fuzz target with Address Sanitizer (ASAN) enabled
# Detects memory safety bugs: buffer overflows, use-after-free, etc.
#
# Note: ASAN instrumentation may fail on some systems due to proc-macro conflicts.
# This is a known Rust/LLVM limitation. OSS-Fuzz containers may work better.

set -e

if [ $# -lt 1 ]; then
    echo "Usage: $0 <target_name> [fuzzer_args...]"
    echo ""
    echo "Examples:"
    echo "  $0 fuzz_lex_python -max_len=4096"
    echo "  $0 fuzz_format_html corpus/fuzz_format_html/"
    echo ""
    echo "Note: If ASAN build fails with proc-macro errors, use the base fuzzer:"
    echo "  ./target/release/fuzz_lex_python -max_len=4096"
    exit 1
fi

TARGET=$1
shift
FUZZER_ARGS="$@"

echo "=== Running $TARGET with Address Sanitizer ==="
echo "Target: $TARGET"
echo "Fuzzer Args: $FUZZER_ARGS"
echo ""
echo "⚠ Note: ASAN instrumentation may not work on all systems."
echo "If build fails, use the base fuzzer (already built)."

# Try to build with ASAN
echo ""
echo "Attempting to build with ASAN instrumentation..."
export RUSTFLAGS="-Zsanitizer=address"

if ! cargo +nightly build --release 2>&1 | grep "Finished"; then
    echo ""
    echo "⚠ ASAN build failed (expected on some systems)"
    echo "Using base fuzzer instead (still provides good coverage):"
    echo ""
    BINARY="target/release/$TARGET"
else
    BINARY="target/release/$TARGET"
fi

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

echo "Binary ready at $BINARY"
echo ""

# Configure ASAN options (for if instrumentation succeeded)
export ASAN_OPTIONS="detect_leaks=1:detect_odr_violation=1:detect_stack_use_after_return=1:halt_on_error=0"

echo "Starting fuzzer..."
echo ""

# Run (will use ASAN if it was built, or plain if not)
$BINARY $FUZZER_ARGS

echo ""
echo "=== Fuzzing Complete ==="
