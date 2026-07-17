// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Kernel self-attestation, verified before the jump. The bootloader already
//! measures the kernel with BLAKE3; here it checks the kernel's transparent,
//! post-quantum STARK proof that this measurement is enrolled under the boot
//! trust root, bound to the boot epoch. It is the same money-grade membership
//! the kernel runs one layer down for capsules, verified by the same crate, so
//! the prover and the verifier agree by construction. No trusted setup, no
//! pairing: trust rests only on the hash.

use nonos_stark::air::{verify_membership_trailer, Poseidon, RATE};
use nonos_stark::field::Fp;

const LOG_ROUNDS: u32 = 3;
const DEPTH: usize = 8;
const N_QUERIES: usize = 32;
const GRIND_BITS: u32 = 16;
const EXTRA_BLOWUP_BITS: u32 = 3;
const BOOT_EPOCH: u64 = 1;

/// The enrolled kernel measurement root the boot chain trusts. The kernel
/// enrollment step provisions the real root here (or via a build input); a
/// zeroed root accepts nothing, so an un-enrolled build cannot be spoofed into
/// trusting a proof.
const KERNEL_ROOT: [u8; 32] = [0u8; 32];

/// Verify the kernel self-attestation `trailer` against the enrolled root, bound
/// to the measurement of `kernel_bytes` and the boot epoch. True only for a
/// money-grade membership proof of exactly this kernel under exactly this root.
#[must_use = "the boot chain must halt if the kernel does not self-attest"]
pub fn verify_kernel_self_attestation(kernel_bytes: &[u8], trailer: &[u8]) -> bool {
    let measurement = *blake3::hash(kernel_bytes).as_bytes();
    let mut ctx = [0u8; 40];
    ctx[..32].copy_from_slice(&measurement);
    ctx[32..40].copy_from_slice(&BOOT_EPOCH.to_be_bytes());

    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    verify_membership_trailer(
        &hasher,
        LOG_ROUNDS,
        KERNEL_ROOT,
        DEPTH,
        trailer,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    )
}
