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

// Single-shot blocking wait. `grant_id == 0` waits on every grant
// the caller owns; nonzero waits on that grant alone. Returns when
// the combined seq moves past `last_seq`, the timeout lapses, or
// any other wake lands (IPC sends wake the pid too) — spurious
// returns are part of the contract, callers re-issue the wait.
// `out_ptr` receives the seq to pass back as the next `last_seq`.

use core::mem::size_of;

use crate::hardware::broker::IrqError;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use crate::usercopy::{validate_user_write, write_user_value};

const DEFAULT_WAIT_MS: u64 = 100;

fn errno_for(e: IrqError) -> i64 {
    match e {
        IrqError::NotHolder => ERRNO_PERM,
        IrqError::UnknownGrant => ERRNO_INVAL,
        IrqError::PlatformError => ERRNO_NODEV,
    }
}

pub fn sys_irq_wait(grant_id: u64, last_seq: u64, timeout_ms: u64, out_ptr: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    if out_ptr == 0 || validate_user_write(out_ptr, size_of::<u64>()).is_err() {
        return ERRNO_FAULT;
    }
    // Token before arming: an interrupt that fires after the arm bumps the
    // caller's wake generation, and the guarded sleep below refuses to sleep
    // through it. Without the token that wake lands on a still-Running
    // process as a no-op, and the caller sleeps out its whole slice with the
    // completion already posted — the lost-wakeup shape again, costing a full
    // timeout per interrupt that races the arm.
    let token = crate::sched::wake_token(pid);
    let armed = match crate::hardware::broker::irq_wait_arm(pid, grant_id) {
        Ok(s) => s,
        Err(e) => return errno_for(e),
    };
    if armed == last_seq {
        let wait = if timeout_ms == 0 { DEFAULT_WAIT_MS } else { timeout_ms };
        let deadline = crate::time::timestamp_millis().saturating_add(wait);
        crate::sched::sleep_until_unless_woken(pid, deadline, token);
        crate::sched::yield_now();
    }
    let current = match crate::hardware::broker::irq_wait_disarm(pid, grant_id) {
        Ok(s) => s,
        Err(_) => armed,
    };
    if write_user_value(out_ptr, &current).is_err() {
        return ERRNO_FAULT;
    }
    0
}
