// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Decode a count of names, then that many length-prefixed names. The count is
//! a hint, never a licence to read: every step is bounded by the bytes the
//! server actually sent, so an oversized count or a truncated tail yields the
//! names parsed so far instead of an over-read.

use alloc::vec::Vec;

use super::constants::{HDR_LEN, MAX_ENTRIES};

pub(super) fn names(rx: &[u8], total: usize) -> Vec<Vec<u8>> {
    let end = total.min(rx.len());
    let mut out = Vec::new();
    if end < HDR_LEN + 4 {
        return out;
    }
    let count =
        u32::from_le_bytes([rx[HDR_LEN], rx[HDR_LEN + 1], rx[HDR_LEN + 2], rx[HDR_LEN + 3]]);
    let mut at = HDR_LEN + 4;
    for _ in 0..(count as usize).min(MAX_ENTRIES) {
        if at >= end {
            break;
        }
        let len = rx[at] as usize;
        at += 1;
        if len == 0 || len > end - at {
            break;
        }
        out.push(rx[at..at + len].to_vec());
        at += len;
    }
    out
}
