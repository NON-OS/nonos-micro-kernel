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

use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::tables::sl_pte::{leaf, level_span, SL_LARGE};
use crate::arch::x86_64::iommu::tables::walk::walk_create_to;
use crate::arch::x86_64::iommu::types::VtdError;

/// Identity map `[0, limit)`, leaves at `leaf_level`: 1 for 4 KiB, 2 for
/// 2 MiB, 3 for 1 GiB. `limit` rounds up to the leaf size, since a partial
/// leaf cannot be expressed and under-mapping breaks a device mid transfer.
///
/// This is what a device gets when the kernel knows about it but has not yet
/// been taught which memory it legitimately touches. It is not protection for
/// that device; it is what lets translation be turned on at all, so every
/// device the kernel did not enumerate is denied.
pub fn map_identity(root: u64, levels: u8, limit: u64, leaf_level: u8) -> Result<u64, VtdError> {
    if leaf_level == 0 || leaf_level > levels {
        return Err(VtdError::RangeOutOfBounds);
    }
    let span = level_span(leaf_level);
    let reach = 1u64 << (12 + 9 * levels as u32);
    let rounded = limit.checked_add(span - 1).ok_or(VtdError::RangeOutOfBounds)? & !(span - 1);
    let end = if rounded > reach { reach } else { rounded };

    // Above the last level an entry without this bit is read as a pointer to
    // another table, and the unit walks into nothing.
    let large = if leaf_level > 1 { SL_LARGE } else { 0 };

    let mut addr = 0u64;
    while addr < end {
        let slot = walk_create_to(root, addr, levels, leaf_level)?;
        entries_mut(slot.table_phys)?[slot.index] = leaf(addr, true, true, true) | large;
        addr += span;
    }
    Ok(end)
}
