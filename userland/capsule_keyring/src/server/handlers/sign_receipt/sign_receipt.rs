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

use crate::protocol::{encode_response, Request, EACCES, EINVAL};
use crate::server::eip712::{receipt_digest, struct_hash};
use crate::server::ethaddr::address_of;
use crate::server::zeroize::zeroize32;
use crate::store::{Store, StoreError};

use super::read_fields::read_fields;

pub fn sign_receipt(store: &mut Store, req: Request<'_>) -> Vec<u8> {
    const HDR: usize = 4 + 4 + 32 + 20 + 32 + 32 + 32 + 32 + 32;
    if req.payload.len() != HDR {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let caller_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let id = u32::from_le_bytes([p[4], p[5], p[6], p[7]]);
    let mut secret = match store.eth_secret(id, caller_pid) {
        Ok(s) => s,
        Err(StoreError::AccessDenied) => return encode_response(req.seq, EACCES, &[]),
        Err(_) => return encode_response(req.seq, EINVAL, &[]),
    };
    let user = match address_of(&secret) {
        Some(a) => a,
        None => {
            zeroize32(&mut secret);
            return encode_response(req.seq, EINVAL, &[]);
        }
    };
    let sh = struct_hash(&read_fields(p, user));
    let digest = sh.and_then(|h| receipt_digest(&h).map(|d| (h, d)));
    let (sh, digest) = match digest {
        Some(v) => v,
        None => {
            zeroize32(&mut secret);
            return encode_response(req.seq, EINVAL, &[]);
        }
    };
    let mut sig = [0u8; 65];
    let rc = nonos_libc::crypto_secp256k1_sign(secret.as_ptr(), digest.as_ptr(), sig.as_mut_ptr());
    zeroize32(&mut secret);
    if rc != 65 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let mut out = Vec::with_capacity(117);
    out.extend_from_slice(&user);
    out.extend_from_slice(&sh);
    out.extend_from_slice(&sig);
    encode_response(req.seq, 0, &out)
}
