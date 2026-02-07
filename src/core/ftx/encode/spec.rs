// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/core/ftx/encode/spec.rs
// Specific coding method.

use super::base;

const ALL38_TYPES: [base::CodeType; 13] = [base::CodeType::Char38; 13];

pub fn encode(s: &str, len: usize, code_type: &[base::CodeType]) -> Option<u64> {
    base::char_encode(s, len, code_type)
}

pub enum EncodeType {
    All38,
}

impl EncodeType {
    fn get_array(&self) -> &[base::CodeType] {
        match self {
            EncodeType::All38 => &ALL38_TYPES,
        }
    }
}
