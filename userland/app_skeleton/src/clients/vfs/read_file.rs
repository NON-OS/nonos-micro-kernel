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

use crate::discover::lookup_service;
use crate::wire::HDR_LEN;

pub fn read_file(owner_pid: u32, path: &[u8], max_bytes: u32) -> Result<Vec<u8>, &'static str> {
    let peer = lookup_service(super::types::NAME).ok_or("vfs unavailable")?;
    let mut open = Vec::with_capacity(9 + path.len());
    open.extend_from_slice(&owner_pid.to_le_bytes());
    open.push(path.len() as u8);
    open.extend_from_slice(path);
    open.extend_from_slice(&0u32.to_le_bytes());
    let mut rx = vec![0u8; HDR_LEN + 4 + max_bytes as usize];
    let (status, total) = super::call::call(peer.port, super::types::OP_OPEN, 2, &open, &mut rx)?;
    if status != 0 || total < HDR_LEN + 8 {
        return Err("vfs open failed");
    }
    let fd = u32::from_le_bytes(rx[HDR_LEN + 4..HDR_LEN + 8].try_into().unwrap());
    let mut read = Vec::with_capacity(12);
    read.extend_from_slice(&owner_pid.to_le_bytes());
    read.extend_from_slice(&fd.to_le_bytes());
    read.extend_from_slice(&max_bytes.to_le_bytes());
    let (status, total) = super::call::call(peer.port, super::types::OP_READ, 3, &read, &mut rx)?;
    let mut close = [0u8; 8];
    close[..4].copy_from_slice(&owner_pid.to_le_bytes());
    close[4..8].copy_from_slice(&fd.to_le_bytes());
    let _ = super::call::call(peer.port, super::types::OP_CLOSE, 4, &close, &mut rx);
    if status != 0 {
        return Err("vfs read failed");
    }
    Ok(Vec::from(&rx[HDR_LEN + 4..total]))
}
