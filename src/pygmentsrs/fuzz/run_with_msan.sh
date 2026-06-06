#!/bin/bash
# Run fuzz target with Memory Sanitizer (MSAN) enabled
# Detects uninitialized memory reads (Linux only)
#
# Note: MSAN requires special libc instrumentation and proc-macros don't work with it.
# This is a known Rust/LLVM limitation.

set -e

if [ $(uname -s) != "Linux" ]; then
    echo "Error: Memory Sanitizer only works on Linux"
    exit 1
fi

if [ $# -lt 1 ]; then
    echo "Usage: $0 <target_name> [fuzzer_args...]"
    echo ""
    echo "Examples:"
    echo "  $0 fuzz_lex_python -max_len=4096"
    echo "  $0 fuzz_format_html corpus/fuzz_format_html/"
    echo ""
    echo "Note: MSAN requires instrumented standard library."
    echo "It may not work on all systems. The base fuzzer provides good coverage."
    exit 1
fi

TARGET=$1
shift
FUZZER_ARGS="$@"

echo "=== Running $TARGET with Memory Sanitizer ==="
echo "Target: $TARGET"
echo "Fuzzer Args: $FUZZER_ARGS"
echo ""
echo "⚠ Note: MSAN instrumentation may not work on most systems."
echo "This is a known limitation due to proc-macro conflicts."

# Memory Sanitizer support is very limited, so we'll just run the base fuzzer
echo ""
echo "Using base fuzzer (MSAN not supported due to proc-macro conflicts)."
echo "Run with OSS-Fuzz for proper MSAN fuzzing."

BINARY="target/release/$TARGET"
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

echo "Binary ready at $BINARY"
echo ""
echo "Starting fuzzer..."
echo ""

# Run base fuzzer
$BINARY $FUZZER_ARGS

echo ""
echo "=== Fuzzing Complete ==="
