#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

source ci/_

source ci/rust-version.sh nightly

export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

_ cargo +"$rust_nightly" build --all ${V:+--verbose}
