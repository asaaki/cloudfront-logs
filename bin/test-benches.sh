#!/bin/sh
set -eu

ROOT=$(git rev-parse --show-toplevel)
TMP=$(mktemp -d 2>/dev/null || mktemp -d -t cloudfront-test-benches)
trap 'rm -rf "$TMP"' EXIT INT TERM HUP

FAKE_BIN="$TMP/bin"
mkdir "$FAKE_BIN"

cat >"$FAKE_BIN/cargo" <<'EOF'
#!/bin/sh
if [ "$1" = "--version" ]; then
    printf '%s\n' 'cargo 1.0.0 (fake)'
    exit 0
fi
printf '%s\n' "$*" >>"$CARGO_ARGUMENTS_FILE"
invocation=$(wc -l <"$CARGO_ARGUMENTS_FILE" | tr -d ' ')
if [ "${FAIL_CARGO_AT:-0}" -eq "$invocation" ]; then
    printf '%s\n' 'benchmark failed' >&2
    exit 23
fi
printf 'cargo %s\n' "$*"
EOF
chmod +x "$FAKE_BIN/cargo"

cat >"$FAKE_BIN/rustc" <<'EOF'
#!/bin/sh
printf '%s\n' 'rustc 1.0.0 (fake)'
EOF
chmod +x "$FAKE_BIN/rustc"

PATH="$FAKE_BIN:$PATH"
export PATH CARGO_ARGUMENTS_FILE

expected_arguments='bench -q --no-default-features --bench brwv
bench -q --no-default-features --bench brwu
bench -q --no-default-features --features jiff --bench brwv
bench -q --no-default-features --features jiff --bench brwu
bench -q --no-default-features --features time --bench brwv
bench -q --no-default-features --features time --bench brwu
bench -q --no-default-features --features chrono --bench brwv
bench -q --no-default-features --features chrono --bench brwu
bench -q --no-default-features --features parquet --bench brwv
bench -q --no-default-features --features parquet --bench brwu'

for platform in macos linux wsl; do
    CARGO_ARGUMENTS_FILE="$TMP/$platform-cargo-arguments.txt"
    export CARGO_ARGUMENTS_FILE
    output="$TMP/$platform.md"
    "$ROOT/bin/benches.sh" --test-platform "$platform" --output "$output"
    test -f "$output"
    grep -Fq -- "- Platform: \`$platform\`" "$output"
    test "$(grep -c '^## Configuration:' "$output")" -eq 5
    test "$(grep -c '^### `brwv`' "$output")" -eq 5
    test "$(grep -c '^### `brwu`' "$output")" -eq 5
    test "$(cat "$CARGO_ARGUMENTS_FILE")" = "$expected_arguments"
done

output="$TMP/unchanged.md"
printf '%s\n' 'existing content' >"$output"
CARGO_ARGUMENTS_FILE="$TMP/failing-cargo-arguments.txt"
export CARGO_ARGUMENTS_FILE
if FAIL_CARGO_AT=8 "$ROOT/bin/benches.sh" --test-platform linux --output "$output" 2>"$TMP/failure.txt"; then
    printf '%s\n' 'expected Cargo failure' >&2
    exit 1
fi
test "$(cat "$output")" = 'existing content'
grep -Fq 'configuration chrono' "$TMP/failure.txt"
grep -Fq "benchmark target 'brwu'" "$TMP/failure.txt"

CARGO_ARGUMENTS_FILE="$TMP/series-arguments.txt"
export CARGO_ARGUMENTS_FILE
"$ROOT/bin/benches.sh" --test-platform linux --output "$TMP/series.md" --repetitions 2 --config-order chrono,no-features --alloc -- --sample-count 2 --sample-size 1
for repetition in 001 002; do
    report="$TMP/series.run-$repetition.md"
    test -f "$report"
    grep -Fq 'Configuration order: `chrono,no-features`' "$report"
    grep -Fq 'bench-alloc' "$report"
