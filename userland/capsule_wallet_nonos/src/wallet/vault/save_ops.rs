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

//! The four vfs_pool calls a save is made of.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::vfs::{call, HDR_LEN, OP_CLOSE, OP_MKDIR, OP_OPEN, OP_WRITE, O_CREATE, O_TRUNC};

/// Missing is the normal case on a fresh machine and already-there is the
/// normal case after that, so the answer is not checked.
pub(super) fn mkdir(path: &[u8]) {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(5 + path.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    let mut rx = vec![0u8; 64];
    let _ = call(OP_MKDIR, &body, &mut rx);
}

pub(super) fn open_created(path: &[u8]) -> Option<u32> {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(9 + path.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    body.extend_from_slice(&(O_CREATE | O_TRUNC).to_le_bytes());
    let mut rx = vec![0u8; 64];
    let total = call(OP_OPEN, &body, &mut rx).len()?;
    if total < HDR_LEN + 8 {
        return None;
    }
    Some(u32::from_le_bytes([rx[HDR_LEN + 4], rx[HDR_LEN + 5], rx[HDR_LEN + 6], rx[HDR_LEN + 7]]))
}

pub(super) fn write_all(fd: u32, data: &[u8]) -> bool {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(8 + data.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.extend_from_slice(&fd.to_le_bytes());
    body.extend_from_slice(data);
    let mut rx = vec![0u8; 64];
    call(OP_WRITE, &body, &mut rx).worked()
}

pub(super) fn close(fd: u32) {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(8);
    body.extend_from_slice(&pid.to_le_bytes());
    body.extend_from_slice(&fd.to_le_bytes());
    let mut rx = vec![0u8; 64];
    let _ = call(OP_CLOSE, &body, &mut rx);
}
