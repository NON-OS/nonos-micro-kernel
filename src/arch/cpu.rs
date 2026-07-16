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

//! Free-function shims that route through the active arch backend.
//!
//! Generic kernel code can call any of the names below. Internally
//! every shim dispatches through `<Arch as ArchOps>::method()` so
//! the implementation stays one place per arch.
//!
//! `init_cpu_features` is x86_64-only kernel-startup setup of CR0
//! / CR4 / XCR0 bits. It does not belong in the cross-arch trait
//! and stays here as a cfg-gated free function.

use core::arch::asm;

use super::abi::ArchOps;
use super::Arch;

// cpu_yield: halt until next interrupt with the current mask state.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn cpu_yield() {
    unsafe {
        asm!("hlt", options(nomem, nostack));
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
#[inline(always)]
pub fn cpu_yield() {
    unsafe {
        asm!("wfi", options(nomem, nostack));
    }
}

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
    // Callers loop over scheduler state (run queue, process table) right
    // after idling and rely on the kernel-wide IF=0 discipline for those
    // locks; returning with interrupts still enabled lets the timer ISR
    // land inside the caller's next lock section and spin-deadlock the core.
    unsafe {
        asm!("cli", options(nomem, nostack));
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

#[inline(always)]
pub fn disable_interrupts() {
    unsafe { Arch::disable_interrupts() }
}

#[inline(always)]
pub fn enable_interrupts() {
    unsafe { Arch::enable_interrupts() }
}

#[inline(always)]
pub fn interrupts_enabled() -> bool {
    Arch::interrupts_enabled()
}

#[inline(always)]
pub fn get_cpu_id() -> u32 {
    Arch::current_cpu_id()
}

/// Monotonic per-CPU tick counter. Backend-defined unit.
#[inline(always)]
pub fn read_time_counter() -> u64 {
    Arch::read_time_counter()
}

/// Bring the CPU into a usable state on boot. CR0 / CR4 / XCR0 bits
/// for x87, SSE, OSXSAVE, and AVX. Kernel boot path only.
#[cfg(target_arch = "x86_64")]
pub fn init_cpu_features() {
    unsafe {
        let cr0: u64;
        asm!("mov {}, cr0", out(reg) cr0, options(nomem, nostack));
        let cr0 = (cr0 | (1 << 1)) & !(1 << 2);
        asm!("mov cr0, {}", in(reg) cr0, options(nomem, nostack));
        let cr4: u64;
        asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack));
        let cr4 = cr4 | (1 << 9) | (1 << 10) | (1 << 18);
        asm!("mov cr4, {}", in(reg) cr4, options(nomem, nostack));
        let mut xcr0: u64 = 1;
        xcr0 |= 1 << 1;
        xcr0 |= 1 << 2;
        asm!(
            "xsetbv",
            in("ecx") 0u32,
            in("eax") xcr0 as u32,
            in("edx") (xcr0 >> 32) as u32,
            options(nomem, nostack),
        );
    }
}
