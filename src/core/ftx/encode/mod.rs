// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/core/ftx/encode/mod.rs
// Encoder module of FTx.

mod enchar;
mod enstr;
mod entag;

pub(super) use entag::tag_encode;
