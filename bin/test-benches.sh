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

unsupported_output="$TMP/windows.md"
if "$ROOT/bin/benches.sh" --test-platform windows --output "$unsupported_output" 2>"$TMP/unsupported.txt"; then
    printf '%s\n' 'expected unsupported platform failure' >&2
    exit 1
fi
test ! -e "$unsupported_output"
grep -Fq 'unsupported test platform: windows' "$TMP/unsupported.txt"

printf '%s\n' 'PASS'
