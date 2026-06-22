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

use crate::crypto::zk_kernel::syscall::ZkError;
use crate::crypto::zk_kernel::{KernelZkVerifier, ZkResult};
use alloc::vec::Vec;

pub(super) fn verify_plonk(
    verifier: &mut KernelZkVerifier,
    proof_data: &[u8],
    public_input: &[u8],
) -> Result<ZkResult, ZkError> {
    if proof_data.len() < 384 {
        return Err(ZkError::MalformedInput);
    }
    let mut pub_inputs = Vec::with_capacity(public_input.len() / 32);
    for chunk in public_input.chunks_exact(32) {
        let mut inp = [0u8; 32];
        inp.copy_from_slice(chunk);
        pub_inputs.push(inp);
    }
    Ok(verifier.verify_plonk(proof_data, &pub_inputs))
}
