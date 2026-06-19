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
use crate::constants::regs::{CMD_CR, CMD_FR, CMD_FRE, CMD_ST, PORT_CMD};
use crate::regs::Regs;

pub(super) fn stop(regs: Regs, base: u32) {
    unsafe {
        let cmd = regs.r32(base + PORT_CMD);
        regs.w32(base + PORT_CMD, cmd & !CMD_ST);
        let mut spin = 0u32;
        while regs.r32(base + PORT_CMD) & (CMD_FR | CMD_CR) != 0 && spin < COMPLETION_POLL_LIMIT {
            spin += 1;
            core::hint::spin_loop();
        }
        let cmd = regs.r32(base + PORT_CMD);
        regs.w32(base + PORT_CMD, cmd & !CMD_FRE);
    }
}
