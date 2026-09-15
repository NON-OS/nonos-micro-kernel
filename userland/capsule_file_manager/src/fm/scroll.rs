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

use super::row_geom::visible_rows;
use super::state::State;

// Slide the listing window so the cursor stays on screen: page up when the
// cursor moves above the top row, page down when it falls past the last
// visible row. `visible_rows` is the count the slot lists were actually built
// from, so scrolling matches whatever is on screen rather than the raw
// `view_rows` estimate.
pub fn ensure_visible(state: &mut State) {
    let vis = visible_rows(state);
    if state.cursor < state.scroll {
        state.scroll = state.cursor;
    } else if state.cursor >= state.scroll + vis {
        state.scroll = state.cursor + 1 - vis;
    }
}
