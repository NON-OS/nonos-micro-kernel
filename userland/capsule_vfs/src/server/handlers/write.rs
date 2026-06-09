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

use super::util::{map_store_err, split_caller};
use crate::protocol::{encode_response, Request, EINVAL, EMSGSIZE, MAX_DATA_BYTES, OP_WRITE};
use crate::store::Store;

// Payload: u32 caller_pid, u32 fd, data bytes.
pub fn write(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_WRITE, req.flags, req.request_id, s, &[]),
    };
    if rest.len() < 4 {
        return encode_response(OP_WRITE, req.flags, req.request_id, EINVAL, &[]);
    }
    let fd = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
    let data = &rest[4..];
    if data.len() > MAX_DATA_BYTES as usize {
        return encode_response(OP_WRITE, req.flags, req.request_id, EMSGSIZE, &[]);
    }
    match store.write(fd, pid, data) {
        Ok(n) => encode_response(OP_WRITE, req.flags, req.request_id, 0, &n.to_le_bytes()),
        Err(e) => encode_response(OP_WRITE, req.flags, req.request_id, map_store_err(e), &[]),
    }
}
