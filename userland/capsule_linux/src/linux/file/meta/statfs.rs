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

//! How much room the store has, in the shape `statfs` expects.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// `struct statfs` on x86_64 is 120 bytes.
const STATFS: usize = 120;

/// The store addresses bytes, not blocks, so a block size is a fiction either
/// way.
const BSIZE: u64 = 1024;

pub fn statfs(guest: &Guest, out: u64) -> u64 {
    let Ok((_, bytes, max)) = vfs::usage(mk_getpid()) else {
        return errno::fail(errno::EIO);
    };
    // The vfs reports its ceiling as 32 bits and its usage as 64.
    let (used, max) = (bytes, u64::from(max));
    let total = max / BSIZE;
    let free = max.saturating_sub(used) / BSIZE;

    let mut buf = [0u8; STATFS];
    put(&mut buf, 0, 0x6E6F6E6F); // f_type, "nono"
    put(&mut buf, 8, BSIZE); // f_bsize
    put(&mut buf, 16, total); // f_blocks
    put(&mut buf, 24, free); // f_bfree
    put(&mut buf, 32, free); // f_bavail
    put(&mut buf, 56, 255); // f_namelen, the vfs path limit
    put(&mut buf, 64, BSIZE); // f_frsize
    match guest.write(out, &buf) {
        n if n < 0 => errno::fail(errno::EFAULT),
        _ => errno::ok(0),
    }
}

fn put(buf: &mut [u8; STATFS], at: usize, v: u64) {
    buf[at..at + 8].copy_from_slice(&v.to_le_bytes());
}
