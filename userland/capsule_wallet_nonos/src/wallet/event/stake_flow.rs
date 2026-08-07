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

use crate::wallet::state::{record_tx, State};
use crate::wallet::tx_hash::tx_hash;

// The full staking lifecycle in one button. Step 0 signs and broadcasts
// approve(stakingProxy, amount); once it lands the button advances to step 1,
// which signs and broadcasts stake(amount). Real amount, live nonce, live fee.
// Both transactions are ownership-gated signatures from the keyring.
pub fn stake_flow(state: &mut State) -> EventOutcome {
    if let Some(why) = super::stake_guard::refusal(state) {
        state.status = why;
        return EventOutcome::Repaint;
    }
    // Unstaking closes a position rather than moving an amount, so it needs
    // neither the approve step nor a figure: the position index is what the
    // contract is given.
    if state.stake_unstake == 1 {
        return super::unstake_flow::unstake_flow(state);
    }
    let amount = super::stake_wei::stake_wei(state);
    let step = state.stake_step;
    let raw = super::stake_sign::sign_next(state, amount);
    let Ok(raw) = raw else {
        state.status = b"stake sign failed";
        return EventOutcome::Repaint;
    };
    let mut hash = [0u8; 32];
    if !tx_hash(&raw, &mut hash) {
        state.status = b"stake hash failed";
        return EventOutcome::Repaint;
    }
    let kind: &'static [u8] = if step == 0 { b"APPROVE" } else { b"STAKE" };
    record_tx(state, kind, &raw, hash);
    let out = super::broadcast::broadcast(state);
    if state.broadcast_ready {
        if step == 0 {
            state.stake_step = 1;
            state.status = b"approve sent, press Stake once it confirms";
        } else {
            state.stake_step = 0;
            state.status = b"stake sent";
        }
    }
    out
}
