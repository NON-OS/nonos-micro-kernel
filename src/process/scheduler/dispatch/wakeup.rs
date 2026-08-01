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

use super::super::preemption::{set_reschedule, SCHEDULER_STATS};
use super::run_queue::runnable_process_count;
use super::sleep::check_sleeping_processes;
use core::sync::atomic::Ordering;

pub fn wakeup() {
    SCHEDULER_STATS.wakeups.fetch_add(1, Ordering::Relaxed);
    check_sleeping_processes();
    if runnable_process_count() > 0 {
        set_reschedule();
    }
}
