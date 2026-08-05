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

//! Changing what a running search is looking for.

use super::search_place::apply;
use crate::term::state::State;

/// Take a typed byte into the search.
pub fn search_type(state: &mut State, byte: u8) {
    edit(state, |needle| needle.push(byte));
}

/// Undo the last typed byte.
pub fn search_backspace(state: &mut State) {
    edit(state, |needle| {
        needle.pop();
    });
}

/// Change the needle and show what it now matches.
///
/// The search restarts from the newest line rather than continuing from
/// where it had walked to, so a reader who mistypes and corrects does not
/// have to walk back down to find what they meant.
fn edit(state: &mut State, change: impl FnOnce(&mut alloc::vec::Vec<u8>)) {
    let Some(mut search) = state.search.take() else {
        return;
    };
    change(&mut search.needle);
    search.at = state.history.count();
    apply(state, &mut search);
    state.search = Some(search);
}
