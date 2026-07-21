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
use crate::wallet::state::State;

pub fn probe_net(state: &mut State) -> EventOutcome {
    state.net = probe_network();
    if state.address_ready && state.net.rpc_chain_ok {
        let account = crate::wallet::net::probe_account(&state.address);
        state.balance_ready = account.balance_ready;
        state.balance_wei = account.balance_wei;
        state.nonce_ready = account.nonce_ready;
        state.live_nonce = account.nonce;
        state.fee_ready = account.fee_ready;
        state.fee_wei = account.fee_wei;
        state.nox = crate::wallet::net::probe_nox(&state.address);
    }
    state.status = state.net.status;
    EventOutcome::Repaint
}
