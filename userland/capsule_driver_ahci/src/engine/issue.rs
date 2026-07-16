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

use crate::constants::ata::COMPLETION_POLL_LIMIT;
use crate::constants::regs::{IS_ERR_MASK, PORT_CI, PORT_IS, PORT_TFD, TFD_BSY, TFD_DRQ, TFD_ERR};
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

pub(super) fn issue_slot0(regs: Regs, base: u32) -> AhciResult<()> {
    unsafe {
        regs.w32(base + PORT_IS, u32::MAX);
        let mut spin = 0u32;
        while regs.r32(base + PORT_TFD) & (TFD_BSY | TFD_DRQ) != 0 {
            spin += 1;
            if spin >= COMPLETION_POLL_LIMIT {
                return Err(AhciError::Timeout);
            }
            core::hint::spin_loop();
        }
        // The HBA DMA-reads the command list and table only after it sees the
        // command-issue bit set. Order those stores ahead of the CI write so it
        // never fetches a stale command header, FIS, or PRDT.
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        regs.w32(base + PORT_CI, 1);
        spin = 0;
        loop {
            if regs.r32(base + PORT_IS) & IS_ERR_MASK != 0
                || regs.r32(base + PORT_TFD) & TFD_ERR != 0
            {
                return Err(AhciError::CommandFailed);
            }
            if regs.r32(base + PORT_CI) & 1 == 0 {
                return Ok(());
            }
            spin += 1;
            if spin >= COMPLETION_POLL_LIMIT {
                return Err(AhciError::Timeout);
            }
            core::hint::spin_loop();
        }
    }
}
