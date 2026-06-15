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

use super::handlers;
use crate::protocol::{
    encode_response, Request, EINVAL, OP_AES256_GCM_OPEN, OP_AES256_GCM_SEAL, OP_BLAKE3_HASH,
    OP_CHACHA20_POLY1305_OPEN, OP_CHACHA20_POLY1305_SEAL, OP_ED25519_VERIFY, OP_HEALTHCHECK,
    OP_HKDF_SHA256, OP_HMAC_SHA256, OP_P256_ECDSA_VERIFY, OP_P384_ECDSA_VERIFY,
    OP_SHA256_HASH, OP_SHA3_256_HASH, OP_SHA384_HASH, OP_SHA512_HASH, OP_X25519_PUBLIC,
    OP_X25519_SHARED,
};

pub fn dispatch(req: Request<'_>) -> Vec<u8> {
    match req.op {
        OP_BLAKE3_HASH => handlers::blake3_hash(req),
        OP_SHA3_256_HASH => handlers::sha3_256_hash(req),
        OP_SHA256_HASH => handlers::sha256_hash(req),
        OP_SHA384_HASH => handlers::sha384_hash(req),
        OP_SHA512_HASH => handlers::sha512_hash(req),
        OP_ED25519_VERIFY => handlers::ed25519_verify(req),
        OP_P256_ECDSA_VERIFY => handlers::p256_ecdsa_verify(req),
        OP_P384_ECDSA_VERIFY => handlers::p384_ecdsa_verify(req),
        OP_CHACHA20_POLY1305_SEAL => handlers::chacha20_poly1305_seal(req),
        OP_CHACHA20_POLY1305_OPEN => handlers::chacha20_poly1305_open(req),
        OP_AES256_GCM_SEAL => handlers::aes256_gcm_seal(req),
        OP_AES256_GCM_OPEN => handlers::aes256_gcm_open(req),
        OP_X25519_PUBLIC => handlers::x25519_public(req),
        OP_X25519_SHARED => handlers::x25519_shared(req),
        OP_HMAC_SHA256 => handlers::hmac_sha256(req),
        OP_HKDF_SHA256 => handlers::hkdf_sha256(req),
        OP_HEALTHCHECK => handlers::healthcheck(req),
        _ => encode_response(req.op, req.flags, req.request_id, EINVAL, &[]),
    }
}
