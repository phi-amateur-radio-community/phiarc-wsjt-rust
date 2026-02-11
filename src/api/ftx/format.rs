// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/api/ftx/format.rs
// Format display.

use std::fmt;
use crate::core::ftx::tag::*;
use super::msg::*;
use bitvec::prelude::{BitVec, Msb0};

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = self.msg_content.iter().map("x" | x.to_string()).collect::<Vec<_>>().join(" ");
        write!(f, "FTx Message\r\nType: {}\r\n{}", self.msg_type, content)?;
        Ok(())
    }
}

impl fmt::Display for MsgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            MsgType::Msg0_0 => "Free text",
            MsgType::Msg0_1 => "DXpedition mode",
            MsgType::Msg0_3 => "ARRL Field Day 1-16",
            MsgType::Msg0_4 => "ARRL Field Day 17-32",
            MsgType::Msg0_5 => "Telemetry",
            MsgType::Msg1 => "Standard message",
            MsgType::Msg2 => "EU VHF contest",
            MsgType::Msg3 => "ARRL RTTY Roundup",
            MsgType::Msg4 => "Nonstandard calls",
            MsgType::Msg5 => "EU VHF Contest",
        };
        write!(f, "{}", message)?;
        Ok(())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Tag::Sc1(c) => if c {"CQ"} else {""},
            Tag::Sc28(c) => {
                match c {
                    Sc28Type::Callsign(call) => call,
                    Sc28Type::Hash(h) => format!("<{}>", h),
                    Sc28Type::Cq(content) => {
                        let msg = match content {
                            Sc28CqType::Empty => "",
                            Sc28CqType::Number(n) => format!(" {}", n),
                            Sc28CqType::Letter(s, _) => format!(" {}", s),
                        };
                        format!("CQ{}", msg)
                    }
                    Sc28Type::De => "DE",
                    Sc28Type::Qrz => "QRZ",
                }
            }
            Tag::Sc58(c) => c,
            Tag::Sf71(c) => c,
            Tag::Sg15(c) => {
                match c {
                    Sg15Type::Blank => "NULL",
                    Sg15Type::Rrr => "RRR",
                    Sg15Type::Rr73 => "RR73",
                    Sg15Type::N73 => "73",
                    Sg15Type::Grid4(s) => s,
                }
            }
            Tag::Sg25(c) => c,
            Tag::Sh1(c) => if c {"<1>"} else {"<0>"},
            Tag::Sh10(c) => format!("<{}>", c),
            Tag::Sh12(c) => format!("<{}>", c),
            Tag::Sh22(c) => format!("<{}>", c),
            Tag::Sk3(c) => ((b"A" + c) as char).to_string(),
            Tag::Sn4(c) => format!("MaxOff{}", c),
            Tag::Sp1(c) => if c {"/P"} else {""},
            Tag::Sr1(c) => if c {"/R"} else {""},
            Tag::Sr2(c) => {
                match c {
                    Sr2Type::Blank => "NULL",
                    Sr2Type::Rrr => "RRR",
                    Sr2Type::Rr73 => "RR73",
                    Sr2Type::N73 => "73",
                }
            }
            Tag::Sr3(c) => format!("5{}(9)", c),
            Tag::R1(c) => if c {"R"} else {""},
            Tag::Sr5(c) => format!("{}dB", c),
            Tag::Ss11(c) => format!("{}", c), //TODO Serial decode
            Tag::Ss13(c) => format!("{}", c), //TODO Serial decode
            Tag::S7(c) => format!("{}", c), //TODO Serial decode
            Tag::St1(c) => if c {"TU"} else {""},
            Tag::St71(c) => format!("{}", c), //TODO Telemetry data
        };
        write!(f, "{}", message)?;
        Ok(())
    }
}
