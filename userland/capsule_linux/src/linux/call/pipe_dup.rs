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

//! `dup` and `dup2`: a second descriptor onto the same thing.

use crate::linux::abi::errno;
use crate::linux::file::install;
use crate::linux::guest::{Fd, Guest, Kind};

/// `dup2` puts the copy at a number the caller chose, which is how a
/// shell wires a pipe onto stdout before it runs a command.
pub fn dup2(guest: &mut Guest, from: u64, to: u64) -> u64 {
    let Some(source) = guest.fds.get(from as usize).filter(|f| f.is_open()).map(Fd::clone_of)
    else {
        return errno::fail(errno::EBADF);
    };
    if from == to {
        return errno::ok(to);
    }
    while guest.fds.len() <= to as usize {
        guest.fds.push(Fd::empty(Kind::Free));
    }
    guest.fds[to as usize] = source;
    errno::ok(to)
}

pub fn dup(guest: &mut Guest, from: u64) -> u64 {
    let Some(source) = guest.fds.get(from as usize).filter(|f| f.is_open()).map(Fd::clone_of)
    else {
        return errno::fail(errno::EBADF);
    };
    match install(guest, source) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}
