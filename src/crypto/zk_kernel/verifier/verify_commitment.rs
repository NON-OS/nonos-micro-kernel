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

use super::{KernelZkVerifier, ZkResult};
use crate::crypto::zk_kernel::pedersen::PedersenCommitment;

impl KernelZkVerifier {
    pub fn verify_commitment(
        &mut self,
        commitment: &[u8; 32],
        value: &[u8; 32],
        blinding: &[u8; 32],
    ) -> ZkResult {
        let comm = PedersenCommitment { commitment: *commitment };
        self.proofs_verified += 1;
        if comm.verify(value, blinding) {
            self.proofs_valid += 1;
            ZkResult::Valid
        } else {
            self.proofs_invalid += 1;
            ZkResult::Invalid
        }
    }
}
