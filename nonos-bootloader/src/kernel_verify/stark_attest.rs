// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Kernel self-attestation, verified before the jump. The bootloader already
//! measures the kernel with BLAKE3; here it checks the kernel's transparent,
//! post-quantum STARK proof that this measurement is enrolled under the boot
//! trust root, bound to the boot epoch. It is the same money-grade membership
//! the kernel runs one layer down for capsules, verified by the same crate, so
//! the prover and the verifier agree by construction. No trusted setup, no
//! pairing: trust rests only on the hash.

use nonos_stark::air::{verify_membership_trailer, Poseidon, RATE};
use nonos_stark::field::Fp;
// One definition, in nonos_stark. Prover and verifier must
// agree exactly; a drift downward in queries or grinding still verifies.
use nonos_stark::attest_params::{GRIND_BITS, LOG_ROUNDS, N_QUERIES, EXTRA_BLOWUP_BITS as EXTRA_BLOWUP_BITS};

const DEPTH: usize = 8;
const BOOT_EPOCH: u64 = 1;

/// The enrolled kernel measurement root the boot chain trusts, provisioned by
/// `build.rs` from `NONOS_KERNEL_ATTEST_ROOT` (zeroed until the kernel is
/// enrolled, which accepts nothing).
include!(concat!(env!("OUT_DIR"), "/kernel_attest_root.rs"));

/// Verify the kernel self-attestation `trailer` against the enrolled root, bound
/// to the measurement of `kernel_bytes` and the boot epoch. True only for a
/// money-grade membership proof of exactly this kernel under exactly this root.
#[must_use = "the boot chain must halt if the kernel does not self-attest"]
pub fn verify_kernel_self_attestation(kernel_bytes: &[u8], trailer: &[u8]) -> bool {
    // An unenrolled build carries an all-zero root. Nothing should verify under
    // it, but that rests on nobody finding a fold to zero. Refuse it outright:
    // a build that was never enrolled has nothing to say, and the alternative
    // is a gate whose safety is a hash assumption rather than a check.
    if KERNEL_ATTEST_ROOT == [0u8; 32] {
        return false;
    }
    let measurement = *blake3::hash(kernel_bytes).as_bytes();
    let mut ctx = [0u8; 40];
    ctx[..32].copy_from_slice(&measurement);
    ctx[32..40].copy_from_slice(&BOOT_EPOCH.to_be_bytes());

    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    verify_membership_trailer(
        &hasher,
        LOG_ROUNDS,
        KERNEL_ATTEST_ROOT,
        DEPTH,
        trailer,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    )
}
