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

use core::sync::atomic::Ordering;

use crate::arch::riscv64::asm::_riscv64_secondary_start;
use crate::arch::riscv64::boot::info::BootInfo;
use crate::arch::riscv64::boot::stack::get_kernel_stack;
use crate::arch::riscv64::sbi;

use super::state::HARTS_ONLINE;

pub fn start_secondary_harts(boot_info: &BootInfo) {
    for hart in 0..boot_info.hart_count {
        if hart == boot_info.boot_hart {
            continue;
        }

        let stack_top = get_kernel_stack(hart as usize);
        let entry = _riscv64_secondary_start as u64;

        if sbi::hart_start(hart as u64, entry, stack_top).is_err() {
            crate::arch::riscv64::cpu::halt();
        }
    }

    while HARTS_ONLINE.load(Ordering::Acquire) < boot_info.hart_count {
        core::hint::spin_loop();
    }
}
