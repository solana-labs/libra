#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

source ci/_
source ci/rust-version.sh stable

export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

_ cargo +"$rust_stable" fmt --all -- --check
_ cargo +"$rust_stable" clippy --version
_ cargo +"$rust_stable" clippy --all -- --deny=warnings
_ cargo +"$rust_stable" audit --version
_ cargo +"$rust_stable" audit --ignore RUSTSEC-2019-0011 # https://github.com/solana-labs/solana/issues/5207
_ ci/nits.sh
_ ci/order-crates-for-publishing.py

echo --- ok
