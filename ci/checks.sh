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
cargo audit
#ci/nits.sh
ci/order-crates-for-publishing.py
