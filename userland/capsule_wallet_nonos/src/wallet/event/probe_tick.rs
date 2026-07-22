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
///
/// Returns Repaint only when the fetched value actually differs from what is
/// already shown, so a steady balance or fee does not trigger a full-window
/// recomposite every cycle and compete with the user's input for the core.
pub fn probe_tick(state: &mut State) -> EventOutcome {
    let step = state.probe_step;
    state.probe_step = (step + 1) % STEPS;

    if step == 0 {
        let net = probe_network();
        let changed = net.status != state.status || !state.net.rpc_chain_ok && net.rpc_chain_ok;
        state.net = net;
        state.status = state.net.status;
        return outcome(changed);
    }
    if !state.net.rpc_chain_ok {
        return EventOutcome::Idle;
    }
    // Balances first (steps 1-2) so the headline figures refresh soonest, then
    // nonce, fee, and the staking reads. Each field repaints only on a change.
    let changed = match step {
        1 => read_field::eth_balance(&state.address)
            .map(|b| {
                let c = !state.balance_ready || b != state.balance_wei;
                state.balance_wei = b;
                state.balance_ready = true;
                c
            })
            .unwrap_or(false),
        2 => read_field::nox_balance(&state.address)
            .map(|b| {
                let c = !state.nox.balance_ready || b != state.nox.balance_wei;
                state.nox.balance_wei = b;
                state.nox.balance_ready = true;
                c
            })
            .unwrap_or(false),
        3 => read_field::nonce(&state.address)
            .map(|n| {
                let c = !state.nonce_ready || n != state.live_nonce;
                if !state.nonce_ready {
                    state.send_nonce = n;
                }
                state.live_nonce = n;
                state.nonce_ready = true;
                c
            })
            .unwrap_or(false),
        4 => read_field::fee()
            .map(|f| {
                let c = !state.fee_ready || f != state.fee_wei;
                state.fee_wei = f;
                state.fee_ready = true;
                c
            })
            .unwrap_or(false),
        5 => read_field::nox_claimable(&state.address)
            .map(|c| {
                let ch = !state.nox.claimable_ready || c != state.nox.claimable_wei;
                state.nox.claimable_wei = c;
                state.nox.claimable_ready = true;
                ch
            })
            .unwrap_or(false),
        6 => read_field::nox_positions(&state.address)
            .map(|p| {
                let c = !state.nox.positions_ready || p != state.nox.positions;
                state.nox.positions = p;
                state.nox.positions_ready = true;
                c
            })
            .unwrap_or(false),
        7 => read_field::nox_stats()
            .map(|s| {
                let c = !state.nox.stats_ready
                    || s.total != state.nox.total_staked_wei
                    || s.rewards != state.nox.rewards_distributed_wei;
                state.nox.total_staked_wei = s.total;
                state.nox.rewards_distributed_wei = s.rewards;
                state.nox.stats_ready = true;
                if let Some(bps) = s.apr {
                    state.nox.apr_bps = bps;
                    state.nox.apr_ready = true;
                }
                c
            })
            .unwrap_or(false),
        _ => false,
    };
    outcome(changed)
}

fn outcome(changed: bool) -> EventOutcome {
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

/// Kick a fresh refresh cycle from the top (network status first) without
/// blocking now. The tick loop fills the fields in over the next few ticks.
pub fn probe_kick(state: &mut State) -> EventOutcome {
    state.probe_step = 0;
    EventOutcome::Repaint
}
