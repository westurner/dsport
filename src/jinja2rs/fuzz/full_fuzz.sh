#!/usr/bin/env bash
set -euo pipefail

RUST_TARGETS=(
    render
    sandbox
)

cd "$(dirname "${BASH_SOURCE[0]}")"

FUZZ_MAX_TOTAL_TIME="${FUZZ_MAX_TOTAL_TIME:-300}"
FUZZ_MAX_LEN="${FUZZ_MAX_LEN:-1048576}"
FUZZ_TIMEOUT="${FUZZ_TIMEOUT:-10}"
FUZZ_CORPUS_ROOT="${FUZZ_CORPUS_ROOT:-corpus}"
FUZZ_ARTIFACT_ROOT="${FUZZ_ARTIFACT_ROOT:-artifacts}"
FUZZ_DICTIONARY="${FUZZ_DICTIONARY:-$PWD/jinja.dict}"

run_target() {
    local mode="$1"
    local target="$2"
    shift 2
    local feature_args=("$@")
    local corpus_dir="$FUZZ_CORPUS_ROOT/$mode/$target"
    local artifact_dir="$FUZZ_ARTIFACT_ROOT/$mode/$target"
    local fuzz_args=(
        "-max_total_time=$FUZZ_MAX_TOTAL_TIME"
        "-max_len=$FUZZ_MAX_LEN"
        "-timeout=$FUZZ_TIMEOUT"
        "-print_final_stats=1"
        "-artifact_prefix=$artifact_dir/"
    )

    mkdir -p "$corpus_dir" "$artifact_dir"
    if [[ -f "$FUZZ_DICTIONARY" ]]; then
        fuzz_args+=("-dict=$FUZZ_DICTIONARY")
    fi

    printf '\n=== START %s/%s ===\n' "$mode" "$target"
    if [[ "$target" == "python_bridge" ]]; then
        LSAN_OPTIONS="${LSAN_OPTIONS:+$LSAN_OPTIONS:}detect_leaks=0" \
            cargo +nightly fuzz run \
            --no-default-features \
            "${feature_args[@]}" \
            "$target" "$corpus_dir" -- \
            "${fuzz_args[@]}"
    else
        cargo +nightly fuzz run \
            --no-default-features \
            "${feature_args[@]}" \
            "$target" "$corpus_dir" -- \
            "${fuzz_args[@]}"
    fi
    printf '=== PASS %s/%s ===\n' "$mode" "$target"
}

run_mode() {
    local mode="$1"
    shift
    local feature_args=("$@")
    local targets=("${RUST_TARGETS[@]}")
    if [[ "$mode" == "python" ]]; then
        targets+=(python_bridge)
    fi

    printf '\n=== BUILD %s ===\n' "$mode"
    cargo +nightly fuzz build \
        --no-default-features \
        "${feature_args[@]}"

    printf 'Configuration: mode=%s max_total_time=%s max_len=%s targets=%s\n' \
        "$mode" "$FUZZ_MAX_TOTAL_TIME" "$FUZZ_MAX_LEN" "${#targets[@]}"

    for target in "${targets[@]}"; do
        run_target "$mode" "$target" "${feature_args[@]}"
    done
}

run_mode no-python
run_mode python --features python

printf '\nAll Jinja fuzz targets completed successfully in both modes.\n'
