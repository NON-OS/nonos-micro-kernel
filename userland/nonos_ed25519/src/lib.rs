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

//! ed25519, in userland.
//!
//! The mixnet is what uses this curve. `capsule_net_nym` signs its gateway
//! handshake, derives its own identity key, and verifies the directory's
//! signatures over the topology, and every one of those was a syscall, which
//! put the curve's point, field and scalar arithmetic in ring 0 to serve one
//! capsule.
//!
//! What stays in the kernel is verification alone, for the boot chain:
//! `secure_boot::verify` checks a capsule's signature before it loads, and
//! that check is the root of trust. It cannot move out, because the thing it
//! protects is the loading of everything that would be outside.
//!
//! The arithmetic is the kernel's, moved rather than rewritten. Rewriting to
//! relocate would trade an implementation that has been verifying signatures
//! on this system for an unknown one, and the point of the move is where the
//! code runs, not what it computes.
//!
//! `KeyPair::generate` is not ported. It called a global RNG; the seed is the
//! caller's to supply, and a signing library with ambient access to randomness
//! is one that can be made to sign under a key someone else chose.

#![no_std]
/*
 * The field, point and scalar arithmetic is moved verbatim and is written the
 * way this kind of code is normally written: indexed loops over limb arrays,
 * casts that spell out the width being worked in. Rewriting any of it to
 * satisfy a style lint would be editing signing code for no reason. The
 * vectors in `tests` are what stands behind the move.
 */

extern crate alloc;

mod field;
mod point;
mod scalar;
mod signature;

pub use signature::{sign, verify, KeyPair, Signature};

/// The public key for a secret seed.
pub fn pubkey_from_secret(secret: &[u8; 32]) -> [u8; 32] {
    KeyPair::from_seed(*secret).public
}

#[cfg(test)]
mod tests;
