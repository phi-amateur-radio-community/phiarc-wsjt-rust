// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/lib.rs
// Root file of library.

pub mod api;
pub(crate) mod core;
pub(crate) mod util;

#[cfg(all(feature = "ftx", not(any(feature = "ft8"))))]
compile_error!("FTx feature cannot be selected separately");

#[cfg(test)]
mod tests {
    use super::*;
}
