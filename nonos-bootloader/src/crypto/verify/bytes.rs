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

use super::error::VerifyError;
use super::SIG_LEN;
use crate::crypto::keys::{derive_keyid, is_initialized, KeyId, KeyStatus, KEYSTORE};
use core::convert::TryInto;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

pub fn verify_signature_bytes(data: &[u8], sig_bytes: &[u8]) -> Result<KeyId, VerifyError> {
    if sig_bytes.len() != SIG_LEN {
        return Err(VerifyError::MalformedSignature);
    }
    if !is_initialized() {
        return Err(VerifyError::NotInitialized);
    }
    let sig_arr: [u8; 64] = sig_bytes.try_into().map_err(|_| VerifyError::MalformedSignature)?;
    let sig = Signature::from_bytes(&sig_arr);
    let store = KEYSTORE.lock();
    for i in 0..store.count {
        let pk_bytes = &store.keys[i];
        let version = store.versions[i];
        if let Ok(pk) = VerifyingKey::from_bytes(pk_bytes) {
            if pk.verify(data, &sig).is_ok() {
                let key_id = derive_keyid(pk_bytes);
                match store.validate_key(pk_bytes, version) {
                    KeyStatus::Valid => return Ok(key_id),
                    KeyStatus::Revoked => return Err(VerifyError::KeyRevoked),
                    KeyStatus::VersionTooOld => return Err(VerifyError::KeyVersionTooOld),
                    KeyStatus::Expired => return Err(VerifyError::KeyRevoked),
                    KeyStatus::Unknown => return Err(VerifyError::KeyNotFound),
                }
            }
        }
    }
    Err(VerifyError::InvalidSignature)
}
