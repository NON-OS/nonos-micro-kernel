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

use super::types::DebugInfo;

pub fn read_debug_info() -> DebugInfo {
    let dr6: u64;
    unsafe {
        core::arch::asm!("mov {}, dr6", out(reg) dr6, options(nomem, nostack));
    }
    DebugInfo {
        dr6,
        breakpoint_0: (dr6 & 0x01) != 0,
        breakpoint_1: (dr6 & 0x02) != 0,
        breakpoint_2: (dr6 & 0x04) != 0,
        breakpoint_3: (dr6 & 0x08) != 0,
        single_step: (dr6 & 0x4000) != 0,
        task_switch: (dr6 & 0x8000) != 0,
    }
}
