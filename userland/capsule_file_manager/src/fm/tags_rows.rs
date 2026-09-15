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

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::layout::FOOTER_H;
use super::screen_list::Line;
use super::screen_row::LIST_ROW_H;
use super::state::State;
use super::tags_geom::chips_bottom;

/// The rows under the chip row: the paths carrying the active filter, clipped to
/// the same bottom the painter stops at. With no filter chosen there are no
/// rows, which is why the screen shows only the chips until one is picked.
pub fn tag_lines(state: &State) -> Vec<Line> {
    let bottom = state.win_h.saturating_sub(FOOTER_H);
    let mut y = chips_bottom(state);
    let mut out = Vec::new();
    if state.tag_filter.is_empty() {
        return out;
    }
    for path in state.tags.paths_with(state.tag_filter.as_str()) {
        if y + LIST_ROW_H > bottom {
            break;
        }
        out.push(Line::row(y, LIST_ROW_H, path, String::new(), path.ends_with('/')));
        y += LIST_ROW_H;
    }
    out
}
