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

use super::types::{ChaCha20, CONSTANTS, KEY_BYTES, NONCE_BYTES};

impl ChaCha20 {
    /// The block counter starts at zero. RFC 8439 starts its AEAD at one
    /// because block zero is spent on the Poly1305 key, but LIONESS calls the
    /// bare cipher and the reference starts at zero.
    pub fn new(key: &[u8; KEY_BYTES], nonce: &[u8; NONCE_BYTES]) -> Self {
        let mut state = [0u32; 16];
        state[..4].copy_from_slice(&CONSTANTS);
        for i in 0..8 {
            let b = i * 4;
            state[4 + i] = u32::from_le_bytes([key[b], key[b + 1], key[b + 2], key[b + 3]]);
        }
        for i in 0..3 {
            let b = i * 4;
            state[13 + i] =
                u32::from_le_bytes([nonce[b], nonce[b + 1], nonce[b + 2], nonce[b + 3]]);
        }
        Self { state }
    }
}
