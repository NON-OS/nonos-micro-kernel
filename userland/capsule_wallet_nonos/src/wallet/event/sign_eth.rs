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

use crate::wallet::ipc::sign_eth_transfer;
use crate::wallet::state::State;

pub fn sign_eth(state: &mut State) -> EventOutcome {
    if state.wallet_id == 0 {
        state.status = b"generate wallet first";
        return EventOutcome::Repaint;
    }
    let Some(to) = super::recipient::recipient(state) else {
        state.status = b"recipient incomplete";
        return EventOutcome::Repaint;
    };
    let Some(value) = super::eth_value::eth_value_wei(state.send_amount_milli_eth) else {
        state.status = b"amount too large";
        return EventOutcome::Repaint;
    };
    // Fresh nonce and fee at send time, or refuse rather than sign a bad tx.
    if !super::tx_freshen::freshen_nonce_and_fee(state) {
        state.status = b"cannot reach network for nonce and fee, try again";
        return EventOutcome::Repaint;
    }
    let raw = sign_eth_transfer(
        state.keyring_port,
        state.owner_pid,
        state.wallet_id,
        to,
        state.send_nonce,
        value,
        state.fee_wei,
    );
    super::sign_result::sign_result(state, b"ETH", raw)
}
