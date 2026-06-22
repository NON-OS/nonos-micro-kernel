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

use super::ZkError;
use crate::crypto::rng::get_random_bytes;
use crate::crypto::zk_kernel::pedersen::PedersenCommitment;

pub fn syscall_zk_commit(value: &[u8; 32]) -> Result<([u8; 32], [u8; 32]), ZkError> {
    let blinding = get_random_bytes();
    let comm = PedersenCommitment::commit(value, &blinding);
    Ok((comm.commitment, blinding))
}
