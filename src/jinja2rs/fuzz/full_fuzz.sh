#!/usr/bin/env bash
set -euo pipefail

FUZZ_MAX_TOTAL_TIME="${FUZZ_MAX_TOTAL_TIME:-300}"
FUZZ_MAX_LEN="${FUZZ_MAX_LEN:-1048576}"
RUST_TARGETS=(
    render
    sandbox
)

cd "$(dirname "${BASH_SOURCE[0]}")"

run_mode() {
    local mode="$1"
    shift
    local feature_args=("$@")
    local targets=("${RUST_TARGETS[@]}")
    local env_args=()

    if [[ "$mode" == "python" ]]; then
        targets+=(python_bridge)
        env_args+=(LSAN_OPTIONS=detect_leaks=0)
    fi

    printf '\n=== BUILD %s ===\n' "$mode"
    cargo +nightly fuzz build \
        --no-default-features \
        "${feature_args[@]}"

    printf 'Configuration: mode=%s max_total_time=%s max_len=%s targets=%s\n' \
        "$mode" "$FUZZ_MAX_TOTAL_TIME" "$FUZZ_MAX_LEN" "${#targets[@]}"

    for target in "${targets[@]}"; do
        printf '\n=== START %s/%s ===\n' "$mode" "$target"
        env "${env_args[@]}" cargo +nightly fuzz run \
            --no-default-features \
            "${feature_args[@]}" \
            "$target" -- \
            -max_total_time="$FUZZ_MAX_TOTAL_TIME" \
            -max_len="$FUZZ_MAX_LEN" \
            -print_final_stats=1
        printf '=== PASS %s/%s ===\n' "$mode" "$target"
    done
}

run_mode no-python
run_mode python --features python

printf '\nAll Jinja fuzz targets completed successfully in both modes.\n'
