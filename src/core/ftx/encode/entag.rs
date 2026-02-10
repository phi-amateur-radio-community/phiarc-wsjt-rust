// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/core/ftx/encode/entag.rs
// Encode the tag.

use super::super::tag::*;
use super::spec::*;
use crate::util::ftx::bits::bits_extend;
use bitvec::prelude::{BitVec, Msb0};
use std::ops::{BitAnd, Shr};

pub fn tag_encode(tag: &Tag) -> Option<BitVec<u8, Msb0>> {
    let mut bits: BitVec<u8, Msb0> = BitVec::with_capacity(tag.get_len());
    match tag {
        Tag::Sc1(data) => bits.push(*data),
        Tag::Sc28(data) => encode_c28(data, &mut bits)?,
        Tag::Sc58(data) => encode_c58(data, &mut bits)?,
        Tag::Sf71(data) => encode_f71(data, &mut bits)?,
        Tag::Sg15(data) => encode_g15(data, &mut bits)?,
        Tag::Sg25(data) => encode_g25(data, &mut bits)?,
        Tag::Sh1(data) => bits.push(*data),
        Tag::Sh10(data) => encode_number::<u16>(*data, &mut bits, 10u8),
        Tag::Sh12(data) => encode_number::<u16>(*data, &mut bits, 12u8),
        Tag::Sh22(data) => encode_number::<u32>(*data, &mut bits, 22u8),
        Tag::Sk3(data) => encode_number::<u8>(*data, &mut bits, 3u8),
        Tag::Sn4(data) => encode_number::<u8>(*data, &mut bits, 3u8),
        Tag::Sp1(data) => bits.push(*data),
        Tag::Sr1(data) => bits.push(*data),
        Tag::Sr2(data) => encode_r2(data, &mut bits)?,
        Tag::Sr3(data) => encode_number::<u8>(*data, &mut bits, 3u8),
        Tag::R1(data) => bits.push(*data),
        Tag::Sr5(data) => encode_number::<u8>((*data + 30) as u8 / 2, &mut bits, 5u8),
        Tag::Ss11(data) => encode_number::<u16>(*data, &mut bits, 11u8),
        Tag::Ss13(data) => encode_number::<u16>(*data, &mut bits, 13u8),
        Tag::S7(data) => encode_number::<u8>(*data, &mut bits, 7u8),
        Tag::St1(data) => bits.push(*data),
        Tag::St71(data) => encode_number::<u128>(*data, &mut bits, 71u8),
    }
    Some(bits)
}

const C28_CQ_EMPTY: [u32; 5] = [3, 1003, 1030, 1759, 21442];
const C28_HASH: u32 = 2063592;
const C28_CALL: u32 = 6257896;

fn encode_c28(data: &Sc28Type, bits: &mut BitVec<u8, Msb0>) -> Option<()> {
    let n: u32 = match data {
        Sc28Type::De => 0,
        Sc28Type::Qrz => 1,
        Sc28Type::Cq(content) => match content {
            Sc28CqType::Empty => 2,
            Sc28CqType::Number(num) => {
                let number = *num;
                if number > 999 {
                    return None;
                }
                (number + 3) as u32
            }
            Sc28CqType::Letter(s, l) => {
                let len = *l;
                if len > 4 {
                    return None;
                }
                let c = str_encode(&s, len, EncodeType::CqContent4)? as u32;
                c + C28_CQ_EMPTY[len as usize]
            }
        },
        Sc28Type::Hash(content) => content + C28_HASH,
        Sc28Type::Callsign(content) => {
            C28_CALL + str_encode(&content, 6, EncodeType::Callsign6)? as u32
        }
    };
    bits_extend::<u32>(bits, n, 28u8);
    Some(())
}

fn encode_c58(data: &str, bits: &mut BitVec<u8, Msb0>) -> Option<()> {
    let n: u64 = str_encode(data, 11, EncodeType::All13)?;
    bits_extend::<u64>(bits, n, 58u8);
    Some(())
}

fn encode_f71(data: &str, bits: &mut BitVec<u8, Msb0>) -> Option<()> {
    let n: u128 = str_long_encode(data, 13, EncodeType::Free13)?;
    bits_extend::<u128>(bits, n, 71u8);
    Some(())
}

const G15_MAX: u64 = 32400;

fn encode_g15(data: &Sg15Type, bits: &mut BitVec<u8, Msb0>) -> Option<()> {
    let n: u16 = match data {
        Sg15Type::Blank => G15_MAX + 1,
        Sg15Type::Rrr => G15_MAX + 2,
        Sg15Type::Rr73 => G15_MAX + 3,
        Sg15Type::N73 => G15_MAX + 4,
        Sg15Type::Grid4(g) => str_encode(&g, 4, EncodeType::Grid6)?,
    } as u16;
    bits_extend::<u16>(bits, n, 15u8);
    Some(())
}

fn encode_g25(data: &str, bits: &mut BitVec<u8, Msb0>) -> Option<()> {
    let n: u32 = str_encode(data, 6, EncodeType::Grid6)? as u32;
    bits_extend::<u32>(bits, n, 15u8);
    Some(())
}

fn encode_r2(data: &Sr2Type, bits: &mut BitVec<u8, Msb0>) -> Option<()> {
    let n: u8 = match data {
        Sr2Type::Blank => 0,
        Sr2Type::Rrr => 1,
        Sr2Type::Rr73 => 2,
        Sr2Type::N73 => 3,
    };
    bits_extend::<u8>(bits, n, 2u8);
    Some(())
}

fn encode_number<T>(data: T, bits: &mut BitVec<u8, Msb0>, len: u8)
where
    T: BitAnd<Output = T> + PartialEq + From<u8> + Copy + Shr<Output = T>,
{
    bits_extend::<T>(bits, data, len);
}
