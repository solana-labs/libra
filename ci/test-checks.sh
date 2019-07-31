#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

source ci/_
source ci/rust-version.sh nightly

export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

#_ cargo +"$rust_nightly" fmt --all -- --check
#_ cargo +"$rust_nightly" clippy --version
#_ cargo +"$rust_nightly" clippy --all -- --deny=warnings
_ cargo +"$rust_nightly" update
_ cargo +"$rust_nightly" audit --version
_ cargo +"$rust_nightly" audit --ignore RUSTSEC-2019-0011 # https://github.com/solana-labs/solana/issues/5207
#_ ci/nits.sh
_ ci/order-crates-for-publishing.py

echo --- ok
