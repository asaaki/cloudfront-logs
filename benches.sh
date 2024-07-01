#!/bin/sh
# export RUST_BACKTRACE=1
export RUSTFLAGS="-Ctarget-cpu=native"
cargo bench -q --all-features
