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

//! The deterministic nonce, RFC 6979 with HMAC-SHA256.
//!
//! A signature nonce that repeats for two different messages under one key
//! publishes the key. Deriving it from the key and the message hash removes
//! the entropy source from the signing path entirely, so a signature is a
//! pure function of what it signs, and a machine with a weak or sabotaged RNG
//! still cannot be made to leak a wallet through its signatures.

use crate::hmac_drbg::Drbg;
use crate::scalar::Scalar;

/// The retry bound from the RFC's generation loop. Reaching it means the
/// generator produced 256 consecutive values outside the curve order, which
/// does not happen for any real key; returning `None` rather than looping
/// keeps a caller from hanging on a case that cannot occur.
const MAX_ATTEMPTS: usize = 256;

pub(crate) fn rfc6979_generate_k(sk: &[u8; 32], message_hash: &[u8; 32]) -> Option<Scalar> {
    let mut drbg = Drbg::new();
    let out = run(&mut drbg, sk, message_hash);
    drbg.wipe();
    out
}

fn run(drbg: &mut Drbg, sk: &[u8; 32], message_hash: &[u8; 32]) -> Option<Scalar> {
    drbg.seed(sk, message_hash)?;
    for _ in 0..MAX_ATTEMPTS {
        drbg.v = crate::hmac::hmac_sha256(&drbg.k, &drbg.v)?;
        /*
         * A candidate at or above the group order, or zero, is discarded
         * rather than reduced. Reducing would bias the nonce towards small
         * values, and a biased nonce leaks the key over enough signatures.
         */
        if let Some(candidate) = Scalar::from_bytes(&drbg.v) {
            if !candidate.is_zero() {
                return Some(candidate);
            }
        }
        drbg.reject()?;
    }
    None
}
