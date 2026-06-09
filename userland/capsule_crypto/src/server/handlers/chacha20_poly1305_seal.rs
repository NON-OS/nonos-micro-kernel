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

use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

use super::aead_frame::{nonce_is_degenerate, parse_seal, FrameError};
use crate::protocol::{encode_response, Request, EINVAL, EIO, EMSGSIZE, OP_CHACHA20_POLY1305_SEAL};

pub fn chacha20_poly1305_seal(req: Request<'_>) -> Vec<u8> {
    let frame = match parse_seal(req.payload) {
        Ok(f) => f,
        Err(FrameError::Short) | Err(FrameError::OversizeAad) => {
            return encode_response(
                OP_CHACHA20_POLY1305_SEAL,
                req.flags,
                req.request_id,
                EINVAL,
                &[],
            );
        }
        Err(FrameError::OversizePayload) => {
            return encode_response(
                OP_CHACHA20_POLY1305_SEAL,
                req.flags,
                req.request_id,
                EMSGSIZE,
                &[],
            );
        }
    };
    if nonce_is_degenerate(frame.nonce) {
        return encode_response(
            OP_CHACHA20_POLY1305_SEAL,
            req.flags,
            req.request_id,
            EINVAL,
            &[],
        );
    }
    let key = Key::from_slice(frame.key);
    let nonce = Nonce::from_slice(frame.nonce);
    let cipher = ChaCha20Poly1305::new(key);
    match cipher.encrypt(nonce, Payload { msg: frame.plaintext, aad: frame.aad }) {
        Ok(ct) => encode_response(OP_CHACHA20_POLY1305_SEAL, req.flags, req.request_id, 0, &ct),
        Err(_) => {
            encode_response(OP_CHACHA20_POLY1305_SEAL, req.flags, req.request_id, EIO, &[])
        }
    }
}
