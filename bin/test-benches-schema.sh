#!/bin/sh
set -eu

ROOT=$(git rev-parse --show-toplevel)

extract_bash_schema() {
    sed -n '/^done <<'\''EOF'\''$/,/^EOF$/p' "$ROOT/bin/benches.sh" |
        sed '1d;$d' |
        while IFS='|' read -r configuration feature_args; do
            for target in brwv brwu; do
                printf '%s|%s|%s\n' "$configuration" "$feature_args" "$target"
            done
        done
}

expected='no-features|--no-default-features|brwv
no-features|--no-default-features|brwu
jiff|--no-default-features --features jiff|brwv
jiff|--no-default-features --features jiff|brwu
time|--no-default-features --features time|brwv
time|--no-default-features --features time|brwu
chrono|--no-default-features --features chrono|brwv
chrono|--no-default-features --features chrono|brwu
parquet|--no-default-features --features parquet|brwv
parquet|--no-default-features --features parquet|brwu'

test "$(extract_bash_schema)" = "$expected"

for value in \
    'Label = "no-features"; Features = @("--no-default-features")' \
    'Label = "jiff"; Features = @("--no-default-features", "--features", "jiff")' \
    'Label = "time"; Features = @("--no-default-features", "--features", "time")' \
    'Label = "chrono"; Features = @("--no-default-features", "--features", "chrono")' \
    'Label = "parquet"; Features = @("--no-default-features", "--features", "parquet")' \
    'Name = "brwv"; Description = "validated parsers"' \
    'Name = "brwu"; Description = "unvalidated parsers"'
do
    grep -Fq "$value" "$ROOT/bin/benches.ps1"
done

grep -Fq 'if (-not $IsWindows -and $TestPlatform -ne "windows")' "$ROOT/bin/benches.ps1"
grep -Fq 'Configuration '\''$($configuration.Label)'\'', benchmark target '\''$($benchmark.Name)'\''' "$ROOT/bin/benches.ps1"
grep -Fq 'Write-Warning "Could not remove benchmark backup' "$ROOT/bin/benches.ps1"

printf '%s\n' 'PASS'
