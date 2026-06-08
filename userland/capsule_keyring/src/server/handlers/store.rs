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

use crate::protocol::{encode_response, Request, EACCES, EINVAL, ENOSPC};
use crate::store::{KeyType, Store, StoreError};

const HDR: usize = 4 + 8 + 8 + 1 + 2;

pub fn store(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    if req.payload.len() < HDR {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let payload_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let caller_pid = match super::super::caller::resolve_caller(payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let now = u64::from_le_bytes([p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11]]);
    let expires_at = u64::from_le_bytes([p[12], p[13], p[14], p[15], p[16], p[17], p[18], p[19]]);
    let key_type = match KeyType::from_u8(p[20]) {
        Some(t) => t,
        None => return encode_response(req.seq, EINVAL, &[]),
    };
    let data_len = u16::from_le_bytes([p[21], p[22]]) as usize;
    if p.len() != HDR + data_len {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let data = &p[HDR..HDR + data_len];
    match store.store(key_type, data, caller_pid, now, expires_at) {
        Ok(id) => encode_response(req.seq, 0, &id.to_le_bytes()),
        Err(StoreError::InvalidArgument) => encode_response(req.seq, EINVAL, &[]),
        Err(StoreError::Full) => encode_response(req.seq, ENOSPC, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    }
}
