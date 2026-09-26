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

//! Giving a fresh guest its first user context.

use crate::kernel_core::process_spawn::{allocate_user_stack, setup_initial_user_context};
use crate::process::core::{claim_new, release_new};
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_NOMEM, ERRNO_PERM};

pub(super) fn install(pid: u32, entry: u64, rsp: u64) -> i64 {
    // Claimed before the context is built, not after.
    if !claim_new(pid) {
        return ERRNO_PERM;
    }
    let stack = match rsp {
        0 => match allocate_user_stack(pid) {
            Ok(top) => top,
            Err(_) => {
                release_new(pid);
                return ERRNO_NOMEM;
            }
        },
        given => given,
    };
    if setup_initial_user_context(pid, entry, stack).is_err() {
        release_new(pid);
        return ERRNO_FAULT;
    }
    crate::sched::add_to_run_queue(pid);
    0
}
