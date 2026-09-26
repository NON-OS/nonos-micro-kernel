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

//! `fork`.

use nonos_libc::{mk_foreign_fork, mk_foreign_resume};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::fork_copy::copy_spans;

pub fn fork(guest: &mut Guest) -> Answer {
    let child = mk_foreign_fork(guest.pid);
    if child < 0 {
        return Answer::value(errno::fail(errno::ENOMEM));
    }
    let child = child as u32;
    if !copy_spans(guest, child) {
        return Answer::value(errno::fail(errno::ENOMEM));
    }
    if mk_foreign_resume(child) < 0 {
        return Answer::value(errno::fail(errno::ENOMEM));
    }
    guest.children.push(child);
    Answer::value(errno::ok(child as u64))
}
