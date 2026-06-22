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

use crate::crypto::zk_kernel::syscall::ZkError;
use crate::crypto::zk_kernel::{KernelZkVerifier, ZkResult};

pub(super) fn verify_commitment(
    verifier: &mut KernelZkVerifier,
    proof_data: &[u8],
    public_input: &[u8],
) -> Result<ZkResult, ZkError> {
    if proof_data.len() < 32 || public_input.len() < 64 {
        return Err(ZkError::MalformedInput);
    }
    let mut commitment = [0u8; 32];
    let mut value = [0u8; 32];
    let mut blinding = [0u8; 32];
    commitment.copy_from_slice(&proof_data[..32]);
    value.copy_from_slice(&public_input[..32]);
    blinding.copy_from_slice(&public_input[32..64]);
    Ok(verifier.verify_commitment(&commitment, &value, &blinding))
}
