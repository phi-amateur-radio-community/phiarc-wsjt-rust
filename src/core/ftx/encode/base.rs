// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/core/ftx/encode/base.rs
// Basic coding method.

pub fn char_encode(s: &str, len: usize, coders: &[CodeType]) -> Option<u64> {
    let mut count: u64 = 0;
    for (c, t) in s.chars().zip(coders.iter()).take(len) {
        count += t.encode(c)? as u64;
        count *= t.get_len() as u64;
    }
    return Some(count);
}

#[derive(Copy, Clone)]
pub enum CodeType {
    Char38,
    Char37,
    Char36,
    Char27,
    Char10,
    Char0,
}

impl CodeType {
    pub fn encode(&self, c: char) -> Option<u8> {
        match self {
            CodeType::Char38 => char38(c),
            CodeType::Char37 => char37(c),
            CodeType::Char36 => char36(c),
            CodeType::Char27 => char27(c),
            CodeType::Char10 => char10(c),
            CodeType::Char0 => None,
        }
    }

    pub fn get_len(&self) -> u8 {
        match self {
            CodeType::Char38 => 38,
            CodeType::Char37 => 37,
            CodeType::Char36 => 36,
            CodeType::Char27 => 27,
            CodeType::Char10 => 10,
            CodeType::Char0 => 0,
        }
    }
}

fn char38(c: char) -> Option<u8> {
    match c {
        ' ' => Some(0),
        '0'..='9' => Some(c as u8 - b'0' + 1),
        'A'..='Z' => Some(c as u8 - b'A' + 11),
        '/' => Some(37),
        _ => None,
    }
}

fn char37(c: char) -> Option<u8> {
    match c {
        ' ' => Some(0),
        '0'..='9' => Some(c as u8 - b'0' + 1),
        'A'..='Z' => Some(c as u8 - b'A' + 11),
        _ => None,
    }
}

fn char36(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'A'..='Z' => Some(c as u8 - b'A' + 10),
        _ => None,
    }
}

fn char27(c: char) -> Option<u8> {
    match c {
        ' ' => Some(0),
        'A'..='Z' => Some(c as u8 - b'A' + 1),
        _ => None,
    }
}

fn char10(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        _ => None,
    }
}
