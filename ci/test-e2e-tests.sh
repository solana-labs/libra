#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

source ci/_

source ci/rust-version.sh stable

export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

  echo "Testing e2e_tests"

  _ cargo +"$rust_stable" build --manifest-path language/e2e_tests/Cargo.toml ${V:+--verbose}
  _ cargo +"$rust_stable" test --manifest-path language/e2e_tests/Cargo.toml ${V:+--verbose} -- --nocapture

