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

use core::mem::size_of;

use crate::hardware::broker::IrqError;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use crate::usercopy::{validate_user_write, write_user_value};

use super::out::IrqPollOut;

// Poll only exposes the caller-owned grant's pending state; the
// broker's ownership check prevents cross-pid leak.
pub fn sys_irq_poll(grant_id: u64, out_ptr: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    if out_ptr == 0 {
        return ERRNO_FAULT;
    }
    if validate_user_write(out_ptr, size_of::<IrqPollOut>()).is_err() {
        return ERRNO_FAULT;
    }
    let res = match crate::hardware::broker::irq_poll(pid, grant_id) {
        Ok(r) => r,
        Err(IrqError::NotHolder) => return ERRNO_PERM,
        Err(IrqError::UnknownGrant) => return ERRNO_INVAL,
        Err(IrqError::PlatformError) => return ERRNO_NODEV,
    };
    let out = IrqPollOut { seq: res.seq, overflow: res.overflow };
    if write_user_value(out_ptr, &out).is_err() {
        return ERRNO_FAULT;
    }
    0
}
