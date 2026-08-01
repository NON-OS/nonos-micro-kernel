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

use crate::arch::cpu::idle_cpu;
use crate::arch::riscv64::cpu;
use crate::arch::riscv64::interrupts::install_stvec;
use crate::arch::riscv64::plic::init_plic_hart;
use crate::arch::riscv64::security;
use crate::arch::riscv64::timer::init_timer_hart;

use super::state::HARTS_ONLINE;

#[no_mangle]
pub extern "C" fn riscv64_ap_entry(_hart_id: u64, _stack_top: u64) -> ! {
    install_stvec();
    cpu::init_cpu();
    if security::init_all().is_err() {
        cpu::halt();
    }
    if init_plic_hart().is_err() {
        cpu::halt();
    }
    if init_timer_hart().is_err() {
        cpu::halt();
    }
    HARTS_ONLINE.fetch_add(1, Ordering::AcqRel);
    let hart = cpu::id::hart_id();
    loop {
        idle_cpu();
    }
}
