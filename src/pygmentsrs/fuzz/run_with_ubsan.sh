#!/bin/bash
# Run fuzz target with Undefined Behavior Sanitizer (UBSAN) enabled
# Note: UBSAN is built into ASAN, use address sanitizer for UB detection

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

echo "=== Running $TARGET with Address Sanitizer (includes UB detection) ==="
echo "Target: $TARGET"
echo "Fuzzer Args: $FUZZER_ARGS"
echo ""
echo "Note: ASAN automatically enables undefined behavior detection"
echo "For pure UB detection, compile with: rustc -C overflow-checks=on"

# Build with Address Sanitizer (which includes UB detection)
echo ""
echo "Building with Address Sanitizer instrumentation (includes UB detection)..."
export RUSTFLAGS="-Zsanitizer=address"
cargo +nightly build --release 2>&1 | grep -E "(Compiling|Finished|error)" || true

BINARY="target/release/$TARGET"
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

echo "Binary ready at $BINARY"
echo ""

# Configure ASAN + UB detection options
export ASAN_OPTIONS="detect_leaks=1:detect_odr_violation=1:detect_stack_use_after_return=1:halt_on_error=0"

echo "ASAN_OPTIONS: $ASAN_OPTIONS"
echo "Starting fuzzer..."
echo ""

# Run with ASAN (which provides UB detection)
$BINARY $FUZZER_ARGS

echo ""
echo "=== Fuzzing Complete ==="
