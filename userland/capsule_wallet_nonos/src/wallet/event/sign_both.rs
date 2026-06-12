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

use crate::wallet::ipc::{sign_eth_transfer, sign_nox_approve};
use crate::wallet::state::{record_tx, State};
use crate::wallet::tx_hash::tx_hash;

pub fn sign_both(state: &mut State) -> EventOutcome {
    if state.wallet_id == 0 {
        state.status = b"generate wallet first";
        return EventOutcome::Repaint;
    }
    let Ok(eth) = sign_eth_transfer(state.keyring_port, state.owner_pid, state.wallet_id) else {
        state.status = b"ETH proof failed";
        return EventOutcome::Repaint;
    };
    let Ok(nox) = sign_nox_approve(state.keyring_port, state.owner_pid, state.wallet_id) else {
        state.status = b"NOX proof failed";
        return EventOutcome::Repaint;
    };
    let mut hash = [0u8; 32];
    if !tx_hash(&eth, &mut hash) || !tx_hash(&nox, &mut hash) {
        state.status = b"proof hash failed";
        return EventOutcome::Repaint;
    }
    record_tx(state, b"NOX", &nox, hash);
    state.proof_count = 2;
    state.status = b"2 tx proofs ready";
    EventOutcome::Repaint
}
