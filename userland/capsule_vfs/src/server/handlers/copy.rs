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

use super::path::{is_read_only, normalize};
use super::util::{map_store_err, split_caller};
use crate::protocol::{encode_response, Request, EACCES, EINVAL, MAX_PATH_BYTES, OP_COPY};
use crate::store::Store;

// Payload: u32 caller_pid, u8 src_len, src, u8 dst_len, dst, u8 recursive. A
// non-zero recursive byte copies a directory's whole subtree.
pub fn copy(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_COPY, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_COPY, req.flags, req.request_id, EINVAL, &[]);
    }
    let sl = rest[0] as usize;
    if sl == 0 || sl > MAX_PATH_BYTES as usize || rest.len() < 1 + sl + 1 {
        return encode_response(OP_COPY, req.flags, req.request_id, EINVAL, &[]);
    }
    let after = &rest[1 + sl..];
    let dl = after[0] as usize;
    if dl == 0 || dl > MAX_PATH_BYTES as usize || after.len() < 1 + dl {
        return encode_response(OP_COPY, req.flags, req.request_id, EINVAL, &[]);
    }
    let src = match str::from_utf8(&rest[1..1 + sl]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_COPY, req.flags, req.request_id, EINVAL, &[]),
    };
    let dst = match str::from_utf8(&after[1..1 + dl]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_COPY, req.flags, req.request_id, EINVAL, &[]),
    };
    let recursive = after.get(1 + dl).is_some_and(|&b| b != 0);
    let src = normalize(src);
    let dst = normalize(dst);
    // Copying out of /capsules is fine; creating or overwriting inside it is not.
    if is_read_only(&dst) {
        return encode_response(OP_COPY, req.flags, req.request_id, EACCES, &[]);
    }
    match store.copy(&src, &dst, recursive) {
        Ok(_) => encode_response(OP_COPY, req.flags, req.request_id, 0, &[]),
        Err(e) => encode_response(OP_COPY, req.flags, req.request_id, map_store_err(e), &[]),
    }
}
