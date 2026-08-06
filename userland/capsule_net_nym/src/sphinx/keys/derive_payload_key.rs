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

use super::super::constants::{
    PAYLOAD_KEY_HKDF_INFO, PAYLOAD_KEY_HKDF_SALT, PAYLOAD_KEY_SEED_SIZE, PAYLOAD_KEY_SIZE,
};
use crate::crypto::kdf::hkdf_sha256;
use crate::crypto::types::CryptoError;

/// Stretch a v2 seed into the 192-byte LIONESS key.
pub fn derive_payload_key(
    seed: &[u8; PAYLOAD_KEY_SEED_SIZE],
) -> Result<[u8; PAYLOAD_KEY_SIZE], CryptoError> {
    let mut key = [0u8; PAYLOAD_KEY_SIZE];
    hkdf_sha256(PAYLOAD_KEY_HKDF_SALT, seed, PAYLOAD_KEY_HKDF_INFO, &mut key)?;
    Ok(key)
}
