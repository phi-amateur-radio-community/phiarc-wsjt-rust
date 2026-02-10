// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/api/ftx/common.rs
// FTx common API.

use super::msg;
use bitvec::prelude::{BitVec, Msb0};

pub fn encode77(content: msg::Msg) -> Option<BitVec<u8, Msb0>> {
    content.encode()
}

//TODO encode91
//TODO encode174
