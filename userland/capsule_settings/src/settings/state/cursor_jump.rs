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

//! Jumping the cursor a page or the whole list at a time. Arrow keys alone
//! meant eighteen presses to cross the Security tab.

use crate::settings::paint::visible_rows::visible_rows;

use super::focused_count::focused_count;
use super::state::State;
use super::track_scroll::track_scroll;

/// Move the cursor to the first row.
pub fn cursor_home(state: &mut State) {
    let i = state.category as usize;
    state.cursor[i] = 0;
    track_scroll(state);
}

/// Move the cursor to the last row.
pub fn cursor_end(state: &mut State) {
    let i = state.category as usize;
    let n = focused_count(state.category);
    state.cursor[i] = n.saturating_sub(1);
    track_scroll(state);
}

/// Move the cursor one visible page up or down. Clamps rather than wrapping,
/// which is what the arrows do, because a page jump that wrapped would lose
/// the user's place in a long list.
pub fn cursor_page(state: &mut State, down: bool) {
    let i = state.category as usize;
    let n = focused_count(state.category);
    if n == 0 {
        return;
    }
    let page = visible_rows(state.win_h).max(1);
    let cursor = state.cursor[i];
    state.cursor[i] = if down {
        core::cmp::min(cursor.saturating_add(page), n - 1)
    } else {
        cursor.saturating_sub(page)
    };
    track_scroll(state);
}
