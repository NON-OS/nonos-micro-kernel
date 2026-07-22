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
use super::constants::{HDR_LEN, OP_SIGN_ETH_TRANSFER};
use super::eip1559_fees::eip1559_fees;
use super::push_word::push_word;

pub fn sign_eth_transfer(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    to: [u8; 20],
    nonce: u64,
    value_wei: u64,
    gas_price_wei: u64,
) -> Result<Vec<u8>, i32> {
    let (max_priority, max_fee) = eip1559_fees(gas_price_wei);
    let mut payload = Vec::with_capacity(188);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&wallet_id.to_le_bytes());
    payload.extend_from_slice(&to);
    push_word(&mut payload, nonce as u128);
    push_word(&mut payload, max_priority);
    push_word(&mut payload, max_fee);
    // A plain value transfer to an account always costs exactly 21000 gas.
    push_word(&mut payload, 21_000);
    push_word(&mut payload, value_wei as u128);
    let rx = keyring_call(port, OP_SIGN_ETH_TRANSFER, &payload, 256)?;
    if rx.len() <= HDR_LEN {
        return Err(-11);
    }
    Ok(rx[HDR_LEN..].to_vec())
}
