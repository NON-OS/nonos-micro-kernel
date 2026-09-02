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
use x25519_dalek::{PublicKey, StaticSecret};

use crate::protocol::{encode_response, Request, EINVAL, OP_X25519_SHARED, X25519_KEY_BYTES};
use crate::server::wipe::wipe;

pub fn x25519_shared(req: Request<'_>) -> Vec<u8> {
    if req.payload.len() != X25519_KEY_BYTES * 2 {
        return encode_response(OP_X25519_SHARED, req.flags, req.request_id, EINVAL, &[]);
    }
    let mut private = [0u8; X25519_KEY_BYTES];
    let mut public = [0u8; X25519_KEY_BYTES];
    private.copy_from_slice(&req.payload[..X25519_KEY_BYTES]);
    public.copy_from_slice(&req.payload[X25519_KEY_BYTES..]);
    let secret = StaticSecret::from(private);
    let peer = PublicKey::from(public);
    let shared = secret.diffie_hellman(&peer);
    // A low-order peer point drives the shared secret to all-zero, which RFC
    // 7748 requires be rejected: it is a value the peer forced, not one the
    // exchange agreed on. Refuse it rather than feed a predictable secret into
    // a key schedule.
    if !shared.was_contributory() {
        wipe(&mut private);
        return encode_response(OP_X25519_SHARED, req.flags, req.request_id, EINVAL, &[]);
    }
    let resp = encode_response(OP_X25519_SHARED, req.flags, req.request_id, 0, shared.as_bytes());
    wipe(&mut private);
    resp
}
