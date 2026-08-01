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

use super::state::{AP_STARTUP_BARRIER, CPU_DESCRIPTORS};
use super::types::CpuState;
use crate::process::scheduler::dispatch::runnable_process_count;
use crate::process::scheduler::preemption::{clear_reschedule, need_reschedule};
use core::sync::atomic::Ordering;

#[no_mangle]
pub unsafe extern "C" fn ap_entry(cpu_id: u32) {
    // Per-CPU LAPIC bring-up: this AP's LAPIC comes out of INIT/SIPI
    // software-disabled. The global mode and MMIO mapping were adopted from
    // the BSP before the SIPI, so only the register programming runs here.
    unsafe { crate::arch::x86_64::interrupt::apic::init_ap_lapic() };
    let apic_id = crate::arch::interrupt_controller::local_id();

    // GDT/TSS before anything that can take an exception.
    unsafe {
        let _ = crate::arch::x86_64::cpu::init_ap(cpu_id as u16, apic_id);
    }

    // BSP prepared the global IDT; APs just need lidt on their own CPU.
    unsafe {
        crate::arch::x86_64::idt::load_on_ap();
    }

    super::percpu::init_ap(cpu_id as usize);

    // BSP already registered the IRQ-0 handler; each AP just arms its
    // own LAPIC timer.
    crate::arch::x86_64::interrupt::apic::preemption::install_on_ap();

    CPU_DESCRIPTORS[cpu_id as usize].set_state(CpuState::Online);

    AP_STARTUP_BARRIER.fetch_add(1, Ordering::Release);

    unsafe {
        core::arch::asm!("sti", options(nostack, nomem));
    }

    ap_idle_loop(cpu_id);
}

/// Where an AP lives when it has nothing to run.
///
/// The halt decision is made against the run queue itself, not against the
/// reschedule flag alone. A CPU that enqueues work may not know this one is
/// idle and may skip the IPI; if the flag were the only signal, the task would
/// sit unclaimed until some unrelated interrupt happened to land here. The IPI
/// is a latency optimisation on top, not the correctness condition.
fn ap_idle_loop(cpu_id: u32) -> ! {
    let cpu = &CPU_DESCRIPTORS[cpu_id as usize];
    loop {
        // Interrupts off across the test so work that appears between the test
        // and the halt cannot be missed. `sti` does not take effect until after
        // the instruction following it, so `sti; hlt` halts with the window
        // already closed, and a pending interrupt wakes the CPU immediately.
        unsafe {
            core::arch::asm!("cli", options(nostack, nomem));
        }

        if need_reschedule() || runnable_process_count() > 0 {
            clear_reschedule();
            cpu.idle.store(false, Ordering::Relaxed);
            unsafe {
                core::arch::asm!("sti", options(nostack, nomem));
            }
            crate::sched::schedule();
            continue;
        }

        cpu.idle.store(true, Ordering::Release);

        // SAFETY: no state of ours is live across the halt; the CPU resumes at
        // the next instruction once any interrupt is delivered.
        unsafe {
            core::arch::asm!("sti; hlt", options(nostack, nomem));
        }

        cpu.idle.store(false, Ordering::Relaxed);
        cpu.idle_cycles.fetch_add(1, Ordering::Relaxed);
    }
}
