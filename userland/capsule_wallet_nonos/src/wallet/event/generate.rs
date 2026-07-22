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
const ATTEMPTS: u32 = 3;

pub fn generate(state: &mut State) -> EventOutcome {
    let mut id = 0u32;
    let mut ok = false;
    let mut last_err = 0i32;
    for _ in 0..ATTEMPTS {
        match generate_wallet(state.keyring_port, state.owner_pid) {
            Ok(new_id) => {
                id = new_id;
                ok = true;
                break;
            }
            Err(e) => last_err = e,
        }
        for _ in 0..300 {
            nonos_libc::mk_yield();
        }
    }
    if !ok {
        // Name the real reason instead of always blaming entropy: the keyring
        // returns EACCES (-13) when the caller identity does not match, ENOSPC
        // (-28) when its slots are full, and -11 when it cannot be reached.
        state.status = match last_err {
            -13 => b"generate blocked: keyring rejected caller".as_slice(),
            -28 => b"keyring is full".as_slice(),
            -11 => b"keyring unreachable".as_slice(),
            _ => b"generate failed: no entropy source".as_slice(),
        };
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
