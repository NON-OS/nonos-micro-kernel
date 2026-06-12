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

use alloc::{vec, vec::Vec};

use super::frame::{frame, BODY_OFF, STATUS_OFF};
use crate::io::{Error, ErrorKind, Result};

const VFS_NAME: &[u8] = b"vfs_pool";

pub(super) struct Vfs {
    pub port: u32,
    pub pid: u32,
}

pub(super) fn connect() -> Result<Vfs> {
    let mut port = 0u32;
    let mut owner = 0u32;
    let rc = nonos_libc::mk_service_lookup(VFS_NAME.as_ptr(), VFS_NAME.len(), &mut port, &mut owner);
    if rc != 0 {
        return Err(Error::new(ErrorKind::NotFound, "vfs unavailable"));
    }
    let pid = nonos_libc::mk_getpid();
    if pid == 0 {
        return Err(Error::new(ErrorKind::PermissionDenied, "no caller pid"));
    }
    Ok(Vfs { port, pid })
}

pub(super) fn call(port: u32, op: u16, body: &[u8], reply_cap: usize) -> Result<Vec<u8>> {
    let tx = frame(op, body);
    let mut rx = vec![0u8; BODY_OFF + reply_cap];
    let rc = nonos_libc::mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (STATUS_OFF + 4) as i64 {
        return Err(Error::new(ErrorKind::Other, "vfs ipc failed"));
    }
    let status =
        i32::from_le_bytes([rx[STATUS_OFF], rx[STATUS_OFF + 1], rx[STATUS_OFF + 2], rx[STATUS_OFF + 3]]);
    if status != 0 {
        return Err(Error::new(ErrorKind::Other, "vfs op rejected"));
    }
    rx.truncate(rc as usize);
    Ok(rx)
}
