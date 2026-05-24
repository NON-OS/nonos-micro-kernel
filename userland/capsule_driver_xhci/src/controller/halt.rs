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
use crate::constants::{USBCMD_RUN, USBSTS_HCH};
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{usbcmd_read, usbcmd_write, usbsts_read};
const HALT_POLL_LIMIT: u32 = 200_000;
pub fn halt(op_base: u64) -> XhciResult<()> {
    let cur = usbcmd_read(op_base);
    if cur & USBCMD_RUN != 0 {
        usbcmd_write(op_base, cur & !USBCMD_RUN);
    }
    for _ in 0..HALT_POLL_LIMIT {
        if usbsts_read(op_base) & USBSTS_HCH != 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(XhciError::HaltTimeout)
}