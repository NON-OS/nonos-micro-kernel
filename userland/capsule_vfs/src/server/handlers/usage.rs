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

use super::util::split_caller;
use crate::protocol::{encode_response, Request, OP_USAGE};
use crate::store::Store;

// Payload: u32 caller_pid. Reply body: u32 file_count, u64 bytes_used,
// u32 max_files.
pub fn usage(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    if let Err(s) = split_caller(req.payload, sender_pid) {
        return encode_response(OP_USAGE, req.flags, req.request_id, s, &[]);
    }
    let (files, bytes, max) = store.usage();
    let mut body = Vec::with_capacity(16);
    body.extend_from_slice(&files.to_le_bytes());
    body.extend_from_slice(&bytes.to_le_bytes());
    body.extend_from_slice(&max.to_le_bytes());
    encode_response(OP_USAGE, req.flags, req.request_id, 0, &body)
}
