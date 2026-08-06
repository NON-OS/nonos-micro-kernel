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

use super::super::hash::blake3;
use super::super::types::CryptoError;
use alloc::vec::Vec;

/// BLAKE3's block size. HMAC pads the key to this width, so it has to be the
/// hash's real block size and not its output size.
const BLOCK: usize = 64;

/// HMAC-BLAKE3, RFC 2104 over BLAKE3.
///
/// Nym's gateway derives its shared key with HKDF instantiated over BLAKE3
/// rather than SHA-256. Using the wrong hash yields a key that is perfectly
/// usable and shared with nobody.
pub fn hmac_blake3(key: &[u8], message: &[u8], out: &mut [u8; 32]) -> Result<(), CryptoError> {
    let mut padded = [0u8; BLOCK];
    if key.len() > BLOCK {
        let mut digest = [0u8; 32];
        blake3(key, &mut digest)?;
        padded[..32].copy_from_slice(&digest);
    } else {
        padded[..key.len()].copy_from_slice(key);
    }

    let mut inner = Vec::with_capacity(BLOCK + message.len());
    for byte in padded.iter() {
        inner.push(byte ^ 0x36);
    }
    inner.extend_from_slice(message);
    let mut inner_digest = [0u8; 32];
    blake3(&inner, &mut inner_digest)?;

    let mut outer = Vec::with_capacity(BLOCK + 32);
    for byte in padded.iter() {
        outer.push(byte ^ 0x5c);
    }
    outer.extend_from_slice(&inner_digest);
    blake3(&outer, out)
}
