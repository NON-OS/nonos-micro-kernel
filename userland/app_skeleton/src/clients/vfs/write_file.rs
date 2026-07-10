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

// Bytes written per request. The vfs caps a write payload at MAX_DATA_BYTES and
// the whole request (caller pid + fd + data) at MAX_PAYLOAD_BYTES, so the data
// slice stays comfortably under 64 KiB; a larger file is sent across several
// writes whose positions the server advances on the open fd.
const CHUNK: usize = 60 * 1024;

pub fn write_file(owner_pid: u32, path: &[u8], data: &[u8]) -> Result<(), &'static str> {
    if path.is_empty() || path.len() > 255 {
        return Err("vfs path invalid");
    }
    let port = super::resolve::vfs_port();
    let mut open = Vec::with_capacity(9 + path.len());
    open.extend_from_slice(&owner_pid.to_le_bytes());
    open.push(path.len() as u8);
    open.extend_from_slice(path);
    open.extend_from_slice(&(super::types::O_CREATE | super::types::O_TRUNC).to_le_bytes());
    let mut rx = vec![0u8; HDR_LEN + 8];
    let (status, total) = super::call::call(port, super::types::OP_OPEN, 5, &open, &mut rx)?;
    if status != 0 || total < HDR_LEN + 8 {
        return Err("vfs open failed");
    }
    let fd = read_u32(&rx, HDR_LEN + 4)?;
    let result = write_all(port, owner_pid, fd, data, &mut rx);
    let mut close = [0u8; 8];
    close[..4].copy_from_slice(&owner_pid.to_le_bytes());
    close[4..8].copy_from_slice(&fd.to_le_bytes());
    let _ = super::call::call(port, super::types::OP_CLOSE, 7, &close, &mut rx);
    result
}

// Send the whole buffer to the open fd one chunk at a time, advancing across
// the file as the server tracks the fd position. An O_TRUNC open already sized
// the file to zero, so an empty buffer writes nothing and succeeds.
fn write_all(
    port: u32,
    owner_pid: u32,
    fd: u32,
    data: &[u8],
    rx: &mut [u8],
) -> Result<(), &'static str> {
    let mut sent = 0;
    while sent < data.len() {
        let end = (sent + CHUNK).min(data.len());
        let chunk = &data[sent..end];
        let mut write = Vec::with_capacity(8 + chunk.len());
        write.extend_from_slice(&owner_pid.to_le_bytes());
        write.extend_from_slice(&fd.to_le_bytes());
        write.extend_from_slice(chunk);
        let (status, total) = super::call::call(port, super::types::OP_WRITE, 6, &write, rx)?;
        if status != 0 || total < HDR_LEN + 8 {
            return Err(super::errmsg::errmsg(status));
        }
        let wrote = read_u32(rx, HDR_LEN + 4)? as usize;
        if wrote != chunk.len() {
            return Err("vfs short write");
        }
        sent += wrote;
    }
    Ok(())
}
