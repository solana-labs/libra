#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

cargo fmt --all -- --check
#cargo clippy --version
#cargo clippy --all -- --deny=warnings
cargo update
cargo audit --version
cargo audit --ignore RUSTSEC-2019-0011 # https://github.com/solana-labs/solana/issues/5207
#ci/nits.sh
ci/order-crates-for-publishing.py
