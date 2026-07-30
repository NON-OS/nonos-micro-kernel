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

use super::boot;

/// Assumed counter rate when the platform never reported one.
///
/// Only x86_64 reaches this: `CNTFRQ_EL0` always answers, while a TSC
/// frequency has to come from a CPUID leaf or a PIT measurement and either
/// can fail. Time then runs at the wrong rate, but it runs, which is what
/// every sleeping process and every timeout needs. It is deliberately the
/// same 2.5 GHz the x86 timer assumed before this module existed.
const ASSUMED_HZ: u64 = 2_500_000_000;

/// Nanoseconds since the clock was anchored.
///
/// Monotonic by construction: it is a saturating delta over a counter that
/// only counts up, scaled by a frequency latched once. It is not wall-clock
/// time and says nothing about the date.
#[inline]
pub fn now_ns() -> u64 {
    let hz = match boot::counter_hz() {
        0 => ASSUMED_HZ,
        hz => hz,
    };

    // Widened before scaling: at a nanosecond scale a 64-bit product overflows
    // after a few seconds of counter ticks, which is how a clock ends up
    // wrapping to zero while the machine is still booting.
    let ticks = boot::ticks_since_anchor() as u128;
    ((ticks * 1_000_000_000u128) / hz as u128) as u64
}
