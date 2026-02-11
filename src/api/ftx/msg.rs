// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/api/ftx/msg.rs
// Code about FTx message.

use crate::core::ftx::tag::{Tag, TagType};
use crate::util::ftx::bits::bits_extend;
use bitvec::prelude::{BitVec, Msb0};

pub struct Msg {
    msg_type: MsgType,
    msg_content: Vec<Tag>,
}

impl Msg {
    pub fn encode(&self) -> Option<BitVec<u8, Msb0>> {
        let mut bits = self.msg_type.get_title();
        for (t, o) in self
            .msg_content
            .iter()
            .zip(self.msg_type.get_order().iter())
        {
            if t.get_type() == *o {
                bits.extend(t.encode()?);
            } else {
                return None;
            }
        }
        Some(bits)
    }
}

pub enum MsgType {
    Msg0_0,
    Msg0_1,
    Msg0_3,
    Msg0_4,
    Msg0_5,
    Msg1,
    Msg2,
    Msg3,
    Msg4,
    Msg5,
}

const MSG0_0: [TagType; 1] = [TagType::Sf71];
const MSG0_1: [TagType; 4] = [TagType::Sc28, TagType::Sc28, TagType::Sh10, TagType::Sr5];
const MSG0_3: [TagType; 6] = [
    TagType::Sc28,
    TagType::Sc28,
    TagType::R1,
    TagType::Sn4,
    TagType::Sk3,
    TagType::S7,
];
const MSG0_4: [TagType; 6] = [
    TagType::Sc28,
    TagType::Sc28,
    TagType::R1,
    TagType::Sn4,
    TagType::Sk3,
    TagType::S7,
];
const MSG0_5: [TagType; 1] = [TagType::St71];
const MSG1: [TagType; 6] = [
    TagType::Sc28,
    TagType::Sr1,
    TagType::Sc28,
    TagType::Sr1,
    TagType::R1,
    TagType::Sg15,
];
const MSG2: [TagType; 6] = [
    TagType::Sc28,
    TagType::Sp1,
    TagType::Sc28,
    TagType::Sp1,
    TagType::R1,
    TagType::Sg15,
];
const MSG3: [TagType; 6] = [
    TagType::St1,
    TagType::Sc28,
    TagType::Sc28,
    TagType::R1,
    TagType::Sr3,
    TagType::Ss13,
];
const MSG4: [TagType; 5] = [
    TagType::Sh12,
    TagType::Sc58,
    TagType::Sh1,
    TagType::Sr2,
    TagType::Sc1,
];
const MSG5: [TagType; 6] = [
    TagType::Sh12,
    TagType::Sh22,
    TagType::R1,
    TagType::Sr3,
    TagType::Ss11,
    TagType::Sg25,
];

impl MsgType {
    fn new(i3: u8, n3: u8) -> Option<Self> {
        match (i3, n3) {
            (0u8, 0u8) => Some(MsgType::Msg0_0),
            (0u8, 1u8) => Some(MsgType::Msg0_1),
            (0u8, 3u8) => Some(MsgType::Msg0_3),
            (0u8, 4u8) => Some(MsgType::Msg0_4),
            (0u8, 5u8) => Some(MsgType::Msg0_5),
            (1u8, _) => Some(MsgType::Msg1),
            (2u8, _) => Some(MsgType::Msg2),
            (3u8, _) => Some(MsgType::Msg3),
            (4u8, _) => Some(MsgType::Msg4),
            (5u8, _) => Some(MsgType::Msg5),
            _ => None,
        }
    }

    fn get_title(&self) -> BitVec<u8, Msb0> {
        let (bit3, have_n3): (u8, bool) = match self {
            MsgType::Msg0_0 => (0, true),
            MsgType::Msg0_1 => (1, true),
            MsgType::Msg0_3 => (3, true),
            MsgType::Msg0_4 => (4, true),
            MsgType::Msg0_5 => (5, true),
            MsgType::Msg1 => (1, false),
            MsgType::Msg2 => (2, false),
            MsgType::Msg3 => (3, false),
            MsgType::Msg4 => (4, false),
            MsgType::Msg5 => (5, false),
        };
        let mut bits: BitVec<u8, Msb0> = BitVec::with_capacity(77);
        if have_n3 {
            bits_extend::<u8>(&mut bits, 0x00, 3);
        }
        bits_extend::<u8>(&mut bits, bit3, 3);
        bits
    }

    fn get_order(&self) -> &[TagType] {
        match self {
            MsgType::Msg0_0 => &MSG0_0,
            MsgType::Msg0_1 => &MSG0_1,
            MsgType::Msg0_3 => &MSG0_3,
            MsgType::Msg0_4 => &MSG0_4,
            MsgType::Msg0_5 => &MSG0_5,
            MsgType::Msg1 => &MSG1,
            MsgType::Msg2 => &MSG2,
            MsgType::Msg3 => &MSG3,
            MsgType::Msg4 => &MSG4,
            MsgType::Msg5 => &MSG5,
        }
    }
}
