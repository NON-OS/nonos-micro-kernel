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

use super::consts::{APPROVE_SELECTOR, STAKE_SELECTOR, STAKING_PROXY};

// approve(stakingProxy, amount) on the NOX token: authorise the staking contract
// to move the amount being staked. Selector, spender right-aligned in a word,
// then the amount.
pub fn stake_approve_calldata(amount: &[u8; 32]) -> [u8; 68] {
    let mut out = [0u8; 68];
    out[0..4].copy_from_slice(&APPROVE_SELECTOR);
    out[16..36].copy_from_slice(&STAKING_PROXY);
    out[36..68].copy_from_slice(amount);
    out
}

// stake(amount) on the staking proxy: selector then the amount word.
pub fn stake_calldata(amount: &[u8; 32]) -> [u8; 36] {
    let mut out = [0u8; 36];
    out[0..4].copy_from_slice(&STAKE_SELECTOR);
    out[4..36].copy_from_slice(amount);
    out
}
