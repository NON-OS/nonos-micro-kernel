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

use super::sizes::{EPHEMERAL_BYTES, SALT_BYTES};
use crate::crypto::ecdh::x25519_shared;
use crate::crypto::hkdf_blake3::{expand, extract};
use crate::crypto::types::CryptoError;

/// Gateway shared key: HKDF over BLAKE3, salted with the initiator's salt and
/// keyed by the ephemeral Diffie-Hellman result. BLAKE3, not SHA-256: the
/// reference instantiates HKDF with blake3::Hasher.
pub fn derive_shared_key(
    own_ephemeral_secret: &[u8; 32],
    remote_ephemeral: &[u8; EPHEMERAL_BYTES],
    salt: &[u8; SALT_BYTES],
) -> Result<[u8; 32], CryptoError> {
    let mut dh = [0u8; 32];
    x25519_shared(own_ephemeral_secret, remote_ephemeral, &mut dh)?;
    let mut prk = [0u8; 32];
    extract(salt, &dh, &mut prk)?;
    let mut key = [0u8; 32];
    expand(&prk, &[], &mut key)?;
    Ok(key)
}
