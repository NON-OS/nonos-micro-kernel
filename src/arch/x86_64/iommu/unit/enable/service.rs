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

use super::root_table::install_root_table;
use super::translation::enable_translation;
use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::invalidate::invalidate_all;

/// Install, invalidate, enable. Refuses a unit firmware left enabled: its
/// tables describe transfers already in flight, and swapping the root would
/// fault DMA the kernel did not issue and cannot retry.
///
/// # Safety
/// As `install_root_table`.
pub unsafe fn bring_into_service(
    unit: &RemapUnit,
    root_phys: u64,
    ecap: u64,
) -> Result<(), VtdError> {
    if unit.read32(offsets::GSTS) & offsets::GSTS_TES != 0 {
        return Err(VtdError::FirmwareOwnsUnit);
    }
    // SAFETY: eK@nonos.systems - the caller's promise about `root_phys`.
    unsafe {
        install_root_table(unit, root_phys)?;
    }
    invalidate_all(unit, ecap)?;
    // SAFETY: eK@nonos.systems - root installed and caches dropped above.
    unsafe { enable_translation(unit) }
}
