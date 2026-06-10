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
use super::store_core::KeystoreV2;
use super::types_consts::MAX_TRUSTED_KEYS;
use super::types_result::KeyValidationResult;
use super::util::constant_time_eq;
use crate::crypto::keys::KeyId;
use alloc::vec::Vec;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

impl KeystoreV2 {
    pub fn verify_multisig(
        &self,
        data: &[u8],
        signatures: &[([u8; 32], [u8; 64])],
        threshold: usize,
        timestamp: u64,
    ) -> Result<Vec<KeyId>, &'static str> {
        if signatures.len() < threshold {
            return Err("insufficient signatures");
        }
        let mut verified = Vec::new();
        let mut seen = [[0u8; 32]; MAX_TRUSTED_KEYS];
        let mut seen_count = 0;
        for (pubkey, sig_bytes) in signatures {
            let Some(key) = self.find_key_by_pubkey(pubkey) else { continue };
            if key.is_valid_at(timestamp, self.minimum_version) != KeyValidationResult::Valid {
                continue;
            }
            let mut already = false;
            for i in 0..seen_count {
                if constant_time_eq(&seen[i], &key.key_id) {
                    already = true;
                    break;
                }
            }
            if already {
                continue;
            }
            let sig = Signature::from_bytes(sig_bytes);
            if let Ok(vk) = VerifyingKey::from_bytes(&key.public_key) {
                if vk.verify(data, &sig).is_ok() {
                    verified.push(key.key_id);
                    if seen_count < MAX_TRUSTED_KEYS {
                        seen[seen_count] = key.key_id;
                        seen_count += 1;
                    }
                }
            }
        }
        if verified.len() >= threshold {
            Ok(verified)
        } else {
            Err("threshold not met")
        }
    }
}
