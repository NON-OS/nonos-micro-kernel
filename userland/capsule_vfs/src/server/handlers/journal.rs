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

use super::path::normalize;
use super::util::split_caller;
use crate::protocol::{
    encode_response, Request, EINVAL, MAX_PATH_BYTES, OP_JOURNAL_LIST, OP_JOURNAL_TOUCH,
};
use crate::store::Store;

// Clamped on both sides of the wire: JOURNAL_CAP entries at a 255-byte path
// would overrun MAX_PAYLOAD_BYTES, so server and client agree on 200.
const JOURNAL_LIST_MAX: usize = 200;

pub fn journal_touch(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_JOURNAL_TOUCH, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_JOURNAL_TOUCH, req.flags, req.request_id, EINVAL, &[]);
    }
    let n = rest[0] as usize;
    if n == 0 || n > MAX_PATH_BYTES as usize || rest.len() < 1 + n {
        return encode_response(OP_JOURNAL_TOUCH, req.flags, req.request_id, EINVAL, &[]);
    }
    let path = match str::from_utf8(&rest[1..1 + n]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_JOURNAL_TOUCH, req.flags, req.request_id, EINVAL, &[]),
    };
    store.journal_touch(&normalize(path));
    encode_response(OP_JOURNAL_TOUCH, req.flags, req.request_id, 0, &[])
}

pub fn journal_list(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_JOURNAL_LIST, req.flags, req.request_id, s, &[]),
    };
    if rest.len() < 4 {
        return encode_response(OP_JOURNAL_LIST, req.flags, req.request_id, EINVAL, &[]);
    }
    let max = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
    let listed = store.journal_list(max.min(JOURNAL_LIST_MAX));
    let mut body = Vec::new();
    body.extend_from_slice(&(listed.len() as u32).to_le_bytes());
    for (atime, path) in listed.iter() {
        body.extend_from_slice(&atime.to_le_bytes());
        body.push(path.len() as u8);
        body.extend_from_slice(path.as_bytes());
    }
    encode_response(OP_JOURNAL_LIST, req.flags, req.request_id, 0, &body)
}
