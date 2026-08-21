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
            let _ = send_ipi(cpu.get_apic_id(), Ipi::Reschedule);
        }
    }
}

/// Hand newly runnable work to a CPU that is halted, if there is one.
///
/// One CPU, not a broadcast: waking every idle CPU for a single task has them
/// all take the run queue lock, and all but one find it empty again. The task
/// producer calls this after the queue is populated, so the woken CPU sees the
/// work when it looks. Waking nobody is not a failure; every CPU reaches the
/// scheduler on its own timer tick regardless, and the idle loop tests the run
/// queue rather than trusting the wake.
pub fn wake_idle_cpu() {
    let me = super::cpu::cpu_id();
    for id in 0..super::state::cpu_count() {
        if id == me {
            continue;
        }
        let Some(cpu) = get_cpu(id) else { continue };
        if cpu.is_online() && cpu.idle.load(Ordering::Acquire) {
            let _ = send_ipi(cpu.get_apic_id(), Ipi::Reschedule);
            return;
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
