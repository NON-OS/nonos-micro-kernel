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

/// Drop every cached context entry, so a rewritten root or context table is
/// what the unit consults from now on.
pub fn invalidate_context_global(unit: &RemapUnit) -> Result<(), VtdError> {
    // Table writes must be visible before the command that re-reads them, or
    // the unit refills from what it just dropped.
    compiler_fence(Ordering::SeqCst);
    // SAFETY: eK@nonos.systems - discarding cached copies of entries this
    // module wrote cannot grant access.
    unsafe {
        unit.write64(offsets::CCMD, offsets::CCMD_ICC | offsets::CCMD_CIRG_GLOBAL);
    }
    for _ in 0..offsets::COMMAND_SPINS {
        if unit.read64(offsets::CCMD) & offsets::CCMD_ICC == 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(VtdError::Timeout)
}
