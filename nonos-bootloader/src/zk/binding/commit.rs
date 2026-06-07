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

use blake3;

use super::types::{BindingInput, DS_COMMITMENT};

/// Compute BLAKE3 commitment with domain separation
#[inline]
fn blake3_commit(bytes: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key(DS_COMMITMENT);
    h.update(bytes);
    *h.finalize().as_bytes()
}

/// Compute commitment from binding input
pub fn compute_commit(binding: BindingInput<'_>) -> [u8; 32] {
    match binding {
        BindingInput::PublicInputs(pi) => blake3_commit(pi),
        BindingInput::Manifest(m) => blake3_commit(m),
    }
}

pub fn compute_capsule_commitment(
    kernel_hash: &[u8; 32],
    _boot_nonce: &[u8; 32],
    _machine_id: &[u8; 32],
    program_hash: &[u8; 32],
) -> [u8; 32] {
    // Must match the prover (embed-zk-proof create_circuit_params): the kernel
    // commitment binds kernel_hash, program_hash, and a zero capability mask,
    // the same shape as a per-capsule commitment. Boot nonce and machine id are
    // no longer part of the preimage.
    let mut hasher = blake3::Hasher::new_derive_key(DS_COMMITMENT);
    hasher.update(kernel_hash);
    hasher.update(program_hash);
    hasher.update(&0u64.to_be_bytes());
    *hasher.finalize().as_bytes()
}

/// Verify that a commitment matches the binding
pub fn verify_commitment(binding: BindingInput<'_>, expected: &[u8; 32]) -> bool {
    let computed = compute_commit(binding);
    // Constant-time comparison
    let mut x = 0u8;
    for i in 0..32 {
        x |= computed[i] ^ expected[i];
    }
    x == 0
}
