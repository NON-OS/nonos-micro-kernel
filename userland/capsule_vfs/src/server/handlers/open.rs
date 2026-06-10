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
use crate::protocol::{
    encode_response, Request, EINVAL, MAX_PATH_BYTES, OP_OPEN, O_APPEND, O_CREATE, O_TRUNC,
};
use crate::store::Store;

// Payload: u32 caller_pid, u8 path_len, path bytes, u32 flags.
pub fn open(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_OPEN, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_OPEN, req.flags, req.request_id, EINVAL, &[]);
    }
    let path_len = rest[0] as usize;
    if path_len == 0 || path_len > MAX_PATH_BYTES as usize {
        return encode_response(OP_OPEN, req.flags, req.request_id, EINVAL, &[]);
    }
    if rest.len() < 1 + path_len + 4 {
        return encode_response(OP_OPEN, req.flags, req.request_id, EINVAL, &[]);
    }
    let path_bytes = &rest[1..1 + path_len];
    let path = match str::from_utf8(path_bytes) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_OPEN, req.flags, req.request_id, EINVAL, &[]),
    };
    let flags_off = 1 + path_len;
    let flags = u32::from_le_bytes([
        rest[flags_off],
        rest[flags_off + 1],
        rest[flags_off + 2],
        rest[flags_off + 3],
    ]);
    let create = flags & O_CREATE != 0;
    let truncate = flags & O_TRUNC != 0;
    let append = flags & O_APPEND != 0;
    match store.open(path, pid, create, truncate, append) {
        Ok(fd) => encode_response(OP_OPEN, req.flags, req.request_id, 0, &fd.to_le_bytes()),
        Err(e) => encode_response(OP_OPEN, req.flags, req.request_id, map_store_err(e), &[]),
    }
}
