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

use crate::settings::paint::visible_rows::visible_rows;

use super::state::State;

pub fn track_scroll(state: &mut State) {
    let i = state.category as usize;
    let cursor = state.cursor[i];
    let top = state.scroll_top[i];
    let rows = visible_rows(state.win_h);
    if rows == 0 {
        state.scroll_top[i] = cursor;
        return;
    }
    if cursor < top {
        state.scroll_top[i] = cursor;
        return;
    }
    if cursor >= top + rows {
        state.scroll_top[i] = cursor + 1 - rows;
    }
}
