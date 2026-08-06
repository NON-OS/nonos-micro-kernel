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

/// Encrypt and authenticate. The tag is appended, as every caller expects.
pub fn seal(key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let (auth_key, enc_key) = derive_keys(key, nonce);
    let tag = compute_tag(&auth_key, &enc_key, nonce, aad, plaintext);
    let mut out = Vec::with_capacity(plaintext.len() + 16);
    out.extend_from_slice(plaintext);
    apply_ctr32(&enc_key, &tag, &mut out);
    out.extend_from_slice(&tag);
    out
}
