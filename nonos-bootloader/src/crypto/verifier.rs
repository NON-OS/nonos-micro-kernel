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

use super::keys::{init_production_keys, NONOS_SIGNING_KEY};
use super::verify::{verify_signature_bytes, SignatureStatus, VerifyError};
use ed25519_dalek::VerifyingKey;

pub struct SignatureVerifier {
    initialized: bool,
}

impl SignatureVerifier {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Ok(());
        }
        init_production_keys()?;
        self.initialized = true;
        Ok(())
    }

    pub fn verify(&self, data: &[u8], signature: &[u8]) -> SignatureStatus {
        if !self.initialized {
            return SignatureStatus::Error;
        }
        match verify_signature_bytes(data, signature) {
            Ok(_) => SignatureStatus::Valid,
            Err(VerifyError::InvalidSignature) => SignatureStatus::Invalid,
            Err(_) => SignatureStatus::Error,
        }
    }
}

pub fn perform_crypto_self_test() -> bool {
    let blake3_ok = {
        let h1 = blake3::hash(b"NONOS-crypto-selftest");
        let h2 = blake3::hash(b"NONOS-crypto-selftest");
        h1.as_bytes() == h2.as_bytes()
    };
    let ed25519_ok = VerifyingKey::from_bytes(NONOS_SIGNING_KEY).is_ok();
    blake3_ok && ed25519_ok
}
