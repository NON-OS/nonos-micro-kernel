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

use crate::wallet::state::State;

// One press signs the transfer and, if the signature succeeded, broadcasts it.
// The button reads "Sign & send", so a click does exactly that: build and sign
// the EIP-1559 transfer in the keyring, then push the raw transaction over the
// RPC. A failure at either step leaves a clear status and stops.
pub fn send_now(state: &mut State) -> EventOutcome {
    let signed = super::sign_eth::sign_eth(state);
    if state.tx_ready && !state.tx_raw.is_empty() {
        return super::broadcast::broadcast(state);
    }
    signed
}
