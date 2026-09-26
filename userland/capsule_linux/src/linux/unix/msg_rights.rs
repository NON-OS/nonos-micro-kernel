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


//! The descriptors in a control block.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::msg::u64le;

/// struct cmsghdr: len, level, type, then the data.
const CMSG_DATA: usize = 16;
const SOL_SOCKET: u32 = 1;
const SCM_RIGHTS: u32 = 1;

/// Every descriptor in an SCM_RIGHTS block. Anything else in the control
/// data is skipped rather than guessed at.
pub(super) fn rights(guest: &Guest, at: u64, len: u64) -> Vec<u32> {
    let mut out = Vec::new();
    let Some(raw) = guest.read(at, len.min(1024) as usize) else {
        return out;
    };
    let mut off = 0usize;
    while off + CMSG_DATA <= raw.len() {
        let size = u64le(&raw, off) as usize;
        let level = u32le(&raw, off + 8);
        let kind = u32le(&raw, off + 12);
        if size < CMSG_DATA || off + size > raw.len() {
            break;
        }
        if level == SOL_SOCKET && kind == SCM_RIGHTS {
            let mut at = off + CMSG_DATA;
            while at + 4 <= off + size {
                out.push(u32le(&raw, at));
                at += 4;
            }
        }
        off += (size + 7) & !7;
    }
    out
}

fn u32le(b: &[u8], at: usize) -> u32 {
    let mut w = [0u8; 4];
    w.copy_from_slice(&b[at..at + 4]);
    u32::from_le_bytes(w)
}
