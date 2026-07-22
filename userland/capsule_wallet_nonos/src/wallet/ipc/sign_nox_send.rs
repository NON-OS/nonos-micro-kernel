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
use super::constants::{HDR_LEN, OP_SIGN_NOX_TRANSFER};
use super::eip1559_fees::eip1559_fees;
use super::push_word::push_word;

// Sign transfer(to, amount) on the NOX token to send NOX to another address.
pub fn sign_nox_transfer(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    nonce: u64,
    to: [u8; 20],
    amount_wei: u128,
    gas_price_wei: u64,
) -> Result<Vec<u8>, i32> {
    let (max_priority, max_fee) = eip1559_fees(gas_price_wei);
    let mut p = Vec::with_capacity(188);
    p.extend_from_slice(&owner_pid.to_le_bytes());
    p.extend_from_slice(&wallet_id.to_le_bytes());
    push_word(&mut p, nonce as u128);
    push_word(&mut p, max_priority);
    push_word(&mut p, max_fee);
    push_word(&mut p, 90_000);
    p.extend_from_slice(&to);
    push_word(&mut p, amount_wei);
    let rx = keyring_call(port, OP_SIGN_NOX_TRANSFER, &p, 384)?;
    if rx.len() <= HDR_LEN {
        return Err(-11);
    }
    Ok(rx[HDR_LEN..].to_vec())
}
