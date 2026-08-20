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

use crate::process::scheduler::dispatch::runnable_process_count;
use crate::process::scheduler::preemption::{clear_reschedule, need_reschedule};
use crate::smp::state::CPU_DESCRIPTORS;
use core::sync::atomic::Ordering;

/// Where an AP lives when it has nothing to run.
///
/// The halt decision is made against the run queue itself, not the reschedule
/// flag alone. A CPU that enqueues work may not know this one is idle and may
/// skip the IPI; if the flag were the only signal, that task would sit
/// unclaimed until some unrelated interrupt happened to land here. The IPI is
/// a latency optimisation, not the correctness condition.
pub(super) fn ap_idle_loop(cpu_id: u32) -> ! {
    let cpu = &CPU_DESCRIPTORS[cpu_id as usize];
    loop {
        // Interrupts off across the test, so work appearing between the test
        // and the halt cannot be missed.
        // SAFETY: eK@nonos.systems - masking interrupts on this CPU only. The
        // window is closed again by the `sti; hlt` below on every path.
        unsafe {
            core::arch::asm!("cli", options(nostack, nomem));
        }

        if need_reschedule() || runnable_process_count() > 0 {
            clear_reschedule();
            cpu.idle.store(false, Ordering::Relaxed);
            // SAFETY: eK@nonos.systems - re-enabling interrupts on this CPU
            // before entering the scheduler, which must not run masked.
            unsafe {
                core::arch::asm!("sti", options(nostack, nomem));
            }
            crate::sched::schedule();
            continue;
        }

        cpu.idle.store(true, Ordering::Release);

        // `sti` does not take effect until after the following instruction, so
        // `sti; hlt` halts with the window already closed and a pending
        // interrupt wakes the CPU immediately.
        // SAFETY: eK@nonos.systems - no state of ours is live across the halt;
        // the CPU resumes at the next instruction once an interrupt arrives.
        unsafe {
            core::arch::asm!("sti; hlt", options(nostack, nomem));
        }

        cpu.idle.store(false, Ordering::Relaxed);
        cpu.idle_cycles.fetch_add(1, Ordering::Relaxed);
    }
}
