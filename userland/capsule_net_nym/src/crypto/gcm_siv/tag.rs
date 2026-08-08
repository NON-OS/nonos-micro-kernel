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
use super::super::polyval::Polyval;

/// The synthetic IV: POLYVAL over padded AAD and plaintext plus their bit
/// lengths, folded with the nonce and encrypted.
pub fn compute_tag(
    auth_key: &[u8; 16],
    enc_key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    plaintext: &[u8],
) -> [u8; 16] {
    let mut hash = Polyval::new(auth_key);
    absorb(&mut hash, aad);
    absorb(&mut hash, plaintext);
    let mut lengths = [0u8; 16];
    lengths[..8].copy_from_slice(&((aad.len() as u64) * 8).to_le_bytes());
    lengths[8..].copy_from_slice(&((plaintext.len() as u64) * 8).to_le_bytes());
    hash.update(&lengths);

    let mut s = hash.finalize();
    for i in 0..12 {
        s[i] ^= nonce[i];
    }
    // The top bit is cleared here and set again for the counter, so the tag
    // and the keystream can never collide on the same block.
    s[15] &= 0x7f;
    Aes256::new(enc_key).encrypt_block(&mut s);
    s
}

fn absorb(hash: &mut Polyval, data: &[u8]) {
    for chunk in data.chunks(16) {
        let mut block = [0u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);
        hash.update(&block);
    }
}
