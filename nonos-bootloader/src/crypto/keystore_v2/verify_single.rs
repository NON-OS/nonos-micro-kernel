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

use super::store_core::KeystoreV2;
use super::types_key::KeyType;
use super::types_result::KeyValidationResult;
use crate::crypto::keys::KeyId;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

impl KeystoreV2 {
    pub fn verify_signature(
        &self,
        data: &[u8],
        signature: &[u8; 64],
        timestamp: u64,
    ) -> Result<KeyId, KeyValidationResult> {
        let sig = Signature::from_bytes(signature);
        for i in 0..self.key_count {
            let key = &self.keys[i];
            if key.is_valid_at(timestamp, self.minimum_version) != KeyValidationResult::Valid {
                continue;
            }
            if let Ok(vk) = VerifyingKey::from_bytes(&key.public_key) {
                if vk.verify(data, &sig).is_ok() {
                    if self.require_cosign && key.key_type == KeyType::Secondary {
                        return Err(KeyValidationResult::RequiresCoSignature);
                    }
                    return Ok(key.key_id);
                }
            }
        }
        Err(KeyValidationResult::NotFound)
    }
}
