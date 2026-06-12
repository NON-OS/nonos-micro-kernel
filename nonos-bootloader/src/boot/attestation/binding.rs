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

use crate::zk::attest::types::ZkProofBlock;
use crate::zk::BootAttestationResult;

pub fn verify_kernel_in_proof(
    _result: &BootAttestationResult,
    actual_kernel_hash: &[u8; 32],
    proof_block: &ZkProofBlock,
) -> Result<(), &'static str> {
    if !proof_block.kernel_hash_matches(actual_kernel_hash) {
        return Err("ZK binding: kernel hash in proof does not match loaded kernel");
    }
    Ok(())
}

pub fn verify_commitment_binding(
    capsule_commitment: &[u8; 32],
    expected_commitment: &[u8; 32],
) -> Result<(), &'static str> {
    if !ct_eq32(capsule_commitment, expected_commitment) {
        return Err("ZK binding: capsule commitment mismatch");
    }
    Ok(())
}

#[inline]
fn ct_eq32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut x = 0u8;
    for i in 0..32 {
        x |= a[i] ^ b[i];
    }
    x == 0
}
