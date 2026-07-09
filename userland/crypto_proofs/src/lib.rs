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

//! Known-answer proofs for the kernel crypto primitives. Each `#[path]` include
//! pulls the real `src/crypto` source, so the tests check production code
//! against the official standard vectors (RFC / NIST), not a copy of it. Run
//! with `cargo test` from this directory.

extern crate alloc;

// The real crypto source under test, with the minimal supporting types the
// primitives expect from their parent module.
pub mod crypto;
pub mod hash;

#[cfg(test)]
mod aesgcm_tests;
#[cfg(test)]
mod blake3_tests;
#[cfg(test)]
mod chacha_tests;
#[cfg(test)]
mod constant_time_tests;
#[cfg(test)]
mod ed25519_tests;
#[cfg(test)]
mod hex;
#[cfg(test)]
mod hkdf_tests;
#[cfg(test)]
mod hmac_tests;
#[cfg(test)]
mod p256_tests;
#[cfg(test)]
mod p384_tests;
#[cfg(test)]
mod rsa_tests;
#[cfg(test)]
mod secp256k1_tests;
#[cfg(test)]
mod zk_tests;
#[cfg(test)]
mod sha256_tests;
#[cfg(test)]
mod sha3_tests;
#[cfg(test)]
mod sha512_tests;

#[cfg(kani)]
mod ct_kani;
