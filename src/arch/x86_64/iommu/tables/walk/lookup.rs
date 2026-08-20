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
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::tables::sl_pte::{entry_address, index_for, is_present};
use crate::arch::x86_64::iommu::types::VtdError;

/// `None` when no path reaches this address, the normal answer for an address
/// that was never mapped.
pub fn walk_lookup(root: u64, iova: u64, levels: u8) -> Result<Option<LeafSlot>, VtdError> {
    let mut current = root;
    let mut level = levels;
    while level > 1 {
        let entry = entries_mut(current)?[index_for(iova, level)];
        if !is_present(entry) {
            return Ok(None);
        }
        current = entry_address(entry);
        level -= 1;
    }
    Ok(Some(LeafSlot { table_phys: current, index: index_for(iova, 1) }))
}
