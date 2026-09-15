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

//! Bringing back the wallet this machine kept.
//!
//! Runs once, when a window comes up with no wallet of its own. There are
//! three outcomes and they are different things: there was no vault, which is
//! an ordinary new machine; there was one and it opened, which is a wallet
//! back without a word typed; and there was one and it would not open, which
//! means this is not the machine that sealed it, or not the state it was
//! sealed in. The last of those is the only one worth a line of status, and
//! it must not read like a lost wallet, because the phrase still works.

use crate::wallet::ipc::wallet_address;
use crate::wallet::state::State;
use crate::wallet::vault::{recall, Recall};

pub fn restore(state: &mut State) {
    if state.wallet_id != 0 || state.vault_restore_tried {
        return;
    }
    let id = match recall(state.keyring_port, state.owner_pid) {
        Recall::Wallet(id) => id,
        Recall::Nothing => {
            state.vault_restore_tried = true;
            return;
        }
        /*
         * Nothing is settled, so nothing is latched. The next hydrate asks
         * again, and the store is usually up within a second or two of boot.
         */
        Recall::NotYet => return,
        /*
         * There is a wallet on this disk and it is not this machine's to open.
         * Saying so matters: without it the window looks like a machine that
         * never had a wallet, and the next thing the user does is make one,
         * over the top of a vault they may still have the phrase for.
         */
        Recall::Sealed { machine_changed } => {
            state.vault_restore_tried = true;
            state.status = super::restore_words::sealed(machine_changed);
            return;
        }
    };
    state.vault_restore_tried = true;
    match wallet_address(state.keyring_port, state.owner_pid, id) {
        Ok(addr) => {
            state.wallet_id = id;
            state.address = addr;
            state.address_ready = true;
            state.vault_saved = true;
            state.status = b"wallet restored from this machine";
        }
        /*
         * The keyring took the key but will not name it, which is a keyring
         * fault rather than a vault one; leave the wallet unset rather than
         * show an id with no address behind it.
         */
        Err(_) => state.status = b"vault opened but the address could not be read",
    }
}
