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
use super::util::{map_blk_err, split_caller};
use crate::protocol::{encode_response, Request, EINVAL, ENOENT, MAX_PATH_BYTES, OP_STORE_PERSIST};
use crate::store::Store;

pub fn store_persist(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_STORE_PERSIST, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_STORE_PERSIST, req.flags, req.request_id, EINVAL, &[]);
    }
    let len = rest[0] as usize;
    if len == 0 || len > MAX_PATH_BYTES as usize || rest.len() < 1 + len {
        return encode_response(OP_STORE_PERSIST, req.flags, req.request_id, EINVAL, &[]);
    }
    let path = match str::from_utf8(&rest[1..1 + len]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_STORE_PERSIST, req.flags, req.request_id, EINVAL, &[]),
    };
    let path = normalize(path);
    let data = match store.bytes_of(&path) {
        Some(d) => d,
        None => return encode_response(OP_STORE_PERSIST, req.flags, req.request_id, ENOENT, &[]),
    };
    match crate::blk::store_write::append(&path, &data) {
        Ok(()) => encode_response(OP_STORE_PERSIST, req.flags, req.request_id, 0, &[]),
        Err(e) => {
            encode_response(OP_STORE_PERSIST, req.flags, req.request_id, map_blk_err(e), &[])
        }
    }
}
