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

use core::sync::atomic::{compiler_fence, Ordering};

use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;

/// # Safety
/// `root_phys` must be a root table this kernel owns and keeps alive: the unit
/// walks it by physical address from here on.
pub unsafe fn install_root_table(unit: &RemapUnit, root_phys: u64) -> Result<(), VtdError> {
    // The table must be in memory before the unit is told where it is; it may
    // read it the moment the pointer is set.
    compiler_fence(Ordering::SeqCst);
    // SAFETY: eK@nonos.systems - the caller promised a live root table, and
    // translation table mode zero is what these tables are.
    unsafe {
        unit.write64(offsets::RTADDR, root_phys);
        unit.write32(offsets::GCMD, offsets::GCMD_SRTP);
    }
    for _ in 0..offsets::COMMAND_SPINS {
        if unit.read32(offsets::GSTS) & offsets::GSTS_RTPS != 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(VtdError::Timeout)
}
