// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transparent, post-quantum STARK verification primitives. Hash-based and
//! curve-free, so verification relies only on the strength of the hash. Built
//! bottom up: the Goldilocks field, a Merkle commitment over it, polynomials,
//! a Fiat-Shamir transcript, and the FRI low-degree test on top.
//!
//! Shared between the kernel (which proves and verifies) and the bootloader
//! (which verifies the kernel self-attestation before jumping). Both link the
//! same code, so the Fiat-Shamir keccak and the measurement blake3 are
//! byte-identical on the prover and the verifier.

#![no_std]

extern crate alloc;

pub mod hash;

pub mod air;
pub mod field;
pub mod fri;
pub mod fri_ext;
pub mod fri_poseidon;
pub mod fri_poseidon_ext;
pub mod merkle;
pub mod poly;
pub mod poseidon_merkle;
pub mod poseidon_transcript;
pub mod transcript;
