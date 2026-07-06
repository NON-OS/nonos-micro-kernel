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

// Store occupancy: (file_count, bytes_used, max_files). Lets a caller show how
// full the filesystem is.
pub fn usage(owner_pid: u32) -> Result<(u32, u64, u32), &'static str> {
    let peer = lookup_service(super::types::NAME).ok_or("vfs unavailable")?;
    let body = owner_pid.to_le_bytes();
    let mut rx = vec![0u8; HDR_LEN + 20];
    let (status, total) = super::call::call(peer.port, super::types::OP_USAGE, 14, &body, &mut rx)?;
    if status != 0 || total < HDR_LEN + 20 {
        return Err("vfs usage failed");
    }
    let files = read_u32(&rx, HDR_LEN + 4)?;
    let bytes = read_u64(&rx, HDR_LEN + 8)?;
    let max = read_u32(&rx, HDR_LEN + 16)?;
    Ok((files, bytes, max))
}
