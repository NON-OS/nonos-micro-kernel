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

use super::state::CLAIMS;
use super::types::ClaimError;

// Release a claim held by `pid`. Returns the released epoch on
// success.
pub fn release(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    let mut claims = CLAIMS.lock();
    let idx = claims.iter().position(|c| c.device_id == device_id).ok_or(ClaimError::NotClaimed)?;
    if claims[idx].pid != pid {
        return Err(ClaimError::NotHolder);
    }
    let epoch = claims[idx].epoch;
    claims.remove(idx);
    Ok(epoch)
}

// Release every claim held by `pid`. Called from the kernel's
// `MkExit` path so a dying capsule cannot leak grants. Returns the
// number of claims revoked.
pub fn release_all_for_pid(pid: u32) -> usize {
    let mut claims = CLAIMS.lock();
    let before = claims.len();
    claims.retain(|c| c.pid != pid);
    before - claims.len()
}
