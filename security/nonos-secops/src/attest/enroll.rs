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

//! Enroll a kernel image and build the trailer that proves its membership. Same
//! padding, same commitment, same trailer the build side produces.

use super::constants::{EXTRA_BLOWUP_BITS, GRIND_BITS, LEAVES, LOG_ROUNDS, N_QUERIES, PAD_IMAGE};
use super::context::{kernel_context, root_to_bytes};
use nonos_stark::air::{build_attestation_trailer, enroll_policy_root, Poseidon, RATE};
use nonos_stark::field::Fp;

/// Enroll a kernel image: pad the tree to the gate depth, commit, and build the
/// trailer bound to the kernel context. Returns the serialized root and trailer.
pub fn enroll_kernel(kernel_bytes: &[u8]) -> ([u8; 32], Vec<u8>) {
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let mut images: Vec<&[u8]> = vec![kernel_bytes];
    while images.len() < LEAVES {
        images.push(PAD_IMAGE);
    }
    let root = root_to_bytes(enroll_policy_root(&hasher, &images));
    let ctx = kernel_context(kernel_bytes);
    let trailer = build_attestation_trailer(
        &hasher,
        LOG_ROUNDS,
        &images,
        0,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    (root, trailer)
}
