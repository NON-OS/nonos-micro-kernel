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
use digest::Digest;
use sha2::Sha384;

use crate::protocol::{encode_response, Request, EMSGSIZE, MAX_INPUT_BYTES, OP_SHA384_HASH};

pub fn sha384_hash(req: Request<'_>) -> Vec<u8> {
    if req.payload.len() > MAX_INPUT_BYTES as usize {
        return encode_response(OP_SHA384_HASH, req.flags, req.request_id, EMSGSIZE, &[]);
    }
    let mut hasher = Sha384::new();
    hasher.update(req.payload);
    let out = hasher.finalize();
    encode_response(OP_SHA384_HASH, req.flags, req.request_id, 0, &out)
}
