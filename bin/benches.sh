#!/bin/sh
set -eu

usage() {
    cat <<'EOF'
Usage: ./bin/benches.sh [--doc] [--doc-file PATH]

Runs benchmark targets `brwv` and `brwu`.

Options:
  --doc            Print a BENCHMARK.md-compatible markdown block to stdout.
  --doc-file PATH  Write the markdown block to PATH.
  -h, --help       Show this help.
EOF
}

PRINT_DOC=0
WRITE_DOC=0
DOC_FILE=""

while [ "$#" -gt 0 ]; do
    case "$1" in
        --doc)
            PRINT_DOC=1
            ;;
        --doc-file)
            shift
            if [ "$#" -eq 0 ]; then
                printf '%s\n' 'error: missing PATH for --doc-file' >&2
                exit 2
            fi
            WRITE_DOC=1
            DOC_FILE="$1"
            ;;
        -h | --help)
            usage
            exit 0
            ;;
        *)
            printf 'error: unknown option: %s\n' "$1" >&2
            usage >&2
            exit 2
            ;;
    esac
    shift
done

cd "$(git rev-parse --show-toplevel)"

# export RUST_BACKTRACE=1
: "${RUSTFLAGS:=-Ctarget-cpu=native}"
export RUSTFLAGS

RUN_DATE="$(date '+%Y-%m-%d %H:%M:%S %z' 2>/dev/null || date)"
OS_NAME="$(uname -srmo 2>/dev/null || uname -a)"
CPU_MODEL="$(lscpu 2>/dev/null | sed -n 's/^Model name:[[:space:]]*//p' | head -n1 | sed -e 's/[[:space:]]*$//' || true)"
if [ -z "$CPU_MODEL" ] && [ -r /proc/cpuinfo ]; then
    CPU_MODEL="$(sed -n 's/^model name[[:space:]]*:[[:space:]]*//p' /proc/cpuinfo | head -n1 | sed -e 's/[[:space:]]*$//' || true)"
fi
[ -n "$CPU_MODEL" ] || CPU_MODEL="unknown"

RAM_GIB="$(awk '/MemTotal:/ {printf "%.1f", $2/1024/1024}' /proc/meminfo 2>/dev/null || true)"
if [ -n "$RAM_GIB" ]; then
    RAM_TOTAL="${RAM_GIB} GiB"
else
    RAM_TOTAL="unknown"
fi

RUSTC_VERSION="$(rustc --version 2>/dev/null || echo 'rustc (not found)')"
CARGO_VERSION="$(cargo --version 2>/dev/null || echo 'cargo (not found)')"
GIT_COMMIT="$(git rev-parse --short HEAD 2>/dev/null || echo 'unknown')"

printf '%s\n' "Benchmark run metadata:"
printf '  Run date: %s\n' "$RUN_DATE"
printf '  OS: %s\n' "$OS_NAME"
printf '  CPU: %s\n' "$CPU_MODEL"
printf '  RAM: %s\n' "$RAM_TOTAL"
printf '  Toolchain: %s\n' "$RUSTC_VERSION"
printf '  Cargo: %s\n' "$CARGO_VERSION"
printf '  Git commit: %s\n' "$GIT_COMMIT"
printf '  RUSTFLAGS: %s\n' "$RUSTFLAGS"
printf '\n'

TMP_DIR="$(mktemp -d 2>/dev/null || mktemp -d -t cloudfront-benches)"
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM HUP

BRWV_OUT="$TMP_DIR/brwv.txt"
BRWU_OUT="$TMP_DIR/brwu.txt"

cargo bench -q --all-features --bench brwv 2>&1 | tee "$BRWV_OUT"
cargo bench -q --all-features --bench brwu 2>&1 | tee "$BRWU_OUT"

if [ "$PRINT_DOC" -eq 1 ] || [ "$WRITE_DOC" -eq 1 ]; then
    DOC_TMP="$TMP_DIR/benchmark-doc-block.md"

    cat >"$DOC_TMP" <<EOF
# Benchmarks

This document tracks the current benchmark targets defined in \`Cargo.toml\`:

- \`brwv\` -> \`benches/borrowed-real-world-validated.rs\`
- \`brwu\` -> \`benches/borrowed-real-world-unvalidated.rs\`

## Benchmark environment

- Run date: \`$RUN_DATE\`
- OS: \`$OS_NAME\`
- CPU: \`$CPU_MODEL\`
- RAM: \`$RAM_TOTAL\`
- Toolchain: \`$RUSTC_VERSION\`
- Cargo: \`$CARGO_VERSION\`
- Git commit: \`$GIT_COMMIT\`

## Commands

\`\`\`shell
RUSTFLAGS="-Ctarget-cpu=native" cargo bench -q --all-features --bench brwv
RUSTFLAGS="-Ctarget-cpu=native" cargo bench -q --all-features --bench brwu
\`\`\`

## Results: \`brwv\` (validated parsers)

\`\`\`txt
$(cat "$BRWV_OUT")
\`\`\`

## Results: \`brwu\` (unvalidated parsers)

\`\`\`txt
$(cat "$BRWU_OUT")
\`\`\`

These numbers are synthetic and depend on hardware, toolchain version, and CPU frequency scaling.
EOF

    if [ "$WRITE_DOC" -eq 1 ]; then
        cp "$DOC_TMP" "$DOC_FILE"
        printf 'Wrote markdown doc block to %s\n' "$DOC_FILE" >&2
    fi

    if [ "$PRINT_DOC" -eq 1 ]; then
        cat "$DOC_TMP"
    fi
fi
