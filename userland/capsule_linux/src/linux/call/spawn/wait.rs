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

//! `wait4`: which of this guest's children has ended.

use nonos_libc::mk_pid_alive;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// Set by a caller that will not wait.
const WNOHANG: u64 = 1;

pub fn wait4(guest: &mut Guest, want: u64, status: u64, flags: u64) -> Answer {
    if guest.children.is_empty() {
        return Answer::value(errno::fail(errno::ECHILD));
    }
    let gone = guest.children.iter().copied().find(|pid| {
        (want as i64) <= 0 || want as u32 == *pid
    }).filter(|pid| !mk_pid_alive(*pid));
    let Some(pid) = gone else {
        let _ = flags & WNOHANG;
        return Answer::value(errno::fail(errno::EAGAIN));
    };
    guest.children.retain(|p| *p != pid);
    /*
     * The exit code a guest passed to exit is not readable from here: the
     * kernel records it and nothing hands it back.
     */
    if status != 0 && guest.write(status, &0u32.to_le_bytes()) < 4 {
        return Answer::value(errno::fail(errno::EFAULT));
    }
    Answer::value(errno::ok(pid as u64))
}
