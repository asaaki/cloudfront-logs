#!/bin/sh

cd $(git rev-parse --show-toplevel)

RUSTDOCFLAGS="--cfg docsrs" \
    cargo +nightly doc \
    --all-features --no-deps
