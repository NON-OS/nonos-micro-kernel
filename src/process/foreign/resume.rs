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

//! Waking a guest on state it already holds.

use crate::process::core::claim_new;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};

/// Make a guest runnable on the state it already carries. Used by a
/// fork, whose child was born holding its parent's registers.
pub(super) fn resume(pid: u32) -> i64 {
    let has_state =
        crate::process::with_process(pid, |pcb| pcb.saved_user_context.lock().is_some());
    if has_state != Some(true) {
        return ERRNO_INVAL;
    }
    /*
     * The claim is the check. A fork started twice would otherwise put
     * the child on a run queue twice.
     */
    if !claim_new(pid) {
        return ERRNO_PERM;
    }
    crate::sched::add_to_run_queue(pid);
    0
}
