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

use crate::constants::{GHC_AE, GHC_HR, HBA_GHC, HBA_IS};
use crate::regs::Regs;

// GHC.HR self-clears within 1s on a healthy HBA; bound the wait generously.
const RESET_POLL_LIMIT: u32 = 2_000_000;

pub fn enable_ahci(regs: Regs) {
    unsafe {
        // AHCI 1.3.1 section 10.4.3. Enter AHCI mode, then reset the HBA so we
        // start from a known state rather than trusting whatever the firmware
        // left mid-configuration. HR self-clears when the reset completes and
        // also clears AE, so AHCI mode is re-entered afterwards.
        regs.w32(HBA_GHC, regs.r32(HBA_GHC) | GHC_AE);
        regs.w32(HBA_GHC, regs.r32(HBA_GHC) | GHC_HR);
        for _ in 0..RESET_POLL_LIMIT {
            if regs.r32(HBA_GHC) & GHC_HR == 0 {
                break;
            }
            core::hint::spin_loop();
        }
        regs.w32(HBA_GHC, regs.r32(HBA_GHC) | GHC_AE);
        regs.w32(HBA_IS, 0xffff_ffff);
    }
}
