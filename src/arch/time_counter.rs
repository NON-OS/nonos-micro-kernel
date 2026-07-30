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

//! The CPU's free-running cycle counter, read through the arch boundary.
//!
//! Callers use this for elapsed-time sampling and for stirring entropy pools,
//! not for wall-clock time: the counter's tick rate is architecture and part
//! specific, and it is not synchronised across sockets. `x86_64` reads the TSC,
//! `aarch64` the generic timer counter.

use crate::arch::abi::ArchOps;

/// Current value of the CPU cycle counter.
#[inline]
pub(crate) fn read_time_counter() -> u64 {
    crate::arch::Arch::read_time_counter()
}

/// How many counter ticks make a second, or zero when the platform cannot say.
///
/// aarch64 publishes it in `CNTFRQ_EL0`, so the answer is exact and free. A PC
/// has no such register: the TSC frequency is either reported through a CPUID
/// leaf or has to be measured against the PIT, and either can come back
/// unknown. Zero is that answer, and `sys::clock` treats it as "not
/// calibrated" rather than dividing by it.
pub(crate) fn time_counter_hz() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        use crate::arch::x86_64::time::tsc;
        if let Some(freq) = tsc::get_cpuid_frequency() {
            return freq;
        }
        return tsc::calibrate_with_pit().map(|(freq, _confidence)| freq).unwrap_or(0);
    }
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::timer::frequency();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return 0;
}
