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

//! What happens to the other threads when one of them calls `execve`.

use nonos_libc::{mk_foreign_reply, mk_kill};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const SIGKILL: u64 = 9;

pub fn reap(guest: &mut Guest, caller: u32) {
    for tid in core::mem::take(&mut guest.threads) {
        if tid == caller {
            guest.threads.push(tid);
            continue;
        }
        /*
         * A thread parked in a futex has to be let out of the kernel before it
         * can be ended: the kill marks it, but nothing collects a thread that
         * is still waiting for an answer.
         */
        guest.waits.retain(|(w, _)| *w != tid);
        let _ = mk_foreign_reply(tid, errno::fail(errno::EINTR));
        let _ = mk_kill(tid as u64, SIGKILL);
    }
}
