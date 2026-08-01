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

use super::super::types::SchedulerStats;
use core::sync::atomic::Ordering;

pub const DEFAULT_TIME_SLICE: u64 = 10;
pub(crate) static SCHEDULER_STATS: SchedulerStats = SchedulerStats::new();

// The slice counter and the reschedule flag belong to the CPU running the
// task, not to the machine. As single globals every CPU's timer tick decayed
// the same slice, so N cores exhausted it N times faster than one, and a
// reschedule raised on any core was observed by all of them.

/// Ticks left in the running task's slice on this CPU.
pub fn time_slice() -> u64 {
    crate::smp::percpu::current().time_slice.load(Ordering::Relaxed)
}

/// Give the running task a fresh slice on this CPU.
pub fn set_time_slice(ticks: u64) {
    crate::smp::percpu::current().time_slice.store(ticks, Ordering::Relaxed);
}

/// Spend one tick. Returns the value before the decrement, saturating at zero,
/// so the caller can tell the tick that exhausted the slice from the rest.
pub fn spend_time_slice() -> u64 {
    let slot = &crate::smp::percpu::current().time_slice;
    let before = slot.load(Ordering::Relaxed);
    if before > 0 {
        slot.store(before - 1, Ordering::Relaxed);
    }
    before
}

pub fn need_reschedule() -> bool {
    crate::smp::percpu::current().need_resched.load(Ordering::Relaxed) != 0
}

pub fn set_reschedule() {
    crate::smp::percpu::current().need_resched.store(1, Ordering::Release);
}

pub fn clear_reschedule() {
    crate::smp::percpu::current().need_resched.store(0, Ordering::Relaxed);
}
