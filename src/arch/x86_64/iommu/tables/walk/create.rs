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

use super::slot::LeafSlot;
use crate::arch::x86_64::iommu::tables::frame::{allocate_table, entries_mut};
use crate::arch::x86_64::iommu::tables::sl_pte::{entry_address, index_for, is_present, table};
use crate::arch::x86_64::iommu::types::VtdError;

pub fn walk_create(root: u64, iova: u64, levels: u8) -> Result<LeafSlot, VtdError> {
    walk_create_to(root, iova, levels, 1)
}

/// Stopping above level 1 yields the slot a large-page entry goes in, which is
/// what makes the entry cover that level's whole span.
///
/// A new table is zeroed before the entry pointing at it is written, so a
/// device walking concurrently sees either no path or a complete one, never a
/// table of whatever the frame held in its previous life.
pub fn walk_create_to(
    root: u64,
    iova: u64,
    levels: u8,
    stop_level: u8,
) -> Result<LeafSlot, VtdError> {
    if stop_level == 0 || stop_level > levels {
        return Err(VtdError::RangeOutOfBounds);
    }
    let mut current = root;
    let mut level = levels;
    while level > stop_level {
        let index = index_for(iova, level);
        let entry = entries_mut(current)?[index];
        current = if is_present(entry) {
            entry_address(entry)
        } else {
            let next = allocate_table()?;
            entries_mut(current)?[index] = table(next);
            next
        };
        level -= 1;
    }
    Ok(LeafSlot { table_phys: current, index: index_for(iova, stop_level) })
}
