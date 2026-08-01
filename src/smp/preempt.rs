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

use super::cpu::current_cpu;
use crate::process::scheduler::preemption::{clear_reschedule, need_reschedule};
use core::sync::atomic::Ordering;

#[inline]
pub fn preempt_disable() {
    current_cpu().preempt_disable_count.fetch_add(1, Ordering::Relaxed);
}

#[inline]
pub fn preempt_enable() {
    let count = current_cpu().preempt_disable_count.fetch_sub(1, Ordering::Relaxed);
    if count == 1 {
        maybe_reschedule();
    }
}

#[inline]
pub fn preempt_enabled() -> bool {
    current_cpu().preempt_disable_count.load(Ordering::Relaxed) == 0
}

fn maybe_reschedule() {
    if need_reschedule() {
        clear_reschedule();
        crate::sched::schedule();
    }
}
