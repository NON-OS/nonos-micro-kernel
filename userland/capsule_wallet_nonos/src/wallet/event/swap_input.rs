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

//! Typing a trade.

use nonos_app_skeleton::EventOutcome;

use super::swap_amount::{backspace, digit, point};
use super::swap_pair::{cycle_from, cycle_to, flip};
use crate::wallet::state::State;

/// Keys the trading screen answers.
///
/// Editing anything clears the quote rather than leaving the old one on
/// screen. A figure that belongs to the previous amount is worse than no
/// figure at all, because it looks like an answer to the question just
/// asked.
pub fn swap_input(state: &mut State, code: u32) -> Option<EventOutcome> {
    let handled = match code {
        c if (b'0' as u32..=b'9' as u32).contains(&c) => digit(state, (c - b'0' as u32) as u8),
        c if c == b'.' as u32 => point(state),
        // Backspace and delete both mean the same thing to somebody
        // correcting a figure.
        0x08 | 0x7F => backspace(state),
        c if c == b'f' as u32 || c == b'F' as u32 => cycle_from(state),
        c if c == b't' as u32 || c == b'T' as u32 => cycle_to(state),
        c if c == b'o' as u32 || c == b'O' as u32 => flip(state),
        _ => false,
    };
    if handled {
        Some(EventOutcome::Repaint)
    } else {
        None
    }
}
