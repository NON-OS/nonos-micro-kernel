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

//! `clone`, for threads only.

use nonos_libc::{mk_foreign_thread, ForeignFrame};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

const CLONE_VM: u64 = 0x100;
const CLONE_THREAD: u64 = 0x10000;

/// musl's `__clone` resumes the child at the instruction after its own
/// `syscall`, with rax zero and rsp pointing at the function and argument it
/// pushed.
pub fn clone(guest: &mut Guest, frame: &ForeignFrame) -> Answer {
    let a = frame.args();
    let (flags, stack, tls) = (a[0], a[1], a[4]);
    if flags & (CLONE_VM | CLONE_THREAD) != CLONE_VM | CLONE_THREAD {
        /*
         * A new process, not a thread. That is fork, and fork needs an
         * address space copy no peer call offers.
         */
        return Answer::value(errno::fail(errno::ENOSYS));
    }
    if frame.rip == 0 {
        /*
         * The kernel is not yet passing the guest's return address, so there
         * is nowhere correct to start the child.
         */
        return Answer::value(errno::fail(errno::ENOSYS));
    }
    if stack == 0 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    let tid = mk_foreign_thread(guest.pid, frame.rip, stack, tls);
    if tid < 0 {
        return Answer::value(errno::fail(errno::ENOMEM));
    }
    guest.threads.push(tid as u32);
    Answer::value(errno::ok(tid as u64))
}
