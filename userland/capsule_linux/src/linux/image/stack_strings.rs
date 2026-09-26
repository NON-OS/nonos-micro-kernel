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

//! The string block at the very top of a new stack.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

/// Where each string ended up, in the order it was given, and the lowest
/// address the block reached.
pub struct Placed {
    pub at: Vec<u64>,
    pub floor: u64,
}

pub fn place(guest: &Guest, top: u64, strings: &[Vec<u8>]) -> Option<Placed> {
    let mut at = Vec::with_capacity(strings.len());
    let mut cursor = top;
    // Downward, so the first string ends up highest.
    for s in strings.iter().rev() {
        let len = s.len() as u64 + 1;
        cursor = cursor.checked_sub(len)?;
        let mut bytes = s.clone();
        bytes.push(0);
        if guest.write(cursor, &bytes) < 0 {
            return None;
        }
        at.push(cursor);
    }
    at.reverse();
    Some(Placed { at, floor: cursor })
}
