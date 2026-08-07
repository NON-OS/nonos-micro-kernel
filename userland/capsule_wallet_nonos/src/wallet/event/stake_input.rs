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

use nonos_app_skeleton::{EventOutcome, KEY_ENTER};

use super::stake_amount::{backspace, digit, point};
use super::stake_set::{clear, set_max};
use crate::wallet::state::State;

/// Keys the staking screen answers.
///
/// Returns `None` for anything it does not use, so every key that is not
/// part of typing an amount still reaches the view shortcuts.
pub fn stake_input(state: &mut State, code: u32) -> Option<EventOutcome> {
    // Closing a position takes no figure, so on that tab the digits choose
    // which position rather than typing an amount that has nowhere to go.
    if state.stake_unstake == 1 {
        return unstake_input(state, code);
    }
    let handled = match code {
        c if (b'0' as u32..=b'9' as u32).contains(&c) => digit(state, (c - b'0' as u32) as u8),
        c if c == b'.' as u32 => point(state),
        // Backspace and delete both mean the same thing to somebody
        // correcting a figure.
        0x08 | 0x7F => backspace(state),
        c if c == KEY_ENTER => return Some(super::stake_flow::stake_flow(state)),
        c if c == b'a' as u32 || c == b'A' as u32 => set_max(state),
        c if c == b'c' as u32 || c == b'C' as u32 => clear(state),
        _ => false,
    };
    if handled {
        Some(EventOutcome::Repaint)
    } else {
        None
    }
}

/// Keys the Unstake tab answers: a digit picks the position to close, Enter
/// closes it. Positions are numbered as the contract numbers them, from zero,
/// so what is typed is what is signed.
fn unstake_input(state: &mut State, code: u32) -> Option<EventOutcome> {
    if code == KEY_ENTER {
        return Some(super::stake_flow::stake_flow(state));
    }
    let d = code.checked_sub(b'0' as u32)?;
    if d >= 10 {
        return None;
    }
    state.stake_position = d as u64;
    Some(EventOutcome::Repaint)
}
