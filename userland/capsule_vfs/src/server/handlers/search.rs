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

use alloc::vec::Vec;
use core::str;

use super::util::split_caller;
use crate::protocol::{encode_response, Request, EINVAL, OP_SEARCH};
use crate::store::Store;

// Clamped on both sides: 200 hits at a 255-byte path stays inside
// MAX_PAYLOAD_BYTES, the full 512 would not.
const SEARCH_MAX_HITS: usize = 200;

pub fn search(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_SEARCH, req.flags, req.request_id, s, &[]),
    };
    if rest.len() < 9 {
        return encode_response(OP_SEARCH, req.flags, req.request_id, EINVAL, &[]);
    }
    let flags = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
    let max_hits = u32::from_le_bytes([rest[4], rest[5], rest[6], rest[7]]) as usize;
    let n = rest[8] as usize;
    if n == 0 || rest.len() < 9 + n {
        return encode_response(OP_SEARCH, req.flags, req.request_id, EINVAL, &[]);
    }
    let query = match str::from_utf8(&rest[9..9 + n]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_SEARCH, req.flags, req.request_id, EINVAL, &[]),
    };
    let hits = store.search(query, flags, max_hits.min(SEARCH_MAX_HITS));
    let mut body = Vec::new();
    body.extend_from_slice(&(hits.len() as u32).to_le_bytes());
    for (kind, line, path) in hits.iter() {
        body.extend_from_slice(&kind.to_le_bytes());
        body.extend_from_slice(&line.to_le_bytes());
        body.push(path.len() as u8);
        body.extend_from_slice(path.as_bytes());
    }
    encode_response(OP_SEARCH, req.flags, req.request_id, 0, &body)
}
