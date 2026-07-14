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

use core::arch::asm;

// idle_cpu: enable interrupts atomically with the halt. When the platform
// cannot guarantee the LAPIC timer survives a halt (no ARAT and C1E not
// disabled), spin with interrupts enabled instead so the scheduler tick
// keeps arriving. The spin still lets any pending interrupt fire (sti) and
// `pause` keeps it cheap; the next scheduler pass re-checks the runqueue.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn idle_cpu() {
    if crate::arch::x86_64::interrupt::apic::idle_timer::halt_safe() {
        unsafe {
            asm!("sti; hlt", options(nomem, nostack));
        }
    } else {
        unsafe {
            asm!("sti", options(nomem, nostack));
        }
        for _ in 0..4096 {
            core::hint::spin_loop();
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn idle_cpu() {
    unsafe {
        asm!("msr daifclr, #2", "wfi", options(nomem, nostack));
    }
}

#[cfg(target_arch = "riscv64")]
#[inline(always)]
pub fn idle_cpu() {
    unsafe {
        asm!("csrsi sstatus, 2", "wfi", options(nomem, nostack));
    }
}
