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

//! secp256k1, in userland.
//!
//! This curve is the wallet's, not the kernel's. It sat in ring 0 answering
//! `CryptoSecp256k1Sign` and `CryptoSecp256k1Pubkey` for capsules, which made
//! 23,256 bytes of point, field, scalar and ECDSA arithmetic part of the
//! trusted computing base with no in-kernel caller at all. A bug in it was a
//! bug with kernel privilege, in code that exists to serve one application.
//!
//! The arithmetic is the same code, moved rather than rewritten: rewriting it
//! to relocate it would trade a known implementation for an unknown one.
//!
//! `generate_keypair` did not come along. It called a global RNG, which is
//! ambient authority a signing library should not hold. Nor did Ethereum
//! address derivation: what a public key means is the application's idea.

#![no_std]
/*
 * The arithmetic is indexed loops over four-limb arrays, which is how this
 * kind of code is written and read. Rewriting it into iterator form for a
 * style lint would be editing signing code for no reason, and what stands
 * behind the move is that the bytes did not change.
 */

extern crate alloc;

mod ecdsa_types;
mod field;
mod hmac;
mod hmac_drbg;
mod hmac_pad;
mod low_s;
mod point;
mod pubkey;
mod recover;
mod recover_point;
mod rfc6979;
mod scalar;
mod sign;
mod verify;
mod verify_parts;
mod wipe;

pub use ecdsa_types::{CompressedPublicKey, PublicKey, RecoverableSignature, SecretKey, Signature};
pub use field::FieldElement;
pub use point::{AffinePoint, ProjectivePoint};
pub use scalar::Scalar;
pub use pubkey::public_key_from_secret;
pub use sign::sign;
pub use recover::recover_public_key;
pub use verify::verify;

#[cfg(test)]
mod tests;
