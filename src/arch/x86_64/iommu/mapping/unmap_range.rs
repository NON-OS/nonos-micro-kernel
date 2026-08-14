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

use super::validate_range::validate_range;
use crate::arch::x86_64::iommu::globals::state::STATE;
use crate::arch::x86_64::iommu::globals::{is_enforcing, page_levels};
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::tables::sl_pte::is_present;
use crate::arch::x86_64::iommu::tables::walk::walk_lookup;
use crate::arch::x86_64::iommu::types::{DomainId, VtdError, MAX_VTD_DOMAINS, PAGE_SIZE_4K};

/// Entries are cleared, not marked: zero is the state a fresh table has.
///
/// The tables the range hung from stay allocated. Freeing them would race a
/// device still walking toward a sibling page.
pub fn unmap_range(domain: DomainId, iova: u64, size: usize) -> Result<(), VtdError> {
    if !is_enforcing() {
        return Err(VtdError::NotEnforcing);
    }
    let pages = validate_range(iova, size)?;
    let levels = page_levels().ok_or(VtdError::DepthUnknown)?;
    let index = domain.as_u16() as usize;
    if index >= MAX_VTD_DOMAINS {
        return Err(VtdError::DomainNotFound);
    }

    let state = STATE.lock();
    if !state.domains[index].used {
        return Err(VtdError::DomainNotFound);
    }
    let root = state.domains[index].root;

    // A caller naming a range it does not hold does not lose the part it does.
    for page in 0..pages {
        let addr = iova + (page * PAGE_SIZE_4K) as u64;
        match walk_lookup(root, addr, levels)? {
            Some(slot) if is_present(entries_mut(slot.table_phys)?[slot.index]) => {}
            _ => return Err(VtdError::RangeNotMapped),
        }
    }
    for page in 0..pages {
        let addr = iova + (page * PAGE_SIZE_4K) as u64;
        if let Some(slot) = walk_lookup(root, addr, levels)? {
            entries_mut(slot.table_phys)?[slot.index] = 0;
        }
    }
    Ok(())
}
