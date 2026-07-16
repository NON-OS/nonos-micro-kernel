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

//! Keep the local-APIC timer running while the CPU is idle.
//!
//! The scheduler tick is the LAPIC timer, and the idle loop halts the CPU
//! with `hlt` waiting for it. On a plain `hlt` (C1) the timer keeps
//! counting, but laptops enable C1E, an enhanced halt that gates the APIC
//! timer clock: the tick then stops the instant the CPU goes idle, which
//! freezes the clock, preemption, and every timer-driven wakeup. Emulators
//! never show this because their virtual APIC timer always runs and they
//! advertise the ARAT feature that promises exactly that.
//!
//! Two defenses, applied at boot:
//!   1. If the CPU advertises ARAT (CPUID.06H:EAX bit 2) the timer is
//!      reliable in every C-state and `hlt` is safe as-is.
//!   2. Otherwise, on Intel, clear the C1E-enable bit in POWER_CTL so `hlt`
//!      stays in plain C1 where the timer runs. When neither guarantee
//!      holds, `halt_safe()` reports false and the idle loop spins instead
//!      of halting, trading power for a tick that cannot stall.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::arch::x86_64::boot::cpu_ops::{cpuid, rdmsr, wrmsr};

const CPUID_THERMAL_POWER: u32 = 0x06;
const ARAT_BIT: u32 = 1 << 2;

const MSR_IA32_POWER_CTL: u32 = 0x1FC;
const POWER_CTL_C1E_ENABLE: u64 = 1 << 1;

static HALT_SAFE: AtomicBool = AtomicBool::new(true);

fn is_intel() -> bool {
    let (_, ebx, ecx, edx) = cpuid(0);
    // "GenuineIntel" == ebx="Genu", edx="ineI", ecx="ntel"
    ebx == 0x756E_6547 && edx == 0x4965_6E69 && ecx == 0x6C65_746E
}

fn arat_supported() -> bool {
    let (eax, _, _, _) = cpuid(CPUID_THERMAL_POWER);
    eax & ARAT_BIT != 0
}

/// Configure the CPU so a halted core still receives LAPIC timer ticks, and
/// record whether `hlt` is safe for the idle loop to use.
pub fn init() {
    if arat_supported() {
        HALT_SAFE.store(true, Ordering::Release);
        return;
    }
    if is_intel() {
        // SAFETY: POWER_CTL is a documented Intel MSR; clearing the C1E
        // enable bit is a supported, reversible power-policy change and
        // keeps `hlt` in the timer-preserving C1 state.
        unsafe {
            let cur = rdmsr(MSR_IA32_POWER_CTL);
            if cur & POWER_CTL_C1E_ENABLE != 0 {
                wrmsr(MSR_IA32_POWER_CTL, cur & !POWER_CTL_C1E_ENABLE);
            }
        }
        HALT_SAFE.store(true, Ordering::Release);
        return;
    }
    // No ARAT and not an Intel part we can pin to C1: do not trust the timer
    // to survive a deep halt. The idle loop spins instead.
    HALT_SAFE.store(false, Ordering::Release);
}

/// Whether the idle loop may `hlt`. False means halting could stop the
/// scheduler tick, so the caller must spin-idle instead.
#[inline]
pub fn halt_safe() -> bool {
    HALT_SAFE.load(Ordering::Relaxed)
}
