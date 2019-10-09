#!/usr/bin/env bash
set -xe

cd "$(dirname "$0")/.."

export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

cargo fmt --all -- --check
#cargo clippy --version
#cargo clippy --all -- --deny=warnings
cargo update
cargo audit --version
cargo audit --ignore RUSTSEC-2016-0005 --ignore RUSTSEC-2018-0015
#ci/nits.sh
#ci/order-crates-for-publishing.py #https://github.com/solana-labs/libra/issues/36
