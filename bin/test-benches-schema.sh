#!/bin/sh
set -eu

# Historical entry point: exercise emitted reports and Cargo arguments instead
# of grepping source text. Windows has the matching bin/test-benches.ps1 suite.
ROOT=$(git rev-parse --show-toplevel)
exec sh "$ROOT/bin/test-benches.sh"
