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

use super::ctr32::apply_ctr32;
use super::derive_keys::derive_keys;
use super::tag::compute_tag;
use alloc::vec::Vec;

/// Decrypt and verify. The tag is recomputed over the recovered plaintext and
/// compared without an early exit; a mismatch yields nothing at all, so a
/// caller cannot be handed unauthenticated bytes by accident.
pub fn open(key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], sealed: &[u8]) -> Option<Vec<u8>> {
    if sealed.len() < 16 {
        return None;
    }
    let (body, tag) = sealed.split_at(sealed.len() - 16);
    let (auth_key, enc_key) = derive_keys(key, nonce);
    let mut expected = [0u8; 16];
    expected.copy_from_slice(tag);
    let mut plaintext = Vec::with_capacity(body.len());
    plaintext.extend_from_slice(body);
    apply_ctr32(&enc_key, &expected, &mut plaintext);
    let recomputed = compute_tag(&auth_key, &enc_key, nonce, aad, &plaintext);
    let mut diff = 0u8;
    for (a, b) in recomputed.iter().zip(expected.iter()) {
        diff |= a ^ b;
    }
    if diff != 0 {
        return None;
    }
    Some(plaintext)
}
