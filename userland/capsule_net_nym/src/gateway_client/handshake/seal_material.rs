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

use super::material::Material;
use super::sizes::{EPHEMERAL_BYTES, NONCE_BYTES, SIGNATURE_BYTES};
use crate::crypto::gcm_siv::seal;
use crate::crypto::random::fill_random;
use crate::crypto::types::CryptoError;
use nonos_libc::crypto_ed25519_sign;

/// Sign both ephemeral keys with our identity and seal under the derived key.
///
/// Both halves are signed because a signature over our own key alone would be
/// replayable into a different session.
pub fn seal_material(
    identity_seed: &[u8; 32],
    own_ephemeral: &[u8; EPHEMERAL_BYTES],
    remote_ephemeral: &[u8; EPHEMERAL_BYTES],
    shared_key: &[u8; 32],
) -> Result<Material, CryptoError> {
    let mut signed = [0u8; EPHEMERAL_BYTES * 2];
    signed[..EPHEMERAL_BYTES].copy_from_slice(own_ephemeral);
    signed[EPHEMERAL_BYTES..].copy_from_slice(remote_ephemeral);
    let mut signature = [0u8; SIGNATURE_BYTES];
    let n = crypto_ed25519_sign(
        identity_seed.as_ptr(),
        signed.as_ptr(),
        signed.len(),
        signature.as_mut_ptr(),
    );
    if n != SIGNATURE_BYTES as i64 {
        return Err(CryptoError::Mac);
    }
    let mut nonce = [0u8; NONCE_BYTES];
    fill_random(&mut nonce)?;
    Ok(Material { sealed: seal(shared_key, &nonce, &[], &signature), nonce })
}
