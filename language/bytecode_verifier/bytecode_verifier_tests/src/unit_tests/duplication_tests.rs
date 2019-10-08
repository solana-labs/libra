// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use proptest::prelude::*;
use solana_libra_bytecode_verifier::DuplicationChecker;
use solana_libra_vm::file_format::CompiledModule;

proptest! {
    #[test]
    fn valid_duplication(module in CompiledModule::valid_strategy(20)) {
        let duplication_checker = DuplicationChecker::new(&module);
        prop_assert!(!duplication_checker.verify().is_empty());
    }
}
