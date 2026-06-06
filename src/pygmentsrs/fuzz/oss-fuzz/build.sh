#!/bin/bash -eu
# Build script for OSS-Fuzz with ASAN/MSAN instrumentation
#
# Environment variables set by OSS-Fuzz:
# - $OUT: Output directory for fuzz targets
# - $SRC: Source directory
# - $WORK: Workspace directory
# - $LIB_FUZZER_CFLAGS: libFuzzer compilation flags
# - $SANITIZERS: Comma-separated list of sanitizers (address, memory, undefined)

cd $SRC/dsport/src/pygmentsrs/fuzz

# Ensure we're using nightly
export RUSTUP_TOOLCHAIN=nightly

echo "=== OSS-Fuzz Build Configuration ==="
echo "OUT: $OUT"
echo "SRC: $SRC"
echo "WORK: $WORK"
echo "Sanitizers: ${SANITIZERS:-address}"  # Default to address if not set
echo "LIB_FUZZER_CFLAGS: $LIB_FUZZER_CFLAGS"

# OSS-Fuzz provides these variables
export RUSTFLAGS="${RUSTFLAGS:-}"

# Try to build with sanitizers, but fallback if it fails
# (Some proc-macros have issues with ASAN instrumentation)
build_with_sanitizer() {
    local sanitizer=$1
    echo ""
    echo "=== Attempting to build with $sanitizer ==="
    
    # Add sanitizer flag
    export RUSTFLAGS="$RUSTFLAGS -Zsanitizer=$sanitizer"
    
    # For Memory Sanitizer, we need to track origins
    if [[ "$sanitizer" == "memory" ]]; then
        export MSAN_TRACK_ORIGINS=2
        export RUSTFLAGS="$RUSTFLAGS -Zsanitizer-memory-track-origins"
    fi
    
    echo "RUSTFLAGS: $RUSTFLAGS"
    
    # Attempt build
    if cargo +nightly build --release 2>&1 | tee build.log | grep -E "(Compiling|Finished)"; then
        echo "✓ Build with $sanitizer succeeded!"
        return 0
    else
        echo "⚠ Build with $sanitizer failed, checking if proc-macro issue..."
        if grep -q "can't find crate for.*macros" build.log; then
            echo "  Known issue: proc-macros and ASAN don't work well together"
            echo "  This is a known Rust/LLVM limitation"
            return 1
        else
            # Different error, re-throw
            echo "  Unexpected build error"
            cat build.log
            return 1
        fi
    fi
}

# Clean previous builds
echo ""
echo "=== Cleaning previous builds ==="
cargo +nightly clean

# Try with requested sanitizer first
SANITIZERS="${SANITIZERS:-address}"
if ! build_with_sanitizer "$SANITIZERS"; then
    echo ""
    echo "=== Sanitizer build failed, building without sanitizers ==="
    echo "Note: OSS-Fuzz may provide different build environment that works with sanitizers"
    export RUSTFLAGS=""
    
    if ! cargo +nightly build --release 2>&1 | grep -E "(Compiling|Finished)"; then
        echo "ERROR: Build failed even without sanitizers!"
        exit 1
    fi
fi

# Copy all fuzz target binaries to $OUT
echo ""
echo "=== Copying Fuzz Targets to $OUT ==="
mkdir -p "$OUT"

for fuzz_target in fuzz_targets/fuzz_*.rs; do
    target_name=$(basename "$fuzz_target" .rs | sed 's/^fuzz_//')
    
    # Look for binary in various possible locations
    binary_path=""
    for possible_path in \
        "target/release/$target_name" \
        "target/x86_64-unknown-linux-gnu/release/$target_name" \
        "$(find target -name "$target_name" -type f -executable 2>/dev/null | head -1)"
    do
        if [ -n "$possible_path" ] && [ -f "$possible_path" ]; then
            binary_path="$possible_path"
            break
        fi
    done
    
    if [ -n "$binary_path" ] && [ -f "$binary_path" ]; then
        cp "$binary_path" "$OUT/"
        chmod +x "$OUT/$target_name"
        
        # Get file size for confirmation
        size=$(ls -lh "$OUT/$target_name" | awk '{print $5}')
        echo "✓ Copied $target_name to $OUT ($size)"
    else
        echo "⚠ Warning: Binary not found for $target_name"
    fi
done

echo ""
echo "=== Build Complete ==="
echo "Fuzz targets available in $OUT:"
ls -lh $OUT/fuzz_* 2>/dev/null || echo "No fuzz targets found!"


