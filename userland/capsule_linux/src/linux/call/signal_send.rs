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

//! `kill` and `tkill`, for the guest's own threads and children.

use nonos_libc::mk_kill;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// The only signals the kernel can carry. Anything else is accepted as
/// a request it cannot honour rather than silently dropped.
const SIGKILL: u64 = 9;
const SIGTERM: u64 = 15;

pub fn kill(guest: &mut Guest, pid: u64, signo: u64) -> u64 {
    let target = pid as u32;
    /*
     * A guest may signal itself, its threads and its children, and nothing
     * else.
     */
    if !guest.owns(target) && !guest.children.contains(&target) {
        return errno::fail(errno::ESRCH);
    }
    if signo == 0 {
        return errno::ok(0);
    }
    if signo != SIGKILL && signo != SIGTERM {
        return errno::fail(errno::EINVAL);
    }
    /*
     * A thread that was parked in a futex has to be let out before it
     * can be collected; the reply is the wake.
     */
    guest.waits.retain(|(w, _)| *w != target);
    guest.threads.retain(|t| *t != target);
    match mk_kill(target as u64, signo) {
        n if n < 0 => errno::fail(errno::EPERM),
        _ => errno::ok(0),
    }
}
