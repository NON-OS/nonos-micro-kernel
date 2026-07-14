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

use super::arat_supported::arat_supported;
use super::consts::{HALT_SAFE, MSR_IA32_POWER_CTL, POWER_CTL_C1E_ENABLE};
use super::is_intel::is_intel;
use crate::arch::x86_64::boot::cpu_ops::{rdmsr, wrmsr};

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
