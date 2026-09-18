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


//! Taking a path out of a guest.
//!
//! The string is NUL terminated and its length is not known before it is
//! read, so it is read one page at a time: a copy that crossed into an
//! unmapped page would fail as a whole and lose the part that was there,
//! and a program keeps its arguments wherever it likes.

use alloc::vec::Vec;

use crate::linux::guest::{page_down, Guest, PAGE};

/// The server takes a single length byte, so a longer path could not be
/// asked for without truncating into a different path than the guest meant.
pub const MAX_PATH: usize = 255;

pub fn read_path(guest: &Guest, addr: u64) -> Option<Vec<u8>> {
    if addr == 0 {
        return None;
    }
    let mut out: Vec<u8> = Vec::new();
    let mut at = addr;
    while out.len() <= MAX_PATH {
        let page_end = page_down(at) + PAGE;
        let room = (MAX_PATH + 1 - out.len()) as u64;
        let take = (page_end - at).min(room);
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
