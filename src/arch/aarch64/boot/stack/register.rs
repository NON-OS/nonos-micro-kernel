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

use core::arch::asm;

pub fn switch_to(kernel_top: u64, irq_top: u64) {
    unsafe {
        asm!("mov sp, {0}", in(reg) kernel_top, options(nostack));
        asm!("msr sp_el0, {0}", in(reg) irq_top, options(nostack));
    }
}

pub fn current_stack_pointer() -> u64 {
    let sp: u64;
    unsafe {
        asm!("mov {}, sp", out(reg) sp, options(nostack));
    }
    sp
}
