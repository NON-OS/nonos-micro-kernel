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

// Nothing is runnable: every process is parked on a timeout or an
// IRQ/IPC wake. Yield used to plain-return here, which sent the
// caller's recv/wait loop spinning at CPL=0 with IF=0 (SFMASK) —
// the timer could never fire, so time froze and no sleeper could
// ever wake. The `sti; hlt` pair is the canonical race-free idle:
// STI's one-instruction shadow means a wake that is already
// pending still lands inside the HLT, and the handler (timer tick
// or broker IRQ) refills the run queue before control returns.
pub(super) fn idle_until_interrupt() {
    // Halt only where the LAPIC timer is known to keep counting through it;
    // on a laptop in C1E the timer clock gates off during hlt and the tick
    // never returns. Where halting is unsafe, spin briefly with interrupts
    // enabled so the wake still lands and the tick still fires.
    if crate::arch::x86_64::interrupt::apic::idle_timer::halt_safe() {
        unsafe {
            core::arch::asm!("sti", "hlt", "cli", options(nomem, nostack));
        }
    } else {
        unsafe {
            core::arch::asm!("sti", options(nomem, nostack));
        }
        for _ in 0..4096 {
            core::hint::spin_loop();
        }
        unsafe {
            core::arch::asm!("cli", options(nomem, nostack));
        }
    }
}
