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
use crate::crypto::zk_kernel::plonk::plonk_prove;
use alloc::vec::Vec;

pub fn syscall_zk_prove_plonk(witness: &[[u8; 32]]) -> Result<Vec<u8>, ZkError> {
    match plonk_prove(witness) {
        Ok(proof) => Ok(proof.to_bytes()),
        Err(_) => Err(ZkError::InternalError),
    }
}
