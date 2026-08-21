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
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;

/// # Safety
/// A root table must already be installed and the caches invalidated, or the
/// unit translates through whatever it last cached.
pub unsafe fn enable_translation(unit: &RemapUnit) -> Result<(), VtdError> {
    // GCMD is write-only, so the bits that persist come from status; reading
    // GCMD itself returns unrelated data.
    let persistent = unit.read32(offsets::GSTS) & !offsets::GSTS_TES;
    // SAFETY: eK@nonos.systems - the caller promised the ordering above, and
    // the tables this enables deny by default.
    unsafe {
        unit.write32(offsets::GCMD, persistent | offsets::GCMD_TE);
    }
    for _ in 0..offsets::COMMAND_SPINS {
        if unit.read32(offsets::GSTS) & offsets::GSTS_TES != 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(VtdError::Timeout)
}
