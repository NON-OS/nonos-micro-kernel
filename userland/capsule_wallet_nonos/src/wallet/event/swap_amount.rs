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

//! Building the amount to pay, one keystroke at a time.

use crate::wallet::state::State;

/// Longest figure worth accepting. Past this the amount exceeds any supply
/// that exists and the arithmetic stops being meaningful.
const MAX_DIGITS: u32 = 30;

/// Append a digit to the amount.
///
/// The figure is held in the token's smallest unit, so a digit multiplies
/// what is there and adds. Kept saturating because a reader holding the key
/// down should reach a ceiling rather than wrap to nothing.
pub fn digit(state: &mut State, d: u8) -> bool {
    if state.swap_digits >= MAX_DIGITS {
        return true;
    }
    state.swap_in = state.swap_in.saturating_mul(10).saturating_add(d as u128);
    state.swap_digits += 1;
    if state.swap_point {
        state.swap_places += 1;
    }
    clear_quote(state);
    true
}

/// Start the fraction. A second point is ignored rather than refused,
/// because a reader who types one twice meant it once.
pub fn point(state: &mut State) -> bool {
    state.swap_point = true;
    true
}

/// Take back the last keystroke.
pub fn backspace(state: &mut State) -> bool {
    if state.swap_places > 0 {
        state.swap_places -= 1;
    } else if state.swap_point {
        state.swap_point = false;
        return true;
    }
    state.swap_in /= 10;
    state.swap_digits = state.swap_digits.saturating_sub(1);
    if state.swap_digits == 0 {
        state.swap_in = 0;
    }
    clear_quote(state);
    true
}

/// A quote belongs to the amount it was fetched for, so a changed amount
/// re-reads the pool rather than keeping the old figure.
fn clear_quote(state: &mut State) {
    crate::wallet::event::swap_quote::refresh(state);
}
