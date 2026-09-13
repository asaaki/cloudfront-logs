#!/bin/sh
set -eu

fail() {
    printf 'error: %s\n' "$1" >&2
    exit 2
}

usage() {
    cat <<'EOF'
Usage: ./bin/benches.sh [--output PATH] [--repetitions N]
                      [--config-order no-features,jiff,time,chrono,parquet]
                      [--alloc] [--test-platform PLATFORM] [-- DIVAN_ARGS...]

Runs benchmark targets `brwv` and `brwu` for each feature configuration and
writes a complete Markdown report. Repetitions use .run-NNN suffixes and never
replace existing reports. Custom/short runs require an explicit output path.
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
REPETITIONS=1
CONFIG_ORDER=no-features,jiff,time,chrono,parquet
CUSTOM_ORDER=0
ALLOC_PROFILE=0
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
        --repetitions)
            shift
            [ "$#" -gt 0 ] || fail 'missing N for --repetitions'
            REPETITIONS=$1
            ;;
        --config-order)
            shift
            [ "$#" -gt 0 ] || fail 'missing list for --config-order'
            CONFIG_ORDER=$1
            CUSTOM_ORDER=1
            ;;
        --alloc) ALLOC_PROFILE=1 ;;
        --)
            shift
            break
            ;;
        *) fail "unknown option: $1" ;;
    esac
    shift
done

case "$REPETITIONS" in '' | *[!0-9]*) fail 'repetitions must be an integer from 1 to 10000' ;; esac
[ "$REPETITIONS" -ge 1 ] && [ "$REPETITIONS" -le 10000 ] || fail 'repetitions must be from 1 to 10000'
if [ "$#" -gt 0 ] || [ "$ALLOC_PROFILE" -eq 1 ] || [ "$REPETITIONS" -gt 1 ] || [ "$CUSTOM_ORDER" -eq 1 ] || [ -n "${DIVAN_SAMPLE_COUNT:-}${DIVAN_SAMPLE_SIZE:-}${DIVAN_MIN_TIME:-}${DIVAN_MAX_TIME:-}${DIVAN_SKIP_EXT_TIME:-}${CF_BENCH_RECORDS:-}" ]; then
    [ -n "$OUTPUT" ] || fail 'custom runs require --output to protect published platform reports'
fi
case "$CONFIG_ORDER" in '' | ,* | *, | *,,*) fail 'configuration order must be nonempty without empty entries' ;; esac
case "$CONFIG_ORDER" in *[!a-z,-]*) fail 'configuration order must contain comma-separated feature names' ;; esac
SEEN=,
CONFIGURATIONS=$(printf '%s' "$CONFIG_ORDER" | tr ',' ' ')
for configuration in $CONFIGURATIONS; do
    case "$configuration" in no-features | jiff | time | chrono | parquet) ;; *) fail "unknown configuration: $configuration" ;; esac
    case "$SEEN" in *,$configuration,*) fail "duplicate configuration: $configuration" ;; esac
    SEEN="$SEEN$configuration,"
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
if GIT_STATUS=$(git status --porcelain 2>/dev/null); then
    if [ -n "$GIT_STATUS" ]; then WORKTREE_STATE=dirty; else WORKTREE_STATE=clean; fi
else
    WORKTREE_STATE=unknown
fi

[ -n "$OUTPUT" ] || OUTPUT="benchmarks/$PLATFORM.md"
OUTPUT_DIR=$(dirname "$OUTPUT")
BASE_OUTPUT=$OUTPUT

report_path() {
    if [ "$REPETITIONS" -eq 1 ]; then
        printf '%s\n' "$BASE_OUTPUT"
    else
        filename=$(basename "$BASE_OUTPUT")
        case "$filename" in
            *.*) stem=${filename%.*}; extension=.${filename##*.} ;;
            *) stem=$filename; extension='' ;;
        esac
        printf '%s/%s.run-%03d%s\n' "$OUTPUT_DIR" "$stem" "$1" "$extension"
    fi
}

