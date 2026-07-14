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

use super::state::{next_epoch, CLAIMS};
use super::types::{Claim, ClaimError};

// Register a claim. Returns the granted epoch on success. The caller
// must have already verified that `device_id` exists in the broker
// table.
pub fn claim(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    let epoch = {
        let mut claims = CLAIMS.lock();
        if claims.iter().any(|c| c.device_id == device_id) {
            return Err(ClaimError::AlreadyClaimed);
        }
        let epoch = next_epoch();
        claims.push(Claim { pid, device_id, epoch });
        epoch
    };
    // Bring the device to power state D0 before its driver maps MMIO. Done
    // outside the claims lock: it touches config space and settles for a moment.
    crate::hardware::broker::power::power_on_device(device_id);
    Ok(epoch)
}
