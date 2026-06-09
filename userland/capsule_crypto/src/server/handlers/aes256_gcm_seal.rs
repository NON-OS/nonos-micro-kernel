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

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Key, Nonce};

use super::aead_frame::{nonce_is_degenerate, parse_seal, FrameError};
use crate::protocol::{encode_response, Request, EINVAL, EIO, EMSGSIZE, OP_AES256_GCM_SEAL};

pub fn aes256_gcm_seal(req: Request<'_>) -> Vec<u8> {
    let frame = match parse_seal(req.payload) {
        Ok(f) => f,
        Err(FrameError::Short) | Err(FrameError::OversizeAad) => {
            return encode_response(OP_AES256_GCM_SEAL, req.flags, req.request_id, EINVAL, &[]);
        }
        Err(FrameError::OversizePayload) => {
            return encode_response(
                OP_AES256_GCM_SEAL,
                req.flags,
                req.request_id,
                EMSGSIZE,
                &[],
            );
        }
    };
    if nonce_is_degenerate(frame.nonce) {
        return encode_response(OP_AES256_GCM_SEAL, req.flags, req.request_id, EINVAL, &[]);
    }
    let key = Key::<Aes256Gcm>::from_slice(frame.key);
    let nonce = Nonce::<aes_gcm::aes::cipher::consts::U12>::from_slice(frame.nonce);
    let cipher = Aes256Gcm::new(key);
    match cipher.encrypt(nonce, Payload { msg: frame.plaintext, aad: frame.aad }) {
        Ok(ct) => encode_response(OP_AES256_GCM_SEAL, req.flags, req.request_id, 0, &ct),
        Err(_) => encode_response(OP_AES256_GCM_SEAL, req.flags, req.request_id, EIO, &[]),
    }
}
