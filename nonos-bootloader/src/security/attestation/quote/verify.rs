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

use super::types::AttestationQuote;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

impl AttestationQuote {
    pub fn verify(&self, attestation_public_key: &[u8; 32]) -> bool {
        let quote_hash = self.compute_quote_hash();
        let vk = match VerifyingKey::from_bytes(attestation_public_key) {
            Ok(k) => k,
            Err(_) => return false,
        };
        let sig = Signature::from_bytes(&self.quote_signature);
        vk.verify(&quote_hash, &sig).is_ok()
    }
}
