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

//! Reading a guest's argv or envp.

use alloc::vec::Vec;

use crate::linux::file::read_cstr;
use crate::linux::guest::{Guest, STACK_SIZE};

/// Enough for any real command line. A caller handing over more than
/// this is not going to be helped by us trying.
const MAX_ENTRIES: usize = 4096;

/// Linux's own ceiling on one argument, thirty-two pages.
const MAX_ARG: usize = 32 * 4096;

/// The whole vector, a quarter of the stack the guest wakes on.
const MAX_TOTAL: usize = STACK_SIZE as usize / 4;

pub fn vector(guest: &Guest, mut array: u64) -> Option<Vec<Vec<u8>>> {
    let mut out = Vec::new();
    if array == 0 {
        return Some(out);
    }
    let mut total = 0usize;
    while out.len() < MAX_ENTRIES {
        let slot = guest.read(array, 8)?;
        let ptr = u64::from_le_bytes(slot.as_slice().try_into().ok()?);
        if ptr == 0 {
            return Some(out);
        }
        let arg = read_cstr(guest, ptr, MAX_ARG)?;
        /*
         * Each string costs its bytes and the terminator the stack
         * block will need for it.
         */
        total = total.checked_add(arg.len() + 1)?;
        if total > MAX_TOTAL {
            return None;
        }
        out.push(arg);
        array = array.checked_add(8)?;
    }
    Some(out)
}
