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

//! One field, one connection. The incremental probe calls exactly one of these
//! per tick so the UI never blocks on a burst of round-trips, and the RPC never
//! sees a rate-limiting flood that would drop a read.

use alloc::vec::Vec;

use super::constants::{SERVICE_DNS, SERVICE_SOCKETS};
use crate::wallet::nox::constants::{
    NOX_TOKEN, SEL_ACTIVE_POSITIONS, SEL_BALANCE_OF, SEL_PENDING_REWARDS, SEL_PROTOCOL_STATS,
    STAKING_PROXY, STATS_EMISSION_RATE, STATS_REWARDS_DISTRIBUTED, STATS_TOTAL_STAKED,
};
use crate::wallet::nox::{apr_bps, calldata_addr, q32_to_u128};

/// Protocol-wide staking figures returned by a single stats call.
pub struct NoxStats {
    pub total: [u8; 32],
    pub rewards: [u8; 32],
    pub apr: Option<u64>,
}

fn ports() -> Option<(u32, u32)> {
    let dns = super::lookup::lookup(SERVICE_DNS);
    let sockets = super::lookup::lookup(SERVICE_SOCKETS);
    if dns == 0 || sockets == 0 {
        None
    } else {
        Some((dns, sockets))
    }
}

fn call(body: &[u8]) -> Option<Vec<u8>> {
    let (dns, sockets) = ports()?;
    super::fetch_rpc::fetch_rpc(dns, sockets, body)
}

pub fn eth_balance(addr: &[u8; 20]) -> Option<[u8; 32]> {
    let resp = call(&super::super::rpc::request_balance(addr, 2))?;
    super::super::rpc::parse_quantity32(&resp)
}

pub fn nonce(addr: &[u8; 20]) -> Option<u64> {
    let resp = call(&super::super::rpc::request_nonce(addr, 3))?;
    super::super::rpc::parse_u64(&resp)
}

pub fn fee() -> Option<u64> {
    let resp = call(&super::super::rpc::request_fee(4))?;
    super::super::rpc::parse_u64(&resp)
}

pub fn nox_balance(addr: &[u8; 20]) -> Option<[u8; 32]> {
    let data = calldata_addr(&SEL_BALANCE_OF, addr);
    let resp = call(&super::super::rpc::request_eth_call(&NOX_TOKEN, &data, 10))?;
    super::super::rpc::parse_quantity32(&resp)
}

pub fn nox_claimable(addr: &[u8; 20]) -> Option<[u8; 32]> {
    let data = calldata_addr(&SEL_PENDING_REWARDS, addr);
    let resp = call(&super::super::rpc::request_eth_call(&STAKING_PROXY, &data, 11))?;
    super::super::rpc::parse_quantity32(&resp)
}

pub fn nox_positions(addr: &[u8; 20]) -> Option<u64> {
    let data = calldata_addr(&SEL_ACTIVE_POSITIONS, addr);
    let resp = call(&super::super::rpc::request_eth_call(&STAKING_PROXY, &data, 12))?;
    let word = super::super::rpc::parse_quantity32(&resp)?;
    q32_to_u128(&word).map(|n| n as u64)
}

pub fn nox_stats() -> Option<NoxStats> {
    let resp = call(&super::super::rpc::request_eth_call(&STAKING_PROXY, &SEL_PROTOCOL_STATS, 13))?;
    let total = super::super::rpc::parse_call_word(&resp, STATS_TOTAL_STAKED)?;
    let emission = super::super::rpc::parse_call_word(&resp, STATS_EMISSION_RATE)?;
    let rewards =
        super::super::rpc::parse_call_word(&resp, STATS_REWARDS_DISTRIBUTED).unwrap_or([0; 32]);
    let apr = match (q32_to_u128(&emission), q32_to_u128(&total)) {
        (Some(e), Some(t)) => apr_bps(e, t),
        _ => None,
    };
    Some(NoxStats { total, rewards, apr })
}
