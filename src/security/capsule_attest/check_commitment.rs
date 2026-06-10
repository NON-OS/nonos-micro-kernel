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

use super::commitment::commitment;
use super::error::AttestError;
use super::layout::POLICY_EPOCH;

pub(super) fn check_commitment(
    capsule: &[u8; 32],
    policy: &[u8; 32],
    caps: u64,
    trailer: &[u8; 32],
    public: &[u8; 32],
) -> Result<(), AttestError> {
    if trailer != public {
        return Err(AttestError::CommitmentMismatch);
    }
    if &commitment(capsule, policy, POLICY_EPOCH, caps) != public {
        return Err(AttestError::CommitmentMismatch);
    }
    Ok(())
}
