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

/// Drop every cached translation. Runs after a context invalidation: a stale
/// translation outlives the context entry that produced it.
pub fn invalidate_iotlb_global(unit: &RemapUnit, ecap: u64) -> Result<(), VtdError> {
    compiler_fence(Ordering::SeqCst);
    let reg = offsets::iotlb_offset(ecap);
    // SAFETY: eK@nonos.systems - discarding cached translations can only
    // narrow what a device reaches.
    unsafe {
        unit.write64(reg, offsets::IOTLB_IVT | offsets::IOTLB_IIRG_GLOBAL);
    }
    for _ in 0..offsets::COMMAND_SPINS {
        let value = unit.read64(reg);
        if value & offsets::IOTLB_IVT == 0 {
            if value & offsets::IOTLB_IAIG_MASK == 0 {
                return Err(VtdError::Timeout);
            }
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(VtdError::Timeout)
}
