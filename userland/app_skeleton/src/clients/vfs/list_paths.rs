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

use crate::discover::lookup_service;
use crate::wire::HDR_LEN;

// The vfs caps a listing at MAX_LIST_BYTES; the receive buffer must hold the
// full reply (status word + that body) or a large directory overflows it and
// entries are silently dropped.
const MAX_LIST_BYTES: usize = 65536;

pub fn list_paths(owner_pid: u32, prefix: &[u8]) -> Result<Vec<String>, &'static str> {
    // The length prefix is a single byte, so a longer path would truncate and
    // the server would parse a different path than intended.
    if prefix.len() > 255 {
        return Err("vfs path invalid");
    }
    let peer = lookup_service(super::types::NAME).ok_or("vfs unavailable")?;
    let mut body = Vec::with_capacity(5 + prefix.len());
    body.extend_from_slice(&owner_pid.to_le_bytes());
    body.push(prefix.len() as u8);
    body.extend_from_slice(prefix);
    let mut rx = vec![0u8; HDR_LEN + 4 + MAX_LIST_BYTES];
    let (status, total) = super::call::call(peer.port, super::types::OP_LIST, 1, &body, &mut rx)?;
    if status != 0 {
        return Err("vfs list failed");
    }
    let mut out = Vec::new();
    let mut off = HDR_LEN + 4;
    while off < total {
        let n = rx[off] as usize;
        off += 1;
        if off + n > total {
            return Err("vfs list malformed");
        }
        let name = core::str::from_utf8(&rx[off..off + n]).map_err(|_| "vfs list malformed")?;
        out.push(String::from(name));
        off += n;
    }
    Ok(out)
}
