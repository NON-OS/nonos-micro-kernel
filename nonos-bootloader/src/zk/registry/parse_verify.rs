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

extern crate alloc;
use crate::zk::verify::ct_eq32;
use alloc::vec::Vec;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

pub fn verify_section_signature(
    section: &[u8],
    signature: &[u8; 64],
    signer: &[u8; 32],
    trusted_signers: &[[u8; 32]],
) -> Result<(), &'static str> {
    let mut trusted = false;
    for ts in trusted_signers {
        if ct_eq32(ts, signer) {
            trusted = true;
            break;
        }
    }
    if !trusted {
        return Err("circuit: untrusted signer");
    }
    let vk = VerifyingKey::from_bytes(signer).map_err(|_| "circuit: invalid signer public key")?;
    let sig = Signature::from_bytes(signature);
    let mut signed_data = Vec::with_capacity(section.len() - 64);
    signed_data.extend_from_slice(&section[0..16]);
    signed_data.extend_from_slice(&section[80..]);
    vk.verify(&signed_data, &sig).map_err(|_| "circuit: signature verification failed")
}
