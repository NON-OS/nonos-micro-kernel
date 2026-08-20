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

use super::context_table::{context_table_for, slot_of};
use crate::arch::x86_64::iommu::tables::context::{
    context_high, context_index, context_low, is_present,
};
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::types::{DomainId, SourceId, VtdError};

/// Point one device at a domain's second-level tables. `address_width` is the
/// AGAW encoding, not a bit count. The present bit is written last, so a unit
/// reading concurrently sees the old state or a complete new one.
pub fn set_context(
    source: SourceId,
    sl_root: u64,
    domain: DomainId,
    address_width: u8,
) -> Result<(), VtdError> {
    let table = context_table_for(source.bus())?;
    let slot = slot_of(context_index(source.device(), source.function()));
    let entries = entries_mut(table)?;
    if is_present(entries[slot]) {
        return Err(VtdError::DeviceAlreadyAttached);
    }
    entries[slot + 1] = context_high(domain.as_u16(), address_width);
    entries[slot] = context_low(sl_root);
    Ok(())
}
