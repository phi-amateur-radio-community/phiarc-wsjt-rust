// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/util/ftx/crc.rs
// Calculate CRC.

use bitvec::prelude::{BitVec, Msb0};
use std::ops::BitXorAssign;

pub fn get_crc(m_bits: &BitVec<u8, Msb0>, g_bits: &BitVec<u8, Msb0>) -> BitVec<u8, Msb0> {
    let mut buffer_bits = m_bits.clone();
    let mut i = 0;
    let m_len = m_bits.len();
    let g_len = g_bits.len();
    let crc_len = g_len - 1;
    let buf_len = crc_len + m_len;
    buffer_bits.resize(buf_len, false);
    while g_len + i < buf_len {
        if buffer_bits[i] {
            let buffer_slice = &mut buffer_bits[i..(i + g_len)];
            buffer_slice.bitxor_assign(g_bits);
        }
        i += 1;
    }
    buffer_bits[m_len..buf_len].to_bitvec()
}
