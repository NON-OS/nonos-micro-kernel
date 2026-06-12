// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::types::{KeyId, PK_LEN};
const DS_KEYID: &str = "NONOS:KEYID:ED25519:v1";

pub fn derive_keyid(pubkey: &[u8; PK_LEN]) -> KeyId {
    let mut h = blake3::Hasher::new_derive_key(DS_KEYID);
    h.update(pubkey);
    let mut id = [0u8; 32];
    id.copy_from_slice(&h.finalize().as_bytes()[0..32]);
    id
}

#[inline(never)]
pub fn constant_time_eq(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

pub fn is_zero_key(key: &[u8; 32]) -> bool {
    let mut zero = true;
    for b in key {
        if *b != 0 {
            zero = false;
        }
    }
    zero
}
