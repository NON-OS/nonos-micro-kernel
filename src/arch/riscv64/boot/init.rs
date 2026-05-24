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

use super::info::BootInfo;
use crate::arch::riscv64::{cpu, interrupts, mmu, plic, security, timer, uart};

pub fn init(boot_info: &BootInfo) {
    uart::init_uart(boot_info.uart_base);
    cpu::init_cpu();
    interrupts::install_stvec();
    if security::init_all().is_err() {
        cpu::halt();
    }
    mmu::init_mmu(boot_info);
    if boot_info.plic_base != 0 {
        if plic::init_plic(boot_info.plic_base).is_err() {
            cpu::halt();
        }
    }
    if timer::init_timer().is_err() {
        cpu::halt();
    }
    if boot_info.hart_count > 1 {
        super::multicore::start_secondary_harts(boot_info);
    }
}
