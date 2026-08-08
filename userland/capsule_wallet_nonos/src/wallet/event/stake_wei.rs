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

use crate::wallet::nox::NOX_DECIMALS;
use crate::wallet::state::State;

/// The typed stake amount as the chain counts it.
///
/// A reader types whole tokens with a fraction; the contract wants wei. The
/// figure and its decimal places are kept exactly as typed and scaled only
/// here, so nothing rounds on the way in and the amount signed is the amount
/// shown.
pub fn stake_wei(state: &State) -> u128 {
    let places = state.stake_places.min(NOX_DECIMALS);
    let mut v = state.stake_amount;
    let mut shift = NOX_DECIMALS - places;
    while shift > 0 {
        v = v.saturating_mul(10);
        shift -= 1;
    }
    v
}
