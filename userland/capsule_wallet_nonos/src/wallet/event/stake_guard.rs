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

use crate::wallet::nox::held_wei;
use crate::wallet::state::State;

use super::stake_wei::stake_wei;

/// Everything that must hold before staking signs anything, as the reason it
/// does not when it fails.
///
/// Kept ahead of the signing path rather than inside it. A transaction that
/// cannot succeed still costs gas to attempt, so the refusal belongs before
/// the signature, and it has to say which condition failed or the reader is
/// left guessing at a button that does nothing.
pub fn refusal(state: &State) -> Option<&'static [u8]> {
    if state.wallet_id == 0 {
        return Some(b"generate or import a wallet first");
    }
    if state.stake_unstake == 1 {
        return unstake_refusal(state);
    }
    if stake_wei(state) == 0 {
        return Some(b"choose an amount to stake");
    }
    match held_wei(state.nox.balance_ready, &state.nox.balance_wei) {
        None => Some(b"waiting for the NOX balance"),
        Some(held) if stake_wei(state) > held => Some(b"more NOX than this wallet holds"),
        Some(_) => None,
    }
}

/// Closing a position takes an index, not an amount, so the checks are about
/// whether that position exists rather than about a figure.
fn unstake_refusal(state: &State) -> Option<&'static [u8]> {
    if !state.nox.positions_ready {
        return Some(b"waiting for the staked positions");
    }
    if state.nox.positions == 0 {
        return Some(b"no staked positions to close");
    }
    if state.stake_position >= state.nox.positions {
        return Some(b"that position does not exist");
    }
    None
}
