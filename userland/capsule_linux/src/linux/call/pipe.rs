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

//! `pipe2`.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use crate::linux::file::flags::O_CLOEXEC;
use crate::linux::file::install;

pub fn pipe2(guest: &mut Guest, out: u64, flags: u64) -> u64 {
    let buffer = guest.pipes.len() as u32;
    guest.pipes.push(Vec::new());
    let Some(read_end) = install(guest, Fd::pipe(buffer, false)) else {
        return errno::fail(errno::EMFILE);
    };
    let Some(write_end) = install(guest, Fd::pipe(buffer, true)) else {
        return errno::fail(errno::EMFILE);
    };
    if flags & O_CLOEXEC != 0 {
        for end in [read_end, write_end] {
            if let Some(fd) = guest.fds.get_mut(end as usize) {
                fd.cloexec = true;
            }
        }
    }
    let mut pair = [0u8; 8];
    pair[..4].copy_from_slice(&(read_end as u32).to_le_bytes());
    pair[4..].copy_from_slice(&(write_end as u32).to_le_bytes());
    if guest.write(out, &pair) < 8 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}

