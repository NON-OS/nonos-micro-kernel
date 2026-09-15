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

//! The HMAC_DRBG state that RFC 6979 runs the nonce out of.
//!
//! Every field here is key material: `seed_buf` holds the private key in
//! bytes 33..65, and `k` is the HMAC key the nonce comes out of. They live in
//! one struct so there is one place that scrubs them, rather than four
//! buffers passed by reference between functions that each have to remember.

use crate::hmac::hmac_sha256;
use crate::wipe::wipe;

pub(crate) struct Drbg {
    pub(crate) v: [u8; 32],
    pub(crate) k: [u8; 32],
    /// `V || tag || key || hash`, the RFC's seeding message, at its full
    /// length so seeding does not allocate.
    seed_buf: [u8; 97],
}

impl Drbg {
    /// The RFC's starting state: V all ones, K all zeros. Neither is secret
    /// yet; [`Self::seed`] is what mixes the key in.
    pub(crate) fn new() -> Self {
        Drbg { v: [0x01; 32], k: [0x00; 32], seed_buf: [0; 97] }
    }

    /// Two rounds with tags 0x00 and 0x01, which is what makes K depend on
    /// both the key and the message rather than staying at its zero start.
    pub(crate) fn seed(&mut self, sk: &[u8; 32], message_hash: &[u8; 32]) -> Option<()> {
        for tag in [0x00u8, 0x01] {
            self.seed_buf[0..32].copy_from_slice(&self.v);
            self.seed_buf[32] = tag;
            self.seed_buf[33..65].copy_from_slice(sk);
            self.seed_buf[65..97].copy_from_slice(message_hash);
            self.k = hmac_sha256(&self.k, &self.seed_buf)?;
            self.v = hmac_sha256(&self.k, &self.v)?;
        }
        Some(())
    }

    /// Advance past a candidate that was not usable, per the RFC's retry step.
    pub(crate) fn reject(&mut self) -> Option<()> {
        let mut retry = [0u8; 33];
        retry[0..32].copy_from_slice(&self.v);
        retry[32] = 0x00;
        self.k = hmac_sha256(&self.k, &retry)?;
        self.v = hmac_sha256(&self.k, &self.v)?;
        wipe(&mut retry);
        Some(())
    }

    /// Called on every path out, including the failing ones.
    pub(crate) fn wipe(&mut self) {
        wipe(&mut self.v);
        wipe(&mut self.k);
        wipe(&mut self.seed_buf);
    }
}
