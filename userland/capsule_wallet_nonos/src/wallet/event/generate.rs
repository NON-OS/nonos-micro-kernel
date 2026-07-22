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

// Generation asks the keyring for a fresh key, which draws from the entropy
// pool. That draw can briefly stall while the pool reseeds, so a single attempt
// sometimes returns an error. Retry a handful of times, yielding between tries
// so the entropy source can recover, before reporting failure.
const ATTEMPTS: u32 = 6;

pub fn generate(state: &mut State) -> EventOutcome {
    let mut id = 0u32;
    let mut ok = false;
    for _ in 0..ATTEMPTS {
        if let Ok(new_id) = generate_wallet(state.keyring_port, state.owner_pid) {
            id = new_id;
            ok = true;
            break;
        }
        for _ in 0..2000 {
            nonos_libc::mk_yield();
        }
    }
    if !ok {
        state.status = b"entropy unavailable, try again";
        return EventOutcome::Repaint;
    }
    match wallet_address(state.keyring_port, state.owner_pid, id) {
        Ok(addr) => {
            state.wallet_id = id;
            state.address = addr;
            state.address_ready = true;
            state.view = crate::wallet::state::VIEW_RECEIVE;
            state.status = b"wallet generated";
            // Immediately pull live account state (balance/nonce/fee) over the
            // RPC stack so the wallet is usable the moment it exists.
            super::probe_tick::probe_kick(state)
        }
        Err(_) => {
            state.status = b"address failed";
            EventOutcome::Repaint
        }
    }
}
