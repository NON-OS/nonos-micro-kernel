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
use crate::constants::{
    PORTSC_CCS, PORTSC_CHANGE_BITS, PORTSC_PED, PORTSC_PLS_MASK, PORTSC_PR, PORTSC_PRC,
};
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{portsc_clear_changes, portsc_read, portsc_write};
const RESET_POLL_LIMIT: u32 = 1_000_000;
pub fn reset_port(op_base: u64, port: u8) -> XhciResult<u32> {
    let first = portsc_read(op_base, port);
    if (first & PORTSC_CCS) == 0 {
        return Err(XhciError::NoDeviceOnPort);
    }
    portsc_clear_changes(op_base, port, first);
    let clean = portsc_read(op_base, port) & !PORTSC_CHANGE_BITS;
    portsc_write(op_base, port, (clean & !PORTSC_PLS_MASK) | PORTSC_PR);
    for _ in 0..RESET_POLL_LIMIT {
        let now = portsc_read(op_base, port);
        if (now & PORTSC_PRC) != 0 && (now & PORTSC_PED) != 0 {
            portsc_clear_changes(op_base, port, now);
            return Ok(portsc_read(op_base, port));
        }
        core::hint::spin_loop();
    }
    Err(XhciError::PortResetTimeout)
}