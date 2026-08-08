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

use super::super::realtime;
use super::proc_ticks;
use super::state::{set_reschedule, spend_time_slice, SCHEDULER_STATS};
use core::sync::atomic::Ordering;

pub fn tick() {
    proc_ticks::charge_tick(crate::process::CURRENT_PID.load(Ordering::Relaxed));
    SCHEDULER_STATS.tick_count.fetch_add(1, Ordering::SeqCst);
    // This CPU's own slice. The tick that takes it from one to zero is the one
    // that exhausted it.
    if spend_time_slice() == 1 {
        SCHEDULER_STATS.time_slice_exhaustions.fetch_add(1, Ordering::SeqCst);
        if crate::sys::policy::kernel_preempt() {
            set_reschedule();
        }
    }
    if realtime::has_realtime_tasks() {
        set_reschedule();
    }
}
