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
use crate::constants::USBCMD_HCRST;
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{usbcmd_read, usbcmd_write};
const RESET_POLL_LIMIT: u32 = 200_000;
pub fn reset(op_base: u64) -> XhciResult<()> {
    usbcmd_write(op_base, usbcmd_read(op_base) | USBCMD_HCRST);
    for _ in 0..RESET_POLL_LIMIT {
        if usbcmd_read(op_base) & USBCMD_HCRST == 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(XhciError::ResetTimeout)
}