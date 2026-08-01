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

use super::super::globals::is_present;
use super::super::globals::state::STATE;
use super::super::types::{DomainId, IommuPageFlags, VtdError, MAX_VTD_DOMAINS, PAGE_SIZE_4K};
use super::validate_range::validate_range;

pub fn map_range(
    domain: DomainId,
    iova: u64,
    phys: u64,
    size: usize,
    flags: IommuPageFlags,
) -> Result<(), VtdError> {
    if !is_present() {
        return Err(VtdError::NotPresent);
    }
    let pages = validate_range(iova, size)?;
    if phys & (PAGE_SIZE_4K as u64 - 1) != 0 {
        return Err(VtdError::AddressMisaligned);
    }
    let index = domain.as_u16() as usize;
    if index >= MAX_VTD_DOMAINS {
        return Err(VtdError::DomainNotFound);
    }
    let state = STATE.lock();
    if !state.domains[index].used {
        return Err(VtdError::DomainNotFound);
    }
    let _ = pages;
    let _ = flags;
    // Validated and dropped: there are no second-level tables to write into.
    Err(VtdError::NotEnforcing)
}
