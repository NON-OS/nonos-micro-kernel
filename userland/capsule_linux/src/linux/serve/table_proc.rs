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

//! Who the guest is, what group it is in, and what time it thinks it is.

use crate::linux::abi::{errno, nr, nr_path as np};
use crate::linux::call;
use crate::linux::guest::Guest;

pub fn proc_ops(guest: &mut Guest, nr: u64, a: [u64; 6]) -> Option<u64> {
    Some(match nr {
        np::KILL | np::TKILL => call::kill(guest, a[0], a[1]),
        np::GETPPID => call::getppid(guest),
        np::SETPGID => call::setpgid(guest, a[0], a[1]),
        np::GETPGRP | np::GETPGID => call::getpgid(guest),
        np::SETSID => call::setsid(guest),
        np::GETSID => call::getsid(guest),
        np::SETUID | np::SETGID => call::setuid(a[0]),
        np::TIME => call::time(guest, a[0]),
        np::GETTIMEOFDAY => call::gettimeofday(guest, a[0]),
        np::NANOSLEEP | np::CLOCK_NANOSLEEP => call::nanosleep(guest, a[0]),
        // A guest yielding is the personality yielding: one slot.
        np::SCHED_YIELD => {
            nonos_libc::mk_yield();
            errno::ok(0)
        }
        nr::SET_TID_ADDRESS | nr::GETTID | nr::GETPID => errno::ok(guest.pid as u64),
        nr::GETUID | nr::GETEUID | nr::GETGID | nr::GETEGID => errno::ok(0),
        nr::CLOCK_GETTIME => call::clock_gettime(guest, a[0], a[1]),
        _ => return None,
    })
}
