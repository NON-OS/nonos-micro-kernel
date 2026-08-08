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

//! Runnable-pid queue. FIFO by arrival so a long-running pid does
//! not starve newcomers the way a pid-sorted set would; `insert`
//! refuses duplicates so the same pid cannot be enqueued twice.

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;

use crate::interrupts::disable_interrupts_guard;

// The timer interrupt reaches this queue through the sleep-wake sweep
// (check_sleeping_processes -> wake_process -> add_to_run_queue). If a spawn or
// scheduler path held the lock with interrupts on and a tick landed, the handler
// would spin on a lock the interrupted code can never release. So every path
// that touches the queue holds interrupts off for the critical section.
static PID_RUN_QUEUE: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());

pub fn add_to_run_queue(pid: u32) {
    if insert(pid, false) {
        crate::smp::wake_idle_cpu();
    }
}

pub fn add_to_run_queue_front(pid: u32) {
    if insert(pid, true) {
        crate::smp::wake_idle_cpu();
    }
}

/// Enqueue `pid` unless it is already queued. Returns whether it went in, so
/// the caller can tell a real arrival from a duplicate and only then pay for
/// waking a CPU.
///
/// The wake happens outside this function on purpose: it sends an IPI, and the
/// woken CPU takes this same lock, so doing it here would have it spin with
/// interrupts off on a lock we still hold.
fn insert(pid: u32, front: bool) -> bool {
    let _irq = disable_interrupts_guard();
    let mut q = PID_RUN_QUEUE.lock();
    if q.iter().any(|p| *p == pid) {
        return false;
    }
    if front {
        q.push_front(pid);
    } else {
        q.push_back(pid);
    }
    true
}

pub fn remove_from_run_queue(pid: u32) {
    let _irq = disable_interrupts_guard();
    let mut q = PID_RUN_QUEUE.lock();
    if let Some(pos) = q.iter().position(|p| *p == pid) {
        q.remove(pos);
    }
}

pub fn is_in_run_queue(pid: u32) -> bool {
    let _irq = disable_interrupts_guard();
    PID_RUN_QUEUE.lock().iter().any(|p| *p == pid)
}

pub fn runnable_process_count() -> usize {
    let _irq = disable_interrupts_guard();
    PID_RUN_QUEUE.lock().len()
}

pub fn get_runnable_pids() -> Vec<u32> {
    let _irq = disable_interrupts_guard();
    PID_RUN_QUEUE.lock().iter().copied().collect()
}

// Pop the head — arrival-order selection. Used by the dispatcher
// when it wants O(1) "next runnable pid" rather than a full
// snapshot to scan.
