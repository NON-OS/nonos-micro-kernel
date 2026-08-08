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

//! A blocking futex over the scheduler's sleep/wake, so std Mutex, RwLock,
//! Condvar, Once and thread parking sleep under contention instead of
//! spinning a yield loop. The wait queue is keyed on (thread group,
//! address): the threads of one capsule share a futex, other capsules stay
//! isolated. A waiter's sleep is capped at `SAFETY_MS`, so a wake that races
//! the enqueue costs a little latency, never a lost wakeup: the std side
//! rechecks the atomic after every return, so the cap is a safety net, not a
//! correctness dependency.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use crate::process::current_pid;

const SAFETY_MS: u64 = 20;

static FUTEX_QUEUE: spin::Mutex<BTreeMap<(u32, u64), Vec<u32>>> = spin::Mutex::new(BTreeMap::new());

fn tgid_of(pid: u32) -> u32 {
    crate::process::nonos_core::PROCESS_TABLE
        .find_by_pid(pid)
        .map(|pcb| pcb.tgid.load(Ordering::Acquire))
        .unwrap_or(pid)
}

/// Block until the 32-bit word at `vaddr` no longer holds `expected`, a
/// waker signals it, or (with a nonzero `timeout_ms`) the timeout lapses.
/// Returns 0 in every non-error case; the caller rechecks the atomic and
/// re-waits if needed, which is why a spurious early return is harmless.
pub fn sys_futex_wait(vaddr: u64, expected: u32, timeout_ms: u64) -> i64 {
    let caller = current_pid().unwrap_or(0);
    if caller == 0 {
        return ERRNO_PERM;
    }
    if vaddr == 0 || (vaddr & 0x3) != 0 {
        return ERRNO_INVAL;
    }
    // Re-read the word under the kernel's eyes: if it already changed since
    // the caller's own check, a wake happened and we must not block.
    let mut buf = [0u8; 4];
    if crate::usercopy::copy_from_user(vaddr, &mut buf).is_err() {
        return ERRNO_FAULT;
    }
    if u32::from_ne_bytes(buf) != expected {
        return 0;
    }

    let key = (tgid_of(caller), vaddr);
    FUTEX_QUEUE.lock().entry(key).or_default().push(caller);

    let cap = if timeout_ms == 0 { SAFETY_MS } else { timeout_ms.min(SAFETY_MS) };
    let deadline = crate::time::timestamp_millis().saturating_add(cap.max(1));
    crate::sched::sleep_until(caller, deadline);
    crate::sched::yield_now();

    // Woken by a waker or the safety deadline. Drop our own entry if the
    // waker did not already remove it.
    let mut q = FUTEX_QUEUE.lock();
    if let Some(v) = q.get_mut(&key) {
        v.retain(|&p| p != caller);
        if v.is_empty() {
            q.remove(&key);
        }
    }
    0
}

/// Wake up to `count` waiters on `vaddr` (0 means all). Returns the number
/// woken.
pub fn sys_futex_wake(vaddr: u64, count: u64) -> i64 {
    let caller = current_pid().unwrap_or(0);
    if caller == 0 {
        return ERRNO_PERM;
    }
    let key = (tgid_of(caller), vaddr);
    // Collect the waiters, then release the queue lock before touching the
    // scheduler, so the futex lock never nests inside the run-queue lock.
    let to_wake: Vec<u32> = {
        let mut q = FUTEX_QUEUE.lock();
        match q.get_mut(&key) {
            Some(v) => {
                let n = if count == 0 { v.len() } else { (count as usize).min(v.len()) };
                let drained: Vec<u32> = v.drain(..n).collect();
                if v.is_empty() {
                    q.remove(&key);
                }
                drained
            }
            None => Vec::new(),
        }
    };
    let woke = to_wake.len() as i64;
    for pid in to_wake {
        crate::sched::wake_process(pid);
    }
    woke
}
