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

use nonos_ed25519::{verify as ed25519_verify, Signature};

use super::trait_def::{Verdict, Verifier};

const PUBKEY_LEN: usize = 32;
const SIG_LEN: usize = 64;

pub struct CryptoVerifier;

impl Verifier for CryptoVerifier {
    fn verify(&self, signed_bytes: &[u8], signature: &[u8], pubkey: &[u8; PUBKEY_LEN]) -> Verdict {
        if signature.len() != SIG_LEN {
            return Verdict::Refused;
        }
        /*
         * Verification is a library call now rather than a syscall. The
         * kernel keeps ed25519 only for the boot chain, which decides whether
         * a capsule loads at all; a market listing's signature is this
         * capsule's own business and does not need ring 0 to check it.
         */
        let mut sig = [0u8; SIG_LEN];
        sig.copy_from_slice(&signature[..SIG_LEN]);
        if ed25519_verify(pubkey, signed_bytes, &Signature::from_bytes(&sig)) {
            Verdict::Accepted
        } else {
            Verdict::Refused
        }
    }
}
