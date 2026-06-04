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

use alloc::vec;

use crate::discover::lookup_service;
use crate::wire::{read_u32, read_u64, HDR_LEN};

pub fn stat(owner_pid: u32, path: &[u8]) -> Result<(u64, bool), &'static str> {
    if path.is_empty() || path.len() > 255 {
        return Err("vfs path invalid");
    }
    let peer = lookup_service(super::types::NAME).ok_or("vfs unavailable")?;
    let mut body = alloc::vec::Vec::with_capacity(5 + path.len());
    body.extend_from_slice(&owner_pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    let mut rx = vec![0u8; HDR_LEN + 16];
    let (status, total) = super::call::call(peer.port, super::types::OP_STAT, 5, &body, &mut rx)?;
    if status != 0 || total < HDR_LEN + 16 {
        return Err("vfs stat failed");
    }
    let size = read_u64(&rx, HDR_LEN + 4)?;
    let flags = read_u32(&rx, HDR_LEN + 12)?;
    Ok((size, flags & 1 != 0))
}
