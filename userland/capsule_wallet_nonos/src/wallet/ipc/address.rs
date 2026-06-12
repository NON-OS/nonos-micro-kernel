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

use alloc::vec::Vec;

use super::call::keyring_call;
use super::constants::{HDR_LEN, OP_WALLET_ADDRESS};

pub fn wallet_address(port: u32, owner_pid: u32, wallet_id: u32) -> Result<[u8; 20], i32> {
    let mut payload = Vec::with_capacity(8);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&wallet_id.to_le_bytes());
    let rx = keyring_call(port, OP_WALLET_ADDRESS, &payload, 20)?;
    if rx.len() < HDR_LEN + 20 {
        return Err(-11);
    }
    let mut out = [0u8; 20];
    out.copy_from_slice(&rx[HDR_LEN..HDR_LEN + 20]);
    Ok(out)
}
