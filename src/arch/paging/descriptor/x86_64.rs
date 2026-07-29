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

//! x86_64 page-table entries.
//!
//! The neutral flag vocabulary was chosen to be these bit positions, so the
//! translation here is the identity and the compiler folds it away.

use super::flags;

/// Output address, bits 51:12.
pub const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;
/// Everything that is not the output address: the low attribute bits and the
/// high ones including NX.
const FLAGS_MASK: u64 = 0xFFF0_0000_0000_0FFF;

/// A leaf entry mapping `pa` with `flags`.
#[inline]
pub const fn leaf(pa: u64, flags: u64) -> u64 {
    (pa & ADDR_MASK) | (flags & FLAGS_MASK)
}

/// An entry pointing at the next level of table.
///
/// Permission is not restricted here. x86_64 intersects the permissions of
/// every level on the way down, so a table entry that denied user access or
/// writes would override the leaf; the leaf is where the decision belongs.
#[inline]
pub const fn table(pa: u64, user_accessible: bool) -> u64 {
    let mut entry = (pa & ADDR_MASK) | flags::PRESENT | flags::WRITABLE;
    if user_accessible {
        entry |= flags::USER;
    }
    entry
}

#[inline]
pub const fn is_present(entry: u64) -> bool {
    entry & flags::PRESENT != 0
}

/// True when this entry maps a block rather than pointing at another table.
/// Only meaningful at the levels where a block is legal.
///
/// Presence is part of the question. An absent entry maps nothing, so it is
/// not a block however its other bits read, and hardware ignores them. Saying
/// so here keeps the answer the same as aarch64's, where the bit that marks a
/// block is only meaningful in a valid descriptor.
#[inline]
pub const fn is_block(entry: u64) -> bool {
    is_present(entry) && entry & flags::HUGE != 0
}

#[inline]
pub const fn address(entry: u64) -> u64 {
    entry & ADDR_MASK
}

#[inline]
pub const fn is_writable(entry: u64) -> bool {
    entry & flags::WRITABLE != 0
}

/// True when EL0 / ring 3 may reach a leaf mapped by this entry.
#[inline]
pub const fn is_user(entry: u64) -> bool {
    entry & flags::USER != 0
}

/// True when this table entry lets user access through to the levels below.
///
/// x86_64 intersects the permission bits of every level on the way down, so a
/// table entry without the user bit denies EL0 no matter what the leaf says.
#[inline]
pub const fn table_grants_user(entry: u64) -> bool {
    entry & flags::USER != 0
}
