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

//! Putting a match on the line.

use crate::term::search::Search;
use crate::term::state::State;

/// Show the newest match at or before where the search is looking.
pub(super) fn apply(state: &mut State, search: &mut Search) {
    place(state, search, search.at);
}

/// Show the match older than the one on screen.
pub(super) fn step_back(state: &mut State, search: &mut Search) {
    place(state, search, search.at.saturating_sub(1));
}

/// Find the newest match before `before` and put it on the line.
///
/// The line is copied out before it is written back, because the match
/// borrows the history and the write needs the state again.
fn place(state: &mut State, search: &mut Search, before: usize) {
    let found =
        state.history.search_back(&search.needle, before).map(|(at, line)| (at, line.to_vec()));
    let Some((at, line)) = found else {
        return;
    };
    // The next step starts just above this match, so pressing again moves.
    search.at = at + 1;
    state.line.replace(&line);
    state.line.move_end();
}
