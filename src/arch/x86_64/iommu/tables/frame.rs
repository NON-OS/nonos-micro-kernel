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

use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc::manager::allocate_frame;
use crate::memory::unified::phys_to_virt;

use super::super::types::VtdError;
use super::sl_pte::ENTRIES;

/// A fresh table must read as all zeroes before any device can reach it: a
/// second-level entry with no permission bits denies rather than faults, so
/// zero is the only safe initial state.
pub fn allocate_table() -> Result<u64, VtdError> {
    let phys = allocate_frame().ok_or(VtdError::PageTableExhausted)?;
    entries_mut(phys.as_u64())?.fill(0);
    Ok(phys.as_u64())
}

/// The kernel's editable view of a table.
///
/// The slice aliases memory a device may be walking. A caller writing through
/// it owns the ordering the spec requires: publish a table's contents before
/// the entry pointing at it, and invalidate afterwards.
pub fn entries_mut(table_phys: u64) -> Result<&'static mut [u64], VtdError> {
    let virt = phys_to_virt(PhysAddr::new(table_phys)).ok_or(VtdError::TableUnreachable)?;
    // SAFETY: eK@nonos.systems - `table_phys` names a frame this module
    // allocated for a remapping table, so it is 4 KiB aligned and owned here.
    // `phys_to_virt` resolved it inside the directmap, which is mapped for the
    // life of the kernel, so the slice cannot outlive its mapping.
    Ok(unsafe { core::slice::from_raw_parts_mut(virt.as_u64() as *mut u64, ENTRIES) })
}
