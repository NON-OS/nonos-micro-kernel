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
use super::constants::{
    HDR_LEN, OP_SIGN_NOX_STAKE, OP_SIGN_NOX_STAKE_APPROVE, OP_SIGN_NOX_STAKE_LOCKED,
    OP_SIGN_NOX_UNSTAKE,
};
use super::eip1559_fees::eip1559_fees;
use super::push_word::push_word;

fn payload(
    owner_pid: u32,
    wallet_id: u32,
    nonce: u64,
    gas: u128,
    amount_wei: u128,
    gas_price_wei: u64,
) -> Vec<u8> {
    let (max_priority, max_fee) = eip1559_fees(gas_price_wei);
    let mut p = Vec::with_capacity(168);
    p.extend_from_slice(&owner_pid.to_le_bytes());
    p.extend_from_slice(&wallet_id.to_le_bytes());
    push_word(&mut p, nonce as u128);
    push_word(&mut p, max_priority);
    push_word(&mut p, max_fee);
    push_word(&mut p, gas);
    push_word(&mut p, amount_wei);
    p
}

fn call(port: u32, op: u16, body: &[u8]) -> Result<Vec<u8>, i32> {
    let rx = keyring_call(port, op, body, 384)?;
    if rx.len() <= HDR_LEN {
        return Err(-11);
    }
    Ok(rx[HDR_LEN..].to_vec())
}

// Step one: approve(stakingProxy, amount) on the NOX token.
pub fn sign_stake_approve(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    nonce: u64,
    amount_wei: u128,
    gas_price_wei: u64,
) -> Result<Vec<u8>, i32> {
    let body = payload(owner_pid, wallet_id, nonce, 60_000, amount_wei, gas_price_wei);
    call(port, OP_SIGN_NOX_STAKE_APPROVE, &body)
}

// Step two: stake(amount) on the staking proxy.
pub fn sign_stake(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    nonce: u64,
    amount_wei: u128,
    gas_price_wei: u64,
) -> Result<Vec<u8>, i32> {
    let body = payload(owner_pid, wallet_id, nonce, 150_000, amount_wei, gas_price_wei);
    call(port, OP_SIGN_NOX_STAKE, &body)
}

// Close a staked position by its index. The contract returns the whole
// position, so the figure carried is a position and never an amount.
pub fn sign_unstake_position(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    nonce: u64,
    position: u64,
    gas_price_wei: u64,
) -> Result<Vec<u8>, i32> {
    let body = payload(owner_pid, wallet_id, nonce, 200_000, position as u128, gas_price_wei);
    call(port, OP_SIGN_NOX_UNSTAKE, &body)
}

// Stake for a term. The lock is seconds and must be one the contract names;
// the caller picks it from the contract's own table rather than inventing a
// duration, since an unnamed term is rejected on chain.
pub fn sign_stake_locked(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    nonce: u64,
    amount_wei: u128,
    lock_seconds: u32,
    gas_price_wei: u64,
) -> Result<Vec<u8>, i32> {
    let mut body = payload(owner_pid, wallet_id, nonce, 200_000, amount_wei, gas_price_wei);
    push_word(&mut body, lock_seconds as u128);
    call(port, OP_SIGN_NOX_STAKE_LOCKED, &body)
}
