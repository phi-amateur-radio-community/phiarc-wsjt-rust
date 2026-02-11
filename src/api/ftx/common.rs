// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/api/ftx/common.rs
// FTx common API.

use super::msg;
use crate::util::ftx::crc;
use bitvec::{
    bitvec,
    prelude::{BitVec, Msb0},
};

pub fn encode77(content: msg::Msg) -> Option<BitVec<u8, Msb0>> {
    content.encode()
}

pub fn encode91(bits: &mut BitVec<u8, Msb0>) {
    let g: BitVec<u8, Msb0> = bitvec![u8, Msb0; 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1];
    let crc_bits = crc::get_crc(bits, &g);
    bits.extend(crc_bits);
}
