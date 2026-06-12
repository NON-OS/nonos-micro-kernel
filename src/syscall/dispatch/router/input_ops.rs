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

use crate::kernel_core::surface_registry::{
    arm_input_waiter, clear_input_waiter, drain_input, input_seq, post_input, InputEvent,
};
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::numbers::SyscallNumber;
use crate::syscall::SyscallResult;
use crate::usercopy::{copy_to_user, read_user_value, validate_user_write, write_user_value};

const EINVAL: i32 = 22;
const EFAULT: i32 = 14;
const EPERM: i32 = 1;
const ENOTSUP: i32 = 95;
const ENOMEM: i32 = 12;
const MAX_DRAIN: usize = 64;
const DEFAULT_WAIT_MS: u64 = 50;

pub(super) fn handle(
    nr: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
) -> SyscallResult {
    match nr {
        SyscallNumber::MkInputEventPost => do_post(a0),
        SyscallNumber::MkInputEventDrain => do_drain(a0, a1),
        SyscallNumber::MkInputEventWait => do_wait(a0, a1, a2),
        _ => errno(ENOTSUP),
    }
}

fn do_post(ev_ptr: u64) -> SyscallResult {
    let ev: InputEvent = match read_user_value(ev_ptr) {
        Ok(v) => v,
        Err(_) => return errno(EFAULT),
    };
    match post_input(ev) {
        Ok(()) => SyscallResult::success_audited(0),
        Err(_) => errno(ENOMEM),
    }
}

fn do_drain(out_ptr: u64, max_events: u64) -> SyscallResult {
    if out_ptr == 0 || max_events == 0 {
        return errno(EINVAL);
    }
    let cap = core::cmp::min(max_events as usize, MAX_DRAIN);
    let mut scratch = [InputEvent::default(); MAX_DRAIN];
    let n = drain_input(&mut scratch[..cap]);
    if n == 0 {
        return SyscallResult::success_audited(0);
    }
    let bytes = n * core::mem::size_of::<InputEvent>();
    let src = unsafe { core::slice::from_raw_parts(scratch.as_ptr() as *const u8, bytes) };
    if copy_to_user(out_ptr, src).is_err() {
        return errno(EFAULT);
    }
    SyscallResult::success_audited(n as i64)
}

fn do_wait(last_seq: u64, timeout_ms: u64, out_ptr: u64) -> SyscallResult {
    if out_ptr == 0 || validate_user_write(out_ptr, core::mem::size_of::<u64>()).is_err() {
        return errno(EFAULT);
    }
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(EPERM),
    };
    let start = crate::time::timestamp_millis();
    loop {
        arm_input_waiter(pid);
        let seq = input_seq();
        let elapsed = crate::time::timestamp_millis().saturating_sub(start);
        if seq != last_seq || (timeout_ms > 0 && elapsed >= timeout_ms) {
            clear_input_waiter();
            if write_user_value(out_ptr, &seq).is_err() {
                return errno(EFAULT);
            }
            return SyscallResult::success_audited(0);
        }
        let deadline = if timeout_ms == 0 {
            crate::time::timestamp_millis().saturating_add(DEFAULT_WAIT_MS)
        } else {
            start.saturating_add(timeout_ms)
        };
        crate::sched::sleep_until(pid, deadline);
        crate::sched::yield_now();
    }
}
