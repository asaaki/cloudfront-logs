#!/bin/sh
cd $(git rev-parse --show-toplevel)

# export RUST_BACKTRACE=1
export RUSTFLAGS="-Ctarget-cpu=native"
cargo bench -q --all-features
