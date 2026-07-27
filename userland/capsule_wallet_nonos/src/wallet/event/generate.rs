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

use crate::wallet::ipc::{generate_wallet_hd, wallet_address};
use crate::wallet::state::State;

// Generation asks the keyring for a fresh HD wallet: real entropy becomes a
// BIP39 phrase, the phrase becomes the m/44'/60'/0'/0/0 account. The response
// carries the words exactly once for the backup screen. A draw can stall
// briefly while the entropy source reseeds, so retry a few times before
// reporting the real failure.
const ATTEMPTS: u32 = 3;

pub fn generate(state: &mut State) -> EventOutcome {
    let mut id = 0u32;
    let mut count = 0u8;
    let mut ok = false;
    let mut last_err = 0i32;
    for _ in 0..ATTEMPTS {
        match generate_wallet_hd(state.keyring_port, state.owner_pid, &mut state.backup_words) {
            Ok((new_id, word_count)) => {
                id = new_id;
                count = word_count;
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
        super::backup::wipe_backup(state);
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
            // Show the one-time backup screen before anything else; the words
            // are wiped the moment the user confirms they are written down.
            state.backup_count = count;
            state.backup_active = true;
            state.status = b"wallet created, write down the phrase";
            super::probe_tick::probe_kick(state)
        }
        Err(_) => {
            super::backup::wipe_backup(state);
            state.status = b"address failed";
            EventOutcome::Repaint
        }
    }
}
