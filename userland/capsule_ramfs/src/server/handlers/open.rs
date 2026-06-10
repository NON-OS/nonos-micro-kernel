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

use alloc::string::String;
use alloc::vec::Vec;

use crate::handles::HandleTable;
use crate::protocol::{
    encode_response, read_u16_le, read_u32_le, Request, EINVAL, EIO, EMFILE, ENOENT,
    OPEN_FLAG_CREATE, OPEN_FLAG_TRUNCATE,
};
use crate::store::Store;

pub fn open(
    store: &mut Store,
    handles: &mut HandleTable,
    req: Request<'_>,
    sender_pid: u32,
) -> Vec<u8> {
    if req.payload.len() < 6 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let flags = match read_u32_le(req.payload, 0) {
        Some(v) => v,
        None => return encode_response(req.seq, EINVAL, &[]),
    };
    let path_len = match read_u16_le(req.payload, 4) {
        Some(v) => v as usize,
        None => return encode_response(req.seq, EINVAL, &[]),
    };
    if req.payload.len() < 6 + path_len {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let path = match core::str::from_utf8(&req.payload[6..6 + path_len]) {
        Ok(s) => String::from(s),
        Err(_) => return encode_response(req.seq, EINVAL, &[]),
    };
    if !store.contains(&path) {
        if flags & OPEN_FLAG_CREATE == 0 {
            return encode_response(req.seq, ENOENT, &[]);
        }
        if store.ensure(&path).is_err() {
            return encode_response(req.seq, EIO, &[]);
        }
    }
    if flags & OPEN_FLAG_TRUNCATE != 0 && store.truncate(&path, 0).is_err() {
        return encode_response(req.seq, EIO, &[]);
    }
    match handles.insert(path, sender_pid) {
        Some(id) => encode_response(req.seq, 0, &id.to_le_bytes()),
        None => encode_response(req.seq, EMFILE, &[]),
    }
}
