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


//! `pread64`: a read at an offset that leaves the descriptor's own
//! position alone, which is what a program doing its own seeking relies
//! on and the reason it uses this call instead of seek and read.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::read::read;

pub fn pread64(guest: &mut Guest, fd: u64, buf: u64, len: u64, at: u64) -> u64 {
    let saved = match guest.fds.get(fd as usize) {
        Some(entry) if entry.kind == Kind::File => entry.offset,
        Some(_) => return errno::fail(errno::ESPIPE),
        None => return errno::fail(errno::EBADF),
    };
    if let Some(entry) = guest.fds.get_mut(fd as usize) {
        entry.offset = at;
    }
    let out = read(guest, fd, buf, len);
    if let Some(entry) = guest.fds.get_mut(fd as usize) {
        entry.offset = saved;
    }
    out
}
