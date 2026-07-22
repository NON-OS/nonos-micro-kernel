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

use crate::wallet::ipc::sign_nox_transfer;
use crate::wallet::state::State;

// One thousandth of an 18-decimal token, so a milli-unit amount becomes wei.
const WEI_PER_MILLI: u128 = 1_000_000_000_000_000;

// One press signs the transfer and, if the signature succeeded, broadcasts it.
// "Sign & send" builds and signs the EIP-1559 transfer in the keyring, then
// pushes the raw transaction over the RPC. The asset toggle picks a plain ETH
// value transfer or an ERC-20 transfer of NOX. A failure at either step leaves a
// clear status and stops.
pub fn send_now(state: &mut State) -> EventOutcome {
    if state.send_token == 1 {
        sign_nox(state);
    } else {
        let _ = super::sign_eth::sign_eth(state);
    }
    if state.tx_ready && !state.tx_raw.is_empty() {
        return super::broadcast::broadcast(state);
    }
    EventOutcome::Repaint
}

fn sign_nox(state: &mut State) {
    if state.wallet_id == 0 {
        state.status = b"generate or import a wallet first";
        return;
    }
    let Some(to) = super::recipient::recipient(state) else {
        state.status = b"recipient incomplete";
        return;
    };
    let amount = state.send_amount_milli_eth as u128 * WEI_PER_MILLI;
    if amount == 0 {
        state.status = b"amount too small";
        return;
    }
    // Fresh nonce and fee at send time, or refuse rather than sign a bad tx.
    if !super::tx_freshen::freshen_nonce_and_fee(state) {
        state.status = b"cannot reach network for nonce and fee, try again";
        return;
    }
    let raw = sign_nox_transfer(
        state.keyring_port,
        state.owner_pid,
        state.wallet_id,
        state.live_nonce,
        to,
        amount,
        state.fee_wei,
    );
    super::sign_result::sign_result(state, b"NOX", raw);
}
