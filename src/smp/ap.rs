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

use super::constants::{IPI_FLAG_PANIC, IPI_FLAG_RESCHEDULE, IPI_FLAG_STOP};
use super::ipi_handler::{handle_panic_ipi, handle_stop_ipi};
use super::state::{AP_STARTUP_BARRIER, CPU_DESCRIPTORS};
use super::types::CpuState;
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

    crate::sched::init_ap_scheduler(cpu_id as usize);

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

fn ap_idle_loop(cpu_id: u32) -> ! {
    loop {
        let cpu = &CPU_DESCRIPTORS[cpu_id as usize];
        let pending = cpu.ipi_pending.load(Ordering::Relaxed);

        if pending & IPI_FLAG_RESCHEDULE != 0 {
            cpu.ipi_pending.fetch_and(!IPI_FLAG_RESCHEDULE, Ordering::Relaxed);
            crate::sched::schedule();
        }

        if pending & IPI_FLAG_PANIC != 0 {
            handle_panic_ipi();
        }

        if pending & IPI_FLAG_STOP != 0 {
            handle_stop_ipi();
        }

        // SAFETY: Enter low-power wait state
        unsafe {
            core::arch::asm!("sti; hlt", options(nostack, nomem));
        }

        cpu.idle_cycles.fetch_add(1, Ordering::Relaxed);
    }
}
