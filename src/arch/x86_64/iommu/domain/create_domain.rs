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
use super::super::tables::frame::allocate_table;
use super::super::types::{DomainId, VtdError, MAX_VTD_DOMAINS};

pub fn create_domain(id: DomainId) -> Result<(), VtdError> {
    if !is_present() {
        return Err(VtdError::NotPresent);
    }
    let index = id.as_u16() as usize;
    if index >= MAX_VTD_DOMAINS {
        return Err(VtdError::DomainTableFull);
    }
    let mut state = STATE.lock();
    let slot = &mut state.domains[index];
    if slot.used {
        return Err(VtdError::DomainAlreadyExists);
    }
    // Allocated before the slot is marked used, so a failure here leaves
    // the domain absent rather than present with nowhere to map into.
    slot.root = allocate_table()?;
    slot.used = true;
    Ok(())
}