done
test "$(wc -l <"$CARGO_ARGUMENTS_FILE" | tr -d ' ')" -eq 8
test "$(head -n 1 "$CARGO_ARGUMENTS_FILE")" = 'bench -q --no-default-features --features chrono,bench-alloc --bench brwv -- --sample-count 2 --sample-size 1'
if "$ROOT/bin/benches.sh" --test-platform linux --output "$TMP/series.md" --repetitions 2 2>"$TMP/collision.txt"; then
    printf '%s\n' 'expected repetition collision failure' >&2
    exit 1
fi
grep -Fq 'already exists' "$TMP/collision.txt"
test "$(wc -l <"$CARGO_ARGUMENTS_FILE" | tr -d ' ')" -eq 8

CARGO_ARGUMENTS_FILE="$TMP/partial-arguments.txt"
export CARGO_ARGUMENTS_FILE
if FAIL_CARGO_AT=6 "$ROOT/bin/benches.sh" --test-platform linux --output "$TMP/partial.md" --repetitions 2 --config-order no-features,jiff 2>"$TMP/partial.txt"; then
    printf '%s\n' 'expected later repetition failure' >&2
    exit 1
fi
test -f "$TMP/partial.run-001.md"
test ! -e "$TMP/partial.run-002.md"

for invalid_order in 'jiff,jiff' 'unknown' 'jiff,,time' ' ' 'jiff time'; do
    if "$ROOT/bin/benches.sh" --test-platform linux --output "$TMP/invalid.md" --config-order "$invalid_order" 2>"$TMP/invalid.txt"; then
        printf '%s\n' 'expected configuration validation failure' >&2
        exit 1
    fi
    test ! -e "$TMP/invalid.md"
done
if "$ROOT/bin/benches.sh" --test-platform linux -- --sample-count 2 2>"$TMP/no-output.txt"; then
    printf '%s\n' 'expected explicit smoke output requirement' >&2
    exit 1
fi
grep -Fq 'require --output' "$TMP/no-output.txt"

# Fail the first fake benchmark if protection is absent, preserving real reports.
CARGO_ARGUMENTS_FILE="$TMP/subset-arguments.txt"
export CARGO_ARGUMENTS_FILE
if FAIL_CARGO_AT=1 "$ROOT/bin/benches.sh" --test-platform linux --config-order jiff 2>"$TMP/subset.txt"; then
    printf '%s\n' 'expected explicit subset output requirement' >&2
    exit 1
fi
grep -Fq 'require --output' "$TMP/subset.txt"
test ! -e "$CARGO_ARGUMENTS_FILE"

for name in DIVAN_MIN_TIME DIVAN_MAX_TIME DIVAN_SKIP_EXT_TIME; do
    if env "$name=1" FAIL_CARGO_AT=1 "$ROOT/bin/benches.sh" --test-platform linux 2>"$TMP/override.txt"; then
        printf '%s\n' 'expected explicit environment override output requirement' >&2
        exit 1
    fi
    grep -Fq 'require --output' "$TMP/override.txt"
    test ! -e "$CARGO_ARGUMENTS_FILE"
    env "$name=1" "$ROOT/bin/benches.sh" --test-platform linux --output "$TMP/$name.md" --config-order jiff
    grep -Fq -- "- $name: \`1\`" "$TMP/$name.md"
    rm "$CARGO_ARGUMENTS_FILE"
done

unsupported_output="$TMP/windows.md"
if "$ROOT/bin/benches.sh" --test-platform windows --output "$unsupported_output" 2>"$TMP/unsupported.txt"; then
    printf '%s\n' 'expected unsupported platform failure' >&2
    exit 1
fi
test ! -e "$unsupported_output"
grep -Fq 'unsupported test platform: windows' "$TMP/unsupported.txt"

printf '%s\n' 'PASS'
