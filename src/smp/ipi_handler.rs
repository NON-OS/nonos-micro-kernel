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

use super::cpu::{current_cpu, get_cpu};
use super::state::CPUS_ONLINE;
use super::types::CpuState;
use crate::arch::interrupt_controller::{broadcast_ipi, send_ipi, Ipi};
use core::sync::atomic::Ordering;

pub fn send_reschedule_ipi(cpu_id: usize) {
    if let Some(cpu) = get_cpu(cpu_id) {
        if cpu.is_online() {
            let _ = send_ipi(cpu.apic_id, Ipi::Reschedule);
        }
    }
}

pub fn send_panic_ipi() {
    let _ = broadcast_ipi(Ipi::Panic);
}

pub fn handle_panic_ipi() -> ! {
    current_cpu().set_state(CpuState::Halted);
    crate::arch::halt_loop()
}

pub fn handle_stop_ipi() -> ! {
    let cpu = current_cpu();
    cpu.set_state(CpuState::GoingOffline);
    cpu.set_state(CpuState::Halted);
    CPUS_ONLINE.fetch_sub(1, Ordering::Release);

    crate::arch::halt_loop()
}
