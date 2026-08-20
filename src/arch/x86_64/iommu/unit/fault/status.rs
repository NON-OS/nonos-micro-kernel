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

use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;

pub(super) fn has_faults(unit: &RemapUnit) -> bool {
    unit.read32(offsets::FSTS) & offsets::FSTS_PPF != 0
}

/// Records were lost because the hardware ran out of room.
pub(super) fn overflowed(unit: &RemapUnit) -> bool {
    unit.read32(offsets::FSTS) & offsets::FSTS_PFO != 0
}

/// # Safety
/// Only after draining, or a pending record is left with nothing advertising
/// it.
pub(super) unsafe fn clear_status(unit: &RemapUnit) {
    // SAFETY: eK@nonos.systems - write-one-to-clear on reporting status; the
    // caller promised the records are already drained.
    unsafe {
        unit.write32(offsets::FSTS, offsets::FSTS_PPF | offsets::FSTS_PFO);
    }
}
