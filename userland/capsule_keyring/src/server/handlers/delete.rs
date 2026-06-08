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

use crate::protocol::{encode_response, Request, EACCES, EINVAL, ENOENT};
use crate::store::{Store, StoreError};

pub fn delete(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    if req.payload.len() != 8 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let payload_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let caller_pid = match super::super::caller::resolve_caller(payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let id = u32::from_le_bytes([p[4], p[5], p[6], p[7]]);
    match store.delete(id, caller_pid) {
        Ok(()) => encode_response(req.seq, 0, &[]),
        Err(StoreError::NotFound) => encode_response(req.seq, ENOENT, &[]),
        Err(StoreError::AccessDenied) => encode_response(req.seq, EACCES, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    }
}
