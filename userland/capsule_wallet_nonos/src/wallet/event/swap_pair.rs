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

//! Choosing what is traded for what.

use crate::wallet::state::State;
use crate::wallet::swap::count;

/// Move to the next token on the paying side, stepping over the one already
/// on the other side so a reader cannot select a pair that is not a trade.
pub fn cycle_from(state: &mut State) -> bool {
    state.swap_from = next(state.swap_from, state.swap_to);
    clear_quote(state);
    true
}

/// Move to the next token on the receiving side.
pub fn cycle_to(state: &mut State) -> bool {
    state.swap_to = next(state.swap_to, state.swap_from);
    clear_quote(state);
    true
}

/// Trade the other way round.
///
/// The amount is not carried over. It was denominated in the token that is
/// now on the other side, and reading it as the new one would be a figure
/// the reader never typed.
pub fn flip(state: &mut State) -> bool {
    core::mem::swap(&mut state.swap_from, &mut state.swap_to);
    state.swap_in = 0;
    state.swap_digits = 0;
    state.swap_places = 0;
    state.swap_point = false;
    clear_quote(state);
    true
}

/// The next index that is not `avoid`.
fn next(from: u8, avoid: u8) -> u8 {
    let n = count();
    let mut i = (from + 1) % n;
    if i == avoid {
        i = (i + 1) % n;
    }
    i
}

/// A new pair is a new trade: re-read the pool, and ask for the allowance
/// again since it was granted against the token that just changed.
fn clear_quote(state: &mut State) {
    crate::wallet::event::swap_quote::refresh(state);
    state.swap_step = 0;
}
