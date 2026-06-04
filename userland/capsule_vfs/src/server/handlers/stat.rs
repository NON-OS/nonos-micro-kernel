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

use super::util::{map_store_err, split_caller};
use crate::protocol::{encode_response, Request, EINVAL, MAX_PATH_BYTES, OP_STAT};
use crate::store::Store;

// Payload: u32 caller_pid, u8 path_len, path bytes.
// Reply body: u64 size, u32 flags (flags reserved for M3-2 routing).
pub fn stat(store: &mut Store, req: Request<'_>) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_STAT, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_STAT, req.flags, req.request_id, EINVAL, &[]);
    }
    let path_len = rest[0] as usize;
    if path_len == 0 || path_len > MAX_PATH_BYTES as usize {
        return encode_response(OP_STAT, req.flags, req.request_id, EINVAL, &[]);
    }
    if rest.len() < 1 + path_len {
        return encode_response(OP_STAT, req.flags, req.request_id, EINVAL, &[]);
    }
    let path = match str::from_utf8(&rest[1..1 + path_len]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_STAT, req.flags, req.request_id, EINVAL, &[]),
    };
    match store.stat(path) {
        Ok((size, is_dir)) => {
            let mut body = Vec::with_capacity(12);
            body.extend_from_slice(&size.to_le_bytes());
            let flags: u32 = if is_dir { 1 } else { 0 };
            body.extend_from_slice(&flags.to_le_bytes());
            encode_response(OP_STAT, req.flags, req.request_id, 0, &body)
        }
        Err(e) => encode_response(OP_STAT, req.flags, req.request_id, map_store_err(e), &[]),
    }
}
