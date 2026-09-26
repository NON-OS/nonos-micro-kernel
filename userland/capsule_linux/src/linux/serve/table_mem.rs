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

//! Calls that shape the guest's address space.

use crate::linux::abi::{errno, nr};
use crate::linux::call;
use crate::linux::guest::Guest;

pub fn mem_ops(guest: &mut Guest, nr: u64, a: [u64; 6]) -> Option<u64> {
    Some(match nr {
        nr::BRK => call::brk(guest, a[0]),
        nr::MMAP => call::mmap(guest, call::MapReq::from_args(a)),
        nr::MUNMAP => call::munmap(guest, a[0], a[1]),
        nr::MPROTECT => call::mprotect(guest, a[0], a[1], a[2]),
        // Advice, and this capsule takes none of it.
        nr::MADVISE => errno::ok(0),
        _ => return None,
    })
}
