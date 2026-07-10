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
use crate::protocol::{encode_response, Request, EINVAL, OP_SEEK, SEEK_CUR, SEEK_END, SEEK_SET};
use crate::store::{SeekWhence, Store};

// Payload: u32 caller_pid, u32 fd, u8 whence, i64 offset.
// Reply body: u64 new absolute position.
pub fn seek(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let (pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_SEEK, req.flags, req.request_id, s, &[]),
    };
    if rest.len() != 13 {
        return encode_response(OP_SEEK, req.flags, req.request_id, EINVAL, &[]);
    }
    let fd = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
    let whence = match rest[4] {
        SEEK_SET => SeekWhence::Set,
        SEEK_CUR => SeekWhence::Cur,
        SEEK_END => SeekWhence::End,
        _ => return encode_response(OP_SEEK, req.flags, req.request_id, EINVAL, &[]),
    };
    let mut off = [0u8; 8];
    off.copy_from_slice(&rest[5..13]);
    let offset = i64::from_le_bytes(off);
    match store.seek(fd, pid, whence, offset) {
        Ok(pos) => encode_response(OP_SEEK, req.flags, req.request_id, 0, &pos.to_le_bytes()),
        Err(e) => encode_response(OP_SEEK, req.flags, req.request_id, map_store_err(e), &[]),
    }
}
