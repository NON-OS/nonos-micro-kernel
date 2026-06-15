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
use p256::ecdsa::{signature::hazmat::PrehashVerifier, Signature, VerifyingKey};

use crate::protocol::{encode_response, Request, EBADMSG, EINVAL, OP_P256_ECDSA_VERIFY, P256_VERIFY_BYTES};

pub fn p256_ecdsa_verify(req: Request<'_>) -> Vec<u8> {
    if req.payload.len() != P256_VERIFY_BYTES {
        return encode_response(OP_P256_ECDSA_VERIFY, req.flags, req.request_id, EINVAL, &[]);
    }
    let key = match VerifyingKey::from_sec1_bytes(&req.payload[..65]) {
        Ok(v) => v,
        Err(_) => return encode_response(OP_P256_ECDSA_VERIFY, req.flags, req.request_id, EINVAL, &[]),
    };
    let sig = match Signature::from_slice(&req.payload[65..129]) {
        Ok(v) => v,
        Err(_) => return encode_response(OP_P256_ECDSA_VERIFY, req.flags, req.request_id, EINVAL, &[]),
    };
    let status = if key.verify_prehash(&req.payload[129..161], &sig).is_ok() { 0 } else { EBADMSG };
    encode_response(OP_P256_ECDSA_VERIFY, req.flags, req.request_id, status, &[])
}
