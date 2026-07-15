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

use super::consts::{DEFAULT_WAIT_MS, EFAULT, EPERM};
use crate::kernel_core::surface_registry::{arm_input_waiter, clear_input_waiter, input_seq};
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::SyscallResult;
use crate::usercopy::{validate_user_write, write_user_value};

pub(super) fn do_wait(last_seq: u64, timeout_ms: u64, out_ptr: u64) -> SyscallResult {
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
