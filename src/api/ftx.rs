// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/api/ftx.rs
// API module of FTx(FT8, FT4 and other).

use bitvec::vec::BitVec;

pub enum FtxType {
    #[cfg(feature = "ft8")]
    Ft8(Msg),
}

pub struct Msg {
    msg: BitVec,
}
