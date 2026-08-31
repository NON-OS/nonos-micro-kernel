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

//! Transparent, post-quantum STARK verification primitives. Hash-based and
//! curve-free, so verification relies only on the strength of the hash. Built
//! bottom up: the Goldilocks field, a Merkle commitment over it, polynomials,
//! a Fiat-Shamir transcript, and the FRI low-degree test on top.
//!
//! Shared between the kernel (which proves and verifies) and the bootloader
//! (which verifies the kernel self-attestation before jumping). Both link the
//! same code, so the Fiat-Shamir keccak and the measurement blake3 are
//! byte-identical on the prover and the verifier.

// The crate is `no_std` for the kernel and bootloader. The `parallel` feature —
// used only by the host emit tooling, never by the kernel — pulls in std and
// rayon so a production emit uses every core; the field arithmetic and the
// commitment layout are identical either way, and a bit-exact gate proves the
// parallel prover emits the same proof bytes as the serial one.
#![cfg_attr(not(feature = "parallel"), no_std)]

extern crate alloc;

pub mod hash;

pub mod air;
pub mod attest_params;
pub mod field;
pub mod fri;
pub mod fri_ext;
pub mod fri_poseidon;
pub mod fri_poseidon_ext;
pub mod merkle;
pub(crate) mod par;
pub mod poly;
pub mod poseidon_merkle;
pub mod poseidon_transcript;
pub mod transcript;
