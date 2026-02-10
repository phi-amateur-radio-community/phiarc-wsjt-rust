// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/core/ftx/encode/base.rs
// Basic coding method.

pub fn char_encode(s: &str, len: u8, coders: &[CharEncodeType]) -> Option<u64> {
    let mut count: u64 = 0;
    for (c, t) in s.chars().zip(coders.iter()).take(len as usize) {
        count += t.encode(c)? as u64;
        count *= t.get_len() as u64;
    }
    return Some(count);
}

pub fn char_long_encode(s: &str, len: u8, coders: &[CharEncodeType]) -> Option<u128> {
    let mut count: u128 = 0;
    for (c, t) in s.chars().zip(coders.iter()).take(len as usize) {
        count += t.encode(c)? as u128;
        count *= t.get_len() as u128;
    }
    return Some(count);
}

#[derive(Copy, Clone)]
pub enum CharEncodeType {
    Char42,
    Char38,
    Char37,
    Char36,
    Char27,
    Char26,
    Char10,
    Char0,
}

impl CharEncodeType {
    fn encode(&self, c: char) -> Option<u8> {
        match self {
            CharEncodeType::Char42 => char42(c),
            CharEncodeType::Char38 => char38(c),
            CharEncodeType::Char37 => char37(c),
            CharEncodeType::Char36 => char36(c),
            CharEncodeType::Char27 => char27(c),
            CharEncodeType::Char26 => char26(c),
            CharEncodeType::Char10 => char10(c),
            CharEncodeType::Char0 => None,
        }
    }

    fn get_len(&self) -> u8 {
        match self {
            CharEncodeType::Char42 => 42,
            CharEncodeType::Char38 => 38,
            CharEncodeType::Char37 => 37,
            CharEncodeType::Char36 => 36,
            CharEncodeType::Char27 => 27,
            CharEncodeType::Char26 => 26,
            CharEncodeType::Char10 => 10,
            CharEncodeType::Char0 => 0,
        }
    }
}

fn char42(c: char) -> Option<u8> {
    match c {
        ' ' => Some(0),
        '0'..='9' => Some(c as u8 - b'0' + 1),
        'A'..='Z' => Some(c as u8 - b'A' + 11),
        '+' => Some(37),
        '-' => Some(38),
        '.' => Some(39),
        '/' => Some(40),
        '?' => Some(41),
        _ => None,
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

fn char26(c: char) -> Option<u8> {
    match c {
        'A'..='Z' => Some(c as u8 - b'A'),
        _ => None,
    }
}
fn char10(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        _ => None,
    }
}
