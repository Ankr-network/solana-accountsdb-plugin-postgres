#!/usr/bin/env bash

set -e
cd "$(dirname "$0")/.."

source ./ci/rust-version.sh stable

export RUSTFLAGS="-D warnings"
export RUSTBACKTRACE=1

set -x

# Build all host crates
cargo +"$rust_stable" build --release

exit 0
