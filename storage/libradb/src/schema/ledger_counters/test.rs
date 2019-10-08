// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use super::*;
use proptest::prelude::*;
use solana_libra_schemadb::schema::assert_encode_decode;

proptest! {
    #[test]
    fn test_encode_decode(counters in any::<LedgerCounters>()) {
        assert_encode_decode::<LedgerCountersSchema>(&0, &counters);
    }
}
