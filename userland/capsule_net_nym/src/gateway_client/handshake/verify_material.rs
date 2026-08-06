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
use super::sizes::{EPHEMERAL_BYTES, IDENTITY_BYTES, SIGNATURE_BYTES};
use crate::crypto::gcm_siv::open;
use nonos_libc::crypto_ed25519_verify;

/// Check the gateway holds the identity the directory lists. Without this the
/// exchange authenticates nobody and anyone answering on the socket could
/// complete it.
pub fn verify_material(
    gateway_identity: &[u8; IDENTITY_BYTES],
    gateway_ephemeral: &[u8; EPHEMERAL_BYTES],
    own_ephemeral: &[u8; EPHEMERAL_BYTES],
    shared_key: &[u8; 32],
    material: &Material,
) -> bool {
    let Some(signature) = open(shared_key, &material.nonce, &[], &material.sealed) else {
        return false;
    };
    if signature.len() != SIGNATURE_BYTES {
        return false;
    }
    // The gateway signs its own key first, the mirror of what we sign, so
    // neither side's material replays back at the other.
    let mut signed = [0u8; EPHEMERAL_BYTES * 2];
    signed[..EPHEMERAL_BYTES].copy_from_slice(gateway_ephemeral);
    signed[EPHEMERAL_BYTES..].copy_from_slice(own_ephemeral);
    // Argument order is (pubkey, signature, message, len), unlike signing.
    crypto_ed25519_verify(
        gateway_identity.as_ptr(),
        signature.as_ptr(),
        signed.as_ptr(),
        signed.len(),
    ) == 0
}
