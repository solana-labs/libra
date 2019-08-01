#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."


export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

cargo build --manifest-path language/e2e_tests/Cargo.toml ${V:+--verbose}
cargo test --manifest-path language/e2e_tests/Cargo.toml ${V:+--verbose} -- --nocapture

