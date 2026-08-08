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

use core::sync::atomic::{AtomicU64, Ordering};

use crate::arch::read_time_counter;

/// Counter reading taken when the clock was anchored, so elapsed time is
/// measured from a known point rather than from whatever the counter held at
/// power-on. `aarch64` resets `CNTPCT_EL0` at reset and a TSC does not, so
/// without an anchor the two architectures disagree about when zero was.
static BOOT_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Frequency latched at anchor time. Calibration on x86_64 can cost a PIT
/// measurement, and time is read constantly, so it is paid once here.
static COUNTER_HZ: AtomicU64 = AtomicU64::new(0);

/// Anchor the clock to now. Called once the counter is trustworthy.
pub(crate) fn anchor() {
    COUNTER_HZ.store(crate::arch::time_counter_hz(), Ordering::Relaxed);
    BOOT_COUNTER.store(read_time_counter(), Ordering::Release);
}

/// Counter ticks since the anchor. Saturating, so a counter that appears to
/// run backwards across a migration reads as no time passed rather than as
/// several centuries.
pub(crate) fn ticks_since_anchor() -> u64 {
    read_time_counter().saturating_sub(BOOT_COUNTER.load(Ordering::Acquire))
}

/// Latched counter frequency, or zero when the platform could not say.
pub(crate) fn counter_hz() -> u64 {
    COUNTER_HZ.load(Ordering::Relaxed)
}
