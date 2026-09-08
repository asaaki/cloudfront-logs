#!/bin/sh
set -eu

fail() {
    printf 'error: %s\n' "$1" >&2
    exit 2
}

usage() {
    cat <<'EOF'
Usage: ./bin/benches.sh [--output PATH] [--test-platform PLATFORM]

Runs benchmark targets `brwv` and `brwu` for each feature configuration and
writes a complete Markdown report.
EOF
}

detect_platform() {
    case "$(uname -s)" in
        Darwin) PLATFORM=macos ;;
        Linux)
            if grep -qi microsoft /proc/version 2>/dev/null || [ -n "${WSL_DISTRO_NAME:-}" ]; then
                PLATFORM=wsl
            else
                PLATFORM=linux
            fi
            ;;
        *) fail "unsupported platform" ;;
    esac
}

OUTPUT=""
TEST_PLATFORM=""
while [ "$#" -gt 0 ]; do
    case "$1" in
        --output)
            shift
            [ "$#" -gt 0 ] || fail "missing PATH for --output"
            OUTPUT=$1
            ;;
        --test-platform)
            shift
            [ "$#" -gt 0 ] || fail "missing PLATFORM for --test-platform"
            TEST_PLATFORM=$1
            ;;
        -h | --help)
            usage
            exit 0
            ;;
        *) fail "unknown option: $1" ;;
    esac
    shift
done

if [ -n "$TEST_PLATFORM" ]; then
    case "$TEST_PLATFORM" in
        macos | linux | wsl) PLATFORM=$TEST_PLATFORM ;;
        *) fail "unsupported test platform: $TEST_PLATFORM" ;;
    esac
else
    detect_platform
fi

ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"

: "${RUSTFLAGS:=-Ctarget-cpu=native}"
export RUSTFLAGS

RUN_DATE=$(date '+%Y-%m-%d %H:%M:%S %z' 2>/dev/null || date)
OS_NAME=$(uname -srmo 2>/dev/null || uname -a)
CPU_MODEL=""
RAM_GIB=""

if [ "$PLATFORM" = macos ]; then
    CPU_MODEL=$(sysctl -n machdep.cpu.brand_string 2>/dev/null || true)
    RAM_BYTES=$(sysctl -n hw.memsize 2>/dev/null || true)
    case "$RAM_BYTES" in
        '' | *[!0-9]*) ;;
        *) RAM_GIB=$(awk "BEGIN {printf \"%.1f\", $RAM_BYTES/1073741824}") ;;
    esac
else
    if command -v lscpu >/dev/null 2>&1; then
        CPU_MODEL=$(lscpu 2>/dev/null | sed -n 's/^Model name:[[:space:]]*//p' | sed -n '1p' || true)
    fi
    if [ -z "$CPU_MODEL" ] && [ -r /proc/cpuinfo ]; then
        CPU_MODEL=$(sed -n 's/^model name[[:space:]]*:[[:space:]]*//p' /proc/cpuinfo | sed -n '1p' || true)
    fi
    if [ -r /proc/meminfo ]; then
        RAM_GIB=$(awk '/MemTotal:/ {printf "%.1f", $2/1024/1024; exit}' /proc/meminfo 2>/dev/null || true)
    fi
fi

[ -n "$CPU_MODEL" ] || CPU_MODEL=unknown
if [ -n "$RAM_GIB" ]; then
    RAM_TOTAL="$RAM_GIB GiB"
else
    RAM_TOTAL=unknown
fi

RUSTC_VERSION=$(rustc --version 2>/dev/null || printf '%s\n' 'unknown')
CARGO_VERSION=$(cargo --version 2>/dev/null || printf '%s\n' 'unknown')
GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || printf '%s\n' 'unknown')

[ -n "$OUTPUT" ] || OUTPUT="benchmarks/$PLATFORM.md"
OUTPUT_DIR=$(dirname "$OUTPUT")
mkdir -p "$OUTPUT_DIR"
TMP_FILE=$(mktemp "$OUTPUT_DIR/.benches.XXXXXX")
trap 'rm -f "$TMP_FILE"' EXIT INT TERM HUP

cat >"$TMP_FILE" <<EOF
# Benchmarks: $PLATFORM

## Benchmark environment

- Platform: \`$PLATFORM\`
- Run date: \`$RUN_DATE\`
- OS: \`$OS_NAME\`
- CPU: \`$CPU_MODEL\`
- RAM: \`$RAM_TOTAL\`
- Toolchain: \`$RUSTC_VERSION\`
- Cargo: \`$CARGO_VERSION\`
- Git commit: \`$GIT_COMMIT\`
- RUSTFLAGS: \`$RUSTFLAGS\`
EOF

run_benchmark() {
    configuration=$1
    feature_args=$2
    target=$3
    description=$4
    result_file=$(mktemp "$OUTPUT_DIR/.benchmark.XXXXXX")

    printf '\n### `%s` (%s)\n\n```txt\n' "$target" "$description" >>"$TMP_FILE"
    if cargo bench -q $feature_args --bench "$target" >"$result_file" 2>&1; then
        cat "$result_file"
        cat "$result_file" >>"$TMP_FILE"
        rm -f "$result_file"
    else
        status=$?
        cat "$result_file" >&2
        rm -f "$result_file"
        fail "configuration $configuration, benchmark target '$target' failed with exit code $status"
    fi
    printf '```\n' >>"$TMP_FILE"
}

while IFS='|' read -r configuration feature_args; do
    printf '\n## Configuration: %s\n' "$configuration" >>"$TMP_FILE"
    run_benchmark "$configuration" "$feature_args" brwv 'validated parsers'
    run_benchmark "$configuration" "$feature_args" brwu 'unvalidated parsers'
done <<'EOF'
no-features|--no-default-features
jiff|--no-default-features --features jiff
time|--no-default-features --features time
chrono|--no-default-features --features chrono
parquet|--no-default-features --features parquet
EOF

mv "$TMP_FILE" "$OUTPUT"
trap - EXIT INT TERM HUP
printf 'Wrote benchmark report to %s\n' "$OUTPUT" >&2
