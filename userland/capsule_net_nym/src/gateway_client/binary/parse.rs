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

use super::kinds::{HEADER_BYTES, NONCE_BYTES};
use crate::crypto::gcm_siv::open;
use alloc::vec::Vec;

pub struct Incoming {
    pub kind: u8,
    pub plaintext: Vec<u8>,
}

/// Unwrap a gateway frame, rejecting anything that fails authentication.
///
/// An unencrypted frame is refused rather than passed through: the flag is
/// attacker-controlled, so honouring a zero would let anyone on the path hand
/// us plaintext we would treat as the gateway's.
pub fn parse_blob(raw: &[u8], key: &[u8; 32]) -> Option<Incoming> {
    if raw.len() < HEADER_BYTES + NONCE_BYTES {
        return None;
    }
    if raw[1] != 1 {
        return None;
    }
    let mut nonce = [0u8; NONCE_BYTES];
    nonce.copy_from_slice(&raw[HEADER_BYTES..HEADER_BYTES + NONCE_BYTES]);
    let plaintext = open(key, &nonce, &[], &raw[HEADER_BYTES + NONCE_BYTES..])?;
    Some(Incoming { kind: raw[0], plaintext })
}
