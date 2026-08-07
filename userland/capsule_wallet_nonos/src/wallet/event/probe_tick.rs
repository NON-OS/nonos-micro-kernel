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

use crate::wallet::net::probe_network;
use crate::wallet::net::read_snapshot::{read_snapshot, Snapshot};
use crate::wallet::state::State;

/// Refresh the account. The expensive network self-diagnostic (DNS, sockets,
/// the full TLS check) runs once to confirm the link; after that every refresh
/// is a single batched round trip that fetches balance, nonce, fee and the
/// staking figures over one connection. Returns Repaint only when a shown value
/// actually changed, so a steady screen does not recomposite every cycle. On a
/// failed fetch the link is marked degraded so the next tick re-runs the
/// diagnostic and reconnects.
pub fn probe_tick(state: &mut State) -> EventOutcome {
    if !state.net.rpc_chain_ok {
        let net = probe_network();
        let changed = net.status != state.status;
        state.net = net;
        state.status = state.net.status;
        return outcome(changed);
    }

    match read_snapshot(&state.address) {
        Some(snap) => outcome(apply(state, snap)),
        None => {
            // The link dropped; fall back to the diagnostic on the next tick.
            state.net.rpc_chain_ok = false;
            state.status = b"reconnecting to the network";
            EventOutcome::Repaint
        }
    }
}

/// Apply a snapshot, returning whether anything shown changed.
fn apply(state: &mut State, snap: Snapshot) -> bool {
    let mut changed = false;
    if let Some(b) = snap.eth_balance {
        changed |= !state.balance_ready || b != state.balance_wei;
        state.balance_wei = b;
        state.balance_ready = true;
    }
    if let Some(n) = snap.nonce {
        changed |= !state.nonce_ready || n != state.live_nonce;
        if !state.nonce_ready {
            state.send_nonce = n;
        }
        state.live_nonce = n;
        state.nonce_ready = true;
    }
    if let Some(f) = snap.fee {
        changed |= !state.fee_ready || f != state.fee_wei;
        state.fee_wei = f;
        state.fee_ready = true;
    }
    if let Some(b) = snap.nox_balance {
        changed |= !state.nox.balance_ready || b != state.nox.balance_wei;
        state.nox.balance_wei = b;
        state.nox.balance_ready = true;
    }
    if let Some(c) = snap.claimable {
        changed |= !state.nox.claimable_ready || c != state.nox.claimable_wei;
        state.nox.claimable_wei = c;
        state.nox.claimable_ready = true;
    }
    if let Some(p) = snap.positions {
        changed |= !state.nox.positions_ready || p != state.nox.positions;
        state.nox.positions = p;
        state.nox.positions_ready = true;
    }
    if let Some(n) = snap.passes {
        changed |= !state.nox.passes_ready || n != state.nox.passes;
        state.nox.passes = n;
        state.nox.passes_ready = true;
    }
    if let Some(s) = snap.stats {
        changed |= !state.nox.stats_ready
            || s.total != state.nox.total_staked_wei
            || s.rewards != state.nox.rewards_distributed_wei;
        state.nox.total_staked_wei = s.total;
        state.nox.rewards_distributed_wei = s.rewards;
        state.nox.stats_ready = true;
        if let Some(bps) = s.apr {
            state.nox.apr_bps = bps;
            state.nox.apr_ready = true;
        }
    }
    changed
}

fn outcome(changed: bool) -> EventOutcome {
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

/// Kick a fresh refresh from the top on the next idle tick without blocking now.
pub fn probe_kick(state: &mut State) -> EventOutcome {
    state.probe_step = 0;
    EventOutcome::Repaint
}
