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

use crate::wallet::nox::{held_wei, NOX_DECIMALS};
use crate::wallet::state::State;

/// Put an exact wei figure in the stake field, as the bar and the max
/// shortcut do.
///
/// Recorded at full precision by treating every one of the token's decimals
/// as typed, so staking the whole balance stakes all of it rather than the
/// part that survives rounding to whole tokens.
pub fn set_wei(state: &mut State, wei: u128) {
    state.stake_amount = wei;
    state.stake_digits = 0;
    state.stake_places = NOX_DECIMALS;
    state.stake_point = false;
}

/// Fill in the whole balance, the one amount worth a shortcut. Does nothing
/// while the balance is unknown, since there is no whole to fill in yet.
pub fn set_max(state: &mut State) -> bool {
    match held_wei(state.nox.balance_ready, &state.nox.balance_wei) {
        Some(wei) => {
            set_wei(state, wei);
            true
        }
        None => false,
    }
}

/// Clear the field.
pub fn clear(state: &mut State) -> bool {
    set_wei(state, 0);
    true
}
