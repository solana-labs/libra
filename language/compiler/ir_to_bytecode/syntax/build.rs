// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "build-lalrpop")]
fn main() {
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .process()
        .unwrap();
}

#[cfg(not(feature = "build-lalrpop"))]
fn main() {}
