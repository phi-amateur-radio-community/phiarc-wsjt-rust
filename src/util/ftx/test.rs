// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/util/ftx/test.rs
// Unit test of ftx tools.

use super::*;
use bitvec::{
    bitvec,
    prelude::{BitVec, Msb0},
};

#[test]
fn test_crc() {
    let m_bits: BitVec<u8, Msb0> = BitVec::<u8, Msb0>::from_element(0x14u8);
    let g_bits: BitVec<u8, Msb0> = bitvec![u8, Msb0; 1, 0, 1, 0];
    let r_bits: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0, 0, 0];
    let c_bits: BitVec<u8, Msb0> = crc::get_crc(&m_bits, &g_bits);
    assert_eq!(c_bits, r_bits);

    let m_bits: BitVec<u8, Msb0> = BitVec::<u8, Msb0>::from_element(0x7u8);
    let g_bits: BitVec<u8, Msb0> = bitvec![u8, Msb0; 1, 1, 0];
    let r_bits: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0, 0];
    let c_bits: BitVec<u8, Msb0> = crc::get_crc(&m_bits, &g_bits);
    assert_eq!(c_bits, r_bits);

    let m_bits: BitVec<u8, Msb0> = BitVec::<u8, Msb0>::from_element(0x47u8);
    let g_bits: BitVec<u8, Msb0> = bitvec![u8, Msb0; 1, 1, 0, 0, 1];
    let r_bits: BitVec<u8, Msb0> = bitvec![u8, Msb0; 1, 1, 1, 0];
    let c_bits: BitVec<u8, Msb0> = crc::get_crc(&m_bits, &g_bits);
    assert_eq!(c_bits, r_bits);
}