if [ "$REPETITIONS" -gt 1 ]; then
    repetition=1
    while [ "$repetition" -le "$REPETITIONS" ]; do
        destination=$(report_path "$repetition")
        [ ! -e "$destination" ] || fail "repetition report already exists: $destination"
        repetition=$((repetition + 1))
    done
fi

mkdir -p "$OUTPUT_DIR"

run_benchmark() {
    configuration=$1
    feature_args=$2
    target=$3
    description=$4
    shift 4
    result_file=$(mktemp "$OUTPUT_DIR/.benchmark.XXXXXX")
    printf '\n### `%s` (%s)\n\n```txt\n' "$target" "$description" >>"$TMP_FILE"
    # Only validated feature names are split; user arguments remain in "$@".
    if cargo bench -q $feature_args --bench "$target" "$@" >"$result_file" 2>&1; then
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

if [ "$#" -gt 0 ]; then set -- -- "$@"; fi
repetition=1
while [ "$repetition" -le "$REPETITIONS" ]; do
OUTPUT=$(report_path "$repetition")
RUN_DATE=$(date '+%Y-%m-%d %H:%M:%S %z' 2>/dev/null || date)
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
- Git worktree: \`$WORKTREE_STATE\`
- RUSTFLAGS: \`$RUSTFLAGS\`
- Configuration order: \`$CONFIG_ORDER\`
- Repetition: $repetition / $REPETITIONS
- Benchmark arguments: \`$*\`
- Allocation profiling: \`$([ "$ALLOC_PROFILE" -eq 1 ] && printf 'bench-alloc (instrumented timing)' || printf disabled)\`
- DIVAN_SAMPLE_COUNT: \`${DIVAN_SAMPLE_COUNT:-unset; see .cargo/config.toml and workload defaults}\`
- DIVAN_SAMPLE_SIZE: \`${DIVAN_SAMPLE_SIZE:-unset; see .cargo/config.toml and workload defaults}\`
- DIVAN_MIN_TIME: \`${DIVAN_MIN_TIME:-unset; workload default}\`
- DIVAN_MAX_TIME: \`${DIVAN_MAX_TIME:-unset; workload default}\`
- DIVAN_SKIP_EXT_TIME: \`${DIVAN_SKIP_EXT_TIME:-unset; workload default}\`
- DIVAN_BYTES_FORMAT: \`${DIVAN_BYTES_FORMAT:-unset; see .cargo/config.toml}\`
- CF_BENCH_RECORDS: \`${CF_BENCH_RECORDS:-unset; workload default}\`
- CARGO_TARGET_DIR: \`${CARGO_TARGET_DIR:-unset}\`
EOF

for configuration in $CONFIGURATIONS; do
    feature_args=--no-default-features
    features=''
    if [ "$configuration" != no-features ]; then features=$configuration; fi
    if [ "$ALLOC_PROFILE" -eq 1 ]; then features=${features:+$features,}bench-alloc; fi
    if [ -n "$features" ]; then feature_args="$feature_args --features $features"; fi
    printf '\n## Configuration: %s\n' "$configuration" >>"$TMP_FILE"
    run_benchmark "$configuration" "$feature_args" brwv 'validated parsers' "$@"
    run_benchmark "$configuration" "$feature_args" brwu 'unvalidated parsers' "$@"
done

if [ "$REPETITIONS" -eq 1 ]; then
    mv "$TMP_FILE" "$OUTPUT"
else
    # Atomic publication without replacement if another writer won the race.
    ln "$TMP_FILE" "$OUTPUT" || fail "cannot publish repetition report: $OUTPUT"
    rm -f "$TMP_FILE"
fi
trap - EXIT INT TERM HUP
printf 'Wrote benchmark report to %s\n' "$OUTPUT" >&2
repetition=$((repetition + 1))
done
