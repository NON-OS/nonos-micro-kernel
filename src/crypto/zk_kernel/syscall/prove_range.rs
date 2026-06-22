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

use super::ZkError;
use crate::crypto::zk_kernel::range::RangeProof;
use alloc::vec::Vec;

pub fn syscall_zk_prove_range(value: u64, bits: u8) -> Result<Vec<u8>, ZkError> {
    match RangeProof::prove(value, bits) {
        Ok(proof) => {
            let mut result = Vec::with_capacity(1 + 32 + proof.bit_commitments.len() * 32);
            result.push(proof.bits);
            result.extend_from_slice(&proof.response);
            for comm in &proof.bit_commitments {
                result.extend_from_slice(comm);
            }
            Ok(result)
        }
        Err(_) => Err(ZkError::InternalError),
    }
}
