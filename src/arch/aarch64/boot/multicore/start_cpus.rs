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

use crate::arch::aarch64::asm::_aarch64_secondary_start;
use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::aarch64::boot::stack::get_kernel_stack;
use crate::arch::aarch64::cpu;
use crate::arch::aarch64::psci;

use super::state::CPUS_ONLINE;



pub fn start_secondary_cpus(boot_info: &BootInfo) {
    for cpu in 1..boot_info.cpu_count {
        let stack_top = get_kernel_stack(cpu as usize);
        let entry = _aarch64_secondary_start as u64;

        if psci::cpu_on(cpu as u64, entry, stack_top).is_err() {
            cpu::halt();
        }
    }

    while CPUS_ONLINE.load(Ordering::Acquire) < boot_info.cpu_count {
        core::hint::spin_loop();
    }
}
