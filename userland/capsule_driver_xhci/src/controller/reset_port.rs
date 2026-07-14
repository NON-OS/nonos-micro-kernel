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

use crate::constants::{
    PORTSC_CCS, PORTSC_CHANGE_BITS, PORTSC_PED, PORTSC_PLS_MASK, PORTSC_PP, PORTSC_PR, PORTSC_PRC,
};
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{portsc_clear_changes, portsc_read, portsc_write};

const RESET_TIMEOUT_MS: u64 = 1_000;
// Time for the port power rail to settle and the controller to sample connect
// status. QEMU asserts CCS immediately; real host controllers need the
// power-good delay before a device shows as connected.
const POWER_SETTLE_MS: u64 = 20;

pub fn reset_port(op_base: u64, port: u8) -> XhciResult<u32> {
    let first = power_on(op_base, port);
    if (first & PORTSC_CCS) == 0 {
        return Err(XhciError::NoDeviceOnPort);
    }
    portsc_clear_changes(op_base, port, first);
    let clean = portsc_read(op_base, port) & !PORTSC_CHANGE_BITS;
    portsc_write(op_base, port, (clean & !PORTSC_PLS_MASK) | PORTSC_PR);
    let deadline = Deadline::after_ms(RESET_TIMEOUT_MS);
    loop {
        let now = portsc_read(op_base, port);
        if (now & PORTSC_PRC) != 0 && (now & PORTSC_PED) != 0 {
            portsc_clear_changes(op_base, port, now);
            return Ok(portsc_read(op_base, port));
        }
        if deadline.expired() {
            return Err(XhciError::PortResetTimeout);
        }
        core::hint::spin_loop();
    }
}

// Assert Port Power and let CCS settle. PP is RW: if the firmware left the
// port unpowered (common on real hardware, unlike QEMU) no device is ever
// reported until we drive it. We preserve the current RW state and only OR in
// PP, avoiding a spurious reset (PR) or disable (PED). PLS is masked so the
// power-up does not double as a link-state strobe.
fn power_on(op_base: u64, port: u8) -> u32 {
    let current = portsc_read(op_base, port);
    if (current & PORTSC_PP) == 0 {
        let base = current & !(PORTSC_CHANGE_BITS | PORTSC_PR | PORTSC_PED | PORTSC_PLS_MASK);
        portsc_write(op_base, port, base | PORTSC_PP);
    }
    let mut last = portsc_read(op_base, port);
    let deadline = Deadline::after_ms(POWER_SETTLE_MS);
    while (last & PORTSC_CCS) == 0 && !deadline.expired() {
        core::hint::spin_loop();
        last = portsc_read(op_base, port);
    }
    last
}
