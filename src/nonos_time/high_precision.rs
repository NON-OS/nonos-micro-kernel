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

pub fn get_tsc() -> u64 {
    crate::arch::read_time_counter()
}

/// The cycle counter, read the way the part exposes it. The name is the x86
/// instruction because that is what every caller already says; elsewhere this is
/// the same counter `get_tsc` reads, so the two never disagree.
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut high: u32;
        let mut low: u32;
        core::arch::asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));
        ((high as u64) << 32) | (low as u64)
    }
    #[cfg(not(target_arch = "x86_64"))]
    crate::arch::read_time_counter()
}

/// Whether the core can park with MONITOR/MWAIT. Only x86_64 has it, so
/// everywhere else the answer is no rather than unimplemented: callers use it to
/// pick an idle strategy and false selects the portable one.
pub fn has_mwait_support() -> bool {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let cpuid = core::arch::x86_64::__cpuid(1);
        (cpuid.ecx & (1 << 3)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

pub struct HighPrecisionTimer {
    start_tsc: u64,
    tsc_frequency: u64,
}

impl HighPrecisionTimer {
    pub fn new() -> Self {
        Self { start_tsc: get_tsc(), tsc_frequency: Self::calibrate_tsc_frequency() }
    }
    pub fn elapsed_ns(&self) -> u64 {
        let diff = get_tsc() - self.start_tsc;
        (diff * 1_000_000_000) / self.tsc_frequency
    }
    pub fn elapsed_us(&self) -> u64 {
        self.elapsed_ns() / 1000
    }
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed_ns() / 1_000_000
    }

    fn calibrate_tsc_frequency() -> u64 {
        let start_tsc = get_tsc();
        let start_ns = crate::arch::x86_64::time::timer::now_ns();
        let target_ns = start_ns + 1_000_000;
        while crate::arch::x86_64::time::timer::now_ns() < target_ns {
            core::hint::spin_loop();
        }
        let end_tsc = get_tsc();
        let end_ns = crate::arch::x86_64::time::timer::now_ns();
        let tsc_diff = end_tsc - start_tsc;
        let ns_diff = end_ns - start_ns;
        if ns_diff > 0 {
            (tsc_diff * 1_000_000_000) / ns_diff
        } else {
            2_000_000_000
        }
    }
}
