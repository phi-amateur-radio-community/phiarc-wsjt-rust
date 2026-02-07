// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/util/ftx/hash.rs
// Generate a hash.

const HASH_MULTIPLIER: u64 = 47055833459;

pub fn generate(number: u64, len: usize) -> u64 {
    (number * HASH_MULTIPLIER) >> len
}
