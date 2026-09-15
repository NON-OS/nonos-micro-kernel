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

use crate::wire::{read_u32, HDR_LEN};

// Matches SEARCH_MAX_HITS in the vfs handler; 200 hits at a 255-byte path is
// the largest reply that fits inside MAX_PAYLOAD_BYTES.
const SEARCH_MAX_HITS: u32 = 200;
const HIT_MAX: usize = 4 + 4 + 1 + 255;

pub const SEARCH_NAMES: u32 = 1;
pub const SEARCH_CONTENT: u32 = 2;
pub const SEARCH_CASE: u32 = 4;

// Store-wide search. Each hit is (kind, line, path); kind 0 is a file matched
// by name and kind 2 a directory matched by name, both with line 0; kind 1 is
// a content match with a 1-based line number.
pub fn search(
    owner_pid: u32,
    query: &[u8],
    flags: u32,
    max_hits: u32,
) -> Result<Vec<(u32, u32, String)>, &'static str> {
    if query.is_empty() || query.len() > 255 {
        return Err("vfs query invalid");
    }
    let max_hits = max_hits.min(SEARCH_MAX_HITS);
    let port = super::resolve::vfs_port();
    let mut body = Vec::with_capacity(13 + query.len());
    body.extend_from_slice(&owner_pid.to_le_bytes());
    body.extend_from_slice(&flags.to_le_bytes());
    body.extend_from_slice(&max_hits.to_le_bytes());
    body.push(query.len() as u8);
    body.extend_from_slice(query);
    let mut rx = vec![0u8; HDR_LEN + 8 + SEARCH_MAX_HITS as usize * HIT_MAX];
    let (status, total) = super::call::call(port, super::types::OP_SEARCH, 25, &body, &mut rx)?;
    if status != 0 || total < HDR_LEN + 8 {
        return Err("vfs search failed");
    }
    let count = read_u32(&rx, HDR_LEN + 4)?;
    let mut out = Vec::new();
    let mut off = HDR_LEN + 8;
    for _ in 0..count {
        if off + 9 > total {
            return Err("vfs search malformed");
        }
        let kind = read_u32(&rx, off)?;
        let line = read_u32(&rx, off + 4)?;
        off += 8;
        let n = rx[off] as usize;
        off += 1;
        if off + n > total {
            return Err("vfs search malformed");
        }
        let path = core::str::from_utf8(&rx[off..off + n]).map_err(|_| "vfs search malformed")?;
        out.push((kind, line, String::from(path)));
        off += n;
    }
    Ok(out)
}
