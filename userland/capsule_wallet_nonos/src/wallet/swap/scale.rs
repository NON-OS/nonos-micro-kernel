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

//! The typed figure as the chain counts it.

use crate::wallet::state::State;
use crate::wallet::swap::token;

/// Scale the amount to the paying token's decimals.
///
/// A reader types a figure in whole units with a fraction; a contract wants
/// the smallest unit. Places beyond what the token divides into are dropped
/// rather than rounded, because rounding up would spend more than was typed.
pub fn scaled(state: &State) -> u128 {
    let decimals = token(state.swap_from).decimals as u32;
    let places = state.swap_places.min(decimals);
    let mut v = state.swap_in;
    let mut shift = decimals - places;
    while shift > 0 {
        v = v.saturating_mul(10);
        shift -= 1;
    }
    v
}
