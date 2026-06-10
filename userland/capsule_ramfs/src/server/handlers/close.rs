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

use crate::handles::{HandleError, HandleTable};
use crate::protocol::{encode_response, read_u64_le, Request, EACCES, EINVAL, ENOENT};

pub fn close(handles: &mut HandleTable, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    if req.payload.len() < 8 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let h = match read_u64_le(req.payload, 0) {
        Some(v) => v,
        None => return encode_response(req.seq, EINVAL, &[]),
    };
    match handles.remove(h, sender_pid) {
        Ok(()) => encode_response(req.seq, 0, &[]),
        Err(HandleError::Denied) => encode_response(req.seq, EACCES, &[]),
        Err(HandleError::NotFound) => encode_response(req.seq, ENOENT, &[]),
    }
}
