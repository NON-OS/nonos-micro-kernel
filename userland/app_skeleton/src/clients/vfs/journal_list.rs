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

use alloc::{string::String, vec, vec::Vec};

use crate::wire::{read_u32, read_u64, HDR_LEN};

// Clamped to match JOURNAL_LIST_MAX in the vfs handler: the full ring at a
// 255-byte path would overrun MAX_PAYLOAD_BYTES.
const JOURNAL_LIST_MAX: u32 = 200;
const ENTRY_MAX: usize = 8 + 1 + 255;

// Most recently accessed paths, newest first, as (atime_ms, path).
pub fn journal_list(owner_pid: u32, max: u32) -> Result<Vec<(u64, String)>, &'static str> {
    let max = max.min(JOURNAL_LIST_MAX);
    let port = super::resolve::vfs_port();
    let mut body = Vec::with_capacity(8);
    body.extend_from_slice(&owner_pid.to_le_bytes());
    body.extend_from_slice(&max.to_le_bytes());
    let mut rx = vec![0u8; HDR_LEN + 8 + JOURNAL_LIST_MAX as usize * ENTRY_MAX];
    let (status, total) =
        super::call::call(port, super::types::OP_JOURNAL_LIST, 24, &body, &mut rx)?;
    if status != 0 || total < HDR_LEN + 8 {
        return Err("vfs journal list failed");
    }
    let count = read_u32(&rx, HDR_LEN + 4)?;
    let mut out = Vec::new();
    let mut off = HDR_LEN + 8;
    for _ in 0..count {
        if off + 9 > total {
            return Err("vfs journal malformed");
        }
        let atime = read_u64(&rx, off)?;
        off += 8;
        let n = rx[off] as usize;
        off += 1;
        if off + n > total {
            return Err("vfs journal malformed");
        }
        let path = core::str::from_utf8(&rx[off..off + n]).map_err(|_| "vfs journal malformed")?;
        out.push((atime, String::from(path)));
        off += n;
    }
    Ok(out)
}
