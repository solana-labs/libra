// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use super::*;
use solana_libra_crypto::HashValue;
use solana_libra_schemadb::schema::assert_encode_decode;
use solana_libra_types::{transaction::TransactionInfo, vm_error::StatusCode};

#[test]
fn test_encode_decode() {
    let txn_info = TransactionInfo::new(
        HashValue::random(),
        HashValue::random(),
        HashValue::random(),
        7,
        StatusCode::EXECUTED,
    );
    assert_encode_decode::<TransactionInfoSchema>(&0u64, &txn_info);
}
