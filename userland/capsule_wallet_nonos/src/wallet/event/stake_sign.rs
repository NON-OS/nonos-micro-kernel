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

use alloc::vec::Vec;

use crate::wallet::ipc::{sign_stake, sign_stake_approve, sign_stake_locked};
use crate::wallet::nox::LOCK_TERMS;
use crate::wallet::state::State;

/// Which transaction the staking button signs next.
///
/// Step zero is always the approve, since the pool cannot take tokens it has
/// no allowance for. After that the chosen term decides: no term is the plain
/// stake, and any other is `stakeLocked`, which commits the stake for that
/// many seconds in exchange for a heavier weight.
pub fn sign_next(state: &State, amount: u128) -> Result<Vec<u8>, i32> {
    let lock = LOCK_TERMS[(state.stake_lock as usize).min(LOCK_TERMS.len() - 1)].0;
    if state.stake_step == 0 {
        return sign_stake_approve(
            state.keyring_port,
            state.owner_pid,
            state.wallet_id,
            state.live_nonce,
            amount,
            state.fee_wei,
        );
    }
    if lock == 0 {
        return sign_stake(
            state.keyring_port,
            state.owner_pid,
            state.wallet_id,
            state.live_nonce,
            amount,
            state.fee_wei,
        );
    }
    sign_stake_locked(
        state.keyring_port,
        state.owner_pid,
        state.wallet_id,
        state.live_nonce,
        amount,
        lock,
        state.fee_wei,
    )
}
