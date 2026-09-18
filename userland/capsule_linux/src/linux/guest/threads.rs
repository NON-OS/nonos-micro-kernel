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


//! The guest's threads, and the ones parked on a futex.

use nonos_libc::mk_foreign_reply;

use super::handle::Guest;

impl Guest {
    /// True for the guest and for every thread of it, which is what the
    /// serve loop needs: a trap arrives under the thread's own tid.
    pub fn owns(&self, pid: u32) -> bool {
        pid == self.pid || self.threads.contains(&pid)
    }

    /// Reply to at most `count` waiters on `uaddr` and report how many.
    /// The reply is the wake: each was left parked inside its own trap.
    pub fn wake(&mut self, uaddr: u64, count: u64) -> u64 {
        let mut woken = 0;
        let mut i = 0;
        while i < self.waits.len() && woken < count {
            if self.waits[i].1 != uaddr {
                i += 1;
                continue;
            }
            let (tid, _) = self.waits.remove(i);
            if mk_foreign_reply(tid, 0) >= 0 {
                woken += 1;
            }
        }
        woken
    }
}
