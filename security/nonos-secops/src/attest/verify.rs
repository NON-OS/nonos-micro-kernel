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

//! Verify a kernel self-attestation exactly as the bootloader does before jump.

use super::constants::{DEPTH, EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use super::context::kernel_context;
use nonos_stark::air::{verify_membership_trailer, Poseidon, RATE};
use nonos_stark::field::Fp;

/// Verify a trailer against an enrolled root, the boot-side check byte for byte.
pub fn verify_kernel_attestation(root: &[u8; 32], kernel_bytes: &[u8], trailer: &[u8]) -> bool {
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    verify_membership_trailer(
        &hasher,
        LOG_ROUNDS,
        *root,
        DEPTH,
        trailer,
        &kernel_context(kernel_bytes),
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    )
}
