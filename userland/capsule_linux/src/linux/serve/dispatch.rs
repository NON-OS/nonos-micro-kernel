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


//! One refused call, answered. The two that can leave a caller parked are
//! taken first; everything else is a plain value.

use nonos_libc::ForeignFrame;

use super::answer::Answer;
use super::table::plain;
use crate::linux::abi::nr;
use crate::linux::call::{clone, exit_thread, futex};
use crate::linux::guest::Guest;

pub fn answer(guest: &mut Guest, frame: &ForeignFrame) -> Answer {
    let a = frame.args();
    match frame.nr {
        nr::CLONE => clone(guest, frame),
        nr::FORK | nr::VFORK => crate::linux::call::fork(guest),
        nr::EXECVE => crate::linux::call::execve(guest, frame.pid, a[0], a[1], a[2]),
        nr::WAIT4 => crate::linux::call::wait4(guest, a[0], a[1], a[2]),
        // A thread exiting is not the process exiting.
        nr::EXIT if frame.pid != guest.pid => Answer::Reply(exit_thread(guest, frame.pid)),
        nr::FUTEX => futex(guest, frame.pid, a[0], a[1], a[2]),
        other => Answer::Reply(plain(guest, frame.pid, other, a)),
    }
}
