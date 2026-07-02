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

use super::constants::PUBLIC_KEY_BYTES;
use crate::crypto::keys::NONOS_MLDSA65_PUBLIC_KEY;

pub fn trusted_public_key() -> &'static [u8; PUBLIC_KEY_BYTES] {
    &NONOS_MLDSA65_PUBLIC_KEY
}

pub fn trusted_key_id() -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key("NONOS:KEYID:MLDSA65:v1");
    h.update(&NONOS_MLDSA65_PUBLIC_KEY);
    *h.finalize().as_bytes()
}
