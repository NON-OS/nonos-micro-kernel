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

use super::constants::{SERVICE_DNS, SERVICE_SOCKETS};
use crate::wallet::nox::constants::{
    NOX_TOKEN, SEL_ACTIVE_POSITIONS, SEL_BALANCE_OF, SEL_PENDING_REWARDS, SEL_PROTOCOL_STATS,
    STAKING_PROXY, STATS_EMISSION_RATE, STATS_REWARDS_DISTRIBUTED, STATS_TOTAL_STAKED,
};
use crate::wallet::nox::{apr_bps, calldata_addr, q32_to_u128, NoxStatus};

// Read the NOX token balance and the staking position, rewards and pool stats
// for one wallet over mainnet eth_call. Each read is independent: a failure on
// one leaves its ready flag clear without poisoning the others.
pub fn probe_nox(address: &[u8; 20]) -> NoxStatus {
    let dns = super::lookup::lookup(SERVICE_DNS);
    let sockets = super::lookup::lookup(SERVICE_SOCKETS);
    let mut out = NoxStatus::empty();
    if dns == 0 || sockets == 0 {
        return out;
    }
    fill_balance(&mut out, dns, sockets, address);
    fill_claimable(&mut out, dns, sockets, address);
    fill_positions(&mut out, dns, sockets, address);
    fill_stats(&mut out, dns, sockets);
    out
}

fn eth_call(
    dns: u32,
    sockets: u32,
    to: &[u8; 20],
    data: &[u8],
    id: u64,
) -> Option<alloc::vec::Vec<u8>> {
    let body = super::super::rpc::request_eth_call(to, data, id);
    super::fetch_rpc::fetch_rpc(dns, sockets, &body)
}

fn fill_balance(out: &mut NoxStatus, dns: u32, sockets: u32, address: &[u8; 20]) {
    let data = calldata_addr(&SEL_BALANCE_OF, address);
    let Some(resp) = eth_call(dns, sockets, &NOX_TOKEN, &data, 10) else { return };
    let Some(word) = super::super::rpc::parse_quantity32(&resp) else { return };
    out.balance_wei = word;
    out.balance_ready = true;
}

fn fill_claimable(out: &mut NoxStatus, dns: u32, sockets: u32, address: &[u8; 20]) {
    let data = calldata_addr(&SEL_PENDING_REWARDS, address);
    let Some(resp) = eth_call(dns, sockets, &STAKING_PROXY, &data, 11) else { return };
    let Some(word) = super::super::rpc::parse_quantity32(&resp) else { return };
    out.claimable_wei = word;
    out.claimable_ready = true;
}

fn fill_positions(out: &mut NoxStatus, dns: u32, sockets: u32, address: &[u8; 20]) {
    let data = calldata_addr(&SEL_ACTIVE_POSITIONS, address);
    let Some(resp) = eth_call(dns, sockets, &STAKING_PROXY, &data, 12) else { return };
    let Some(word) = super::super::rpc::parse_quantity32(&resp) else { return };
    let Some(n) = q32_to_u128(&word) else { return };
    out.positions = n as u64;
    out.positions_ready = true;
}

fn fill_stats(out: &mut NoxStatus, dns: u32, sockets: u32) {
    let Some(resp) = eth_call(dns, sockets, &STAKING_PROXY, &SEL_PROTOCOL_STATS, 13) else {
        return;
    };
    let Some(total) = super::super::rpc::parse_call_word(&resp, STATS_TOTAL_STAKED) else { return };
    let Some(emission) = super::super::rpc::parse_call_word(&resp, STATS_EMISSION_RATE) else {
        return;
    };
    out.total_staked_wei = total;
    out.rewards_distributed_wei =
        super::super::rpc::parse_call_word(&resp, STATS_REWARDS_DISTRIBUTED).unwrap_or([0; 32]);
    out.stats_ready = true;
    if let (Some(e), Some(t)) = (q32_to_u128(&emission), q32_to_u128(&total)) {
        if let Some(bps) = apr_bps(e, t) {
            out.apr_bps = bps;
            out.apr_ready = true;
        }
    }
}
