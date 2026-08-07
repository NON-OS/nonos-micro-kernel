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

use crate::wallet::ipc::sign_unstake_position;
use crate::wallet::state::{record_tx, State};
use crate::wallet::tx_hash::tx_hash;

/// Close a staked position.
///
/// One transaction, not two: the approve exists so the pool may take tokens
/// in, and nothing needs approving to give them back. The position index is
/// the whole of what the contract is told, which is why this path carries no
/// amount at all.
pub fn unstake_flow(state: &mut State) -> EventOutcome {
    let raw = sign_unstake_position(
        state.keyring_port,
        state.owner_pid,
        state.wallet_id,
        state.live_nonce,
        state.stake_position,
        state.fee_wei,
    );
    let Ok(raw) = raw else {
        state.status = b"unstake sign failed";
        return EventOutcome::Repaint;
    };
    let mut hash = [0u8; 32];
    if !tx_hash(&raw, &mut hash) {
        state.status = b"unstake hash failed";
        return EventOutcome::Repaint;
    }
    record_tx(state, b"UNSTAKE", &raw, hash);
    let out = super::broadcast::broadcast(state);
    if state.broadcast_ready {
        state.status = b"unstake sent";
    }
    out
}
