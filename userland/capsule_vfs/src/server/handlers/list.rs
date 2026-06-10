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
use crate::protocol::{
    encode_response, Request, EINVAL, MAX_LIST_BYTES, MAX_PATH_BYTES, OP_LIST,
};
use crate::store::Store;

// Payload: u32 caller_pid, u8 prefix_len, prefix bytes.
// Reply body: concatenated `<u8 name_len><name bytes>` entries, capped
// at MAX_LIST_BYTES.
pub fn list(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_LIST, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_LIST, req.flags, req.request_id, EINVAL, &[]);
    }
    let prefix_len = rest[0] as usize;
    if prefix_len > MAX_PATH_BYTES as usize {
        return encode_response(OP_LIST, req.flags, req.request_id, EINVAL, &[]);
    }
    if rest.len() < 1 + prefix_len {
        return encode_response(OP_LIST, req.flags, req.request_id, EINVAL, &[]);
    }
    let prefix = match str::from_utf8(&rest[1..1 + prefix_len]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_LIST, req.flags, req.request_id, EINVAL, &[]),
    };
    let body: Vec<u8> = store.list(prefix, MAX_LIST_BYTES as usize);
    encode_response(OP_LIST, req.flags, req.request_id, 0, &body)
}
