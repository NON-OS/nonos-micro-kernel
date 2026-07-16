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

use nonos_libc::Deadline;

use super::established::link_established;
use super::ready::device_ready;
use crate::constants::ata::{COMRESET_HOLD_MS, LINK_TIMEOUT_MS};
use crate::constants::regs::{
    PORT_SCTL, PORT_SERR, PORT_SSTS, PORT_TFD, SCTL_DET_COMRESET, SCTL_DET_MASK,
};
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Bring the SATA link up before any command is issued: force a COMRESET so a
/// link the firmware left down is re-established, wait for the PHY to report
/// communication, clear the errors latched during OOB, then wait for the device
/// to finish its power-on diagnostics. Without this, IDENTIFY was issued into a
/// port whose device was still busy, which real drives reject.
pub(crate) fn link_up(regs: Regs, base: u32) -> AhciResult<()> {
    unsafe {
        let sctl = regs.r32(base + PORT_SCTL);
        regs.w32(base + PORT_SCTL, (sctl & !SCTL_DET_MASK) | SCTL_DET_COMRESET);
        let hold = Deadline::after_ms(COMRESET_HOLD_MS);
        while !hold.expired() {
            core::hint::spin_loop();
        }
        regs.w32(base + PORT_SCTL, regs.r32(base + PORT_SCTL) & !SCTL_DET_MASK);

        let deadline = Deadline::after_ms(LINK_TIMEOUT_MS);
        while !link_established(regs.r32(base + PORT_SSTS)) {
            if deadline.expired() {
                return Err(AhciError::Timeout);
            }
            core::hint::spin_loop();
        }
        regs.w32(base + PORT_SERR, regs.r32(base + PORT_SERR));

        let deadline = Deadline::after_ms(LINK_TIMEOUT_MS);
        while !device_ready(regs.r32(base + PORT_TFD)) {
            if deadline.expired() {
                return Err(AhciError::Timeout);
            }
            core::hint::spin_loop();
        }
    }
    Ok(())
}
