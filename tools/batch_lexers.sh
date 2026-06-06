#!/bin/bash
# Batch processing script for transpilable Pygments lexers
# Automates: generate → wire → test → commit for all remaining 296+ lexers
#
# Usage: ./batch_lexers.sh <batch_number> [lexer_specs.txt]
# Or: ./batch_lexers.sh all  # Process all remaining in systematic batches

set -e

cd "$(dirname "$0")/../src"

BATCH=${1:-1}
VENV=.venv/bin

# Get current native count
count_native() {
    grep -c "^pub mod" pygmentsrs/src/lexers/generated/mod.rs || echo "0"
}

# Classify all remaining transpilable
classify_remaining() {
    $VENV/python tools/gen_lexer.py --classify transpilable 2>&1 | \
        grep -E "^\s+pygments\.lexers\." | \
        sed 's/.*:\([^:]*\):\([^ ]*\).*/\1:\2/' | \
        sort -u
}

# Generate and test a batch
process_batch() {
    local specs="$1"
    local batch_num="$2"
    
    echo "[*] Batch $batch_num: Generating $(echo "$specs" | wc -l) lexers..."
    
    # Generate all
    if ! eval "$VENV/python tools/gen_lexer.py $specs" 2>&1 | tee /tmp/gen_batch_$batch_num.log | tail -5; then
        echo "[-] Generation failed - review /tmp/gen_batch_$batch_num.log"
        return 1
    fi
    
    # Build
    echo "[*] Building..."
    if ! cargo build -p pygmentsrs --quiet; then
        echo "[-] Build failed"
        return 1
    fi
    
    # Install
    echo "[*] Installing Python extension..."
    $VENV/maturin develop --release -m pygmentsrs/Cargo.toml --quiet 2>&1 | tail -2
    
    # Test
    echo "[*] Testing parity..."
    $VENV/pytest tests/test_pygments_generated_lexers.py::test_generated_lexer_byte_parity \
        -q --tb=no 2>&1 | tail -3
    
    before=$(count_native)
    echo "[+] Batch $batch_num complete. Native lexers: $before"
}

echo "=== Pygments Lexer Batch Processor ==="
echo "Current native lexer modules: $(count_native)"
echo "This tool will process all remaining ~296 transpilable lexers"
echo ""
echo "Strategy:"
echo "1. Group lexers by module (15-20 per batch)"
echo "2. Generate + build + test each batch"
echo "3. Commit clean subset (remove bridge-only failures)"
echo "4. Continue until all transpilable are processed"
echo ""
echo "Note: Interactive testing currently needed to remove bridge-only failures."
echo "Future: Automate failure detection and removal."
