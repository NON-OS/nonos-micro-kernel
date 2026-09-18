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


//! `getdents64`. The guest walks the listing this capsule took when the
//! directory was opened, one bufferful at a time.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::dirent::{encode, record_len, DT_UNKNOWN};

pub fn getdents64(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize) else {
        return errno::fail(errno::EBADF);
    };
    if entry.kind != Kind::Dir {
        return errno::fail(errno::ENOTDIR);
    }
    let mut out: Vec<u8> = Vec::new();
    let mut at = entry.offset as usize;
    while at < entry.names.len() {
        let name = entry.names[at].as_bytes();
        if out.len() + record_len(name) > len as usize {
            break;
        }
        encode(&mut out, name, at as u64, DT_UNKNOWN);
        at += 1;
    }
    /*
     * A buffer too small for even the first entry is EINVAL, not zero:
     * zero means the directory is finished and a caller would stop.
     */
    if out.is_empty() {
        return match at < entry.names.len() {
            true => errno::fail(errno::EINVAL),
            false => errno::ok(0),
        };
    }
    if guest.write(buf, &out) < out.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    guest.fds[fd as usize].offset = at as u64;
    errno::ok(out.len() as u64)
}
