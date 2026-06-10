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

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureResult {
    Valid,
    InvalidSignature,
    InvalidKey,
    UnsupportedAlgorithm,
    MissingSignature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    RsaPkcs1v15,
    RsaPss,
    EcdsaP256,
    EcdsaP384,
    Ed25519,
}

pub fn verify_signature(
    data: &[u8],
    signature: &[u8],
    public_key: &[u8],
    algorithm: SignatureAlgorithm,
) -> SignatureResult {
    if signature.is_empty() {
        return SignatureResult::MissingSignature;
    }
    match algorithm {
        SignatureAlgorithm::RsaPkcs1v15 | SignatureAlgorithm::RsaPss => {
            SignatureResult::UnsupportedAlgorithm
        }
        SignatureAlgorithm::EcdsaP256 | SignatureAlgorithm::EcdsaP384 => {
            SignatureResult::UnsupportedAlgorithm
        }
        SignatureAlgorithm::Ed25519 => verify_ed25519(data, signature, public_key),
    }
}

pub fn extract_public_key(cert_data: &[u8]) -> Option<&[u8]> {
    if cert_data.len() < 256 {
        return None;
    }
    if &cert_data[0..4] != b"CERT" {
        return None;
    }
    let key_offset =
        u32::from_le_bytes([cert_data[4], cert_data[5], cert_data[6], cert_data[7]]) as usize;
    if key_offset + 256 > cert_data.len() {
        return None;
    }
    Some(&cert_data[key_offset..key_offset + 256])
}

fn verify_ed25519(data: &[u8], signature: &[u8], key: &[u8]) -> SignatureResult {
    if signature.len() != 64 {
        return SignatureResult::InvalidSignature;
    }
    if key.len() < 32 {
        return SignatureResult::InvalidKey;
    }
    let mut key_bytes = [0u8; 32];
    let mut signature_bytes = [0u8; 64];
    key_bytes.copy_from_slice(&key[..32]);
    signature_bytes.copy_from_slice(signature);
    let Ok(verifying_key) = VerifyingKey::from_bytes(&key_bytes) else {
        return SignatureResult::InvalidKey;
    };
    let sig = Signature::from_bytes(&signature_bytes);
    if verifying_key.verify(data, &sig).is_ok() {
        SignatureResult::Valid
    } else {
        SignatureResult::InvalidSignature
    }
}
