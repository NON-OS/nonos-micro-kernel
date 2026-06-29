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
use crate::protocol::{encode_response, Request, EINVAL, EMSGSIZE, MAX_DATA_BYTES, OP_PREAD};
use crate::store::Store;

pub fn pread(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_PREAD, req.flags, req.request_id, s, &[]),
    };
    if rest.len() != 16 {
        return encode_response(OP_PREAD, req.flags, req.request_id, EINVAL, &[]);
    }
    let fd = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
    let offset = i64::from_le_bytes([
        rest[4], rest[5], rest[6], rest[7],
        rest[8], rest[9], rest[10], rest[11],
    ]);
    let max = u32::from_le_bytes([rest[12], rest[13], rest[14], rest[15]]);
    if offset < 0 {
        return encode_response(OP_PREAD, req.flags, req.request_id, EINVAL, &[]);
    }
    if max > MAX_DATA_BYTES {
        return encode_response(OP_PREAD, req.flags, req.request_id, EMSGSIZE, &[]);
    }
    match store.pread(fd, pid, offset as u64, max as usize) {
        Ok(data) => encode_response(OP_PREAD, req.flags, req.request_id, 0, &data),
        Err(e) => encode_response(OP_PREAD, req.flags, req.request_id, map_store_err(e), &[]),
    }
}
