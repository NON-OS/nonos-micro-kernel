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

use crate::wallet::ipc::{generate_wallet, wallet_address};
use crate::wallet::state::State;

pub fn generate(state: &mut State) -> EventOutcome {
    match generate_wallet(state.keyring_port, state.owner_pid) {
        Ok(id) => match wallet_address(state.keyring_port, state.owner_pid, id) {
            Ok(addr) => {
                state.wallet_id = id;
                state.address = addr;
                state.address_ready = true;
                state.view = crate::wallet::state::VIEW_RECEIVE;
                state.status = b"wallet generated";
                // Immediately pull live account state (balance/nonce/fee) over
                // the RPC stack so the wallet is usable the moment it exists.
                super::probe_tick::probe_kick(state)
            }
            Err(_) => {
                state.status = b"address failed";
                EventOutcome::Repaint
            }
        },
        Err(_) => {
            state.status = b"generate failed";
            EventOutcome::Repaint
        }
    }
}
