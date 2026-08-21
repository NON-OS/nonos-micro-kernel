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

use super::context_table::slot_of;
use super::table::root_table;
use crate::arch::x86_64::iommu::tables::context::{context_index, entry_address, is_present};
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::types::{SourceId, VtdError};

/// Deny a device again. The present bit goes first, so the device is denied
/// before the domain it named is forgotten.
pub fn clear_context(source: SourceId) -> Result<(), VtdError> {
    let root = root_table()?;
    let root_slot = slot_of(source.bus() as usize);
    let low = entries_mut(root)?[root_slot];
    if !is_present(low) {
        return Err(VtdError::DeviceNotAttached);
    }
    let table = entry_address(low);
    let slot = slot_of(context_index(source.device(), source.function()));
    let entries = entries_mut(table)?;
    if !is_present(entries[slot]) {
        return Err(VtdError::DeviceNotAttached);
    }
    entries[slot] = 0;
    entries[slot + 1] = 0;
    Ok(())
}
