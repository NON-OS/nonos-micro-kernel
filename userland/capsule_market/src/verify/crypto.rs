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

use nonos_libc::crypto_ed25519_verify;

use super::trait_def::{Verdict, Verifier};

const PUBKEY_LEN: usize = 32;
const SIG_LEN: usize = 64;

pub struct CryptoVerifier;

impl Verifier for CryptoVerifier {
    fn verify(&self, signed_bytes: &[u8], signature: &[u8], pubkey: &[u8; PUBKEY_LEN]) -> Verdict {
        if signature.len() != SIG_LEN {
            return Verdict::Refused;
        }
        let rc = crypto_ed25519_verify(
            pubkey.as_ptr(),
            signature.as_ptr(),
            signed_bytes.as_ptr(),
            signed_bytes.len(),
        );
        if rc == 0 {
            Verdict::Accepted
        } else {
            Verdict::Refused
        }
    }
}
