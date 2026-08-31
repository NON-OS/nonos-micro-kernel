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

//! The kernel's own transparent, post-quantum self-attestation. Signature trust says
//! a key vouched for this image; this says, in zero knowledge and without a trusted
//! setup, that the running kernel's measurement is enrolled under the trust root the
//! boot chain carries, bound to the boot epoch. It is the same money-grade membership
//! the capsule gate uses, one layer up: capsules attest to the kernel, the kernel
//! attests to its own enrolled measurement. The root is the boot chain's, never the
//! trailer's.

use crate::crypto::stark::air::{verify_membership_trailer, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::security::capsule_attest::AttestError;
// One definition, in crate::crypto::stark. Prover and verifier must
// agree exactly; a drift downward in queries or grinding still verifies.
use crate::crypto::stark::attest_params::{GRIND_BITS, LOG_ROUNDS, N_QUERIES, EXTRA_BLOWUP_BITS as EXTRA_BLOWUP_BITS};

const DEPTH: usize = 8;
const BOOT_EPOCH: u64 = 1;

/// Verify the kernel's self-attestation: its measurement is enrolled under `root`,
/// bound to the boot epoch. `root` is the enrolled kernel root the boot chain holds.
/// The image is measured into the context so the proof is tied to exactly this
/// kernel. Refuses on any malformed trailer or failed proof.
#[must_use = "the boot chain must halt if the kernel does not self-attest"]
pub fn verify_kernel_self_attestation(
    root: [u8; 32],
    trailer: &[u8],
    kernel_image: &[u8],
) -> Result<(), AttestError> {
    let measurement = *blake3::hash(kernel_image).as_bytes();
    let mut ctx = [0u8; 40];
    ctx[..32].copy_from_slice(&measurement);
    ctx[32..40].copy_from_slice(&BOOT_EPOCH.to_be_bytes());

    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    if verify_membership_trailer(
        &hasher,
        LOG_ROUNDS,
        root,
        DEPTH,
        trailer,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    ) {
        Ok(())
    } else {
        Err(AttestError::Rejected)
    }
}
