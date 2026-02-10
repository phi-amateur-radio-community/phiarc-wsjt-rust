// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/core/ftx/encode/spec.rs
// Specific coding method.

use super::enchar::{CharEncodeType, char_encode, char_long_encode};

const ALL13_TYPES: [CharEncodeType; 13] = [CharEncodeType::Char38; 13];
const CQ_CONTENT4: [CharEncodeType; 4] = [CharEncodeType::Char27; 4];
const CALLSIGN6_TYPES: [CharEncodeType; 6] = [
    CharEncodeType::Char37,
    CharEncodeType::Char36,
    CharEncodeType::Char10,
    CharEncodeType::Char27,
    CharEncodeType::Char27,
    CharEncodeType::Char27,
];
const FREE13_TYPES: [CharEncodeType; 13] = [CharEncodeType::Char42; 13];
const GRID6_TYPES: [CharEncodeType; 6] = [
    CharEncodeType::Char26,
    CharEncodeType::Char26,
    CharEncodeType::Char10,
    CharEncodeType::Char10,
    CharEncodeType::Char26,
    CharEncodeType::Char26,
];

pub fn str_encode(s: &str, len: u8, code_type: EncodeType) -> Option<u64> {
    char_encode(s, len, code_type.get_array())
}

pub fn str_long_encode(s: &str, len: u8, code_type: EncodeType) -> Option<u128> {
    char_long_encode(s, len, code_type.get_array())
}

pub enum EncodeType {
    All13,
    CqContent4,
    Callsign6,
    Free13,
    Grid6,
}

impl EncodeType {
    fn get_array(&self) -> &[CharEncodeType] {
        match self {
            EncodeType::All13 => &ALL13_TYPES,
            EncodeType::CqContent4 => &CQ_CONTENT4,
            EncodeType::Callsign6 => &CALLSIGN6_TYPES,
            EncodeType::Free13 => &FREE13_TYPES,
            EncodeType::Grid6 => &GRID6_TYPES,
        }
    }
}
