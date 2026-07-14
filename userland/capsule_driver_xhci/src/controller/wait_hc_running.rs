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

use crate::constants::USBSTS_HCH;
use crate::error::{XhciError, XhciResult};
use crate::regs::op::usbsts_read;

const HCH_TIMEOUT_MS: u64 = 1_000;

pub fn wait_hc_running(op_base: u64) -> XhciResult<()> {
    let deadline = Deadline::after_ms(HCH_TIMEOUT_MS);
    loop {
        if usbsts_read(op_base) & USBSTS_HCH == 0 {
            return Ok(());
        }
        if deadline.expired() {
            return Err(XhciError::StartTimeout);
        }
        core::hint::spin_loop();
    }
}
