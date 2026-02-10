// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/util/ftx/bits.rs
// Tools about bit vections.

use bitvec::prelude::{BitVec, Msb0};
use std::ops::{BitAnd, Shr};

pub fn bits_extend<T>(bits: &mut BitVec<u8, Msb0>, number: T, len: u8)
where
    T: BitAnd<Output = T> + Shr<Output = T> + PartialEq + From<u8> + Copy,
{
    for i in (0u8..len).rev() {
        bits.push((number >> T::from(i)) & T::from(1u8) == T::from(1u8));
    }
}
