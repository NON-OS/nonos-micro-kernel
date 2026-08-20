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
use crate::arch::x86_64::iommu::tables::sl_pte::{is_present, leaf};
use crate::arch::x86_64::iommu::tables::walk::walk_create;
use crate::arch::x86_64::iommu::types::{
    DomainId, IommuPageFlags, VtdError, MAX_VTD_DOMAINS, PAGE_SIZE_4K,
};

pub fn map_range(
    domain: DomainId,
    iova: u64,
    phys: u64,
    size: usize,
    flags: IommuPageFlags,
) -> Result<(), VtdError> {
    // Not `is_present`: entries written into tables no unit walks would report
    // an isolation the device does not have.
    if !is_enforcing() {
        return Err(VtdError::NotEnforcing);
    }
    let pages = validate_range(iova, size)?;
    if phys & (PAGE_SIZE_4K as u64 - 1) != 0 {
        return Err(VtdError::AddressMisaligned);
    }
    if !flags.read && !flags.write {
        return Err(VtdError::NoPermissionsRequested);
    }
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
    if iova + size as u64 > 1u64 << (12 + 9 * levels as u32) {
        return Err(VtdError::RangeOutOfBounds);
    }

    // Checked whole first, so a rejected request leaves no partial mapping.
    for page in 0..pages {
        let slot = walk_create(root, iova + (page * PAGE_SIZE_4K) as u64, levels)?;
        if is_present(entries_mut(slot.table_phys)?[slot.index]) {
            return Err(VtdError::RangeAlreadyMapped);
        }
    }
    for page in 0..pages {
        let offset = (page * PAGE_SIZE_4K) as u64;
        let slot = walk_create(root, iova + offset, levels)?;
        entries_mut(slot.table_phys)?[slot.index] =
            leaf(phys + offset, flags.read, flags.write, flags.snoop);
    }
    Ok(())
}
