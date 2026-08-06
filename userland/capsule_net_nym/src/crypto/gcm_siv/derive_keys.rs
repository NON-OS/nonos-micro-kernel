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

use super::super::aes::Aes256;

/// Per-nonce key derivation, RFC 8452 section 4.
///
/// The key handed to the AEAD never encrypts anything itself. Six counter
/// blocks are encrypted under it and their first halves concatenated, giving a
/// fresh authentication and encryption key for every nonce. That is what lets
/// GCM-SIV survive a repeated nonce with only a loss of indistinguishability
/// rather than a total break.
pub fn derive_keys(key: &[u8; 32], nonce: &[u8; 12]) -> ([u8; 16], [u8; 32]) {
    let cipher = Aes256::new(key);
    let mut halves = [[0u8; 8]; 6];
    for (counter, half) in halves.iter_mut().enumerate() {
        let mut block = [0u8; 16];
        block[..4].copy_from_slice(&(counter as u32).to_le_bytes());
        block[4..].copy_from_slice(nonce);
        cipher.encrypt_block(&mut block);
        half.copy_from_slice(&block[..8]);
    }
    let mut auth = [0u8; 16];
    let mut enc = [0u8; 32];
    auth[..8].copy_from_slice(&halves[0]);
    auth[8..].copy_from_slice(&halves[1]);
    for i in 0..4 {
        enc[i * 8..i * 8 + 8].copy_from_slice(&halves[2 + i]);
    }
    (auth, enc)
}
