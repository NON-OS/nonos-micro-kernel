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

extern crate alloc;

mod commitment;
mod plonk;

use super::ZkError;
use crate::crypto::zk_kernel::verifier::{ZkResult, KERNEL_ZK_VERIFIER};

pub fn syscall_zk_verify(
    proof_type: u8,
    proof_data: &[u8],
    public_input: &[u8],
) -> Result<(), ZkError> {
    let mut verifier = KERNEL_ZK_VERIFIER.lock();
    let result = match proof_type {
        1 | 2 => ZkResult::UnsupportedProofType,
        3 => verifier.verify_range(proof_data),
        6 => commitment::verify_commitment(&mut verifier, proof_data, public_input)?,
        7 => plonk::verify_plonk(&mut verifier, proof_data, public_input)?,
        _ => ZkResult::UnsupportedProofType,
    };
    match result {
        ZkResult::Valid => Ok(()),
        ZkResult::Invalid => Err(ZkError::InvalidProof),
        ZkResult::MalformedProof => Err(ZkError::MalformedInput),
        ZkResult::UnsupportedProofType => Err(ZkError::UnsupportedProofType),
    }
}
