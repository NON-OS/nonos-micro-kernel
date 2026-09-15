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
use crate::protocol::{encode_response, Request, EINVAL, MAX_PATH_BYTES, OP_DIRSTAT};
use crate::store::Store;

// Bounds the recursive walk so a large store cannot stall the synchronous
// reply; the caller is told when the numbers are a lower bound.
const DIRSTAT_MAX_NODES: usize = 20_000;

pub fn dirstat(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_DIRSTAT, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_DIRSTAT, req.flags, req.request_id, EINVAL, &[]);
    }
    let path_len = rest[0] as usize;
    if path_len == 0 || path_len > MAX_PATH_BYTES as usize || rest.len() < 1 + path_len {
        return encode_response(OP_DIRSTAT, req.flags, req.request_id, EINVAL, &[]);
    }
    let prefix = match str::from_utf8(&rest[1..1 + path_len]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_DIRSTAT, req.flags, req.request_id, EINVAL, &[]),
    };
    let prefix = normalize(prefix);
    let (files, dirs, bytes, truncated) = store.dirstat(&prefix, DIRSTAT_MAX_NODES);
    let mut body = Vec::with_capacity(20);
    body.extend_from_slice(&files.to_le_bytes());
    body.extend_from_slice(&dirs.to_le_bytes());
    body.extend_from_slice(&bytes.to_le_bytes());
    body.extend_from_slice(&(truncated as u32).to_le_bytes());
    encode_response(OP_DIRSTAT, req.flags, req.request_id, 0, &body)
}
