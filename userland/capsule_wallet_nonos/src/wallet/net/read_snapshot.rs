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

//! A whole account refresh in one round trip. Every field the UI shows is put
//! into a single JSON-RPC batch and sent over one TLS connection, so a refresh
//! costs one handshake instead of one per field. A missing field in the reply
//! leaves that value untouched for the caller; a failed fetch returns None so
//! the caller can mark the link degraded and re-run the diagnostic.

use super::constants::{SERVICE_DNS, SERVICE_SOCKETS};
use crate::wallet::nox::constants::{
    NOX_TOKEN, SEL_ACTIVE_POSITIONS, SEL_BALANCE_OF, SEL_PENDING_REWARDS, SEL_PROTOCOL_STATS,
    STAKING_PROXY, STATS_EMISSION_RATE, STATS_REWARDS_DISTRIBUTED, STATS_TOTAL_STAKED,
};
use crate::wallet::nox::{apr_bps, calldata_addr, q32_to_u128};
use crate::wallet::rpc;

// Request ids, one per field, matched back out of the batch reply.
const ID_ETH_BALANCE: u64 = 2;
const ID_NONCE: u64 = 3;
const ID_FEE: u64 = 4;
const ID_NOX_BALANCE: u64 = 10;
const ID_CLAIMABLE: u64 = 11;
const ID_POSITIONS: u64 = 12;
const ID_STATS: u64 = 13;
const ID_STAKE_INFO: u64 = 14;

pub struct NoxStats {
    pub total: [u8; 32],
    pub rewards: [u8; 32],
    pub apr: Option<u64>,
}

#[derive(Default)]
pub struct Snapshot {
    pub eth_balance: Option<[u8; 32]>,
    pub nonce: Option<u64>,
    pub fee: Option<u64>,
    pub nox_balance: Option<[u8; 32]>,
    pub claimable: Option<[u8; 32]>,
    pub positions: Option<u64>,
    /// ZeroState Passes the staking contract counts for this account. Taken
    /// from getStakeInfo rather than the NFT contract, since the boost is
    /// applied from what staking itself believes.
    pub passes: Option<u64>,
    pub stats: Option<NoxStats>,
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

/// Fetch every displayed field in a single batched round trip. None means the
/// request itself did not complete, so the link is down.
pub fn read_snapshot(addr: &[u8; 20]) -> Option<Snapshot> {
    let (dns, sockets) = ports()?;

    let nox_bal = calldata_addr(&SEL_BALANCE_OF, addr);
    let claim = calldata_addr(&SEL_PENDING_REWARDS, addr);
    let positions = calldata_addr(&SEL_ACTIVE_POSITIONS, addr);
    let info = calldata_addr(&crate::wallet::nox::SEL_GET_STAKE_INFO, addr);

    let r_eth = rpc::request_balance(addr, ID_ETH_BALANCE);
    let r_nonce = rpc::request_nonce(addr, ID_NONCE);
    let r_fee = rpc::request_fee(ID_FEE);
    let r_nox = rpc::request_eth_call(&NOX_TOKEN, &nox_bal, ID_NOX_BALANCE);
    let r_claim = rpc::request_eth_call(&STAKING_PROXY, &claim, ID_CLAIMABLE);
    let r_pos = rpc::request_eth_call(&STAKING_PROXY, &positions, ID_POSITIONS);
    let r_stats = rpc::request_eth_call(&STAKING_PROXY, &SEL_PROTOCOL_STATS, ID_STATS);
    let r_info = rpc::request_eth_call(&STAKING_PROXY, &info, ID_STAKE_INFO);

    let body = rpc::request_batch(&[
        &r_eth, &r_nonce, &r_fee, &r_nox, &r_claim, &r_pos, &r_stats, &r_info,
    ]);
    let resp = super::fetch_rpc::fetch_rpc(dns, sockets, &body)?;

    let obj = |id| rpc::object_for_id(&resp, id);
    Some(Snapshot {
        eth_balance: obj(ID_ETH_BALANCE).and_then(rpc::parse_quantity32),
        nonce: obj(ID_NONCE).and_then(rpc::parse_u64),
        fee: obj(ID_FEE).and_then(rpc::parse_u64),
        nox_balance: obj(ID_NOX_BALANCE).and_then(rpc::parse_quantity32),
        claimable: obj(ID_CLAIMABLE).and_then(rpc::parse_quantity32),
        positions: obj(ID_POSITIONS)
            .and_then(rpc::parse_quantity32)
            .and_then(|w| q32_to_u128(&w))
            .map(|n| n as u64),
        // Word two of getStakeInfo is nftCount.
        passes: obj(ID_STAKE_INFO)
            .and_then(|o| rpc::parse_call_word(o, 2))
            .and_then(|w| q32_to_u128(&w))
            .map(|n| n as u64),
        stats: obj(ID_STATS).and_then(parse_stats),
    })
}

fn parse_stats(obj: &[u8]) -> Option<NoxStats> {
    let total = rpc::parse_call_word(obj, STATS_TOTAL_STAKED)?;
    let emission = rpc::parse_call_word(obj, STATS_EMISSION_RATE)?;
    let rewards = rpc::parse_call_word(obj, STATS_REWARDS_DISTRIBUTED).unwrap_or([0; 32]);
    let apr = match (q32_to_u128(&emission), q32_to_u128(&total)) {
        (Some(e), Some(t)) => apr_bps(e, t),
        _ => None,
    };
    Some(NoxStats { total, rewards, apr })
}
