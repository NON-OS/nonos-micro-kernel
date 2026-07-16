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

// Issue one VFS request and return its reply: frame the body, send it over
// the pool port with MICL, check the status word, and hand back the raw
// reply bytes. read_u32 is the little-endian field reader replies parse with.

use super::consts::{BODY_OFF, STATUS_OFF};
use super::err::{err, vfs_err};
use super::frame::frame;
use super::syscall::{sys5, tag4};
use crate::io;
use crate::vec::Vec;

pub(crate) fn call(port: u32, op: u16, body: &[u8], reply_cap: usize) -> io::Result<Vec<u8>> {
    let tx = frame(op, body);
    let mut rx = crate::vec![0u8; BODY_OFF + reply_cap];
    let n = unsafe {
        sys5(
            tag4(b"MICL"),
            port as u64,
            tx.as_ptr() as u64,
            tx.len() as u64,
            rx.as_mut_ptr() as u64,
            rx.len() as u64,
        )
    };
    if n < (STATUS_OFF + 4) as i64 {
        return Err(err("vfs ipc failed"));
    }
    let status = i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]);
    if status != 0 {
        return Err(vfs_err(status));
    }
    rx.truncate(n as usize);
    Ok(rx)
}

pub(crate) fn read_u32(b: &[u8], off: usize) -> u32 {
    if b.len() < off + 4 {
        return 0;
    }
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}
