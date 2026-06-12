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
use super::constants::{HDR_LEN, OP_SIGN_NOX_APPROVE};
use super::push_word::push_word;

pub fn sign_nox_approve(port: u32, owner_pid: u32, wallet_id: u32) -> Result<Vec<u8>, i32> {
    let mut payload = Vec::with_capacity(168);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&wallet_id.to_le_bytes());
    push_word(&mut payload, 0);
    push_word(&mut payload, 1_500_000_000);
    push_word(&mut payload, 30_000_000_000);
    push_word(&mut payload, 85_000);
    push_word(&mut payload, 1_000_000_000_000_000_000);
    let rx = keyring_call(port, OP_SIGN_NOX_APPROVE, &payload, 384)?;
    if rx.len() <= HDR_LEN {
        return Err(-11);
    }
    Ok(rx[HDR_LEN..].to_vec())
}
