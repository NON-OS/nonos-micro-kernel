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

use super::super::flags;
use super::bits::*;
use crate::arch::aarch64::mmu::MemoryType;

/// A leaf entry mapping `pa` with `flags`.
pub fn leaf(pa: u64, flags: u64) -> u64 {
    let device = flags & flags::NO_CACHE != 0;
    let kind = if device { MemoryType::DeviceNGnRnE } else { MemoryType::NormalWB };
    let user = flags & flags::USER != 0;

    let mut entry = (pa & ADDR_MASK) | AF | (kind.attr_index() << 2);
    // Only the caller decides whether this is a live mapping. Setting VALID
    // regardless would make an absent leaf, the kind that carries swap
    // metadata, fault on x86_64 and map on aarch64 from the same flag word.
    if flags & flags::PRESENT != 0 {
        entry |= VALID;
    }
    if flags & flags::HUGE == 0 {
        entry |= TABLE_OR_PAGE;
    }
    if !device {
        entry |= SH_INNER;
    }
    if user {
        entry |= AP_EL0;
    }
    if flags & flags::WRITABLE == 0 {
        entry |= AP_READ_ONLY;
    }
    entry |= execute_never(user, flags & flags::NO_EXECUTE == 0);
    if flags & flags::GLOBAL == 0 {
        entry |= NOT_GLOBAL;
    }
    entry
}

/// Execution never crosses the privilege boundary whatever was asked for: EL1
/// must not run a user page and EL0 must not run a kernel page. Within a
/// page's own level, `executable` decides.
const fn execute_never(user: bool, executable: bool) -> u64 {
    match (user, executable) {
        (true, true) => PXN,
        (true, false) => PXN | UXN,
        (false, true) => UXN,
        (false, false) => PXN | UXN,
    }
}

/// An entry pointing at the next level of table.
///
/// The hierarchical attribute bits at 63:59 stay clear, so this level
/// restricts nothing and the leaf decides. `user_accessible` is unused for
/// that reason and taken only so both backends share a signature.
#[inline]
pub const fn table(pa: u64, _user_accessible: bool) -> u64 {
    (pa & ADDR_MASK) | VALID | TABLE_OR_PAGE
}
