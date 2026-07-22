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

use nonos_app_skeleton::EventOutcome;

use crate::wallet::net::{probe_network, read_field};
use crate::wallet::state::State;

const STEPS: u8 = 8;

/// Refresh one field per call and advance to the next, so a tick blocks on at
/// most a single RPC round-trip. A full cycle covers the network status, the
/// ETH account (balance, nonce, fee) and the NOX reads (balance, claimable,
/// positions, protocol stats). A read that fails leaves its previous value and
/// is retried on the next cycle, so a transient drop never wipes the screen.
pub fn probe_tick(state: &mut State) -> EventOutcome {
    let step = state.probe_step;
    state.probe_step = (step + 1) % STEPS;

    if step == 0 {
        state.net = probe_network();
        state.status = state.net.status;
        return EventOutcome::Repaint;
    }
    if !state.net.rpc_chain_ok {
        return EventOutcome::Repaint;
    }
    match step {
        1 => {
            if let Some(b) = read_field::eth_balance(&state.address) {
                state.balance_wei = b;
                state.balance_ready = true;
            }
        }
        2 => {
            if let Some(n) = read_field::nonce(&state.address) {
                if !state.nonce_ready {
                    state.send_nonce = n;
                }
                state.live_nonce = n;
                state.nonce_ready = true;
            }
        }
        3 => {
            if let Some(f) = read_field::fee() {
                state.fee_wei = f;
                state.fee_ready = true;
            }
        }
        4 => {
            if let Some(b) = read_field::nox_balance(&state.address) {
                state.nox.balance_wei = b;
                state.nox.balance_ready = true;
            }
        }
        5 => {
            if let Some(c) = read_field::nox_claimable(&state.address) {
                state.nox.claimable_wei = c;
                state.nox.claimable_ready = true;
            }
        }
        6 => {
            if let Some(p) = read_field::nox_positions(&state.address) {
                state.nox.positions = p;
                state.nox.positions_ready = true;
            }
        }
        7 => {
            if let Some(s) = read_field::nox_stats() {
                state.nox.total_staked_wei = s.total;
                state.nox.rewards_distributed_wei = s.rewards;
                state.nox.stats_ready = true;
                if let Some(bps) = s.apr {
                    state.nox.apr_bps = bps;
                    state.nox.apr_ready = true;
                }
            }
        }
        _ => {}
    }
    EventOutcome::Repaint
}

/// Kick a fresh refresh cycle from the top (network status first) without
/// blocking now. The tick loop fills the fields in over the next few ticks.
pub fn probe_kick(state: &mut State) -> EventOutcome {
    state.probe_step = 0;
    EventOutcome::Repaint
}
