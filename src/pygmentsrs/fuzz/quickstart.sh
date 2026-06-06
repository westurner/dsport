#!/bin/bash
# Quick start script for running fuzz tests locally

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_info() {
    echo -e "ℹ $1"
}

# Check nightly toolchain
print_header "Checking Rust Nightly"
if ! rustup toolchain list | grep -q nightly; then
    print_info "Installing nightly toolchain..."
    rustup install nightly
else
    print_success "Nightly toolchain available"
fi

# Build fuzz targets
print_header "Building Fuzz Targets"
cargo +nightly build --release 2>&1 | grep -E "(Compiling|Finished)" || true
print_success "Build complete"

# List available targets
print_header "Available Fuzz Targets"
for binary in target/release/fuzz_*; do
    if [ -x "$binary" ]; then
        name=$(basename "$binary")
        size=$(ls -lh "$binary" | awk '{print $5}')
        echo "  • $name ($size)"
    fi
done

# Run a quick sanity check
print_header "Running Quick Sanity Check"
print_info "Fuzzing fuzz_lex_python for 5 seconds (32KB input limit)..."

timeout 15 ./target/release/fuzz_lex_python \
    -max_total_time=5 \
    -max_len=32768 \
    -verbosity=0 \
    2>&1 | tail -5 || true

print_success "Sanity check complete"

echo ""
print_header "Next Steps"
echo "1. Run specific fuzz target:"
echo "   ./target/release/fuzz_lex_python -max_len=4096"
echo ""
echo "2. Run with corpus:"
echo "   ./target/release/fuzz_lex_python corpus/fuzz_lex_python/ -max_len=4096"
echo ""
echo "3. Run all targets for extended time:"
echo "   for target in target/release/fuzz_*; do"
echo "       name=\$(basename \$target)"
echo "       echo \"Fuzzing \$name...\""
echo "       timeout 60 \$target -max_len=8192 &"
echo "   done"
echo ""
echo "4. View corpus directory:"
echo "   ls -la corpus/"
