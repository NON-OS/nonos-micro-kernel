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

//! The one place a new wallet is written down.
//!
//! Called from generate, import and recover so the three paths cannot drift
//! apart, and so the status line says the same thing about a machine that
//! cannot keep a wallet no matter how the wallet arrived.

use crate::wallet::state::State;
use crate::wallet::vault::remember;

/// Seal the wallet to this machine and store it, then say which happened.
///
/// A failure here is not a failed wallet. The keys are in the keyring and the
/// session works; what is lost is the next boot, and the words on the backup
/// screen are still the way back. Saying so plainly is better than a silent
/// success the user only discovers is false after a reboot.
pub fn keep(state: &mut State) {
    if remember(state.keyring_port, state.owner_pid, state.wallet_id) {
        state.vault_saved = true;
        return;
    }
    state.vault_saved = false;
    state.status = b"this machine cannot keep a wallet, so write down the phrase";
}
