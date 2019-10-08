#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

cargo build --all ${V:+--verbose}
