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

//! Searching back through what has been run.

use alloc::vec::Vec;

use super::search_place::{apply, step_back};
use crate::term::search::Search;
use crate::term::state::State;

/// Begin a search, or step to the next older match if one is running.
///
/// The same key does both, which is what every shell binds it to: the reader
/// presses it again because the match shown is not the one they meant.
pub fn search_step(state: &mut State) {
    match state.search.take() {
        None => {
            let saved = state.line.as_bytes().to_vec();
            let count = state.history.count();
            let mut search = Search::new(saved, count);
            apply(state, &mut search);
            state.search = Some(search);
        }
        Some(mut search) => {
            // Step past the match on screen so the key moves rather than
            // showing the same line again.
            step_back(state, &mut search);
            state.search = Some(search);
        }
    }
}

/// Leave the search, keeping what it found.
pub fn search_accept(state: &mut State) {
    state.search = None;
}

/// Leave the search, restoring the line as it was.
pub fn search_cancel(state: &mut State) {
    if let Some(search) = state.search.take() {
        let saved: Vec<u8> = search.saved;
        state.line.replace(&saved);
        state.line.move_end();
    }
}
