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

use super::types_category::CircuitCategory;
use ed25519_dalek::{Signature, VerifyingKey};

pub const DS_CIRCUIT_SIGN: &str = "NONOS:CIRCUIT:SIGN:v1";

pub fn compute_circuit_signing_data(
    hash: &[u8; 32],
    vk: &[u8],
    name: &str,
    version: &str,
    perms: u32,
    cat: CircuitCategory,
) -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key(DS_CIRCUIT_SIGN);
    h.update(hash);
    h.update(vk);
    h.update(name.as_bytes());
    h.update(version.as_bytes());
    h.update(&perms.to_le_bytes());
    h.update(&[cat as u8]);
    *h.finalize().as_bytes()
}

pub fn verify_circuit_signature(msg: &[u8; 32], sig: &[u8; 64], pubkey: &[u8; 32]) -> bool {
    let Ok(vk) = VerifyingKey::from_bytes(pubkey) else { return false };
    vk.verify_strict(msg, &Signature::from_bytes(sig)).is_ok()
}
