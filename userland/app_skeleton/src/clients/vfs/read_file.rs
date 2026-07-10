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

use crate::wire::{read_u32, HDR_LEN};

// The vfs caps each read at MAX_DATA_BYTES, so a large file is pulled in
// chunks: read up to CHUNK bytes per call, advancing the server-side file
// offset, until a short read signals end of file or max_bytes is reached.
const CHUNK: u32 = 65536;

pub fn read_file(owner_pid: u32, path: &[u8], max_bytes: u32) -> Result<Vec<u8>, &'static str> {
    if path.is_empty() || path.len() > 255 {
        return Err("vfs path invalid");
    }
    let port = super::resolve::vfs_port();
    let mut open = Vec::with_capacity(9 + path.len());
    open.extend_from_slice(&owner_pid.to_le_bytes());
    open.push(path.len() as u8);
    open.extend_from_slice(path);
    open.extend_from_slice(&0u32.to_le_bytes());
    let mut rx = vec![0u8; HDR_LEN + 4 + CHUNK as usize];
    let (status, total) = super::call::call(port, super::types::OP_OPEN, 2, &open, &mut rx)?;
    if status != 0 || total < HDR_LEN + 8 {
        return Err("vfs open failed");
    }
    let fd = read_u32(&rx, HDR_LEN + 4)?;

    let mut out: Vec<u8> = Vec::new();
    let mut err: Option<&'static str> = None;
    loop {
        if out.len() as u32 >= max_bytes {
            break;
        }
        let want = core::cmp::min(CHUNK, max_bytes - out.len() as u32);
        let mut read = [0u8; 12];
        read[..4].copy_from_slice(&owner_pid.to_le_bytes());
        read[4..8].copy_from_slice(&fd.to_le_bytes());
        read[8..12].copy_from_slice(&want.to_le_bytes());
        let (status, total) =
            match super::call::call(port, super::types::OP_READ, 3, &read, &mut rx) {
                Ok(v) => v,
                Err(e) => {
                    err = Some(e);
                    break;
                }
            };
        if status != 0 || total < HDR_LEN + 4 {
            err = Some("vfs read failed");
            break;
        }
        let n = total - (HDR_LEN + 4);
        out.extend_from_slice(&rx[HDR_LEN + 4..total]);
        if (n as u32) < want {
            break;
        }
    }

    let mut close = [0u8; 8];
    close[..4].copy_from_slice(&owner_pid.to_le_bytes());
    close[4..8].copy_from_slice(&fd.to_le_bytes());
    let _ = super::call::call(port, super::types::OP_CLOSE, 4, &close, &mut rx);

    match err {
        Some(e) => Err(e),
        None => Ok(out),
    }
}
