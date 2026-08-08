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

//! Taking the line the terminal is offering.

use crate::term::state::State;

/// Adopt the suggested line, if there is one and the cursor is at the end.
///
/// Bound to the same key that moves to the end of the line, which is where
/// every other shell puts it: a reader at the end of a line asking to go to
/// the end means they want what comes next. Anywhere else in the line it
/// still just moves.
///
/// Returns whether a suggestion was taken, so the caller can fall back to
/// moving rather than doing both.
pub fn accept_suggestion(state: &mut State) -> bool {
    let typed = state.line.as_bytes();
    if state.line.cursor != typed.len() || typed.is_empty() {
        return false;
    }
    let Some(full) = state.history.suggest(typed) else {
        return false;
    };
    // The borrow of history has to end before the line is written, and the
    // suggestion is only valid while it is held, so it is copied out first.
    let mut adopted = alloc::vec::Vec::with_capacity(full.len());
    adopted.extend_from_slice(full);
    state.line.replace(&adopted);
    state.line.move_end();
    true
}
