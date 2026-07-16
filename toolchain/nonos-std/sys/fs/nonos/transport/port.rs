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

// Kernel lookups the VFS backend needs before it can talk: resolve the
// vfs_pool service port (MSVL), and read this process's id (MGPD) that the
// service keys open handles by.

use super::err::err;
use super::syscall::{sys3, sys5, tag4};
use crate::io;

pub(crate) fn vfs_port() -> io::Result<u32> {
    let name = b"vfs_pool";
    let mut port = 0u32;
    let mut owner = 0u32;
    let rc = unsafe {
        sys5(
            tag4(b"MSVL"),
            name.as_ptr() as u64,
            name.len() as u64,
            &mut port as *mut u32 as u64,
            &mut owner as *mut u32 as u64,
            0,
        )
    };
    if rc != 0 {
        return Err(err("vfs unavailable"));
    }
    Ok(port)
}

pub(crate) fn getpid() -> u32 {
    let r = unsafe { sys3(tag4(b"MGPD"), 0, 0, 0) };
    if r < 0 { 0 } else { r as u32 }
}
