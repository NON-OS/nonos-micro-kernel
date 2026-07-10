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

//! Create an empty file. Open it with O_CREATE, then close the descriptor the
//! server hands back so the store does not leak a handle for a file we only
//! wanted to bring into existence.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::call::call;
use super::constants::{HDR_LEN, OP_CLOSE, OP_OPEN, O_CREATE};
use super::path;

pub fn create_file(path: &[u8]) -> bool {
    if !path::is_valid(path) {
        return false;
    }
    match open_created(path) {
        Some(fd) => close(fd),
        None => false,
    }
}

/// Open the path with O_CREATE and return the new descriptor on success.
fn open_created(path: &[u8]) -> Option<u32> {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(9 + path.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    body.extend_from_slice(&O_CREATE.to_le_bytes());
    let mut rx = vec![0u8; 64];
    let total = call(OP_OPEN, &body, &mut rx)?;
    if total < HDR_LEN + 8 {
        return None;
    }
    Some(u32::from_le_bytes([rx[HDR_LEN + 4], rx[HDR_LEN + 5], rx[HDR_LEN + 6], rx[HDR_LEN + 7]]))
}

/// Release a descriptor returned by open.
fn close(fd: u32) -> bool {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(8);
    body.extend_from_slice(&pid.to_le_bytes());
    body.extend_from_slice(&fd.to_le_bytes());
    let mut rx = vec![0u8; 64];
    call(OP_CLOSE, &body, &mut rx).is_some()
}
