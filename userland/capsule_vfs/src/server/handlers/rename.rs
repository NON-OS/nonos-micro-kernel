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
use crate::protocol::{encode_response, Request, EACCES, EINVAL, MAX_PATH_BYTES, OP_RENAME};
use crate::store::Store;

pub fn rename(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_RENAME, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_RENAME, req.flags, req.request_id, EINVAL, &[]);
    }
    let ol = rest[0] as usize;
    if ol == 0 || ol > MAX_PATH_BYTES as usize || rest.len() < 1 + ol + 1 {
        return encode_response(OP_RENAME, req.flags, req.request_id, EINVAL, &[]);
    }
    let after = &rest[1 + ol..];
    let nl = after[0] as usize;
    if nl == 0 || nl > MAX_PATH_BYTES as usize || after.len() < 1 + nl {
        return encode_response(OP_RENAME, req.flags, req.request_id, EINVAL, &[]);
    }
    let old = match str::from_utf8(&rest[1..1 + ol]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_RENAME, req.flags, req.request_id, EINVAL, &[]),
    };
    let new = match str::from_utf8(&after[1..1 + nl]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_RENAME, req.flags, req.request_id, EINVAL, &[]),
    };
    let old = normalize(old);
    let new = normalize(new);
    if is_read_only(&old) || is_read_only(&new) {
        return encode_response(OP_RENAME, req.flags, req.request_id, EACCES, &[]);
    }
    match store.rename(&old, &new) {
        Ok(()) => encode_response(OP_RENAME, req.flags, req.request_id, 0, &[]),
        Err(e) => encode_response(OP_RENAME, req.flags, req.request_id, map_store_err(e), &[]),
    }
}
