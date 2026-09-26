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

//! A NUL-terminated string out of a guest, a page at a time.

use alloc::vec::Vec;

use crate::linux::guest::{page_down, Guest, PAGE};

/// Bytes up to the terminator, or nothing when the string is not
/// terminated inside `max` or reaches memory the guest does not hold.
pub fn read_cstr(guest: &Guest, addr: u64, max: usize) -> Option<Vec<u8>> {
    if addr == 0 {
        return None;
    }
    let mut out: Vec<u8> = Vec::new();
    let mut at = addr;
    while out.len() <= max {
        let page_end = page_down(at).checked_add(PAGE)?;
        let room = (max + 1 - out.len()) as u64;
        let take = (page_end.saturating_sub(at)).min(room);
        let chunk = guest.read(at, take as usize)?;
        if let Some(i) = chunk.iter().position(|b| *b == 0) {
            out.extend_from_slice(&chunk[..i]);
            return Some(out);
        }
        out.extend_from_slice(&chunk);
        at = page_end;
    }
    None
}
