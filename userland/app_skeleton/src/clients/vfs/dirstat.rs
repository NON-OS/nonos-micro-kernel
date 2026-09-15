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
use alloc::vec::Vec;

use crate::wire::{read_u32, read_u64, HDR_LEN};

// Recursive occupancy under a prefix: (file_count, dir_count, total_bytes,
// truncated). `truncated` means the server hit its walk cap and the counts are
// a lower bound.
pub fn dirstat(owner_pid: u32, prefix: &[u8]) -> Result<(u32, u32, u64, bool), &'static str> {
    if prefix.is_empty() || prefix.len() > 255 {
        return Err("vfs path invalid");
    }
    let port = super::resolve::vfs_port();
    let mut body = Vec::with_capacity(5 + prefix.len());
    body.extend_from_slice(&owner_pid.to_le_bytes());
    body.push(prefix.len() as u8);
    body.extend_from_slice(prefix);
    let mut rx = vec![0u8; HDR_LEN + 24];
    let (status, total) = super::call::call(port, super::types::OP_DIRSTAT, 22, &body, &mut rx)?;
    if status != 0 || total < HDR_LEN + 24 {
        return Err("vfs dirstat failed");
    }
    let files = read_u32(&rx, HDR_LEN + 4)?;
    let dirs = read_u32(&rx, HDR_LEN + 8)?;
    let bytes = read_u64(&rx, HDR_LEN + 12)?;
    let truncated = read_u32(&rx, HDR_LEN + 20)?;
    Ok((files, dirs, bytes, truncated != 0))
}
