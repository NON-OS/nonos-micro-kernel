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

use super::table::root_table;
use crate::arch::x86_64::iommu::tables::context::{entry_address, is_present, root_low};
use crate::arch::x86_64::iommu::tables::frame::{allocate_table, entries_mut};
use crate::arch::x86_64::iommu::types::VtdError;

/// Root and context entries are 128 bits stored low half first, so entry `i`
/// occupies slots `2i` and `2i + 1`.
pub(super) const fn slot_of(index: usize) -> usize {
    index * 2
}

/// The context table for a bus, created if this is the bus's first device. A
/// bus with nothing assigned has no table, and its devices are denied by the
/// cleared present bit rather than by an empty table.
pub(super) fn context_table_for(bus: u8) -> Result<u64, VtdError> {
    let root = root_table()?;
    let slot = slot_of(bus as usize);
    let low = entries_mut(root)?[slot];
    if is_present(low) {
        return Ok(entry_address(low));
    }
    let table = allocate_table()?;
    let entries = entries_mut(root)?;
    // The present bit in the low half publishes the entry, so everything it
    // describes is in place first.
    entries[slot + 1] = 0;
    entries[slot] = root_low(table);
    Ok(table)
}
