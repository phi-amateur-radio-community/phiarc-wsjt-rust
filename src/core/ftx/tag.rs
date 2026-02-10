// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/core/ftx/tag.rs
// Tag of FTx.

use super::encode;
use bitvec::prelude::{BitVec, Msb0};

pub enum Tag {
    Sc1(bool),
    Sc28(Sc28Type),
    Sc58(String),
    Sf71(String),
    Sg15(Sg15Type),
    Sg25(String),
    Sh1(bool),
    Sh10(u16),
    Sh12(u16),
    Sh22(u32),
    Sk3(u8),
    Sn4(u8),
    Sp1(bool),
    Sr1(bool),
    Sr2(Sr2Type),
    Sr3(u8),
    R1(bool),
    Sr5(i8),
    Ss11(u16),
    Ss13(u16),
    S7(u8),
    St1(bool),
    St71(u128),
}

#[derive(PartialEq)]
pub enum TagType {
    Sc1,
    Sc28,
    Sc58,
    Sf71,
    Sg15,
    Sg25,
    Sh1,
    Sh10,
    Sh12,
    Sh22,
    Sk3,
    Sn4,
    Sp1,
    Sr1,
    Sr2,
    Sr3,
    R1,
    Sr5,
    Ss11,
    Ss13,
    S7,
    St1,
    St71,
}

pub enum Sc28Type {
    Callsign(String),
    Hash(u32),
    Cq(Sc28CqType),
    De,
    Qrz,
}

pub enum Sc28CqType {
    Empty,
    Number(u16),
    Letter(String, u8),
}

pub enum Sg15Type {
    Blank,
    Rrr,
    Rr73,
    N73,
    Grid4(String),
}

pub enum Sr2Type {
    Blank,
    Rrr,
    Rr73,
    N73,
}

impl Tag {
    pub fn encode(&self) -> Option<BitVec<u8, Msb0>> {
        encode::tag_encode(self)
    }

    pub fn get_type(&self) -> TagType {
        match self {
            Tag::Sc1(_) => TagType::Sc1,
            Tag::Sc28(_) => TagType::Sc28,
            Tag::Sc58(_) => TagType::Sc58,
            Tag::Sf71(_) => TagType::Sf71,
            Tag::Sg15(_) => TagType::Sg15,
            Tag::Sg25(_) => TagType::Sg25,
            Tag::Sh1(_) => TagType::Sh1,
            Tag::Sh10(_) => TagType::Sh10,
            Tag::Sh12(_) => TagType::Sh12,
            Tag::Sh22(_) => TagType::Sh22,
            Tag::Sk3(_) => TagType::Sk3,
            Tag::Sn4(_) => TagType::Sn4,
            Tag::Sp1(_) => TagType::Sp1,
            Tag::Sr1(_) => TagType::Sr1,
            Tag::Sr2(_) => TagType::Sr2,
            Tag::Sr3(_) => TagType::Sr3,
            Tag::R1(_) => TagType::R1,
            Tag::Sr5(_) => TagType::Sr5,
            Tag::Ss11(_) => TagType::Ss11,
            Tag::Ss13(_) => TagType::Ss13,
            Tag::S7(_) => TagType::S7,
            Tag::St1(_) => TagType::St1,
            Tag::St71(_) => TagType::St71,
        }
    }

    pub fn get_len(&self) -> usize {
        match self {
            Tag::Sc1(_) => 1,
            Tag::Sc28(_) => 28,
            Tag::Sc58(_) => 58,
            Tag::Sf71(_) => 71,
            Tag::Sg15(_) => 15,
            Tag::Sg25(_) => 25,
            Tag::Sh1(_) => 1,
            Tag::Sh10(_) => 10,
            Tag::Sh12(_) => 12,
            Tag::Sh22(_) => 22,
            Tag::Sk3(_) => 3,
            Tag::Sn4(_) => 4,
            Tag::Sp1(_) => 1,
            Tag::Sr1(_) => 1,
            Tag::Sr2(_) => 2,
            Tag::Sr3(_) => 3,
            Tag::R1(_) => 1,
            Tag::Sr5(_) => 5,
            Tag::Ss11(_) => 11,
            Tag::Ss13(_) => 13,
            Tag::S7(_) => 7,
            Tag::St1(_) => 1,
            Tag::St71(_) => 71,
        }
    }
}
